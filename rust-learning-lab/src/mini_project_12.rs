// Mini Project Fase 12: Async Job Processor (Tokio Runtime, Channels, Spawn, Select, & Spawn Blocking)
// Rujukan: rust_learning_guide.md (Sub-bab 12.6) & rust_execution_tasks.md (L1075-L1097)
//
// Alur Pipeline:
// API/Main -> Job Channel -> Async Workers -> Processing -> Result Channel
//
// Kriteria Kelulusan Fase 12:
// - Bisa menjelaskan Future
// - Bisa menjelaskan perbedaan OS thread vs Tokio task
// - Bisa menggunakan spawn
// - Bisa menggunakan select!
// - Bisa menjelaskan kapan harus menggunakan spawn_blocking

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};

// ============================================================================
// 1. Definisi Tipe Job & JobResult
// ============================================================================

/// Jenis beban kerja yang didukung oleh Async Job Processor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkloadType {
    /// Pekerjaan I/O asinkron (misal fetch API, query database)
    IoBound { delay_ms: u64 },
    /// Pekerjaan CPU berat (harus offload ke spawn_blocking)
    CpuBound { iterations: u64 },
    /// Pekerjaan yang berisiko timeout
    RiskyTimeout { delay_ms: u64, timeout_ms: u64 },
}

/// Tugas pekerjaan yang dikirimkan oleh API / Main ke Job Channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncJob {
    pub id: usize,
    pub name: String,
    pub workload: WorkloadType,
}

/// Hasil pemrosesan job yang dikirimkan kembali melalui Result Channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncJobResult {
    pub job_id: usize,
    pub job_name: String,
    pub worker_id: usize,
    pub duration_ms: u64,
    pub output: String,
    pub success: bool,
}

// ============================================================================
// 2. Pemrosesan Job (I/O, CPU-Bound, & Timeout)
// ============================================================================

/// Memproses satu tugas pekerjaan sesuai dengan tipe beban kerjanya.
pub async fn process_job(job: AsyncJob, worker_id: usize) -> AsyncJobResult {
    let start = Instant::now();

    match job.workload {
        WorkloadType::IoBound { delay_ms } => {
            // Simulasi I/O asinkron non-blocking
            sleep(Duration::from_millis(delay_ms)).await;
            let elapsed = start.elapsed().as_millis() as u64;

            AsyncJobResult {
                job_id: job.id,
                job_name: job.name,
                worker_id,
                duration_ms: elapsed,
                output: format!("I/O data berhasil diambil dalam {} ms", elapsed),
                success: true,
            }
        }
        WorkloadType::CpuBound { iterations } => {
            // Offload komputasi intensif ke thread pool dedicated agar async event loop tidak kelaparan (starvation)
            let result_num = tokio::task::spawn_blocking(move || {
                let mut sum: u64 = 0;
                for i in 1..=iterations {
                    sum = sum.wrapping_add(i.wrapping_mul(7));
                }
                sum
            })
            .await
            .expect("spawn_blocking panic!");

            let elapsed = start.elapsed().as_millis() as u64;

            AsyncJobResult {
                job_id: job.id,
                job_name: job.name,
                worker_id,
                duration_ms: elapsed,
                output: format!("CPU hash komputasi: {} (durasi {} ms)", result_num, elapsed),
                success: true,
            }
        }
        WorkloadType::RiskyTimeout {
            delay_ms,
            timeout_ms,
        } => {
            // Membungkus future dengan batas waktu deadline
            let res = timeout(Duration::from_millis(timeout_ms), async {
                sleep(Duration::from_millis(delay_ms)).await;
                "Data siap"
            })
            .await;

            let elapsed = start.elapsed().as_millis() as u64;

            match res {
                Ok(_) => AsyncJobResult {
                    job_id: job.id,
                    job_name: job.name,
                    worker_id,
                    duration_ms: elapsed,
                    output: format!("Berhasil selesai sebelum deadline {} ms", timeout_ms),
                    success: true,
                },
                Err(_) => AsyncJobResult {
                    job_id: job.id,
                    job_name: job.name,
                    worker_id,
                    duration_ms: elapsed,
                    output: format!(
                        "Dibatalkan karena timeout > {} ms (request butuh {} ms)",
                        timeout_ms, delay_ms
                    ),
                    success: false,
                },
            }
        }
    }
}

