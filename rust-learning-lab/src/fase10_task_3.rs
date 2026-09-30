// Fase 10 - Task 3: Smart Pointer RefCell<T> & Interior Mutability
// Rujukan: rust_learning_guide.md (Sub-bab 10.8) & rust_execution_tasks.md (L883-L888)

use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;

// ============================================================================
// 1. Konsep Dasar Interior Mutability: borrow() dan borrow_mut()
// ============================================================================

/// Counter sederhana untuk mendemonstrasikan mutasi di balik referensi immutable (&self).
#[derive(Debug, Default)]
pub struct MockCounter {
    // Data dibungkus dalam RefCell agar bisa dimutasi meskipun struct ini dipinjam secara immutable (&MockCounter)
    count: RefCell<u32>,
}

impl MockCounter {
    pub fn new() -> Self {
        Self {
            count: RefCell::new(0),
        }
    }

    /// Membaca nilai melalui .borrow() (Ref<u32>)
    ///
    /// Menerima &self (immutable borrow), bukan &mut self!
    pub fn get(&self) -> u32 {
        *self.count.borrow()
    }

    /// Memodifikasi nilai melalui .borrow_mut() (RefMut<u32>)
    ///
    /// Menerima &self (immutable borrow), tetapi tetap dapat memutasi data internal!
    pub fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }
}

/// Mendemonstrasikan pembacaan simultan (multiple borrow) dan penulisan eksklusif (borrow_mut).
pub fn demonstrate_borrow_and_borrow_mut() -> (u32, u32) {
    let cell = RefCell::new(10);

    // 1. Banyak pembaca aktif bersamaan (Multiple Immutable Borrows)
    {
        let r1 = cell.borrow();
        let r2 = cell.borrow();
        assert_eq!(*r1, 10);
        assert_eq!(*r2, 10);
        // r1 dan r2 di-drop di akhir scope ini
    }

    // 2. Satu penulis eksklusif (Single Mutable Borrow)
    {
        let mut w = cell.borrow_mut();
        *w += 25;
        // w di-drop di akhir scope ini
    }

    let final_val = *cell.borrow();
    (10, final_val)
}

// ============================================================================
// 2. Eksperimen Double Mutable Borrow & Runtime Panic
// ============================================================================

/// Sengaja memicu pelanggaran aturan Borrowing saat runtime:
/// Mengambil 2 mutable borrow bersamaan dari RefCell yang sama.
///
/// Menggunakan `panic::catch_unwind` agar program tidak langsung terminate,
/// melainkan menangkap pesan error panic secara terkendali.
pub fn demonstrate_double_borrow_mut_panic() -> Result<(), String> {
    let cell = RefCell::new(100);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _w1 = cell.borrow_mut();
        // PELANGGARAN ATURAN BORROWING!
        // _w1 masih aktif, tetapi kita mencoba borrow_mut() sekali lagi:
        let _w2 = cell.borrow_mut(); // Memicu PANIC saat RUNTIME!
    }));

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            // Ambil pesan panic dari runtime
            if let Some(msg) = err.downcast_ref::<&str>() {
                Err(msg.to_string())
            } else if let Some(msg) = err.downcast_ref::<String>() {
                Err(msg.clone())
            } else {
                Err("already borrowed: BorrowMutError".to_string())
            }
        }
    }
}

/// Alternatif Aman Tanpa Panic: try_borrow() dan try_borrow_mut()
///
/// Mengembalikan Result<Ref<T>, BorrowError> atau Result<RefMut<T>, BorrowMutError>
pub fn demonstrate_try_borrow_safe() -> (bool, bool) {
    let cell = RefCell::new(50);

    let _w = cell.borrow_mut(); // w aktif

    // Coba borrow secara aman saat w sedang aktif
    let can_read = cell.try_borrow().is_ok(); // Harus false (Err: BorrowError)
    let can_write_again = cell.try_borrow_mut().is_ok(); // Harus false (Err: BorrowMutError)

    (can_read, can_write_again)
}

// ============================================================================
// 3. Pola Populer di Rust: Rc<RefCell<T>> (Shared Mutable State)
// ============================================================================

/// Trait pengirim pesan / logger
pub trait Messenger {
    fn send(&self, msg: &str);
}

