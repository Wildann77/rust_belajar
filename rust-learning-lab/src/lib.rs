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
pub mod fase13_task_1;
pub mod fase14_task_1;
pub mod fase14_task_2;
pub mod fase14_task_3;
pub mod mini_project_13;
pub mod mini_project_14;
pub mod models;
pub mod services;

// Re-export (Facade Pattern) agar konsumen luar library dapat mengimpor langsung
// tanpa harus mengetahui path submodul secara mendalam.
// Contoh: `use rust_learning_lab::Task;` alih-alih `use rust_learning_lab::models::Task;`
pub use auth::{Claims, authenticate};
pub use fase13_task_1::{Account, AccountError, FLOAT_EPSILON};
pub use fase14_task_1::{
    RawBufferInspector, SafetyInvariant, create_raw_pointers, create_raw_pointers_modern,
    custom_split_at_mut, is_pointer_aligned, read_raw_pointer, write_raw_pointer,
};
pub use fase14_task_2::{
    AbiPillar, CPoint2D, FfiBridgeError, c_add_integers, c_calculate_hypotenuse, safe_c_strlen,
    safe_compute_distance, safe_divide_via_c_abi, safe_read_c_string,
};
pub use fase14_task_3::{
    GeneratedItem, MacroFragmentKind, ResponseMetric, compute_metrics, get_default_system_config,
};
pub use mini_project_13::{TaskManager, TaskManagerError, TaskPriority, TaskRecord};
pub use mini_project_14::{GraduationEvaluationItem, get_phase14_graduation_evaluation};
pub use models::{Priority, Task, TaskStatus};
pub use services::TaskService;
