// Mini Project Fase 10: Multi-Threaded Shared Counter (Smart Pointers & Concurrency)
// Rujukan: rust_learning_guide.md (Sub-bab 10.11) & rust_execution_tasks.md (L903-L917)

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Abstraksi Counter Bersama Terproteksi Mutex lintas Thread.
#[derive(Debug, Clone)]
pub struct SharedCounter {
    counter: Arc<Mutex<i32>>,
}

impl SharedCounter {
    /// Membuat SharedCounter baru dengan nilai awal.
    pub fn new(initial: i32) -> Self {
        Self {
            counter: Arc::new(Mutex::new(initial)),
        }
    }

    /// Mengakses nilai saat ini (membuka kunci lock).
    pub fn get_value(&self) -> i32 {
        *self
            .counter
            .lock()
            .expect("Gagal mengunci Mutex saat membaca nilai")
    }

    /// Menambahkan nilai counter secara langsung.
    pub fn increment_by(&self, amount: i32) {
        let mut guard = self
            .counter
            .lock()
            .expect("Gagal mengunci Mutex saat increment");
        *guard += amount;
    }

    /// Mendapatkan klon pointer `Arc<Mutex<i32>>` untuk diserahkan ke thread lain.
    pub fn get_arc(&self) -> Arc<Mutex<i32>> {
        Arc::clone(&self.counter)
    }

    /// Menjalankan N worker thread secara konkuren di mana setiap worker melakukan sejumlah increment.
    ///
    /// Menjamin hasil akhir DETERMINISTIK karena setiap akses mutasi diproteksi oleh Mutex.
    pub fn execute_workers(
        &self,
        num_workers: usize,
        increments_per_worker: usize,
        step: i32,
    ) -> Vec<JoinHandle<usize>> {
        let mut handles = Vec::with_capacity(num_workers);

        for worker_id in 0..num_workers {
            let counter_clone = Arc::clone(&self.counter);

            let handle = thread::spawn(move || {
                for _ in 0..increments_per_worker {
                    // Scope lock minimal: lock diambil, nilai dimutasi, guard langsung di-drop
                    let mut guard = counter_clone.lock().expect("Mutex poison error");
                    *guard += step;
                }
                worker_id
            });

            handles.push(handle);
        }

        handles
    }
}

// ----------------------------------------------------------------------------
// Evaluasi Kriteria Lulus Fase 10
// ----------------------------------------------------------------------------

/// Evaluasi Kriteria 1: Perbedaan `Rc` vs `Arc`.
pub fn explain_rc_vs_arc() -> (&'static str, &'static str) {
    let rc_desc = "Rc<T> (Single-Thread): Menggunakan penghitung non-atomic (integer biasa). \
                   Sangat cepat tanpa overhead atomik hardware, tetapi TIDAK aman untuk multi-thread (!Send, !Sync).";
    let arc_desc = "Arc<T> (Multi-Thread): Menggunakan Atomic Reference Counting via instruksi CPU atomik \
                    (fetch_add/fetch_sub). Aman dibagikan lintas thread OS (Send + Sync) dengan sedikit overhead CPU.";
    (rc_desc, arc_desc)
}

/// Evaluasi Kriteria 2: Perbedaan `RefCell` vs `Mutex`.
pub fn explain_refcell_vs_mutex() -> (&'static str, &'static str) {
    let refcell_desc = "RefCell<T> (Single-Thread Interior Mutability): Borrow check diperiksa saat RUNTIME. \
                        Hanya untuk 1 thread. Jika aturan dilanggar, seketika terjadi PANIC.";
    let mutex_desc = "Mutex<T> (Multi-Thread Synchronization): Memproteksi mutabilitas bersama lintas thread. \
                      Thread yang berebut tidak panic, melainkan DIBLOKIR / ANTRE (sleep) hingga lock dilepas via RAII guard.";
    (refcell_desc, mutex_desc)
}

/// Evaluasi Kriteria 3: Kapan Memilih `Mutex` vs `RwLock`.
pub fn explain_mutex_vs_rwlock() -> (&'static str, &'static str) {
    let mutex_choice = "Pilih Mutex<T>: Saat beban kerja dominan TULIS (write-heavy) atau mutasi berlangsung sering. \
                        Overhead locking Mutex jauh lebih ringan dan sederhana dibandingkan RwLock.";
    let rwlock_choice = "Pilih RwLock<T>: Saat beban kerja dominan BACA (read-heavy, misal 90% baca, 10% tulis). \
                         Banyak thread pembaca (read lock) dapat mengakses data bersamaan secara simultan tanpa saling tunggu.";
    (mutex_choice, rwlock_choice)
}

