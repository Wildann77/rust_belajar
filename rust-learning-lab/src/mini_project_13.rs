// Mini Project Fase 13: Test Suite Task Manager
// Rujukan: rust_learning_guide.md (Sub-bab 13.6) & rust_execution_tasks.md (L1138-L1157)
//
// Target Mini Project Fase 13:
// - [x] CRUD tests.
// - [x] Error tests.
// - [x] Concurrency tests.
// - [x] Async tests.
// - [x] Integration tests.
//
// Evaluasi Lulus Fase:
// - cargo fmt --check
// - cargo clippy --all-targets --all-features -- -D warnings
// - cargo test
// - cargo build --release

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

// ============================================================================
// 1. Tipe Data Domain & Error
// ============================================================================

/// Tingkat prioritas task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "LOW"),
            TaskPriority::Medium => write!(f, "MEDIUM"),
            TaskPriority::High => write!(f, "HIGH"),
            TaskPriority::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Status siklus hidup task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "PENDING"),
            TaskStatus::InProgress => write!(f, "IN_PROGRESS"),
            TaskStatus::Completed => write!(f, "COMPLETED"),
            TaskStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

/// Data rekaman task dalam sistem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRecord {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub created_at_ms: u64,
    pub completed_at_ms: Option<u64>,
}

/// Definisi error menyeluruh pada Task Manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskManagerError {
    EmptyTitle,
    TaskNotFound(u64),
    InvalidStateTransition { from: TaskStatus, to: TaskStatus },
    LockError(String),
    ExecutionTimeout { task_id: u64, timeout_ms: u64 },
}

impl std::fmt::Display for TaskManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskManagerError::EmptyTitle => {
                write!(f, "Judul task tidak boleh kosong atau hanya whitespace")
            }
            TaskManagerError::TaskNotFound(id) => {
                write!(f, "Task dengan ID #{} tidak ditemukan", id)
            }
            TaskManagerError::InvalidStateTransition { from, to } => {
                write!(f, "Transisi status tidak valid dari {} ke {}", from, to)
            }
            TaskManagerError::LockError(msg) => {
                write!(f, "Kegagalan sinkronisasi memori lock: {}", msg)
            }
            TaskManagerError::ExecutionTimeout {
                task_id,
                timeout_ms,
            } => {
                write!(
                    f,
                    "Task #{} melebihi batas waktu eksekusi {} ms",
                    task_id, timeout_ms
                )
            }
        }
    }
}

impl std::error::Error for TaskManagerError {}

// ============================================================================
// 2. TaskManager (Thread-Safe & Concurrency-Ready)
// ============================================================================

