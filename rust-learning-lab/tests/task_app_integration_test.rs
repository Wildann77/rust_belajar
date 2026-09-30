// Integration Test untuk Modular Task App
// Lokasi: rust-learning-lab/tests/task_app_integration_test.rs
//
// File di direktori `tests/` dikompilasi oleh Cargo sebagai crate independen
// yang mengonsumsi library crate `rust_learning_lab` melalui API publiknya.

use rust_learning_lab::{Claims, Priority, Task, TaskService, TaskStatus, authenticate};

#[test]
fn test_models_public_api() {
    let mut task = Task::new(1, "Desain Skema DB", Priority::High);
    assert_eq!(task.id, 1);
    assert_eq!(task.title, "Desain Skema DB");
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.status, TaskStatus::Todo);
    assert!(!task.status.is_done());

    // Advance lifecycle
    let next_status = task.advance_status().expect("Gagal advance status");
    assert_eq!(next_status, TaskStatus::InProgress);
    assert_eq!(task.status, TaskStatus::InProgress);

    // Display string
    let display = task.display();
    assert!(display.contains("#1"));
    assert!(display.contains("[HIGH]"));
    assert!(display.contains("[IN PROGRESS]"));
}

#[test]
fn test_auth_authentication_flow() {
    // Valid token
    let auth_res = authenticate("alice:admin");
    assert!(auth_res.is_ok());
    let claims: Claims = auth_res.unwrap();
    assert_eq!(claims.sub, "alice");
    assert_eq!(claims.role, "admin");
    assert!(claims.is_admin());

    // User biasa (bukan admin)
    let user_claims = authenticate("bob:developer").unwrap();
    assert_eq!(user_claims.sub, "bob");
    assert!(!user_claims.is_admin());

    // Invalid token
    assert!(authenticate("").is_err());
    assert!(authenticate("format_salah_tanpa_titik_dua").is_err());
}

#[test]
fn test_task_service_workflow_and_reexports() {
    let mut service = TaskService::new("EnterpriseTaskHub");

    let admin_claims = authenticate("superman:admin").unwrap();
    let dev_claims = authenticate("budi:developer").unwrap();

    // 1. Create standard task oleh developer biasa
    let t1_id = service
        .create_task("Fix bug form login", Priority::Medium, &dev_claims)
        .expect("Harusnya berhasil buat task");
    assert_eq!(t1_id, 1);

    // 2. Create critical task
    let t2_id = service
        .create_task("Security breach patch", Priority::Critical, &admin_claims)
        .expect("Harusnya berhasil buat critical task");
    assert_eq!(t2_id, 2);

    assert_eq!(service.list_tasks().len(), 2);

    // 3. Dev biasa mencoba memajukan task Critical -> HARUS DITOLAK
    let dev_advance_res = service.advance_task(t2_id, &dev_claims);
    assert!(dev_advance_res.is_err());
    assert!(
        dev_advance_res
            .unwrap_err()
            .contains("Hanya admin yang berhak")
    );

    // 4. Admin memajukan task Critical -> HARUS SUKSES
    let admin_advance_res = service.advance_task(t2_id, &admin_claims);
    assert_eq!(admin_advance_res.unwrap(), TaskStatus::InProgress);

    // 5. Filter berdasarkan priority
    let criticals = service.filter_by_priority(Priority::Critical);
    assert_eq!(criticals.len(), 1);
    assert_eq!(criticals[0].id, t2_id);
}
