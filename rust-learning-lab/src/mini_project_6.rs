// Mini Project Fase 6: CLI Task Manager v1 & Error Handling Integration
// Rujukan: rust_learning_guide.md (FASE 6) & rust_execution_tasks.md (L606-L623)

use std::collections::HashMap;
use std::fmt;

/// Representasi status error domain untuk CLI Task Manager
#[derive(Debug, PartialEq, Eq)]
pub enum TaskManagerError {
    TaskNotFound(u32),
    EmptyTitle,
    InvalidId(String),
}

impl fmt::Display for TaskManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskManagerError::TaskNotFound(id) => write!(f, "Task dengan ID #{id} tidak ditemukan"),
            TaskManagerError::EmptyTitle => write!(f, "Judul task tidak boleh kosong"),
            TaskManagerError::InvalidId(raw) => {
                write!(
                    f,
                    "ID task '{raw}' tidak valid (harus berupa bilangan bulat positif)"
                )
            }
        }
    }
}

impl std::error::Error for TaskManagerError {}

/// Model data task dalam sistem
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskItem {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub description: Option<String>,
}

/// Service pengelola task dengan memori Vec dan HashMap
pub struct TaskManager {
    tasks: Vec<TaskItem>,
    next_id: u32,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    /// Inisialisasi TaskManager dengan kapasitas awal
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(16),
            next_id: 1,
        }
    }

    /// 1. Fitur: Add task
    /// Memvalidasi input dan mengembalikan Result<u32, TaskManagerError> tanpa unwrap
    pub fn add_task(
        &mut self,
        title: &str,
        category: &str,
        description: Option<&str>,
    ) -> Result<u32, TaskManagerError> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err(TaskManagerError::EmptyTitle);
        }

        let id = self.next_id;
        self.next_id += 1;

        let task = TaskItem {
            id,
            title: trimmed_title.to_string(),
            category: if category.trim().is_empty() {
                "General".to_string()
            } else {
                category.trim().to_string()
            },
            description: description.map(|d| d.trim().to_string()),
        };

        self.tasks.push(task);
        Ok(id)
    }

    /// 2. Fitur: List task
    pub fn list_tasks(&self) -> &[TaskItem] {
        &self.tasks
    }

    /// 3. Fitur: Find task
    /// Mengembalikan Result<&TaskItem, TaskManagerError> jika tidak ditemukan
    pub fn find_task(&self, id: u32) -> Result<&TaskItem, TaskManagerError> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or(TaskManagerError::TaskNotFound(id))
    }

    /// 4. Fitur: Delete task
    /// Menghapus task via in-place retain dan mengembalikan task yang dihapus
    pub fn delete_task(&mut self, id: u32) -> Result<TaskItem, TaskManagerError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(TaskManagerError::TaskNotFound(id))?;

        Ok(self.tasks.remove(index))
    }

    /// 5. Fitur: Agregasi Kategori menggunakan HashMap Entry API
    pub fn get_category_stats(&self) -> HashMap<String, usize> {
        let mut stats: HashMap<String, usize> = HashMap::new();
        for task in &self.tasks {
            stats
                .entry(task.category.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        stats
    }

    /// 6. Jalur Input Utama: Parsing ID dari string tanpa pernah menggunakan `unwrap()`
    pub fn parse_and_find(&self, raw_id: &str) -> Result<&TaskItem, TaskManagerError> {
        let parsed_id = raw_id
            .trim()
            .parse::<u32>()
            .map_err(|_| TaskManagerError::InvalidId(raw_id.trim().to_string()))?;

        // Menggunakan operator ? untuk propagasi error jika task tidak ditemukan
        self.find_task(parsed_id)
    }

    /// Menjalankan instruksi berbasis perintah teks secara terproteksi
    pub fn execute_command(&mut self, command_line: &str) -> Result<String, TaskManagerError> {
        let mut parts = command_line.split_whitespace();
        let action = parts.next().unwrap_or("");

        match action {
            "ADD" => {
                let title = parts.next().ok_or(TaskManagerError::EmptyTitle)?;
                let category = parts.next().unwrap_or("General");
                let id = self.add_task(title, category, None)?;
                Ok(format!("Task #{id} ('{title}') berhasil ditambahkan"))
            }
            "FIND" => {
                let id_str = parts
                    .next()
                    .ok_or(TaskManagerError::InvalidId("".to_string()))?;
                let task = self.parse_and_find(id_str)?;
                Ok(format!(
                    "Ditemukan: [#{}] {} ({})",
                    task.id, task.title, task.category
                ))
            }
            "DEL" => {
                let id_str = parts
                    .next()
                    .ok_or(TaskManagerError::InvalidId("".to_string()))?;
                let parsed_id = id_str
                    .parse::<u32>()
                    .map_err(|_| TaskManagerError::InvalidId(id_str.to_string()))?;
                let deleted = self.delete_task(parsed_id)?;
                Ok(format!(
                    "Task #{} ('{}') berhasil dihapus",
                    deleted.id, deleted.title
                ))
            }
            _ => Err(TaskManagerError::EmptyTitle),
        }
    }
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Mini Project Fase 6
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Mini Project Fase 6: CLI Task Manager v1 & Error Safe ===");
    println!("============================================================");

    let mut manager = TaskManager::new();

    // 1. Add Task Demo
    println!("\n1. Menambahkan Task (dengan Option description):");
    let t1 = manager.add_task(
        "Implementasi JWT Authentication",
        "Backend",
        Some("Gunakan RS256 algorithm dan refresh token rotation"),
    );
    let t2 = manager.add_task("Desain Database Schema", "Database", None);
    let t3 = manager.add_task("Buat Unit Test Axum Handlers", "Testing", None);
    let t4 = manager.add_task(
        "Setup CI/CD Pipeline",
        "DevOps",
        Some("GitHub Actions workflow"),
    );

    println!("   [+] Task #{} dibuat", t1.as_ref().unwrap());
    println!("   [+] Task #{} dibuat", t2.as_ref().unwrap());
    println!("   [+] Task #{} dibuat", t3.as_ref().unwrap());
    println!("   [+] Task #{} dibuat", t4.as_ref().unwrap());

    // 2. List Tasks Demo
    println!("\n2. Daftar Task Aktif Saat Ini:");
    for task in manager.list_tasks() {
        let desc_display = task.description.as_deref().unwrap_or("(Tanpa deskripsi)");
        println!(
            "   - [#{:<2}] [{:<10}] {:<32} -> {}",
            task.id, task.category, task.title, desc_display
        );
    }

    // 3. Find Task (Sukses vs Error Tidak Ditemukan)
    println!("\n3. Pencarian Task (Find Task):");
    match manager.find_task(1) {
        Ok(t) => println!("   [✓] Ditemukan: [#{}] {}", t.id, t.title),
        Err(e) => println!("   [✗] Error: {e}"),
    }

    match manager.find_task(999) {
        Ok(t) => println!("   [✓] Ditemukan: [#{}] {}", t.id, t.title),
        Err(e) => println!("   [✓ Diharapkan] Error ketika task tidak ditemukan: '{e}'"),
    }

    // 4. Safe Parsing Jalur Input Utama (Tanpa unwrap)
    println!("\n4. Proteksi Jalur Input Utama (Bebas unwrap!):");
    let test_inputs = ["2", "bukan_angka", "888"];
    for input in test_inputs {
        match manager.parse_and_find(input) {
            Ok(t) => println!("   - Input '{input}' -> Ditemukan: '{}'", t.title),
            Err(e) => println!("   - Input '{input}' -> Ditangani Aman: '{e}'"),
        }
    }

    // 5. Delete Task
    println!("\n5. Menghapus Task (Delete Task):");
    match manager.delete_task(2) {
        Ok(deleted) => println!(
            "   [✓] Berhasil menghapus task #{}: '{}'",
            deleted.id, deleted.title
        ),
        Err(e) => println!("   [✗] Gagal menghapus: {e}"),
    }

    match manager.delete_task(2) {
        Ok(_) => println!("   [✗] Seharusnya gagal"),
        Err(e) => println!("   [✓ Diharapkan] Menghapus task yang sudah terhapus: '{e}'"),
    }

    // 6. Kategori Statistik via HashMap Entry API
    println!("\n6. Statistik Kategori via HashMap Entry API:");
    for (cat, count) in manager.get_category_stats() {
        println!("   - Kategori '{cat}': {count} task aktif");
    }

    // 7. Eksekusi Command Parser CLI (execute_command)
    println!("\n7. Eksekusi Perintah Parser CLI (ADD / FIND / DEL):");
    let cmd_add = manager.execute_command("ADD RefactorCode Backend");
    println!("   - Command ADD  -> {:?}", cmd_add);
    let cmd_find = manager.execute_command("FIND 5");
    println!("   - Command FIND -> {:?}", cmd_find);
    let cmd_del = manager.execute_command("DEL 5");
    println!("   - Command DEL  -> {:?}", cmd_del);

    // 8. Ringkasan Kriteria Lulus Fase 6
    println!("\n8. Evaluasi Kriteria Lulus Fase 6:");
    println!(
        "   [x] Kapan Option vs Result: Option untuk ketiadaan nilai wajar; Result untuk operasi yang bisa gagal."
    );
    println!(
        "   [x] Pemakaian Operator ?: Propagasi error otomatis di parse_and_find dan execute_command."
    );
    println!(
        "   [x] Custom Error: TaskManagerError dengan trait Display, Debug, dan std::error::Error."
    );
    println!(
        "   [x] HashMap Entry API: Digunakan pada get_category_stats() dengan entry(), and_modify(), or_insert()."
    );
    println!(
        "   [x] Zero unwrap() pada input: Semua input parsing dipetakan aman via Result & map_err."
    );
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list_tasks() {
        let mut manager = TaskManager::new();
        let id1 = manager
            .add_task("Task 1", "Work", Some("Detail 1"))
            .unwrap();
        let id2 = manager.add_task("Task 2", "Personal", None).unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.list_tasks().len(), 2);
    }

    #[test]
    fn test_add_task_empty_title_error() {
        let mut manager = TaskManager::new();
        let res = manager.add_task("   ", "Work", None);
        assert_eq!(res, Err(TaskManagerError::EmptyTitle));
    }

    #[test]
    fn test_find_task_success_and_not_found() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Belajar Rust", "Study", None).unwrap();

        let found = manager.find_task(id);
        assert!(found.is_ok());
        assert_eq!(found.unwrap().title, "Belajar Rust");

        let not_found = manager.find_task(404);
        assert_eq!(not_found, Err(TaskManagerError::TaskNotFound(404)));
    }

    #[test]
    fn test_delete_task() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Task to delete", "Misc", None).unwrap();

        let deleted = manager.delete_task(id);
        assert!(deleted.is_ok());
        assert_eq!(deleted.unwrap().id, id);

        // Setelah dihapus, pencarian harus mengembalikan TaskNotFound
        assert_eq!(
            manager.find_task(id),
            Err(TaskManagerError::TaskNotFound(id))
        );
    }

    #[test]
    fn test_safe_parsing_input_path() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Clean Code", "Book", None).unwrap();

        // Input valid
        let res_valid = manager.parse_and_find(&id.to_string());
        assert!(res_valid.is_ok());

        // Input invalid format
        let res_invalid = manager.parse_and_find("abc_xyz");
        assert_eq!(
            res_invalid,
            Err(TaskManagerError::InvalidId("abc_xyz".to_string()))
        );

        // Input non-existent ID
        let res_missing = manager.parse_and_find("9999");
        assert_eq!(res_missing, Err(TaskManagerError::TaskNotFound(9999)));
    }

    #[test]
    fn test_category_stats_entry_api() {
        let mut manager = TaskManager::new();
        manager.add_task("T1", "Backend", None).unwrap();
        manager.add_task("T2", "Backend", None).unwrap();
        manager.add_task("T3", "Frontend", None).unwrap();

        let stats = manager.get_category_stats();
        assert_eq!(stats.get("Backend"), Some(&2));
        assert_eq!(stats.get("Frontend"), Some(&1));
        assert_eq!(stats.get("DevOps"), None);
    }
}