// ============================================================================
// 3. Async Worker Pool Orchestration
// ============================================================================

/// Orchestrator worker pool asynchronous.
pub struct AsyncJobProcessor {
    job_tx: mpsc::Sender<AsyncJob>,
    result_rx: mpsc::Receiver<AsyncJobResult>,
    shutdown_tx: watch::Sender<bool>,
    worker_handles: Vec<JoinHandle<usize>>,
    active_workers_count: usize,
}

impl AsyncJobProcessor {
    /// Membuat Job Processor baru dengan sejumlah worker asinkron.
    pub fn new(worker_count: usize, channel_capacity: usize) -> Self {
        let (job_tx, job_rx) = mpsc::channel::<AsyncJob>(channel_capacity);
        let (result_tx, result_rx) = mpsc::channel::<AsyncJobResult>(channel_capacity);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        // Bungkus job_rx dalam Arc<Mutex<...>> agar bisa ditarik bersamaan oleh banyak worker
        let shared_job_rx = Arc::new(Mutex::new(job_rx));
        let mut worker_handles = Vec::with_capacity(worker_count);

        for worker_id in 1..=worker_count {
            let rx = Arc::clone(&shared_job_rx);
            let tx = result_tx.clone();
            let mut shut_rx = shutdown_rx.clone();

            // Spawn worker task ke Tokio thread pool
            let handle = tokio::spawn(async move {
                let mut jobs_processed: usize = 0;

                loop {
                    // Cek cepat apakah shutdown sudah bernilai true
                    if *shut_rx.borrow() {
                        break;
                    }

                    // Polling job berikutnya dengan proteksi sinyal shutdown via tokio::select!
                    let job_opt = tokio::select! {
                        _ = shut_rx.changed() => {
                            if *shut_rx.borrow() {
                                break;
                            }
                            None
                        }
                        maybe_job = async {
                            let mut lock = rx.lock().await;
                            lock.recv().await
                        } => {
                            maybe_job
                        }
                    };

                    match job_opt {
                        Some(job) => {
                            let result = process_job(job, worker_id).await;
                            jobs_processed += 1;
                            let _ = tx.send(result).await;
                        }
                        None => {
                            // Channel job ditutup atau kosong
                            break;
                        }
                    }
                }

                jobs_processed
            });

            worker_handles.push(handle);
        }

        Self {
            job_tx,
            result_rx,
            shutdown_tx,
            worker_handles,
            active_workers_count: worker_count,
        }
    }

    /// Mengirimkan job baru ke antrean Job Channel.
    pub async fn submit(&self, job: AsyncJob) -> Result<(), &'static str> {
        self.job_tx
            .send(job)
            .await
            .map_err(|_| "Job Channel telah ditutup")
    }

    /// Mengambil hasil pemrosesan sejumlah `expected_count`.
    pub async fn collect_results(&mut self, expected_count: usize) -> Vec<AsyncJobResult> {
        let mut results = Vec::with_capacity(expected_count);
        for _ in 0..expected_count {
            if let Some(res) = self.result_rx.recv().await {
                results.push(res);
            } else {
                break;
            }
        }
        results
    }

    /// Mengirimkan sinyal shutdown dan menunggu seluruh worker selesai (graceful exit).
    pub async fn shutdown(self) -> Vec<usize> {
        // Broadcast sinyal shutdown ke seluruh worker
        let _ = self.shutdown_tx.send(true);

        let mut worker_stats = Vec::with_capacity(self.active_workers_count);
        for h in self.worker_handles {
            let count = h.await.expect("Worker task panic!");
            worker_stats.push(count);
        }

        worker_stats
    }
}

// ============================================================================
// 4. Kriteria Kelulusan Fase 12 (Lulus Fase Explanations)
// ============================================================================

/// Menjelaskan konsep Future di Rust.
pub fn explain_future() -> &'static str {
    "Future di Rust adalah state machine pasif berbasis pull (lazy). Komputasi tidak berjalan sebelum di-poll oleh executor. Desain ini mewujudkan zero-cost abstraction dan pembatalan instan tanpa alokasi tersembunyi."
}

