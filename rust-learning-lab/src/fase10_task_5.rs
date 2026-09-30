// Fase 10 - Task 5: Mutex<T> dan RwLock<T> (Shared Mutable State, RAII Guards, Read/Write Locking)
// Rujukan: rust_learning_guide.md (Sub-bab 10.10) & rust_execution_tasks.md (L895-L901)

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

// ============================================================================
// 1. Mutex<T>: Mutual Exclusion, Lock, Guard & Scope Lock
// ============================================================================

/// Struct rekening bank untuk menguji mutasi saldo bersama lintas thread secara aman.
#[derive(Debug)]
#[allow(dead_code)]
pub struct BankAccount {
    pub account_number: String,
    pub balance: f64,
}

impl BankAccount {
    pub fn new(account_number: &str, initial_balance: f64) -> Self {
        Self {
            account_number: account_number.to_string(),
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}

/// Mendemonstrasikan penggunaan Mutex dengan pembatasan durasi lock via RAII Scope Lock.
pub fn demonstrate_mutex_scope_lock() -> f64 {
    let account = Arc::new(Mutex::new(BankAccount::new("ACC-9901", 1000.0)));
    let mut handles = Vec::new();

    // 5 worker threads masing-masing setor Rp200.0
    for _ in 0..5 {
        let acc_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            // Scope lock eksplisit: Guard dilepas segera setelah transaksi selesai
            {
                // .lock() memblokir thread sampai mendapatkan giliran akses
                let mut guard = acc_clone.lock().expect("Gagal mengunci Mutex");
                // MutexGuard mengimplementasikan DerefMut sehingga bisa memanggil deposit()
                guard.deposit(200.0);
                // guard keluar dari scope di kurung kurawal ini -> Lock otomatis terbuka (RAII Drop)!
            }

            // Kode di luar scope lock ini berjalan tanpa menahan antrean thread lain
            thread::sleep(Duration::from_millis(5));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Baca saldo akhir (1000 + 5 * 200 = 2000)
    let final_balance = account.lock().unwrap().balance;
    final_balance
}

// ============================================================================
// 2. RwLock<T>: Read Lock vs Write Lock (Single Writer / Multiple Readers)
// ============================================================================

/// In-memory cache data yang sering dibaca oleh banyak thread, namun jarang diubah.
#[derive(Debug, Default)]
pub struct SharedConfigCache {
    pub version: u32,
    pub values: Vec<String>,
}

/// Mendemonstrasikan bahwa banyak thread pembaca (read lock) dapat membaca bersamaan tanpa saling tunggu.
pub fn demonstrate_rwlock_read_concurrency(num_readers: usize) -> Vec<u32> {
    let cache = Arc::new(RwLock::new(SharedConfigCache {
        version: 1,
        values: vec![String::from("host=localhost"), String::from("port=8080")],
    }));

    let mut handles = Vec::new();

    // Spawn banyak pembaca simultan
    for _ in 0..num_readers {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            // .read() mengembalikan RwLockReadGuard
            // Banyak pembaca boleh memegang read lock bersamaan!
            let read_guard = cache_clone.read().expect("Gagal mendapatkan read lock");
            // Simulasi operasi baca
            let ver = read_guard.version;
            thread::sleep(Duration::from_millis(10));
            ver
        });
        handles.push(handle);
    }

    let mut versions = Vec::new();
    for handle in handles {
        versions.push(handle.join().unwrap());
    }

    versions
}

/// Mendemonstrasikan bahwa penulisan (write lock) bersifat eksklusif.
pub fn demonstrate_rwlock_write_exclusivity() -> (u32, bool) {
    let cache = Arc::new(RwLock::new(SharedConfigCache {
        version: 1,
        values: vec![String::from("init")],
    }));

    // 1. Ambil write lock
    let mut write_guard = cache.write().expect("Gagal mendapatkan write lock");
    write_guard.version = 2;
    write_guard.values.push(String::from("updated"));

    // 2. Coba baca selagi write lock aktif -> try_read() HARUS gagal!
    let try_read_success = cache.try_read().is_ok(); // false, karena sedang di-lock eksklusif oleh write_guard

    // 3. Lepaskan write guard secara eksplisit
    let final_version = write_guard.version;
    drop(write_guard);

    // 4. Sekarang read lock dapat diperoleh kembali
    assert!(cache.try_read().is_ok());

    (final_version, try_read_success)
}

// ============================================================================
// 3. Deteksi Non-Blocking: try_lock()
// ============================================================================

/// Menguji apakah Mutex sedang terkunci tanpa memblokir thread pemanggil.
pub fn check_mutex_contention() -> bool {
    let mutex = Mutex::new(42);
    let _guard = mutex.lock().unwrap();

    // Karena _guard masih aktif, try_lock() akan mengembalikan Err(TryLockError::WouldBlock)
    let is_busy = mutex.try_lock().is_err();
    is_busy
}

// ============================================================================
// 4. Entry Point Eksekusi Modul
// ============================================================================

pub fn run() {
    println!("=== FASE 10 TASK 5: MUTEX<T> & RWLOCK<T> ===");

    // 1. Mutex Scope Lock & Shared Mutable State
    println!("\n1. Mutex<T> Shared Mutable State & Scope Lock:");
    let balance = demonstrate_mutex_scope_lock();
    println!("   - Saldo awal: Rp1000.0, 5 worker masing-masing setor Rp200.0");
    println!("   - Saldo akhir (determinik berkat Mutex): Rp{balance:.2}");
    assert_eq!(balance, 2000.0);

    // 2. Mutex Contention Test (try_lock)
    println!("\n2. Non-blocking try_lock():");
    let is_busy = check_mutex_contention();
    println!(
        "   - Apakah try_lock() mendeteksi lock sedang sibuk? {is_busy} (Aman tanpa deadlock!)"
    );

    // 3. RwLock Read Concurrency
    println!("\n3. RwLock<T> Konkurensi Banyak Pembaca (Read Lock):");
    let versions = demonstrate_rwlock_read_concurrency(4);
    println!("   - Hasil baca simultan dari 4 thread: {:?}", versions);
    assert_eq!(versions, vec![1, 1, 1, 1]);
    println!("   - Terbukti: 4 thread pembaca membaca secara paralel tanpa saling blokir!");

    // 4. RwLock Write Exclusivity
    println!("\n4. RwLock<T> Eksklusivitas Penulis (Write Lock):");
    let (new_ver, read_during_write) = demonstrate_rwlock_write_exclusivity();
    println!("   - Versi cache setelah write lock: v{new_ver}");
    println!("   - Apakah pembaca diizinkan masuk selagi write lock aktif? {read_during_write}");
    println!("   - Terbukti: Write lock mengisolasi data 100% dari semua pembaca & penulis lain!");

    println!("\n[OK] Task Fase 10 (Mutex / RwLock) selesai & terverifikasi.");
}

// ============================================================================
// Unit Tests Komprehensif
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_deposit_deterministic() {
        let final_balance = demonstrate_mutex_scope_lock();
        assert_eq!(final_balance, 2000.0);
    }

    #[test]
    fn test_mutex_try_lock_conflict() {
        assert!(check_mutex_contention());
    }

    #[test]
    fn test_rwlock_multiple_readers() {
        let versions = demonstrate_rwlock_read_concurrency(5);
        assert_eq!(versions.len(), 5);
        assert!(versions.iter().all(|&v| v == 1));
    }

    #[test]
    fn test_rwlock_write_isolation() {
        let (ver, read_allowed) = demonstrate_rwlock_write_exclusivity();
        assert_eq!(ver, 2);
        assert!(!read_allowed);
    }

    #[test]
    fn test_mutex_raii_guard_release() {
        let m = Mutex::new(10);
        {
            let mut g = m.lock().unwrap();
            *g += 5;
        } // guard keluar scope di sini
        assert_eq!(*m.lock().unwrap(), 15);
    }
}
