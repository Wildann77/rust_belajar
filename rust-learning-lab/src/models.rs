// Modul Models: Representasi Domain Entity
// Lokasi: rust-learning-lab/src/models.rs

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

    pub fn next(&self) -> Option<TaskStatus> {
        match self {
            TaskStatus::Todo => Some(TaskStatus::InProgress),
            TaskStatus::InProgress => Some(TaskStatus::Review),
            TaskStatus::Review => Some(TaskStatus::Done),
            TaskStatus::Done => None,
        }
    }
}

// Struct Task dengan kombinasi visibility modifier:
// - `pub` field: bisa diakses bebas dari luar crate.
// - `pub(crate)` field: hanya bisa diakses internal crate (misal oleh service).
// - private field: hanya bisa diakses file models.rs ini saja.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub(crate) internal_notes: String,
    metadata_hash: u64, // Private field murni
}

impl Task {
    pub fn new(id: u64, title: &str, priority: Priority) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status: TaskStatus::Todo,
            internal_notes: String::new(),
            metadata_hash: id * 31,
        }
    }

    pub fn with_status(id: u64, title: &str, priority: Priority, status: TaskStatus) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status,
            internal_notes: String::new(),
            metadata_hash: id * 31,
        }
    }

    pub fn change_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    pub fn advance_status(&mut self) -> Result<TaskStatus, &'static str> {
        match self.status.next() {
            Some(next_status) => {
                self.status = next_status;
                Ok(self.status)
            }
            None => Err("Task sudah mencapai status final [DONE]"),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "#{:<3} {:<10} {:<13} {}",
            self.id,
            self.priority.badge(),
            self.status.badge(),
            self.title
        )
    }

    pub fn match_priority(&self) -> &'static str {
        match self.priority {
            Priority::Low => "Dapat dikerjakan saat ada waktu luang (Backlog).",
            Priority::Medium => "Prioritas standar, harus selesai dalam sprint berjalan.",
            Priority::High => "Prioritas tinggi, butuh perhatian khusus pekan ini.",
            Priority::Critical => "Mendesak! Blokir rilis sampai masalah ini terselesaikan.",
        }
    }

    pub fn match_status(&self) -> &'static str {
        match self.status {
            TaskStatus::Todo => "Task tersimpan di antrean dan belum dimulai.",
            TaskStatus::InProgress => "Sedang aktif dikerjakan oleh engineer.",
            TaskStatus::Review => "Kode disubmit, menunggu QA & Review.",
            TaskStatus::Done => "Pekerjaan tuntas dan siap rilis.",
        }
    }

    // Helper visibility pub(crate): hanya bisa dipanggil modul lain di crate ini
    pub(crate) fn set_internal_notes(&mut self, notes: &str) {
        self.internal_notes = notes.to_string();
    }

    pub(crate) fn internal_notes(&self) -> &str {
        &self.internal_notes
    }

    // Private helper: hanya bisa dipanggil dalam models.rs
    fn _compute_checksum(&self) -> u64 {
        self.metadata_hash ^ (self.id + 100)
    }
}