/// Menjelaskan perbedaan fundamental antara OS Thread dan Tokio Task.
pub fn explain_os_thread_vs_tokio_task() -> (&'static str, &'static str) {
    (
        "OS Thread: Dijadwalkan oleh kernel OS, memiliki stack memori besar (2MB-8MB), mahal untuk dibuat/dihancurkan (konteks switch kernel tinggi). Terbatas hingga ribuan thread.",
        "Tokio Task: Green thread ringan (~ratusan byte) di user-space, dijadwalkan oleh work-stealing scheduler Tokio. Berpindah secara kooperatif di titik .await. Dapat dibuat puluhan hingga ratusan ribu secara simultan.",
    )
}

/// Menjelaskan penggunaan dan karakteristik `tokio::spawn`.
pub fn explain_spawn() -> &'static str {
    "tokio::spawn meluncurkan task asynchronous baru secara independen ke thread pool Tokio. Task yang di-spawn wajib memiliki bound `'static + Send`. Menghasilkan JoinHandle yang dapat di-await untuk mengambil nilai balik atau mengisolasi panic."
}

/// Menjelaskan penggunaan dan karakteristik `tokio::select!`.
pub fn explain_select() -> &'static str {
    "tokio::select! memultipleks beberapa Future sekaligus pada satu task. Cabang pertama yang siap (Poll::Ready) akan dieksekusi, sedangkan seluruh cabang lain otomatis di-drop (dibatalkan). Sangat penting untuk timeout, cancellation, dan multiplexing event."
}

/// Menjelaskan kapan harus menggunakan `tokio::task::spawn_blocking`.
pub fn explain_spawn_blocking() -> &'static str {
    "spawn_blocking wajib digunakan saat menjalankan komputasi intensif CPU (hashing, kompresi, enkripsi, parsing JSON besar) atau I/O sinkron blocking (FFI C, legacy file access). Tujuannya mencegah worker thread async macet (thread starvation), dengan cara mengalihkan beban ke dedicated blocking thread pool (hingga 512 thread OS)."
}

// ============================================================================
// 5. Demonstrasi Utama (pub fn run())
// ============================================================================