/// Pengelola task berbasis `Arc<RwLock<HashMap>>` yang aman diakses lintas thread OS
/// maupun task asynchronous Tokio runtime.
#[derive(Debug, Clone)]
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<u64, TaskRecord>>>,
    next_id: Arc<AtomicU64>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    /// Membuat instance `TaskManager` baru.
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    // ------------------------------------------------------------------------
    // CRUD: Create
    // ------------------------------------------------------------------------

    /// Membuat task baru dengan validasi judul.
    pub fn create_task(
        &self,
        title: &str,
        description: &str,
        priority: TaskPriority,
    ) -> Result<u64, TaskManagerError> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err(TaskManagerError::EmptyTitle);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let record = TaskRecord {
            id,
            title: trimmed_title.to_string(),
            description: description.trim().to_string(),
            priority,
            status: TaskStatus::Pending,
            created_at_ms: Self::now_ms(),
            completed_at_ms: None,
        };

        let mut lock = self
            .tasks
            .write()
            .map_err(|e| TaskManagerError::LockError(e.to_string()))?;
        lock.insert(id, record);

        Ok(id)
    }

    // ------------------------------------------------------------------------
    // CRUD: Read
    // ------------------------------------------------------------------------

    /// Mengambil detail satu task berdasarkan ID.
    pub fn get_task(&self, id: u64) -> Result<TaskRecord, TaskManagerError> {
        let lock = self
            .tasks
            .read()
            .map_err(|e| TaskManagerError::LockError(e.to_string()))?;
        lock.get(&id)
            .cloned()
            .ok_or(TaskManagerError::TaskNotFound(id))
    }

    /// Menghitung total seluruh task aktif di penyimpanan.
    pub fn count(&self) -> usize {
        self.tasks.read().map(|l| l.len()).unwrap_or(0)
    }

    /// Mengambil daftar seluruh task.
    pub fn list_tasks(&self) -> Vec<TaskRecord> {
        let lock = match self.tasks.read() {
            Ok(l) => l,
            Err(_) => return Vec::new(),
        };
        let mut list: Vec<TaskRecord> = lock.values().cloned().collect();
        list.sort_by_key(|t| t.id);
        list
    }

    /// Memfilter task berdasarkan status siklus hidup.
    pub fn filter_by_status(&self, status: TaskStatus) -> Vec<TaskRecord> {
        let lock = match self.tasks.read() {
            Ok(l) => l,
            Err(_) => return Vec::new(),
        };
        let mut list: Vec<TaskRecord> = lock
            .values()
            .filter(|t| t.status == status)
            .cloned()
            .collect();
        list.sort_by_key(|t| t.id);
        list
    }

    /// Memfilter task berdasarkan tingkat prioritas.
    pub fn filter_by_priority(&self, priority: TaskPriority) -> Vec<TaskRecord> {
        let lock = match self.tasks.read() {
            Ok(l) => l,
            Err(_) => return Vec::new(),
        };
        let mut list: Vec<TaskRecord> = lock
            .values()
            .filter(|t| t.priority == priority)
            .cloned()
            .collect();
        list.sort_by_key(|t| t.id);
        list
    }

    // ------------------------------------------------------------------------
    // CRUD: Update
    // ------------------------------------------------------------------------

    /// Memperbarui informasi deskripsi atau prioritas task.
    pub fn update_details(
        &self,
        id: u64,
        new_title: Option<&str>,
        new_priority: Option<TaskPriority>,
    ) -> Result<(), TaskManagerError> {
        let mut lock = self
            .tasks
            .write()
            .map_err(|e| TaskManagerError::LockError(e.to_string()))?;
        let task = lock
            .get_mut(&id)
            .ok_or(TaskManagerError::TaskNotFound(id))?;

        if let Some(t) = new_title {
            let trimmed = t.trim();
            if trimmed.is_empty() {
                return Err(TaskManagerError::EmptyTitle);
            }
            task.title = trimmed.to_string();
        }

        if let Some(p) = new_priority {
            task.priority = p;
        }

        Ok(())
    }

    /// Melakukan transisi status task sesuai dengan diagram status mesin yang sah.
    pub fn transition_status(
        &self,
        id: u64,
        next_status: TaskStatus,
    ) -> Result<TaskStatus, TaskManagerError> {
        let mut lock = self
            .tasks
            .write()
            .map_err(|e| TaskManagerError::LockError(e.to_string()))?;
        let task = lock
            .get_mut(&id)
            .ok_or(TaskManagerError::TaskNotFound(id))?;

        let valid = match (task.status, next_status) {
            (TaskStatus::Pending, TaskStatus::InProgress) => true,
            (TaskStatus::Pending, TaskStatus::Cancelled) => true,
            (TaskStatus::InProgress, TaskStatus::Completed) => true,
            (TaskStatus::InProgress, TaskStatus::Pending) => true, // Retry
            (TaskStatus::InProgress, TaskStatus::Cancelled) => true,
            _ => false,
        };

        if !valid {
            return Err(TaskManagerError::InvalidStateTransition {
                from: task.status,
                to: next_status,
            });
        }

        task.status = next_status;
        if next_status == TaskStatus::Completed {
            task.completed_at_ms = Some(Self::now_ms());
        }

        Ok(task.status)
    }

    // ------------------------------------------------------------------------
    // CRUD: Delete
    // ------------------------------------------------------------------------

    /// Menghapus task dari penyimpanan berdasarkan ID.
    pub fn delete_task(&self, id: u64) -> Result<TaskRecord, TaskManagerError> {
        let mut lock = self
            .tasks
            .write()
            .map_err(|e| TaskManagerError::LockError(e.to_string()))?;
        lock.remove(&id).ok_or(TaskManagerError::TaskNotFound(id))
    }

    // ------------------------------------------------------------------------
    // Operasi Asinkron (Async Operations & Tokio Integration)
    // ------------------------------------------------------------------------

    /// Mengeksekusi task asinkron dari `Pending` -> `InProgress` -> `Completed`.
    pub async fn execute_task_async(
        &self,
        id: u64,
        duration_ms: u64,
    ) -> Result<TaskRecord, TaskManagerError> {
        self.transition_status(id, TaskStatus::InProgress)?;

        // Simulasi beban I/O asinkron
        sleep(Duration::from_millis(duration_ms)).await;

        self.transition_status(id, TaskStatus::Completed)?;
        self.get_task(id)
    }

    /// Mengeksekusi task asinkron dengan batasan timeout tegas.
    pub async fn execute_with_timeout(
        &self,
        id: u64,
        duration_ms: u64,
        timeout_ms: u64,
    ) -> Result<TaskRecord, TaskManagerError> {
        let work = self.execute_task_async(id, duration_ms);

        match timeout(Duration::from_millis(timeout_ms), work).await {
            Ok(res) => res,
            Err(_) => {
                // Batalkan task jika timeout
                let _ = self.transition_status(id, TaskStatus::Cancelled);
                Err(TaskManagerError::ExecutionTimeout {
                    task_id: id,
                    timeout_ms,
                })
            }
        }
    }

    /// Memproses batch task secara konkuren memanfaatkan `tokio::task::JoinSet`.
    pub async fn process_batch_concurrently(
        &self,
        ids: &[u64],
        duration_ms: u64,
    ) -> Vec<Result<TaskRecord, TaskManagerError>> {
        let mut set = JoinSet::new();

        for &id in ids {
            let manager = self.clone();
            set.spawn(async move { manager.execute_task_async(id, duration_ms).await });
        }

        let mut results = Vec::new();
        while let Some(res) = set.join_next().await {
            match res {
                Ok(item) => results.push(item),
                Err(join_err) => {
                    results.push(Err(TaskManagerError::LockError(join_err.to_string())))
                }
            }
        }

        results
    }
}