/// Mock Messenger yang mencatat histori pesan yang dikirimkan.
/// Digunakan pada unit testing untuk memverifikasi apakah fungsi memanggil send().
pub struct MockMessenger {
    // Vector log dibungkus dalam RefCell agar method send(&self) bisa menambahkan log
    pub sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> Self {
        Self {
            sent_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // Mutasi vector di dalam method yang hanya menerima referensi immutable (&self)
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}

/// Struktur data bersama di mana banyak node dapat saling memutasi data yang sama
#[derive(Debug)]
pub struct SharedNode {
    pub id: String,
    pub shared_data: Rc<RefCell<Vec<String>>>,
}

impl SharedNode {
    pub fn new(id: &str, shared_data: Rc<RefCell<Vec<String>>>) -> Self {
        Self {
            id: id.to_string(),
            shared_data,
        }
    }

    pub fn append(&self, text: &str) {
        let entry = format!("[{}] {}", self.id, text);
        self.shared_data.borrow_mut().push(entry);
    }
}

// ============================================================================
// 4. Entry Point Eksekusi Modul
// ============================================================================

pub fn run() {
    println!("=== FASE 10 TASK 3: SMART POINTER REFCELL<T> ===");

    // 1. borrow() & borrow_mut()
    println!("\n1. Demonstrasi borrow() dan borrow_mut():");
    let (initial, updated) = demonstrate_borrow_and_borrow_mut();
    println!("   - Nilai awal cell          : {initial}");
    println!("   - Nilai setelah borrow_mut : {updated}");

    let counter = MockCounter::new();
    counter.increment();
    counter.increment();
    println!(
        "   - MockCounter dipanggil via &counter (&self): count = {}",
        counter.get()
    );

    // 2. Eksperimen Double Mutable Borrow & Runtime Panic
    println!("\n2. Eksperimen Sengaja Double Mutable Borrow:");
    match demonstrate_double_borrow_mut_panic() {
        Ok(_) => println!("   - Tidak terjadi panic (tidak terduga)"),
        Err(err_msg) => {
            println!("   - Terbukti: Panic tertangkap di Runtime!");
            println!("   - Pesan Panic: \"already borrowed: BorrowMutError\"");
            println!("   - Detail error raw: {err_msg}");
        }
    }

    // 3. Alternatif Aman: try_borrow() dan try_borrow_mut()
    println!("\n3. Pemeriksaan Tanpa Panic (try_borrow & try_borrow_mut):");
    let (can_read, can_write) = demonstrate_try_borrow_safe();
    println!("   - try_borrow() saat borrow_mut aktif    : {can_read} (Aman! Mengembalikan Err)");
    println!("   - try_borrow_mut() saat borrow_mut aktif: {can_write} (Aman! Mengembalikan Err)");

    // 4. Pola Rc<RefCell<T>> (Multi-owner Mutable State)
    println!("\n4. Pola Kolaborasi Rc<RefCell<T>>:");
    let shared_log = Rc::new(RefCell::new(Vec::new()));

    let node_a = SharedNode::new("Service-A", Rc::clone(&shared_log));
    let node_b = SharedNode::new("Service-B", Rc::clone(&shared_log));

    node_a.append("Inisialisasi koneksi");
    node_b.append("Menerima request #101");
    node_a.append("Menulis transaksi database");
    node_b.append("Mengirim HTTP Response 200");

    println!("   - Isi log bersama (diakses & dimutasi oleh Node A dan Node B):");
    for (i, log) in shared_log.borrow().iter().enumerate() {
        println!("     {}. {}", i + 1, log);
    }

    // 5. Mock Pattern Testing
    println!("\n5. Mock Messenger Pattern (Interior Mutability):");
    let messenger = MockMessenger::new();
    messenger.send("Alert: CPU usage > 90%");
    messenger.send("Alert: Memory usage > 85%");
    println!(
        "   - Total pesan terkirim yang dicatat: {}",
        messenger.sent_messages.borrow().len()
    );

    println!("\n[OK] Task Fase 10 (RefCell) selesai & terverifikasi.");
}

// ============================================================================
// Unit Tests Komprehensif
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_and_borrow_mut() {
        let (init, updated) = demonstrate_borrow_and_borrow_mut();
        assert_eq!(init, 10);
        assert_eq!(updated, 35);
    }

    #[test]
    fn test_mock_counter_interior_mutability() {
        let c = MockCounter::new();
        assert_eq!(c.get(), 0);
        c.increment();
        c.increment();
        c.increment();
        assert_eq!(c.get(), 3);
    }

    #[test]
    fn test_double_borrow_mut_panics() {
        let res = demonstrate_double_borrow_mut_panic();
        assert!(res.is_err());
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_direct_double_borrow_mut_panic_verification() {
        let cell = RefCell::new(42);
        let _w1 = cell.borrow_mut();
        let _w2 = cell.borrow_mut(); // Panics directly
    }

    #[test]
    fn test_try_borrow_safe_returns_err() {
        let (can_read, can_write) = demonstrate_try_borrow_safe();
        assert!(!can_read);
        assert!(!can_write);
    }

    #[test]
    fn test_rc_refcell_shared_mutability() {
        let shared = Rc::new(RefCell::new(Vec::new()));
        let node_a = SharedNode::new("A", Rc::clone(&shared));
        let node_b = SharedNode::new("B", Rc::clone(&shared));

        node_a.append("Msg1");
        node_b.append("Msg2");

        let logs = shared.borrow();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "[A] Msg1");
        assert_eq!(logs[1], "[B] Msg2");
    }

    #[test]
    fn test_mock_messenger() {
        let mock = MockMessenger::new();
        mock.send("Test 1");
        mock.send("Test 2");
        assert_eq!(mock.sent_messages.borrow().len(), 2);
        assert_eq!(mock.sent_messages.borrow()[0], "Test 1");
    }
}
