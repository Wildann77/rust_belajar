// Integration Test untuk Mini Project Fase 13 — Test Suite Task Manager
// Lokasi: rust-learning-lab/tests/mini_project_13_integration_test.rs
//
// File ini dikompilasi oleh Cargo sebagai crate independen
// yang menguji library crate `rust_learning_lab` dari kacamata konsumen publik.

use rust_learning_lab::mini_project_13::{
    TaskManager, TaskManagerError, TaskPriority, TaskRecord, TaskStatus,
};
use std::sync::Arc;

#[test]
fn test_integration_crud_lifecycle() {
    let manager = TaskManager::new();

    // 1. Create
    let id = manager
        .create_task(
            "Integrasi Payment Gateway",
            "Midtrans & Stripe API",
            TaskPriority::Critical,
        )
        .expect("Harusnya berhasil membuat task");
    assert_eq!(id, 1);
    assert_eq!(manager.count(), 1);

    // 2. Read
    let task = manager.get_task(id).expect("Task harus ada");
    assert_eq!(task.title, "Integrasi Payment Gateway");
    assert_eq!(task.priority, TaskPriority::Critical);
    assert_eq!(task.status, TaskStatus::Pending);

    // 3. Update details
    manager
        .update_details(
            id,
            Some("Integrasi Payment Gateway V2"),
            Some(TaskPriority::High),
        )
        .expect("Update details harus sukses");
    let updated = manager.get_task(id).unwrap();
    assert_eq!(updated.title, "Integrasi Payment Gateway V2");
    assert_eq!(updated.priority, TaskPriority::High);

    // 4. Update status (Transition)
    let in_prog = manager
        .transition_status(id, TaskStatus::InProgress)
        .expect("Transisi ke InProgress harus sukses");
    assert_eq!(in_prog, TaskStatus::InProgress);

    let completed = manager
        .transition_status(id, TaskStatus::Completed)
        .expect("Transisi ke Completed harus sukses");
    assert_eq!(completed, TaskStatus::Completed);
    assert!(manager.get_task(id).unwrap().completed_at_ms.is_some());

    // 5. Delete
    let deleted = manager.delete_task(id).expect("Delete harus sukses");
    assert_eq!(deleted.id, id);
    assert_eq!(manager.count(), 0);
    assert_eq!(
        manager.get_task(id).unwrap_err(),
        TaskManagerError::TaskNotFound(id)
    );
}

#[test]
fn test_integration_error_handling_and_boundaries() {
    let manager = TaskManager::new();

    // 1. Empty title validation
    let err_empty = manager.create_task("    ", "Desc", TaskPriority::Low);
    assert!(err_empty.is_err());
    assert_eq!(err_empty.unwrap_err(), TaskManagerError::EmptyTitle);

    // 2. Not found error
    let err_not_found = manager.get_task(9999);
    assert_eq!(
        err_not_found.unwrap_err(),
        TaskManagerError::TaskNotFound(9999)
    );

    // 3. Invalid transition from Pending straight to Completed
    let id = manager
        .create_task("Illegal Transition", "Desc", TaskPriority::Medium)
        .unwrap();
    let err_trans = manager.transition_status(id, TaskStatus::Completed);
    assert_eq!(
        err_trans.unwrap_err(),
        TaskManagerError::InvalidStateTransition {
            from: TaskStatus::Pending,
            to: TaskStatus::Completed,
        }
    );
}

#[test]
fn test_integration_concurrent_thread_stress() {
    let manager = Arc::new(TaskManager::new());
    let mut handles = Vec::new();

    // 8 OS Thread membuat task bersamaan
    for t in 0..8 {
        let mgr = Arc::clone(&manager);
        let h = std::thread::spawn(move || {
            for i in 0..25 {
                let title = format!("Concurrent-Thread-{}-{}", t, i);
                mgr.create_task(&title, "Desc", TaskPriority::Medium)
                    .unwrap();
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    // Total task harus tepat 8 * 25 = 200
    assert_eq!(manager.count(), 200);

    let all: Vec<TaskRecord> = manager.list_tasks();
    assert_eq!(all.len(), 200);

    // Pastikan tidak ada ID duplikat
    let mut ids: Vec<u64> = all.iter().map(|t| t.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 200);
}

#[tokio::test]
async fn test_integration_async_and_batch_processing() {
    let manager = TaskManager::new();

    let mut task_ids = Vec::new();
    for i in 1..=4 {
        let id = manager
            .create_task(
                &format!("Async Batch {}", i),
                "Batch processing",
                TaskPriority::High,
            )
            .unwrap();
        task_ids.push(id);
    }

    // Proses batch concurrently
    let results = manager.process_batch_concurrently(&task_ids, 15).await;
    assert_eq!(results.len(), 4);

    for res in results {
        let task = res.expect("Batch item harus sukses diproses");
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.completed_at_ms.is_some());
    }

    // Uji timeout
    let timeout_task_id = manager
        .create_task("Slow Async Job", "Timeout test", TaskPriority::Low)
        .unwrap();

    let timeout_res = manager.execute_with_timeout(timeout_task_id, 50, 10).await;

    assert!(timeout_res.is_err());
    assert_eq!(
        timeout_res.unwrap_err(),
        TaskManagerError::ExecutionTimeout {
            task_id: timeout_task_id,
            timeout_ms: 10,
        }
    );

    // Status task harus berubah menjadi Cancelled
    let cancelled = manager.get_task(timeout_task_id).unwrap();
    assert_eq!(cancelled.status, TaskStatus::Cancelled);
}