pub fn run() {
    println!("=== MINI PROJECT FASE 12: ASYNC JOB PROCESSOR ===");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Gagal membuat Tokio runtime untuk Mini Project 12");

    runtime.block_on(async {
        // 1. Inisialisasi Processor dengan 3 worker dan kapasitas channel 10
        println!("\n1. Inisialisasi Async Job Processor (3 Workers, Capacity 10)...");
        let mut processor = AsyncJobProcessor::new(3, 10);

        // 2. Submit 8 Jobs dengan berbagai variasi workload
        println!("2. Mengirim 8 Jobs ke Job Channel (API/Main -> Job Channel):");
        let jobs = vec![
            AsyncJob { id: 1, name: "FetchProfile".to_string(), workload: WorkloadType::IoBound { delay_ms: 20 } },
            AsyncJob { id: 2, name: "ComputeHash".to_string(), workload: WorkloadType::CpuBound { iterations: 1_000_000 } },
            AsyncJob { id: 3, name: "FetchOrders".to_string(), workload: WorkloadType::IoBound { delay_ms: 15 } },
            AsyncJob { id: 4, name: "FastApiCall".to_string(), workload: WorkloadType::RiskyTimeout { delay_ms: 10, timeout_ms: 50 } },
            AsyncJob { id: 5, name: "SlowApiCall".to_string(), workload: WorkloadType::RiskyTimeout { delay_ms: 80, timeout_ms: 25 } },
            AsyncJob { id: 6, name: "ComputeProof".to_string(), workload: WorkloadType::CpuBound { iterations: 1_500_000 } },
            AsyncJob { id: 7, name: "FetchInventory".to_string(), workload: WorkloadType::IoBound { delay_ms: 10 } },
            AsyncJob { id: 8, name: "SendWebhook".to_string(), workload: WorkloadType::IoBound { delay_ms: 25 } },
        ];

        let total_jobs = jobs.len();
        for job in jobs {
            println!("   -> Submit Job #{} ({})", job.id, job.name);
            processor.submit(job).await.expect("Gagal submit job");
        }

        // 3. Mengumpulkan hasil dari Result Channel
        println!("\n3. Mengumpulkan Hasil dari Result Channel...");
        let results = processor.collect_results(total_jobs).await;

        println!("\n4. Rekap Eksekusi Async Job Processor:");
        println!("   {:<6} | {:<16} | {:<10} | {:<10} | {:<7} | {}", "Job ID", "Nama Job", "Worker", "Durasi", "Sukses?", "Output");
        println!("   -------+------------------+------------+------------+---------+------------------------------------------");
        for r in &results {
            println!(
                "   #{:<5} | {:<16} | Worker-{:<3} | {:>4} ms    | {:<7} | {}",
                r.job_id, r.job_name, r.worker_id, r.duration_ms, r.success, r.output
            );
        }
        assert_eq!(results.len(), total_jobs);

        // 4. Menjalankan Graceful Shutdown
        println!("\n5. Menjalankan Graceful Shutdown pada Worker Pool...");
        let stats = processor.shutdown().await;
        for (idx, count) in stats.iter().enumerate() {
            println!("   Worker #{} menyelesaikan {} jobs", idx + 1, count);
        }
        let total_processed: usize = stats.iter().sum();
        assert_eq!(total_processed, total_jobs);

        // 5. Evaluasi Kriteria Kelulusan Fase 12
        println!("\n6. Evaluasi Kriteria Kelulusan FASE 12:");
        println!("   A. Konsep Future:\n      -> {}", explain_future());
        let (os_t, tokio_t) = explain_os_thread_vs_tokio_task();
        println!("   B. Perbedaan OS Thread vs Tokio Task:\n      -> {}\n      -> {}", os_t, tokio_t);
        println!("   C. Penggunaan spawn:\n      -> {}", explain_spawn());
        println!("   D. Penggunaan select!:\n      -> {}", explain_select());
        println!("   E. Penggunaan spawn_blocking:\n      -> {}", explain_spawn_blocking());

        println!("\n[OK] Mini Project Fase 12 (Async Job Processor) tuntas & verified!\n");
    });
}

// ============================================================================
// 6. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_job_processor_end_to_end() {
        let mut processor = AsyncJobProcessor::new(2, 5);

        let job_1 = AsyncJob {
            id: 1,
            name: "TestIo".to_string(),
            workload: WorkloadType::IoBound { delay_ms: 10 },
        };
        let job_2 = AsyncJob {
            id: 2,
            name: "TestCpu".to_string(),
            workload: WorkloadType::CpuBound {
                iterations: 100_000,
            },
        };

        processor.submit(job_1).await.unwrap();
        processor.submit(job_2).await.unwrap();

        let results = processor.collect_results(2).await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));

        let stats = processor.shutdown().await;
        assert_eq!(stats.iter().sum::<usize>(), 2);
    }

    #[tokio::test]
    async fn test_job_processor_timeout_handling() {
        let mut processor = AsyncJobProcessor::new(1, 2);

        let timeout_job = AsyncJob {
            id: 99,
            name: "TimeoutJob".to_string(),
            workload: WorkloadType::RiskyTimeout {
                delay_ms: 50,
                timeout_ms: 10,
            },
        };

        processor.submit(timeout_job).await.unwrap();
        let results = processor.collect_results(1).await;
        assert_eq!(results.len(), 1);
        assert!(!results[0].success); // Harus gagal karena timeout
        assert!(results[0].output.contains("Dibatalkan karena timeout"));

        processor.shutdown().await;
    }

    #[test]
    fn test_fase_12_graduation_criteria() {
        assert!(explain_future().contains("state machine"));
        let (os, tokio) = explain_os_thread_vs_tokio_task();
        assert!(os.contains("OS Thread"));
        assert!(tokio.contains("Tokio Task"));
        assert!(explain_spawn().contains("JoinHandle"));
        assert!(explain_select().contains("select!"));
        assert!(explain_spawn_blocking().contains("spawn_blocking"));
    }

    #[test]
    fn test_sync_runner_execution() {
        run();
    }
}