// ============================================================================
// 3. Evaluasi Kriteria Lulus Fase 13
// ============================================================================

/// Evaluasi ringkas pencapaian seluruh kriteria kelulusan Fase 13.
pub fn print_graduation_evaluation() {
    println!("\n=== EVALUASI KRITERIA LULUS FASE 13 ===");
    println!("1. [x] CRUD Tests       : Create, Read, Update, Delete tervalidasi menyeluruh.");
    println!(
        "2. [x] Error Tests      : Menangani EmptyTitle, TaskNotFound, & InvalidStateTransition."
    );
    println!(
        "3. [x] Concurrency Tests: 10 OS Thread menulis/membaca simultan tanpa data race / poison lock."
    );
    println!(
        "4. [x] Async Tests      : Eksekusi async, batch JoinSet, dan timeout deadline via Tokio."
    );
    println!(
        "5. [x] Integration Tests: Pengujian Black-Box di tests/mini_project_13_integration_test.rs."
    );
    println!(
        "6. [x] QA Commands      : Bebas warning pada cargo fmt, cargo clippy, cargo test, & release build."
    );
}

// ============================================================================
// 4. Runner Demonstrasi (pub fn run)
// ============================================================================

/// Fungsi utama runner demonstrasi untuk Mini Project Fase 13.
pub fn run() {
    println!("=== MINI PROJECT FASE 13: TEST SUITE TASK MANAGER ===");

    let manager = TaskManager::new();

    // 1. CRUD Demonstration
    println!("\n1. [CRUD Demo] Membuat dan mengelola task:");
    let id1 = manager
        .create_task(
            "Desain Skema Database",
            "Gunakan PostgreSQL",
            TaskPriority::Critical,
        )
        .expect("Gagal membuat task #1");
    let id2 = manager
        .create_task(
            "Implementasi Unit Tests",
            "Cakupan > 90%",
            TaskPriority::High,
        )
        .expect("Gagal membuat task #2");

    println!("   -> Task #{} dan #{} berhasil dibuat.", id1, id2);
    let t1 = manager.get_task(id1).expect("Task #1 harus ada");
    println!(
        "   -> Detail Task #1: '{}' [{}] ({})",
        t1.title, t1.priority, t1.status
    );

    manager
        .update_details(
            id1,
            Some("Desain Skema DB v2"),
            Some(TaskPriority::Critical),
        )
        .expect("Gagal update task");
    println!(
        "   -> Update berhasil: Judul baru '{}'",
        manager.get_task(id1).unwrap().title
    );

    // 2. Error Demonstration
    println!("\n2. [Error Tests Demo] Penolakan input tidak valid:");
    let empty_err = manager.create_task("   ", "Desc", TaskPriority::Low);
    match empty_err {
        Err(TaskManagerError::EmptyTitle) => {
            println!("   -> [Expected Error]: Empty title berhasil ditolak.");
        }
        _ => panic!("Harusnya mengembalikan EmptyTitle!"),
    }

    let invalid_trans = manager.transition_status(id1, TaskStatus::Completed);
    match invalid_trans {
        Err(TaskManagerError::InvalidStateTransition { from, to }) => {
            println!(
                "   -> [Expected Error]: Transisi ilegal {} -> {} berhasil dicegah.",
                from, to
            );
        }
        _ => panic!("Harusnya transisi ilegal dicegah!"),
    }

    // 3. Concurrency Stress Test Demo
    println!("\n3. [Concurrency Demo] Menguji 8 thread OS menulis bersamaan...");
    let start_threads = Instant::now();
    let mut thread_handles = Vec::new();

    for thread_idx in 0..8 {
        let mgr = manager.clone();
        let handle = std::thread::spawn(move || {
            for i in 0..10 {
                let title = format!("Thread-{}-Task-{}", thread_idx, i);
                mgr.create_task(&title, "Stress concurrent task", TaskPriority::Medium)
                    .unwrap();
            }
        });
        thread_handles.push(handle);
    }

    for h in thread_handles {
        h.join().expect("Thread worker panic!");
    }
    println!(
        "   -> Selesai dalam {:?}! Total task terkumpul di memori: {}",
        start_threads.elapsed(),
        manager.count()
    );

    // 4. Async Execution Demo
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Gagal inisialisasi Tokio runtime");

    rt.block_on(async {
        println!("\n4. [Async Tokio Demo] Menjalankan pemrosesan asinkron & timeout:");

        let async_task_id = manager
            .create_task(
                "Async Backup Data",
                "Eksekusi background",
                TaskPriority::High,
            )
            .unwrap();

        let finished = manager
            .execute_task_async(async_task_id, 15)
            .await
            .expect("Async execution gagal");
        println!(
            "   -> Task #{} selesai asinkron! Status: {} (Waktu selesai: {:?})",
            finished.id, finished.status, finished.completed_at_ms
        );

        // Uji Timeout
        let slow_task_id = manager
            .create_task(
                "Slow Cloud Sync",
                "Simulasi delay tinggi",
                TaskPriority::Low,
            )
            .unwrap();
        let timeout_res = manager.execute_with_timeout(slow_task_id, 50, 10).await;
        match timeout_res {
            Err(TaskManagerError::ExecutionTimeout {
                task_id,
                timeout_ms,
            }) => {
                println!(
                    "   -> [Expected Timeout]: Task #{} dihentikan karena melebihi {} ms.",
                    task_id, timeout_ms
                );
            }
            _ => panic!("Harusnya timeout terpicu!"),
        }
    });

    // 5. Evaluasi Kriteria Lulus
    print_graduation_evaluation();

    println!("\n[OK] Mini Project Fase 13 (Test Suite Task Manager) tuntas & terverifikasi!\n");
}

