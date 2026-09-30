// Fase 12 - Task 5: Asynchronous Cancellation & Graceful Shutdown Pattern
// Rujukan: rust_learning_guide.md (Sub-bab 12.5) & rust_execution_tasks.md (L1069-L1074)
//
// Cakupan Checklist:
// - Buat worker background.
// - Tambahkan signal shutdown.
// - Hentikan worker secara graceful.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::{oneshot, watch};
use tokio::task::JoinHandle;
use tokio::time::sleep;

// ============================================================================
// 1. Struktur Laporan Worker
// ============================================================================

/// Laporan status akhir dari background worker setelah shutdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerReport {
    pub worker_id: usize,
    pub jobs_processed: usize,
    pub shutdown_clean: bool,
    pub cleanup_message: String,
}

// ============================================================================
// 2. Worker 1: Shutdown Tunggal via oneshot Channel
// ============================================================================

/// Background worker yang memproses pekerjaan berkala dan mendengarkan
/// sinyal shutdown 1-ke-1 melalui `tokio::sync::oneshot`.
pub fn spawn_single_worker_oneshot(
    worker_id: usize,
    job_interval: Duration,
) -> (JoinHandle<WorkerReport>, oneshot::Sender<()>) {
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

    let handle = tokio::spawn(async move {
        let mut jobs_count: usize = 0;

        loop {
            tokio::select! {
                // Cabang 1: Sinyal shutdown diterima
                _ = &mut shutdown_rx => {
                    println!("   [Worker {}] Menerima sinyal shutdown! Melakukan cleanup...", worker_id);
                    // Simulasi flushing buffer / melepaskan koneksi
                    sleep(Duration::from_millis(5)).await;
                    break;
                }
                // Cabang 2: Melakukan pekerjaan rutin
                _ = sleep(job_interval) => {
                    jobs_count += 1;
                    println!("   [Worker {}] Memproses job #{}", worker_id, jobs_count);
                }
            }
        }

        WorkerReport {
            worker_id,
            jobs_processed: jobs_count,
            shutdown_clean: true,
            cleanup_message: format!("Worker {} shutdown sukses tanpa data loss", worker_id),
        }
    });

    (handle, shutdown_tx)
}

// ============================================================================
// 3. Worker 2: Broadcast Shutdown via watch Channel (Multi-Worker)
// ============================================================================

/// Background worker yang mendengarkan sinyal shutdown multi-worker (broadcast)
/// menggunakan `tokio::sync::watch`.
pub fn spawn_broadcast_worker(
    worker_id: usize,
    mut shutdown_rx: watch::Receiver<bool>,
    job_interval: Duration,
    in_flight_queue: Arc<AtomicUsize>,
) -> JoinHandle<WorkerReport> {
    tokio::spawn(async move {
        let mut jobs_done: usize = 0;

        loop {
            // Periksa apakah sinyal shutdown sudah bernilai true
            if *shutdown_rx.borrow() {
                println!(
                    "   [Broadcast Worker {}] Sinyal shutdown aktif terdeteksi saat inisiasi loop!",
                    worker_id
                );
                break;
            }

            tokio::select! {
                // Cabang 1: Nilai channel watch berubah
                changed = shutdown_rx.changed() => {
                    if changed.is_ok() && *shutdown_rx.borrow() {
                        println!("   [Broadcast Worker {}] Menerima sinyal broadcast shutdown!", worker_id);

                        // Menyelesaikan sisa in-flight tasks yang masih antre (drain phase)
                        let remaining = in_flight_queue.swap(0, Ordering::SeqCst);
                        if remaining > 0 {
                            println!("   [Broadcast Worker {}] Menyelesaikan {} sisa in-flight tasks...", worker_id, remaining);
                            jobs_done += remaining;
                        }

                        // Cleanup resource
                        sleep(Duration::from_millis(5)).await;
                        break;
                    }
                }
                // Cabang 2: Pekerjaan rutin
                _ = sleep(job_interval) => {
                    jobs_done += 1;
                }
            }
        }

        WorkerReport {
            worker_id,
            jobs_processed: jobs_done,
            shutdown_clean: true,
            cleanup_message: format!("Broadcast Worker {} graceful shutdown tuntas", worker_id),
        }
    })
}

