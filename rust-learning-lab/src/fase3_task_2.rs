// Fase 3 - Task 2: Enum (Basic, Match, Data-bearing, Tuple-like, Struct-like)
// Rujukan: rust_learning_guide.md (Bagian 3.2)

// ==========================================
// 1. Basic Enum: Status
// Sesuai requirement spesifikasi task: Todo, InProgress, Done
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    // Associated function / Constructor default
    pub fn default_status() -> Self {
        Self::Todo
    }

    // Method membaca (&self) dengan exhaustive pattern matching
    pub fn label(&self) -> &'static str {
        match self {
            Status::Todo => "Menunggu Dikerjakan (TODO)",
            Status::InProgress => "Sedang Berjalan (IN PROGRESS)",
            Status::Done => "Selesai (DONE)",
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Status::Done)
    }

    // Aturan transisi status yang valid
    pub fn next(&self) -> Option<Status> {
        match self {
            Status::Todo => Some(Status::InProgress),
            Status::InProgress => Some(Status::Done),
            Status::Done => None, // Sudah di ujung siklus
        }
    }
}

// ==========================================
// 2. Data-bearing Enum: TaskEvent
// Membawa ragam payload: Unit-like, Tuple-like, Struct-like
// ==========================================
#[derive(Debug, Clone, PartialEq)]
pub enum TaskEvent {
    // 2a. Unit-like Variant (tanpa data)
    Created,

    // 2b. Tuple-like Variant (membawa tipe anonim berbasis urutan)
    AssignedTo(String),
    StatusChanged(Status, Status), // (status_lama, status_baru)
    LoggedHours(f64),

    // 2c. Struct-like Variant (membawa named fields)
    CommentAdded {
        author: String,
        content: String,
        is_internal: bool,
    },
    Rescheduled {
        new_deadline: String,
        reason: String,
    },
}

impl TaskEvent {
    // Method yang mengekstrak dan mendeskripsikan event via exhaustive match
    pub fn describe(&self) -> String {
        match self {
            TaskEvent::Created => "Event: Task baru saja dibuat.".to_string(),

            // Match Tuple-like variant
            TaskEvent::AssignedTo(user) => {
                format!("Event: Task dialihkan penanggung jawabnya ke '{user}'.")
            }
            TaskEvent::StatusChanged(from, to) => {
                format!(
                    "Event: Status berubah dari '{}' -> '{}'.",
                    from.label(),
                    to.label()
                )
            }
            TaskEvent::LoggedHours(hours) => {
                format!("Event: Waktu kerja dicatat sebesar {hours:.1} jam.")
            }

            // Match Struct-like variant
            TaskEvent::CommentAdded {
                author,
                content,
                is_internal,
            } => {
                let badge = if *is_internal {
                    "[Internal]"
                } else {
                    "[Public]"
                };
                format!("Event: Komentar baru dari {author} {badge}: \"{content}\"")
            }
            TaskEvent::Rescheduled {
                new_deadline,
                reason,
            } => {
                format!("Event: Jadwal diundur ke {new_deadline}. Alasan: {reason}")
            }
        }
    }
}

