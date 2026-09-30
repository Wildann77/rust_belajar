// Fase 11 - Task 1: Native OS Threads (thread::spawn, JoinHandle, move Closures, Return Values, & join())
// Rujukan: rust_learning_guide.md (Sub-bab 11.1) & rust_execution_tasks.md (L938-L944)

use std::thread::{self, JoinHandle};
use std::time::Duration;

// ============================================================================
// 1. Spawn 2 Thread & Sinkronisasi Dasar dengan join()
// ============================================================================

/// Struct penampung hasil komputasi dari 2 thread yang berjalan konkuren.
#[derive(Debug, PartialEq, Eq)]
pub struct TwoThreadsResult {
    pub thread_alpha_result: String,
    pub thread_beta_result: String,
}

/// Mendemonstrasikan pembuatan 2 OS thread terpisah dan menunggu keduanya selesai dengan join().
pub fn demonstrate_two_threads() -> TwoThreadsResult {
    println!("   [Main] Memulai spawn 2 worker thread...");

    // Spawn Thread Alpha
    let handle_alpha: JoinHandle<String> = thread::spawn(|| {
        println!("   [Thread Alpha] Mulai bekerja...");
        thread::sleep(Duration::from_millis(20));
        println!("   [Thread Alpha] Selesai!");
        String::from("Hasil dari Alpha: OK")
    });

    // Spawn Thread Beta
    let handle_beta: JoinHandle<String> = thread::spawn(|| {
        println!("   [Thread Beta] Mulai bekerja...");
        thread::sleep(Duration::from_millis(15));
        println!("   [Thread Beta] Selesai!");
        String::from("Hasil dari Beta: OK")
    });

    // Tanpa .join(), main thread bisa keluar sebelum kedua worker thread selesai dieksekusi.
    // .join() memblokir (block) eksekusi main thread hingga thread target selesai berjalan.
    let result_alpha = handle_alpha.join().expect("Thread Alpha mengalami panic!");
    let result_beta = handle_beta.join().expect("Thread Beta mengalami panic!");

    println!("   [Main] Kedua thread telah berhasil di-join.");

    TwoThreadsResult {
        thread_alpha_result: result_alpha,
        thread_beta_result: result_beta,
    }
}

// ============================================================================
// 2. Spawn Banyak Thread (Multi-Threading / Thread Pool Batch) & move Closure
// ============================================================================

/// Mendemonstrasikan spawning banyak thread secara dinamis dalam loop.
/// Menggunakan keyword `move` pada closure agar kepemilikan variabel lokal
/// (seperti worker ID) ditransfer ke masing-masing thread.
pub fn demonstrate_multiple_threads(num_workers: usize) -> Vec<usize> {
    println!(
        "   [Main] Mempersiapkan spawning {} worker threads...",
        num_workers
    );
    let mut handles: Vec<JoinHandle<usize>> = Vec::with_capacity(num_workers);

    for id in 1..=num_workers {
        // Keyword `move` memaksa closure mengambil ownership variabel `id`.
        // Jika tanpa `move`, compiler menolak karena referensi `&id` tidak dijamin
        // hidup selama thread berjalan (bisa terjadi dangling reference).
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(5 * (id as u64)));
            id * 10
        });

        handles.push(handle);
    }

    // Mengumpulkan hasil dari seluruh thread dengan men-join setiap JoinHandle
    let mut results = Vec::with_capacity(num_workers);
    for handle in handles {
        let val = handle.join().expect("Worker thread gagal dieksekusi");
        results.push(val);
    }

    results
}

// ============================================================================
// 3. Return Value dari Thread & Parallel Chunk Reduction (Map-Reduce Sederhana)
// ============================================================================

/// Menghitung total jumlah array angka secara paralel dengan membagi beban kerja ke beberapa thread.
/// Setiap thread mengembalikan (return value) hasil jumlah bagiannya (*partial sum*).
pub fn parallel_sum(data: &[u64], chunks_count: usize) -> u64 {
    if data.is_empty() {
        return 0;
    }

    let chunk_size = (data.len() + chunks_count - 1) / chunks_count;
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();

    for chunk in data.chunks(chunk_size) {
        // Salin chunk ke dalam Vec baru agar data memiliki lifetime 'static dan aman di-move ke thread.
        let chunk_vec = chunk.to_vec();

        let handle = thread::spawn(move || {
            let partial: u64 = chunk_vec.iter().sum();
            partial
        });

        handles.push(handle);
    }

    // Kumpulkan dan jumlahkan seluruh return value dari setiap JoinHandle
    let mut total_sum: u64 = 0;
    for handle in handles {
        let partial_sum = handle.join().expect("Gagal join thread pada parallel_sum");
        total_sum += partial_sum;
    }

    total_sum
}

