// Mini Project Fase 3: Task Domain Model
// Menggabungkan Struct, Enum, Methods, & Pattern Matching
// Rujukan: rust_learning_guide.md (Bagian 3.5)

// ==========================================
// 1. Enum Priority & TaskStatus
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn badge(&self) -> &'static str {
        match self {
            Priority::Low => "[LOW]",
            Priority::Medium => "[MED]",
            Priority::High => "[HIGH]",
            Priority::Critical => "[CRITICAL]",
        }
    }

    pub fn urgency_label(&self) -> &'static str {
        match self {
            Priority::Low => "Rendah - Backlog",
            Priority::Medium => "Sedang - Sprint Reguler",
            Priority::High => "Tinggi - Pekan Ini",
            Priority::Critical => "Kritis - Hotfix Segera",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
}

impl TaskStatus {
    pub fn badge(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "[TODO]",
            TaskStatus::InProgress => "[IN PROGRESS]",
            TaskStatus::Review => "[IN REVIEW]",
            TaskStatus::Done => "[DONE]",
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, TaskStatus::Done)
    }

    // Transisi siklus hidup task
    pub fn next(&self) -> Option<TaskStatus> {
        match self {
            TaskStatus::Todo => Some(TaskStatus::InProgress),
            TaskStatus::InProgress => Some(TaskStatus::Review),
            TaskStatus::Review => Some(TaskStatus::Done),
            TaskStatus::Done => None,
        }
    }
}

// ==========================================
// 2. Struct Task (Domain Model)
// ==========================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
}

impl Task {
    // 2a. Constructor
    pub fn new(id: u64, title: &str, priority: Priority) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status: TaskStatus::Todo, // Status awal default adalah Todo
        }
    }

    pub fn with_status(id: u64, title: &str, priority: Priority, status: TaskStatus) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status,
        }
    }

    // 2b. Change status
    pub fn change_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    pub fn advance_status(&mut self) -> Result<TaskStatus, &'static str> {
        match self.status.next() {
            Some(next_status) => {
                self.status = next_status;
                Ok(self.status)
            }
            None => Err("Task sudah mencapai status final [DONE] dan tidak dapat dimajukan lagi"),
        }
    }

    // 2c. Display task
    pub fn display(&self) -> String {
        format!(
            "#{:<3} {:<10} {:<13} {}",
            self.id,
            self.priority.badge(),
            self.status.badge(),
            self.title
        )
    }

    // 2d. Match berdasarkan priority
    pub fn match_priority(&self) -> &'static str {
        match self.priority {
            Priority::Low => "Dapat dikerjakan saat ada waktu luang (Backlog).",
            Priority::Medium => "Prioritas standar, harus selesai dalam sprint berjalan.",
            Priority::High => "Prioritas tinggi, butuh perhatian khusus pekan ini.",
            Priority::Critical => "Mendesak! Blokir rilis sampai masalah ini terselesaikan.",
        }
    }

    // 2e. Match berdasarkan status
    pub fn match_status(&self) -> &'static str {
        match self.status {
            TaskStatus::Todo => "Task tersimpan di antrean dan belum dimulai.",
            TaskStatus::InProgress => "Sedang aktif dikerjakan oleh software engineer.",
            TaskStatus::Review => "Kode telah disubmit, sedang menunggu QA & Code Review.",
            TaskStatus::Done => "Pekerjaan tuntas, lolos verifikasi, dan siap rilis.",
        }
    }
}

// ==========================================
// 3. Helper Functions: `if let` & `while let`
// ==========================================

// Menggunakan `if let` untuk menyaring task berstatus Critical
pub fn filter_critical_tasks(tasks: &[Task]) -> Vec<&Task> {
    let mut critical_list = Vec::new();
    for task in tasks {
        if let Priority::Critical = task.priority {
            critical_list.push(task);
        }
    }
    critical_list
}

// Menggunakan `while let` untuk memproses antrean task pipeline
pub fn drain_task_pipeline(pipeline: &mut Vec<Task>) -> Vec<String> {
    let mut execution_logs = Vec::new();
    while let Some(mut task) = pipeline.pop() {
        let prev_status = task.status;
        let _ = task.advance_status();
        execution_logs.push(format!(
            "Task #{}: {} diproses dari {:?} -> {:?}",
            task.id, task.title, prev_status, task.status
        ));
    }
    execution_logs
}