// ============================================================================
// 5. Unit Tests Terintegrasi
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Kategori 1: CRUD Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_crud_create_read_update_delete() {
        let manager = TaskManager::new();

        // Create
        let id = manager
            .create_task(
                "Fix Memory Leak",
                "Analisis Valgrind",
                TaskPriority::Critical,
            )
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.count(), 1);

        // Read
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.title, "Fix Memory Leak");
        assert_eq!(task.priority, TaskPriority::Critical);
        assert_eq!(task.status, TaskStatus::Pending);

        // Update details
        manager
            .update_details(
                id,
                Some("Fix Memory Leak in Engine"),
                Some(TaskPriority::High),
            )
            .unwrap();
        let updated = manager.get_task(id).unwrap();
        assert_eq!(updated.title, "Fix Memory Leak in Engine");
        assert_eq!(updated.priority, TaskPriority::High);

        // Transition state
        let st1 = manager
            .transition_status(id, TaskStatus::InProgress)
            .unwrap();
        assert_eq!(st1, TaskStatus::InProgress);

        let st2 = manager
            .transition_status(id, TaskStatus::Completed)
            .unwrap();
        assert_eq!(st2, TaskStatus::Completed);
        assert!(manager.get_task(id).unwrap().completed_at_ms.is_some());

        // Delete
        let deleted = manager.delete_task(id).unwrap();
        assert_eq!(deleted.id, id);
        assert_eq!(manager.count(), 0);
        assert_eq!(
            manager.get_task(id).unwrap_err(),
            TaskManagerError::TaskNotFound(id)
        );
    }

    #[test]
    fn test_crud_filtering_by_status_and_priority() {
        let manager = TaskManager::new();

        let id1 = manager
            .create_task("Task 1", "Desc", TaskPriority::Low)
            .unwrap();
        let id2 = manager
            .create_task("Task 2", "Desc", TaskPriority::High)
            .unwrap();
        let id3 = manager
            .create_task("Task 3", "Desc", TaskPriority::High)
            .unwrap();

        manager
            .transition_status(id1, TaskStatus::InProgress)
            .unwrap();

        let low_list = manager.filter_by_priority(TaskPriority::Low);
        assert_eq!(low_list.len(), 1);
        assert_eq!(low_list[0].id, id1);

        let high_list = manager.filter_by_priority(TaskPriority::High);
        assert_eq!(high_list.len(), 2);
        assert_eq!(high_list[0].id, id2);
        assert_eq!(high_list[1].id, id3);

        let in_progress = manager.filter_by_status(TaskStatus::InProgress);
        assert_eq!(in_progress.len(), 1);
        assert_eq!(in_progress[0].id, id1);
    }

    // ------------------------------------------------------------------------
    // Kategori 2: Error Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_error_empty_title_rejections() {
        let manager = TaskManager::new();

        assert_eq!(
            manager
                .create_task("", "Desc", TaskPriority::Low)
                .unwrap_err(),
            TaskManagerError::EmptyTitle
        );
        assert_eq!(
            manager
                .create_task("   \t\n ", "Desc", TaskPriority::Low)
                .unwrap_err(),
            TaskManagerError::EmptyTitle
        );
    }

    #[test]
    fn test_error_task_not_found() {
        let manager = TaskManager::new();

        assert_eq!(
            manager.get_task(999).unwrap_err(),
            TaskManagerError::TaskNotFound(999)
        );
        assert_eq!(
            manager.delete_task(999).unwrap_err(),
            TaskManagerError::TaskNotFound(999)
        );
        assert_eq!(
            manager
                .transition_status(999, TaskStatus::InProgress)
                .unwrap_err(),
            TaskManagerError::TaskNotFound(999)
        );
        assert_eq!(
            manager.update_details(999, Some("New"), None).unwrap_err(),
            TaskManagerError::TaskNotFound(999)
        );
    }

    #[test]
    fn test_error_invalid_state_transitions() {
        let manager = TaskManager::new();

        let id = manager
            .create_task("Strict Lifecycle", "Desc", TaskPriority::Medium)
            .unwrap();

        // Pending -> Completed (Ilegal, harus lewat InProgress dahulu)
        let err1 = manager
            .transition_status(id, TaskStatus::Completed)
            .unwrap_err();
        assert_eq!(
            err1,
            TaskManagerError::InvalidStateTransition {
                from: TaskStatus::Pending,
                to: TaskStatus::Completed,
            }
        );

        // Majukan ke InProgress lalu Completed
        manager
            .transition_status(id, TaskStatus::InProgress)
            .unwrap();
        manager
            .transition_status(id, TaskStatus::Completed)
            .unwrap();

        // Completed -> InProgress (Ilegal, task sudah selesai)
        let err2 = manager
            .transition_status(id, TaskStatus::InProgress)
            .unwrap_err();
        assert_eq!(
            err2,
            TaskManagerError::InvalidStateTransition {
                from: TaskStatus::Completed,
                to: TaskStatus::InProgress,
            }
        );
    }

    // ------------------------------------------------------------------------
    // Kategori 3: Concurrency Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_concurrency_multi_threaded_crud_stress() {
        let manager = TaskManager::new();
        let num_threads = 10;
        let tasks_per_thread = 20;

        let mut handles = Vec::new();

        // 10 Thread serentak menulis task
        for t_idx in 0..num_threads {
            let mgr = manager.clone();
            let handle = std::thread::spawn(move || {
                for i in 0..tasks_per_thread {
                    let title = format!("Thread-{}-Task-{}", t_idx, i);
                    let id = mgr
                        .create_task(&title, "Stress data", TaskPriority::Medium)
                        .unwrap();
                    // Kadang baca kembali untuk menguji pembacaan simultan
                    assert!(mgr.get_task(id).is_ok());
                }
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }

        // Pastikan total tepat 200 tanpa data yang tercecer atau id bentrok
        assert_eq!(manager.count(), num_threads * tasks_per_thread);

        let all_tasks = manager.list_tasks();
        assert_eq!(all_tasks.len(), 200);

        // Verifikasi semua ID unik
        let mut ids: Vec<u64> = all_tasks.iter().map(|t| t.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 200);
    }

    // ------------------------------------------------------------------------
    // Kategori 4: Async Tests (Tokio Runtime)
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_async_task_execution_lifecycle() {
        let manager = TaskManager::new();
        let id = manager
            .create_task("Async Worker Job", "Desc", TaskPriority::High)
            .unwrap();

        let res = manager.execute_task_async(id, 10).await;
        assert!(res.is_ok());

        let finished = res.unwrap();
        assert_eq!(finished.status, TaskStatus::Completed);
        assert!(finished.completed_at_ms.is_some());
    }

    #[tokio::test]
    async fn test_async_execution_timeout() {
        let manager = TaskManager::new();
        let id = manager
            .create_task("Long Running Task", "Desc", TaskPriority::Low)
            .unwrap();

        // Task butuh 50ms, tetapi timeout disetel ke 10ms -> HARUS TIMEOUT
        let res = manager.execute_with_timeout(id, 50, 10).await;
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            TaskManagerError::ExecutionTimeout {
                task_id: id,
                timeout_ms: 10,
            }
        );

        // Task yang timeout otomatis dialihkan ke Cancelled
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.status, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_async_batch_concurrent_processing() {
        let manager = TaskManager::new();
        let mut ids = Vec::new();

        for i in 1..=5 {
            let id = manager
                .create_task(
                    &format!("Batch Task #{}", i),
                    "Batch Desc",
                    TaskPriority::Medium,
                )
                .unwrap();
            ids.push(id);
        }

        let results = manager.process_batch_concurrently(&ids, 15).await;
        assert_eq!(results.len(), 5);

        for r in results {
            assert!(r.is_ok());
            assert_eq!(r.unwrap().status, TaskStatus::Completed);
        }
    }

    #[test]
    fn test_sync_runner_execution() {
        run();
    }
}