// ============================================================================
// 4. Thread Builder: Memberi Nama Thread & Konfigurasi Stack
// ============================================================================

/// Mendemonstrasikan penggunaan `thread::Builder` untuk memberikan nama eksplisit pada thread,
/// yang sangat berguna untuk proses debugging, profiling, dan tracing log produksi.
pub fn demonstrate_named_threads() -> String {
    let builder = thread::Builder::new().name(String::from("worker-logger-01"));

    let handle = builder
        .spawn(|| {
            let current = thread::current();
            let thread_name = current.name().unwrap_or("unnamed");
            format!("Halo dari thread: {}", thread_name)
        })
        .expect("Gagal men-spawn named thread");

    handle.join().expect("Thread logging panic")
}

// ============================================================================
// 5. Isolasi Panic: Main Thread Tetap Aman Ketika Worker Panic
// ============================================================================

/// Membuktikan bahwa panic di worker thread tidak serta merta membuat main process crash.
/// Pemanggilan `.join()` mengembalikan `Result::Err(Box<dyn Any + Send + 'static>)`.
pub fn demonstrate_panic_isolation() -> bool {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let handle = thread::spawn(|| {
        println!("   [Panic Thread] Memulai task berisiko (simulasi panic)...");
        panic!("Simulasi panic darurat di worker thread!");
    });

    let join_result = handle.join();
    std::panic::set_hook(prev_hook);

    match join_result {
        Ok(_) => {
            println!("   [Main] Thread selesai tanpa panic.");
            false
        }
        Err(_) => {
            println!(
                "   [Main] Berhasil menangkap panic dari thread! Main thread tetap selamat dan berjalan."
            );
            true
        }
    }
}

// ============================================================================
// Runner Entry Point
// ============================================================================

pub fn run() {
    println!("=== FASE 11: Task 1 - Native OS Threads & Concurrency ===");

    // 1. Spawn 2 thread
    println!("\n1. Spawning 2 Independent Threads:");
    let two_res = demonstrate_two_threads();
    println!("   Hasil Alpha: {}", two_res.thread_alpha_result);
    println!("   Hasil Beta : {}", two_res.thread_beta_result);

    // 2. Spawn banyak thread
    println!("\n2. Spawning Multiple Threads (Batch of 5 Workers):");
    let batch_results = demonstrate_multiple_threads(5);
    println!("   Hasil komputasi (id * 10): {:?}", batch_results);
    assert_eq!(batch_results, vec![10, 20, 30, 40, 50]);

    // 3. Return value & Parallel Sum
    println!("\n3. Thread Return Value (Parallel Map-Reduce Sum):");
    let numbers: Vec<u64> = (1..=1000).collect();
    let expected_sum: u64 = (1000 * 1001) / 2;
    let actual_sum = parallel_sum(&numbers, 4);
    println!("   Array: 1 s/d 1000 dibagi ke 4 thread worker.");
    println!(
        "   Hasil Parallel Sum: {} (Ekspektasi: {})",
        actual_sum, expected_sum
    );
    assert_eq!(actual_sum, expected_sum);

    // 4. Named Threads dengan thread::Builder
    println!("\n4. Named Thread via thread::Builder:");
    let named_res = demonstrate_named_threads();
    println!("   Info: {}", named_res);

    // 5. Isolasi Panic pada Thread
    println!("\n5. Panic Isolation via join():");
    let caught_panic = demonstrate_panic_isolation();
    assert!(caught_panic);

    println!("\n[OK] Seluruh demonstrasi FASE 11 Task 1 (Threads) sukses & verified!\n");
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_two_threads_success() {
        let res = demonstrate_two_threads();
        assert_eq!(res.thread_alpha_result, "Hasil dari Alpha: OK");
        assert_eq!(res.thread_beta_result, "Hasil dari Beta: OK");
    }

    #[test]
    fn test_spawn_multiple_threads_ordered_results() {
        let results = demonstrate_multiple_threads(6);
        assert_eq!(results, vec![10, 20, 30, 40, 50, 60]);
    }

    #[test]
    fn test_parallel_sum_correctness() {
        let nums: Vec<u64> = (1..=100).collect();
        let expected: u64 = nums.iter().sum();
        let computed = parallel_sum(&nums, 4);
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_parallel_sum_empty() {
        let empty: Vec<u64> = vec![];
        assert_eq!(parallel_sum(&empty, 2), 0);
    }

    #[test]
    fn test_panic_isolation_returns_true() {
        let panic_caught = demonstrate_panic_isolation();
        assert!(panic_caught);
    }

    #[test]
    fn test_named_thread() {
        let msg = demonstrate_named_threads();
        assert!(msg.contains("worker-logger-01"));
    }
}