// ============================================================================
// 4. Demonstrasi Utama (pub fn run())
// ============================================================================

pub fn run() {
    println!("=== FASE 12 - TASK 5: ASYNC CANCELLATION & SHUTDOWN ===");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Gagal membuat Tokio runtime untuk Task 5");

    runtime.block_on(async {
        // 1. Skenario Single Worker dengan oneshot shutdown
        println!("\n1. Single Background Worker (oneshot shutdown signal):");
        let (worker_handle, shutdown_tx) =
            spawn_single_worker_oneshot(1, Duration::from_millis(10));

        // Biarkan worker berjalan sebentar (misal 35ms -> sempat proses ~3 jobs)
        sleep(Duration::from_millis(35)).await;

        // Kirimkan sinyal shutdown dari main task
        println!("   [Main] Mengirimkan sinyal shutdown ke Worker 1...");
        let _ = shutdown_tx.send(());

        // Tunggu worker berhenti secara graceful
        let report_1 = worker_handle.await.expect("Worker 1 panic!");
        println!("   [Main] Hasil Report Worker 1: {:?}", report_1);
        assert!(report_1.shutdown_clean);
        assert!(report_1.jobs_processed >= 2);

        // 2. Skenario Multi-Worker dengan watch broadcast channel
        println!("\n2. Multi-Worker Pool (watch broadcast shutdown signal):");
        let (shutdown_broadcast_tx, shutdown_broadcast_rx) = watch::channel(false);
        let shared_queue = Arc::new(AtomicUsize::new(5)); // 5 in-flight items

        let mut worker_handles = Vec::new();
        for id in 101..=103 {
            let rx = shutdown_broadcast_rx.clone();
            let queue = Arc::clone(&shared_queue);
            let handle = spawn_broadcast_worker(id, rx, Duration::from_millis(15), queue);
            worker_handles.push(handle);
        }

        // Biarkan worker pool bekerja sejenak
        sleep(Duration::from_millis(35)).await;

        // Broadcast sinyal shutdown ke SELURUH worker sekaligus
        println!("   [Main] Mengirim broadcast shutdown=true ke seluruh worker...");
        shutdown_broadcast_tx
            .send(true)
            .expect("Gagal broadcast shutdown");

        // Join seluruh worker
        let mut reports = Vec::new();
        for h in worker_handles {
            let rep = h.await.expect("Broadcast worker panic!");
            println!(
                "   [Main] Selesai: {} (total jobs: {})",
                rep.cleanup_message, rep.jobs_processed
            );
            assert!(rep.shutdown_clean);
            reports.push(rep);
        }

        assert_eq!(reports.len(), 3);
        println!(
            "\n[OK] FASE 12 Task 5 (Cancellation & Graceful Shutdown) tuntas & terverifikasi!\n"
        );
    });
}

// ============================================================================
// 5. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_single_worker_graceful_shutdown() {
        let (handle, tx) = spawn_single_worker_oneshot(42, Duration::from_millis(5));
        sleep(Duration::from_millis(25)).await;

        // Kirim shutdown
        assert!(tx.send(()).is_ok());

        let report = handle.await.unwrap();
        assert!(report.shutdown_clean);
        assert!(report.jobs_processed >= 3);
    }

    #[tokio::test]
    async fn test_multi_worker_broadcast_shutdown() {
        let (tx, rx) = watch::channel(false);
        let queue = Arc::new(AtomicUsize::new(2));

        let h1 =
            spawn_broadcast_worker(1, rx.clone(), Duration::from_millis(10), Arc::clone(&queue));
        let h2 =
            spawn_broadcast_worker(2, rx.clone(), Duration::from_millis(10), Arc::clone(&queue));

        sleep(Duration::from_millis(20)).await;

        // Broadcast stop
        tx.send(true).unwrap();

        let (r1, r2) = tokio::join!(h1, h2);
        let rep1 = r1.unwrap();
        let rep2 = r2.unwrap();

        assert!(rep1.shutdown_clean);
        assert!(rep2.shutdown_clean);
    }

    #[test]
    fn test_sync_runner_execution() {
        run();
    }
}