// ==========================================
// Runner Function
// ==========================================
pub fn run() {
    println!("=== Mini Project Fase 3: Task Domain Model ===");

    // 1. Constructor Demo
    let mut task1 = Task::new(101, "Setup Database Postgres", Priority::High);
    let task2 = Task::new(102, "Fix Security Vulnerability", Priority::Critical);
    let task3 = Task::with_status(103, "Update Documentation", Priority::Low, TaskStatus::Review);
    let task4 = Task::new(104, "Implement JWT Auth", Priority::Medium);

    println!("1. Daftar Task Awal (Display):");
    println!("   {}", task1.display());
    println!("   {}", task2.display());
    println!("   {}", task3.display());
    println!("   {}", task4.display());

    // 2. Change Status & Advance Demo
    println!("\n2. Perubahan Status:");
    task1.change_status(TaskStatus::InProgress);
    println!("   Setelah change_status: {}", task1.display());
    println!("   Apakah Task 1 sudah selesai? {}", task1.status.is_done());

    let _ = task1.advance_status();
    println!("   Setelah advance_status: {}", task1.display());

    // 3. Match Priority & Status
    println!("\n3. Evaluasi Match:");
    println!("   Task 1 Priority: {}", task1.match_priority());
    println!("   Task 1 Status  : {}", task1.match_status());
    println!("   Task 2 Priority: {}", task2.match_priority());
    println!("   Task 2 Urgency : {}", task2.priority.urgency_label());

    // 4. `if let` Demo (Filter Task Kritis)
    let task_list = vec![task1.clone(), task2.clone(), task3.clone(), task4.clone()];
    let criticals = filter_critical_tasks(&task_list);
    println!("\n4. Filter Task Kritis (`if let`):");
    for crit in criticals {
        println!("   [!] Ditemukan task kritis: {}", crit.display());
    }

    // 5. `while let` Demo (Drain Pipeline)
    let mut pipeline = vec![task1, task2, task4];
    println!("\n5. Pemrosesan Pipeline Antrean (`while let`):");
    let logs = drain_task_pipeline(&mut pipeline);
    for log in logs {
        println!("   - {log}");
    }
}

// ==========================================
// Unit Tests
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_constructor_and_defaults() {
        let t = Task::new(1, "Bikin API", Priority::High);
        assert_eq!(t.id, 1);
        assert_eq!(t.title, "Bikin API");
        assert_eq!(t.priority, Priority::High);
        assert_eq!(t.status, TaskStatus::Todo);
        assert!(!t.status.is_done());
    }

    #[test]
    fn test_task_with_custom_status() {
        let t = Task::with_status(2, "Refactor Core", Priority::Critical, TaskStatus::Review);
        assert_eq!(t.status, TaskStatus::Review);
    }

    #[test]
    fn test_change_status() {
        let mut t = Task::new(3, "Deploy Prod", Priority::Critical);
        t.change_status(TaskStatus::Done);
        assert_eq!(t.status, TaskStatus::Done);
        assert!(t.status.is_done());
    }

    #[test]
    fn test_advance_status_lifecycle() {
        let mut t = Task::new(4, "Fitur Search", Priority::Medium);
        assert_eq!(t.advance_status(), Ok(TaskStatus::InProgress));
        assert_eq!(t.advance_status(), Ok(TaskStatus::Review));
        assert_eq!(t.advance_status(), Ok(TaskStatus::Done));
        assert!(t.advance_status().is_err()); // Tidak bisa maju setelah Done
    }

    #[test]
    fn test_display_formatting() {
        let t = Task::new(5, "Tulis Unit Test", Priority::Low);
        let disp = t.display();
        assert!(disp.contains("#5"));
        assert!(disp.contains("[LOW]"));
        assert!(disp.contains("[TODO]"));
        assert!(disp.contains("Tulis Unit Test"));
    }

    #[test]
    fn test_match_priority_and_status() {
        let t = Task::new(6, "Patch Kernel", Priority::Critical);
        assert!(t.match_priority().contains("Mendesak"));
        assert!(t.match_status().contains("antrean"));

        let mut t_done = t;
        t_done.change_status(TaskStatus::Done);
        assert!(t_done.match_status().contains("tuntas"));
    }

    #[test]
    fn test_filter_critical_tasks_if_let() {
        let tasks = vec![
            Task::new(1, "A", Priority::Low),
            Task::new(2, "B", Priority::Critical),
            Task::new(3, "C", Priority::High),
            Task::new(4, "D", Priority::Critical),
        ];
        let crits = filter_critical_tasks(&tasks);
        assert_eq!(crits.len(), 2);
        assert_eq!(crits[0].id, 2);
        assert_eq!(crits[1].id, 4);
    }

    #[test]
    fn test_drain_task_pipeline_while_let() {
        let mut pipeline = vec![
            Task::new(10, "Task 10", Priority::Medium),
            Task::new(20, "Task 20", Priority::High),
        ];
        let logs = drain_task_pipeline(&mut pipeline);
        assert_eq!(logs.len(), 2);
        assert!(pipeline.is_empty());
    }
}
