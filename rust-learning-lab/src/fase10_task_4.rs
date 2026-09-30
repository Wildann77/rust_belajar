// Fase 10 - Task 4: Smart Pointer Arc<T> (Atomic Reference Counting & Multi-Thread Shared Value)
// Rujukan: rust_learning_guide.md (Sub-bab 10.9) & rust_execution_tasks.md (L890-L893)

use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================================================
// 1. Immutable Shared State Antar-Thread via Arc<T>
// ============================================================================

/// Konfigurasi aplikasi read-only yang dibagikan ke banyak worker threads.
#[derive(Debug)]
#[allow(dead_code)]
pub struct AppConfig {
    pub service_name: String,
    pub max_connections: usize,
    pub api_endpoints: Vec<String>,
}

impl AppConfig {
    pub fn default_config() -> Self {
        Self {
            service_name: String::from("RustLearningServer"),
            max_connections: 100,
            api_endpoints: vec![
                String::from("/api/v1/auth"),
                String::from("/api/v1/tasks"),
                String::from("/api/v1/health"),
            ],
        }
    }
}

/// Mendemonstrasikan pembagian konfigurasi yang sama ke 3 thread pekerja (worker).
pub fn demonstrate_shared_config(num_workers: usize) -> Vec<String> {
    // 1. Bungkus AppConfig dalam Arc (Heap allocation + Atomic ref-counter)
    let config = Arc::new(AppConfig::default_config());
    let mut handles = Vec::new();

    for worker_id in 1..=num_workers {
        // 2. Arc::clone HANYA meng-increment atomic counter, bukan deep copy data heap!
        let config_clone = Arc::clone(&config);

        let handle = thread::spawn(move || {
            // Tiap thread membaca data yang sama dari heap tanpa konflik
            format!(
                "Worker #{} terhubung ke '{}' [Endpoint: {}]",
                worker_id,
                config_clone.service_name,
                config_clone.api_endpoints[worker_id % config_clone.api_endpoints.len()]
            )
        });

        handles.push(handle);
    }

    // 3. Kumpulkan hasil dari semua thread pekerja
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().expect("Worker thread gagal dieksekusi"));
    }

    results
}

// ============================================================================
// 2. Pemrosesan Data Paralel (Concurrent Read / Map-Reduce Sederhana)
// ============================================================================

/// Membagi pembacaan dataset besar ke 4 thread secara paralel menggunakan Arc.
///
/// Tiap thread menjumlahkan bagian data tertentu tanpa menyalin dataset.
pub fn parallel_sum(dataset: Arc<Vec<i64>>, chunk_count: usize) -> i64 {
    let len = dataset.len();
    let chunk_size = (len + chunk_count - 1) / chunk_count;
    let mut handles = Vec::new();

    for i in 0..chunk_count {
        let data_ref = Arc::clone(&dataset);
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, len);

        if start >= len {
            break;
        }

        let handle = thread::spawn(move || {
            let chunk_sum: i64 = data_ref[start..end].iter().sum();
            chunk_sum
        });

        handles.push(handle);
    }

    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().expect("Thread gagal join");
    }

    total_sum
}

// ============================================================================
// 3. Siklus Hidup strong_count pada Arc Lintas Thread
// ============================================================================

/// Memverifikasi peningkatan dan penurunan strong_count saat thread aktif dan selesai.
pub fn observe_arc_refcount_lifecycle() -> (usize, usize, usize) {
    let shared = Arc::new(vec![10, 20, 30, 40]);
    let initial_count = Arc::strong_count(&shared); // = 1

    let shared_thread = Arc::clone(&shared);
    let count_after_clone = Arc::strong_count(&shared); // = 2

    let handle = thread::spawn(move || {
        // Simulasi komputasi singkat
        thread::sleep(Duration::from_millis(10));
        let sum: i32 = shared_thread.iter().sum();
        sum
    });

    let _sum = handle.join().unwrap();
    // Setelah thread selesai dan shared_thread di-drop di dalam thread:
    let count_after_join = Arc::strong_count(&shared); // Kembali = 1

    (initial_count, count_after_clone, count_after_join)
}

// ============================================================================
// 4. Entry Point Eksekusi Modul
// ============================================================================

pub fn run() {
    println!("=== FASE 10 TASK 4: SMART POINTER ARC<T> ===");

    // 1. Shared Config across Worker Threads
    println!("\n1. Berbagi Nilai Immutable ke Multi-Thread (Arc::clone):");
    let worker_reports = demonstrate_shared_config(3);
    for report in &worker_reports {
        println!("   - {report}");
    }

    // 2. Lifecycle Ref-Count Atomik
    println!("\n2. Siklus Hidup Arc strong_count:");
    let (c_init, c_cloned, c_joined) = observe_arc_refcount_lifecycle();
    println!("   - strong_count awal (main thread)          : {c_init}");
    println!("   - strong_count setelah clone untuk thread  : {c_cloned}");
    println!("   - strong_count setelah worker thread join  : {c_joined}");

    // 3. Parallel Sum (Map-Reduce Sederhana)
    println!("\n3. Pemrosesan Paralel Tanpa Copy Heap Memori:");
    let numbers: Vec<i64> = (1..=1000).collect();
    let expected_sum: i64 = numbers.iter().sum();
    let shared_numbers = Arc::new(numbers);

    println!("   - Alamat Heap dataset: {:p}", shared_numbers.as_slice());
    let computed_sum = parallel_sum(Arc::clone(&shared_numbers), 4);
    println!("   - Total komputasi 4 worker thread: {computed_sum}");
    println!("   - Nilai ekspektasi (1..=1000)     : {expected_sum}");
    assert_eq!(computed_sum, expected_sum);
    println!("   - Verifikasi integritas: COCOK 100%!");

    println!("\n[OK] Task Fase 10 (Arc) selesai & terverifikasi.");
}

// ============================================================================
// Unit Tests Komprehensif
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_config_across_threads() {
        let reports = demonstrate_shared_config(4);
        assert_eq!(reports.len(), 4);
        for report in reports {
            assert!(report.contains("RustLearningServer"));
        }
    }

    #[test]
    fn test_parallel_sum_correctness() {
        let data: Vec<i64> = (1..=100).collect();
        let expected: i64 = data.iter().sum();
        let arc_data = Arc::new(data);

        let sum_2_chunks = parallel_sum(Arc::clone(&arc_data), 2);
        let sum_4_chunks = parallel_sum(Arc::clone(&arc_data), 4);

        assert_eq!(sum_2_chunks, expected);
        assert_eq!(sum_4_chunks, expected);
    }

    #[test]
    fn test_arc_refcount_lifecycle() {
        let (init, cloned, joined) = observe_arc_refcount_lifecycle();
        assert_eq!(init, 1);
        assert_eq!(cloned, 2);
        assert_eq!(joined, 1);
    }

    #[test]
    fn test_multi_thread_address_identity() {
        let data = Arc::new(String::from("SharedMemoryLocation"));
        let ptr_main = Arc::as_ptr(&data) as usize;

        let data_thread = Arc::clone(&data);
        let handle = thread::spawn(move || Arc::as_ptr(&data_thread) as usize);

        let ptr_thread = handle.join().unwrap();
        // Membuktikan thread membaca dari lokasi heap memori yang sama persis
        assert_eq!(ptr_main, ptr_thread);
    }
}
