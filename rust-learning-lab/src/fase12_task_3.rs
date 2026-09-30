// Fase 12 - Task 3: Concurrent Tasks (Batch Spawning, JoinHandle Vec, JoinSet, & Aggregation)
// Rujukan: rust_learning_guide.md (Sub-bab 12.3) & rust_execution_tasks.md (L1046-L1061)
//
// Cakupan Checklist:
// - Buat 10 task: Task 1 s/d Task 10
// - Masing-masing task memiliki durasi sleep berbeda
// - Masing-masing task menghasilkan result
// - Seluruh task di-join oleh main / parent task

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::sleep;

// ============================================================================
// 1. Struktur Data Hasil Task
// ============================================================================

/// Struct penampung hasil eksekusi dari masing-masing concurrent task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskResult {
    pub task_id: usize,
    pub task_name: String,
    pub sleep_ms: u64,
    pub result_data: String,
    pub completion_order: usize,
}

// ============================================================================
// 2. Fungsi Worker Async untuk 10 Task
// ============================================================================

/// Fungsi asynchronous worker yang mensimulasikan pekerjaan I/O / komputasi dengan durasi sleep berbeda.
pub async fn execute_worker_task(
    id: usize,
    sleep_ms: u64,
    order_counter: Arc<AtomicUsize>,
) -> TaskResult {
    let task_name = format!("Task {}", id);

    // Sleep non-blocking berbeda untuk masing-masing task
    sleep(Duration::from_millis(sleep_ms)).await;

    // Catat urutan selesai (siapa yang pertama bangun, mendapat nomor urut lebih kecil)
    let order = order_counter.fetch_add(1, Ordering::SeqCst);

    TaskResult {
        task_id: id,
        task_name: task_name.clone(),
        sleep_ms,
        result_data: format!("Hasil kalkulasi dari {} (sleep {}ms)", task_name, sleep_ms),
        completion_order: order,
    }
}

// ============================================================================
// 3. Pendekatan 1: Batch Spawning menggunakan Vec<JoinHandle<T>>
// ============================================================================

/// Menjalankan 10 task konkuren dengan mengumpulkan JoinHandle ke dalam Vec,
/// lalu men-join satu per satu.
pub async fn run_ten_concurrent_tasks_vec() -> (Vec<TaskResult>, u128, u64) {
    let start_time = Instant::now();
    let order_counter = Arc::new(AtomicUsize::new(1));
    let mut handles: Vec<JoinHandle<TaskResult>> = Vec::with_capacity(10);

    // Durasi sleep bervariasi untuk masing-masing 10 task
    // Task 1: 50ms, Task 2: 45ms, ..., Task 10: 5ms (tercepat)
    let mut total_nominal_sleep_ms: u64 = 0;

    for id in 1..=10 {
        // Rumus sleep berbeda: task ber-ID tinggi mendapat delay lebih singkat
        let sleep_ms = ((11 - id) * 5) as u64; // 50, 45, 40, ..., 5 ms
        total_nominal_sleep_ms += sleep_ms;

        let counter = Arc::clone(&order_counter);

        // Spawn task ke Tokio work-stealing thread pool
        let handle = tokio::spawn(async move { execute_worker_task(id, sleep_ms, counter).await });

        handles.push(handle);
    }

    // Join semua task dari parent
    let mut results: Vec<TaskResult> = Vec::with_capacity(10);
    for handle in handles {
        let task_res = handle.await.expect("Task panic saat dieksekusi!");
        results.push(task_res);
    }

    let actual_duration_ms = start_time.elapsed().as_millis();

    (results, actual_duration_ms, total_nominal_sleep_ms)
}

// ============================================================================
// 4. Pendekatan 2: Batch Spawning menggunakan Tokio JoinSet (Out-of-Order Join)
// ============================================================================

/// Menjalankan 10 task konkuren menggunakan `tokio::task::JoinSet`.
/// Mengambil hasil segera setelah masing-masing task selesai (`join_next()`).
pub async fn run_ten_concurrent_tasks_joinset() -> (Vec<TaskResult>, u128) {
    let start_time = Instant::now();
    let order_counter = Arc::new(AtomicUsize::new(1));
    let mut join_set = JoinSet::new();

    for id in 1..=10 {
        let sleep_ms = ((11 - id) * 5) as u64;
        let counter = Arc::clone(&order_counter);

        join_set.spawn(async move { execute_worker_task(id, sleep_ms, counter).await });
    }

    let mut results_in_completion_order = Vec::with_capacity(10);
    while let Some(res) = join_set.join_next().await {
        let task_res = res.expect("JoinSet worker task panic!");
        results_in_completion_order.push(task_res);
    }

    let actual_duration_ms = start_time.elapsed().as_millis();

    (results_in_completion_order, actual_duration_ms)
}

