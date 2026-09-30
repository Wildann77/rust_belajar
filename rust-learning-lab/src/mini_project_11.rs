// Mini Project Fase 11: Worker Pool CLI (Multi-Threading, Arc<Mutex<Receiver>>, Bidirectional Channels, & Graceful Shutdown)
// Rujukan: rust_learning_guide.md (Sub-bab 11.4) & rust_execution_tasks.md (L960-L990)

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// ============================================================================
// 1. Definisi Data Job dan JobResult
// ============================================================================

/// Tugas pekerjaan yang dikirimkan oleh Main Thread ke Worker Pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: usize,
    pub task_name: String,
    pub workload_ms: u64,
}

/// Hasil evaluasi komputasi yang dilaporkan oleh worker kembali ke Main Thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobResult {
    pub job_id: usize,
    pub worker_id: usize,
    pub output: String,
    pub success: bool,
}

// ============================================================================
// 2. Struct Worker & WorkerPool
// ============================================================================

/// Representasi satu worker thread individu di dalam pool.
#[allow(dead_code)]
pub struct Worker {
    pub id: usize,
    pub handle: Option<JoinHandle<()>>,
}

impl Worker {
    /// Membuat worker baru yang terus mendengarkan antrean job bersama.
    pub fn new(
        id: usize,
        job_receiver: Arc<Mutex<Receiver<Job>>>,
        result_sender: Sender<JobResult>,
    ) -> Self {
        let thread_builder = thread::Builder::new().name(format!("worker-pool-{}", id));

        let handle = thread_builder
            .spawn(move || {
                loop {
                    // 1. Mengambil job dari antrean bersama secara thread-safe.
                    // Receiver dibungkus Arc<Mutex<...>> karena Receiver mpsc bersifat Single-Consumer.
                    let job_opt = {
                        let lock = job_receiver.lock().expect("Gagal mengunci job queue Mutex");
                        // .recv() memblokir sampai ada job masuk atau channel ditutup
                        lock.recv().ok()
                    };

                    match job_opt {
                        Some(job) => {
                            // 2. Worker memproses job
                            thread::sleep(Duration::from_millis(job.workload_ms));
                            let output = format!(
                                "Job #{} ({}) selesai diproses oleh Worker-{}",
                                job.id, job.task_name, id
                            );

                            let res = JobResult {
                                job_id: job.id,
                                worker_id: id,
                                output,
                                success: true,
                            };

                            // 3. Worker mengirim hasil kembali ke Result Channel
                            let _ = result_sender.send(res);
                        }
                        None => {
                            // Job channel telah ditutup (Sender di-drop) -> Graceful termination
                            break;
                        }
                    }
                }
            })
            .expect("Gagal men-spawn worker thread");

        Self {
            id,
            handle: Some(handle),
        }
    }
}

/// Manajemen sekumpulan worker thread dengan antrean kerja terkoordinasi.
pub struct WorkerPool {
    workers: Vec<Worker>,
    job_sender: Option<Sender<Job>>,
    result_receiver: Receiver<JobResult>,
}

impl WorkerPool {
    /// Inisialisasi pool dengan sejumlah worker thread.
    pub fn new(num_workers: usize) -> Self {
        assert!(num_workers > 0, "Jumlah worker harus minimal 1");

        let (job_tx, job_rx) = mpsc::channel::<Job>();
        let (result_tx, result_rx) = mpsc::channel::<JobResult>();

        // Bungkus job_rx dalam Arc<Mutex<...>> agar bisa dibagi ke N worker
        let shared_job_rx = Arc::new(Mutex::new(job_rx));

        let mut workers = Vec::with_capacity(num_workers);
        for id in 1..=num_workers {
            let worker = Worker::new(id, Arc::clone(&shared_job_rx), result_tx.clone());
            workers.push(worker);
        }

        Self {
            workers,
            job_sender: Some(job_tx),
            result_receiver: result_rx,
        }
    }

    /// Fitur 1: Kirim job ke antrean pool.
    pub fn submit_job(&self, job: Job) -> Result<(), &'static str> {
        if let Some(ref tx) = self.job_sender {
            tx.send(job).map_err(|_| "Job channel terputus")
        } else {
            Err("Worker pool sudah dalam status shutdown")
        }
    }

    /// Fitur 4 & 5: Main mengumpulkan hasil dan memastikan graceful completion.
    pub fn collect_results(&mut self, total_expected: usize) -> Vec<JobResult> {
        let mut results = Vec::with_capacity(total_expected);

        // Kumpulkan hasil sebanyak total_expected dari result_receiver
        for _ in 0..total_expected {
            if let Ok(res) = self.result_receiver.recv() {
                results.push(res);
            } else {
                break;
            }
        }

        results
    }

    /// Mematikan seluruh worker secara anggun (Graceful Completion).
    pub fn shutdown(&mut self) {
        // Drop job_sender agar lock.recv() pada worker mengembalikan None
        drop(self.job_sender.take());

        // Join seluruh worker thread sampai selesai sepenuhnya
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// ============================================================================
// 3. Evaluasi Kriteria Lulus Fase 11
// ============================================================================

/// Kriteria 1: Perbedaan Concurrency vs Parallelism.
pub fn explain_concurrency_vs_parallelism() -> (&'static str, &'static str) {
    let concurrency = "Concurrency (Menangani banyak hal sekaligus / Struktur Program): \
                       Komposisi proses independen yang dapat dijalankan secara bergantian \
                       atau tumpang tindih (time-slicing) bahkan pada 1 CPU Core.";
    let parallelism = "Parallelism (Melakukan banyak hal bersamaan / Eksekusi Fisik): \
                       Eksekusi simultan tugas komputasi secara nyata di beberapa CPU Core fisik pada waktu yang sama.";
    (concurrency, parallelism)
}

/// Kriteria 2: Penjelasan Marker Trait Send & Sync.
pub fn explain_send_and_sync() -> (&'static str, &'static str) {
    let send = "Send: Garansi bahwa ownership tipe aman dipindahkan melintasi batas thread. \
                Contoh: Arc<T>, Mutex<T>, String, Box<T>. Bukan Send: Rc<T>, raw pointers.";
    let sync = "Sync: Garansi bahwa referensi bersama (&T) aman diakses konkuren dari banyak thread. \
                Aturan emas: T adalah Sync <=> &T adalah Send. Bukan Sync: RefCell<T>, Cell<T>.";
    (send, sync)
}

/// Kriteria 3: Konsep Worker Pool Sederhana.
pub fn explain_worker_pool_concept() -> &'static str {
    "Worker Pool adalah pola arsitektur konkurensi di mana sejumlah worker thread tetap (pre-spawned) \
     standby untuk menerima tugas dari antrean bersama (Job Queue) dan menyetor hasil ke Result Queue. \
     Mencegah overhead fatal dari membuat dan menghancurkan OS thread berulang kali."
}