// ==========================================
// Runner Function
// ==========================================
pub fn run() {
    println!("=== Fase 3 - Task 2: Enums & Data Variants ===");

    // 1. Basic Enum Status & Match
    let todo = Status::default_status();
    let in_progress = Status::InProgress;
    let done = Status::Done;

    println!("1. Basic Enum & Method:");
    println!("   todo label       : {}", todo.label());
    println!("   in_progress label: {}", in_progress.label());
    println!("   done label       : {}", done.label());
    println!("   done is_finished : {}", done.is_finished());

    // Uji transisi status via match
    let mut current = todo;
    println!("2. Transisi Status Siklus Task:");
    while let Some(next_status) = current.next() {
        println!(
            "   {} -> transisi ke: {}",
            current.label(),
            next_status.label()
        );
        current = next_status;
    }
    println!(
        "   Final state: {} (next: {:?})",
        current.label(),
        current.next()
    );

    // 3. Enum Membawa Data: Tuple-like & Struct-like
    let events = vec![
        TaskEvent::Created,
        TaskEvent::AssignedTo("boyblanco".to_string()),
        TaskEvent::StatusChanged(Status::Todo, Status::InProgress),
        TaskEvent::LoggedHours(2.5),
        TaskEvent::CommentAdded {
            author: "reviewer".to_string(),
            content: "Logic borrow checker sudah rapi!".to_string(),
            is_internal: false,
        },
        TaskEvent::Rescheduled {
            new_deadline: "2026-10-01".to_string(),
            reason: "Sprint review diperpanjang".to_string(),
        },
        TaskEvent::StatusChanged(Status::InProgress, Status::Done),
    ];

    println!("3. Event Log (Data-bearing Enums):");
    for event in &events {
        println!("   {}", event.describe());
    }

    // 4. Memory Layout & Tagged Union Inspection
    let status_size = std::mem::size_of::<Status>();
    let event_size = std::mem::size_of::<TaskEvent>();
    println!("4. Memory Footprint (Tagged Union):");
    println!(
        "   size_of::<Status>    : {} byte (hanya butuh 1 byte tag/discriminant)",
        status_size
    );
    println!(
        "   size_of::<TaskEvent> : {} bytes (tag + payload variant terbesar)",
        event_size
    );
}

// ==========================================
// Unit Tests
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_labels_and_defaults() {
        assert_eq!(Status::default_status(), Status::Todo);
        assert_eq!(Status::Todo.label(), "Menunggu Dikerjakan (TODO)");
        assert_eq!(Status::InProgress.label(), "Sedang Berjalan (IN PROGRESS)");
        assert_eq!(Status::Done.label(), "Selesai (DONE)");
    }

    #[test]
    fn test_status_transitions() {
        let s0 = Status::Todo;
        let s1 = s0.next().unwrap();
        assert_eq!(s1, Status::InProgress);
        let s2 = s1.next().unwrap();
        assert_eq!(s2, Status::Done);
        assert_eq!(s2.next(), None);
        assert!(s2.is_finished());
        assert!(!s0.is_finished());
    }

    #[test]
    fn test_tuple_like_variants() {
        let assigned = TaskEvent::AssignedTo("alice".to_string());
        let desc = assigned.describe();
        assert!(desc.contains("alice"));

        let status_changed = TaskEvent::StatusChanged(Status::Todo, Status::InProgress);
        let desc_status = status_changed.describe();
        assert!(desc_status.contains("TODO"));
        assert!(desc_status.contains("IN PROGRESS"));

        let hours = TaskEvent::LoggedHours(4.0);
        assert!(hours.describe().contains("4.0 jam"));
    }

    #[test]
    fn test_struct_like_variants() {
        let comment = TaskEvent::CommentAdded {
            author: "bob".to_string(),
            content: "Revisi arsitektur".to_string(),
            is_internal: true,
        };
        let desc = comment.describe();
        assert!(desc.contains("bob"));
        assert!(desc.contains("[Internal]"));
        assert!(desc.contains("Revisi arsitektur"));

        let rescheduled = TaskEvent::Rescheduled {
            new_deadline: "2026-12-31".to_string(),
            reason: "Budget update".to_string(),
        };
        let desc_res = rescheduled.describe();
        assert!(desc_res.contains("2026-12-31"));
        assert!(desc_res.contains("Budget update"));
    }

    #[test]
    fn test_unit_like_variant_created() {
        let created = TaskEvent::Created;
        assert_eq!(created.describe(), "Event: Task baru saja dibuat.");
    }

    #[test]
    fn test_status_discriminant_size() {
        assert_eq!(std::mem::size_of::<Status>(), 1);
    }
}
