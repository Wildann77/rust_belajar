// Root Library Crate: rust_learning_lab
// Lokasi: rust-learning-lab/src/lib.rs
//
// Struktur modul hierarkis:
// rust_learning_lab
// ├── models (pub mod)
// ├── auth   (pub mod)
// │   └── token
// └── services (pub mod)
//     └── task_service

pub mod auth;
pub mod models;
pub mod services;

// Re-export (Facade Pattern) agar konsumen luar library dapat mengimpor langsung
// tanpa harus mengetahui path submodul secara mendalam.
// Contoh: `use rust_learning_lab::Task;` alih-alih `use rust_learning_lab::models::Task;`
pub use auth::{authenticate, Claims};
pub use models::{Priority, Task, TaskStatus};
pub use services::TaskService;