// ============================================================================
// Runner Entry Point
// ============================================================================

pub fn run() {
    println!("=== Mini Project Fase 11: Worker Pool CLI ===");

    // 1. Inisialisasi Pool dengan 4 Worker Thread
    let num_workers = 4;
    println!(
        "\n1. Inisialisasi Worker Pool dengan {} worker threads...",
        num_workers
    );
    let mut pool = WorkerPool::new(num_workers);

    // 2. Kirim 8 Jobs
    let total_jobs = 8;
    println!("2. Mengirim {} jobs ke Job Channel:", total_jobs);
    for i in 1..=total_jobs {
        let workload = if i % 2 == 0 { 15 } else { 10 };
        let job = Job {
            id: i,
            task_name: format!("ResizeImage_{}", i),
            workload_ms: workload,
        };
        println!("   -> Submit Job #{} ({})", job.id, job.task_name);
        pool.submit_job(job).expect("Gagal submit job");
    }

    // 3. Mengumpulkan Hasil (Main mengumpulkan hasil)
    println!("\n3. Mengumpulkan hasil dari Result Channel...");
    let results = pool.collect_results(total_jobs);

    println!("\n4. Rekap Eksekusi Job oleh Worker Pool:");
    println!("   {:<8} | {:<10} | {:<50}", "Job ID", "Worker", "Output");
    println!("   {:-<8}-+-{:-<10}-+-{:-<50}", "", "", "");
    for res in &results {
        println!(
            "   {:<8} | Worker-{:<3} | {:<50}",
            res.job_id, res.worker_id, res.output
        );
    }
    assert_eq!(results.len(), total_jobs);

    // 5. Graceful Completion
    println!("\n5. Menjalankan Graceful Shutdown...");
    pool.shutdown();
    println!("   Semua worker thread berhasil ditutup secara bersih dan aman.");

    // 6. Ringkasan Lulus Fase 11
    println!("\n6. Evaluasi Kriteria Kelulusan Fase 11:");
    let (c_desc, p_desc) = explain_concurrency_vs_parallelism();
    println!(
        "   A. Concurrency vs Parallelism:\n      - {}\n      - {}",
        c_desc, p_desc
    );

    let (s_desc, sy_desc) = explain_send_and_sync();
    println!(
        "   B. Send vs Sync:\n      - {}\n      - {}",
        s_desc, sy_desc
    );

    println!(
        "   C. Worker Pool Architecture:\n      - {}",
        explain_worker_pool_concept()
    );

    println!("\n[OK] Mini Project Fase 11 (Worker Pool CLI) sukses & verified!\n");
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_pool_execution_and_results() {
        let mut pool = WorkerPool::new(3);
        let jobs_count = 6;

        for i in 1..=jobs_count {
            pool.submit_job(Job {
                id: i,
                task_name: format!("Task_{}", i),
                workload_ms: 5,
            })
            .unwrap();
        }

        let results = pool.collect_results(jobs_count);
        assert_eq!(results.len(), jobs_count);

        let mut job_ids: Vec<usize> = results.iter().map(|r| r.job_id).collect();
        job_ids.sort();
        assert_eq!(job_ids, vec![1, 2, 3, 4, 5, 6]);

        pool.shutdown();
    }

    #[test]
    fn test_submit_after_shutdown_fails() {
        let mut pool = WorkerPool::new(2);
        pool.shutdown();

        let res = pool.submit_job(Job {
            id: 99,
            task_name: "LateJob".to_string(),
            workload_ms: 1,
        });

        assert!(res.is_err());
    }

    #[test]
    fn test_lulus_fase_explanations() {
        let (c, p) = explain_concurrency_vs_parallelism();
        assert!(c.contains("Concurrency"));
        assert!(p.contains("Parallelism"));

        let (s, sy) = explain_send_and_sync();
        assert!(s.contains("Send"));
        assert!(sy.contains("Sync"));

        let wp = explain_worker_pool_concept();
        assert!(wp.contains("Worker Pool"));
    }
}