// ============================================================================
// 5. Demonstrasi Utama (pub fn run())
// ============================================================================

pub fn run() {
    println!("=== FASE 12 - TASK 3: CONCURRENT TASKS (10 TASKS) ===");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Gagal membuat Tokio runtime untuk Task 3");

    runtime.block_on(async {
        // 1. Eksekusi 10 Task via Vec<JoinHandle>
        println!("\n1. Spawning 10 Concurrent Tasks (Vec<JoinHandle>):");
        let (results, actual_ms, nominal_ms) = run_ten_concurrent_tasks_vec().await;

        println!("   Total 10 task berhasil dibuat, dijalankan, dan di-join!");
        println!("   Akumulasi sleep jika sekuensial: {} ms", nominal_ms);
        println!("   Durasi eksekusi riil konkuren : {} ms (jauh lebih cepat!)", actual_ms);

        println!("\n   Daftar Hasil 10 Task:");
        println!("   {:<8} | {:<10} | {:<10} | {:<16} | {}", "ID", "Nama", "Sleep", "Urutan Selesai", "Hasil Data");
        println!("   ---------+------------+------------+------------------+---------------------------------------------");
        for r in &results {
            println!(
                "   {:<8} | {:<10} | {:>4} ms    | Selesai ke-{:<5} | {}",
                r.task_id, r.task_name, r.sleep_ms, r.completion_order, r.result_data
            );
        }

        // Verifikasi semua 10 task ada
        assert_eq!(results.len(), 10);
        for id in 1..=10 {
            assert!(results.iter().any(|r| r.task_id == id));
        }

        // 2. Eksekusi 10 Task via JoinSet (Stream-like Completion)
        println!("\n2. Spawning 10 Concurrent Tasks (tokio::task::JoinSet):");
        let (joinset_results, js_actual_ms) = run_ten_concurrent_tasks_joinset().await;
        println!("   JoinSet memproses hasil task out-of-order begitu task siap dalam {} ms:", js_actual_ms);
        for (idx, r) in joinset_results.iter().enumerate() {
            println!("   [Siap #{}] {} (sleep {}ms)", idx + 1, r.task_name, r.sleep_ms);
        }
        assert_eq!(joinset_results.len(), 10);

        // Task 10 (sleep 5ms) harus selesai lebih awal dibanding Task 1 (sleep 50ms)
        let first_completed = &joinset_results[0];
        let last_completed = &joinset_results[9];
        println!(
            "\n   [Bukti Asinkron & Sleep Berbeda] Task tercepat: {} ({}ms), Task terlama: {} ({}ms)",
            first_completed.task_name, first_completed.sleep_ms, last_completed.task_name, last_completed.sleep_ms
        );
        assert!(first_completed.sleep_ms < last_completed.sleep_ms);

        println!("\n[OK] FASE 12 Task 3 (Concurrent Tasks) tuntas & terverifikasi!\n");
    });
}

// ============================================================================
// 6. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ten_tasks_produce_all_results() {
        let (results, actual_ms, nominal_ms) = run_ten_concurrent_tasks_vec().await;
        assert_eq!(results.len(), 10);

        // Verifikasi ID 1..=10 lengkap
        for id in 1..=10 {
            assert!(results.iter().any(|r| r.task_id == id));
        }

        // Verifikasi konkurensi: durasi riil harus jauh lebih kecil dari akumulasi sekuensial
        assert!(actual_ms < nominal_ms as u128);
    }

    #[tokio::test]
    async fn test_tasks_have_distinct_sleep_durations() {
        let (results, _, _) = run_ten_concurrent_tasks_vec().await;
        let mut sleeps: Vec<u64> = results.iter().map(|r| r.sleep_ms).collect();
        sleeps.sort();
        sleeps.dedup();
        // Harus ada 10 durasi sleep unik yang berbeda
        assert_eq!(sleeps.len(), 10);
    }

    #[tokio::test]
    async fn test_joinset_out_of_order_completion() {
        let (results, _) = run_ten_concurrent_tasks_joinset().await;
        assert_eq!(results.len(), 10);

        // Task pertama yang keluar dari JoinSet harus memiliki sleep lebih kecil dibanding task terakhir
        let first_sleep = results.first().unwrap().sleep_ms;
        let last_sleep = results.last().unwrap().sleep_ms;
        assert!(first_sleep < last_sleep);
    }

    #[test]
    fn test_sync_runner_execution() {
        run();
    }
}