// ----------------------------------------------------------------------------
// Runner Interaktif (Demonstrasi Lengkap)
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== Mini Project Fase 10: Multi-Threaded Shared Counter ===");

    // 1. Inisialisasi Counter bersama
    let initial_value = 0;
    let shared_counter = SharedCounter::new(initial_value);
    println!("1. Inisialisasi: Arc<Mutex<i32>> dengan nilai awal = {initial_value}");

    // 2. Konfigurasi 10 Worker Threads
    let num_workers = 10;
    let increments_per_worker = 100;
    let step = 1;
    let expected_final = initial_value + (num_workers as i32 * increments_per_worker as i32 * step);

    println!("2. Memulai {num_workers} Worker Threads...");
    println!("   - Tiap worker melakukan {increments_per_worker}x increment (+{step})");
    println!("   - Target nilai akhir deterministik: {expected_final}");

    let handles = shared_counter.execute_workers(num_workers, increments_per_worker, step);

    // 3. Join seluruh thread worker
    for handle in handles {
        let finished_worker_id = handle.join().expect("Worker thread panic");
        let _ = finished_worker_id;
    }
    println!("3. Seluruh {num_workers} thread sukses di-join (selesai).");

    // 4. Verifikasi Hasil Akhir Deterministik
    let final_value = shared_counter.get_value();
    println!("4. Hasil Akhir Counter Terbaca: {final_value}");
    if final_value == expected_final {
        println!("   [✓] Sukses: Hasil akhir 100% DETERMINISTIK tanpa data race!");
    } else {
        println!("   [✗] Gagal: Data race terdeteksi!");
    }

    // Demonstrasi get_arc() & increment_by()
    let direct_arc = shared_counter.get_arc();
    let arc_val = *direct_arc.lock().unwrap();
    shared_counter.increment_by(5);
    println!(
        "   - get_arc() membaca: {arc_val}, setelah increment_by(+5): {}",
        shared_counter.get_value()
    );

    // 5. Evaluasi Kriteria Lulus Fase 10
    println!("\n--- Evaluasi & Bukti Kriteria Lulus Fase 10 ---");
    let (rc_info, arc_info) = explain_rc_vs_arc();
    println!("A. Perbedaan Rc vs Arc:");
    println!("   - {rc_info}");
    println!("   - {arc_info}");

    let (refcell_info, mutex_info) = explain_refcell_vs_mutex();
    println!("\nB. Perbedaan RefCell vs Mutex:");
    println!("   - {refcell_info}");
    println!("   - {mutex_info}");

    let (mutex_ch, rwlock_ch) = explain_mutex_vs_rwlock();
    println!("\nC. Panduan Memilih Mutex vs RwLock:");
    println!("   - {mutex_ch}");
    println!("   - {rwlock_ch}");

    println!("\n[OK] Mini Project Fase 10 selesai & terverifikasi.");
}

// ----------------------------------------------------------------------------
// Unit Tests Komprehensif
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_counter_deterministic_10_workers() {
        let counter = SharedCounter::new(0);
        let num_workers = 10;
        let increments_per_worker = 50;
        let step = 2; // Tiap worker tambah 100 total
        let expected = 10 * 50 * 2; // 1000

        let handles = counter.execute_workers(num_workers, increments_per_worker, step);
        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get_value(), expected);
    }

    #[test]
    fn test_single_increment_by_10_workers() {
        let counter = SharedCounter::new(100);
        let num_workers = 10;
        let increments_per_worker = 1;
        let step = 1;
        let expected = 110;

        let handles = counter.execute_workers(num_workers, increments_per_worker, step);
        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get_value(), expected);
    }

    #[test]
    fn test_manual_arc_mutex_sharing() {
        let shared = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let clone = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                let mut guard = clone.lock().unwrap();
                *guard += 1;
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*shared.lock().unwrap(), 10);
    }

    #[test]
    fn test_fase10_graduation_explanations() {
        let (rc, arc) = explain_rc_vs_arc();
        assert!(rc.contains("Single-Thread"));
        assert!(arc.contains("Multi-Thread"));

        let (refcell, mutex) = explain_refcell_vs_mutex();
        assert!(refcell.contains("RUNTIME"));
        assert!(mutex.contains("DIBLOKIR"));

        let (mut_ch, rw_ch) = explain_mutex_vs_rwlock();
        assert!(mut_ch.contains("TULIS"));
        assert!(rw_ch.contains("BACA"));
    }
}
