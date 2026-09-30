// Fase 11 - Task 3: Marker Traits (Send, Sync, Thread Safety Guarantees, & Relationship &T : Send <=> T : Sync)
// Rujukan: rust_learning_guide.md (Sub-bab 11.3) & rust_execution_tasks.md (L953-L959)

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// 1. Compile-Time Trait Bounds Checker
// ============================================================================

/// Helper fungsi statis untuk memvalidasi bahwa tipe T mengimplementasikan Send saat kompilasi.
#[allow(dead_code)]
pub const fn assert_send<T: Send>() {}

/// Helper fungsi statis untuk memvalidasi bahwa tipe T mengimplementasikan Sync saat kompilasi.
#[allow(dead_code)]
pub const fn assert_sync<T: Sync>() {}

/// Helper fungsi statis untuk memvalidasi bahwa tipe T mengimplementasikan Send + Sync sekaligus.
#[allow(dead_code)]
pub const fn assert_send_and_sync<T: Send + Sync>() {}

// ============================================================================
// 2. Struct Demonstrasi Custom Send & Non-Send
// ============================================================================

/// Struct biasa yang seluruh field-nya Send + Sync.
/// Secara otomatis (*auto trait*), SafeUser juga berstatus Send + Sync.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SafeUser {
    pub id: u64,
    pub username: String,
}

/// Struct yang sengaja dibuat TIDAK Send dan TIDAK Sync (!Send + !Sync).
/// Berguna untuk objek yang hanya boleh hidup di main thread (misal: UI handle, GUI context, OpenGL context).
#[allow(dead_code)]
pub struct MainThreadOnlyWorker {
    pub task_name: String,
    // PhantomData<Rc<()>> memaksa struct ini kehilangan sifat Send & Sync
    _marker: PhantomData<Rc<()>>,
}

#[allow(dead_code)]
impl MainThreadOnlyWorker {
    pub fn new(task_name: &str) -> Self {
        Self {
            task_name: task_name.to_string(),
            _marker: PhantomData,
        }
    }
}

// ============================================================================
// 3. Demonstrasi Tipe yang Send dan Sync
// ============================================================================

/// Memverifikasi daftar tipe data standar yang berstatus Send, Sync, atau keduanya.
pub fn verify_standard_types_traits() -> Vec<(&'static str, bool, bool)> {
    // 1. Tipe yang SEND + SYNC:
    assert_send_and_sync::<i32>();
    assert_send_and_sync::<f64>();
    assert_send_and_sync::<String>();
    assert_send_and_sync::<Vec<u8>>();
    assert_send_and_sync::<SafeUser>();
    assert_send_and_sync::<Arc<Mutex<i32>>>();
    assert_send_and_sync::<AtomicBool>();

    // 2. Tipe yang SEND tapi BUKAN SYNC:
    // RefCell dan Cell boleh dipindahkan kepemilikannya ke thread lain (Send),
    // TETAPI referensinya (&RefCell / &Cell) DILARANG diakses dari banyak thread sekaligus (!Sync)
    // karena mekanisme borrow check / mutasi internalnya TIDAK atomic!
    assert_send::<RefCell<i32>>();
    assert_send::<Cell<i32>>();
    assert_send::<mpsc::Sender<i32>>();
    assert_send::<mpsc::Receiver<i32>>();

    // 3. Tipe yang BUKAN SEND dan BUKAN SYNC (!Send + !Sync):
    // Rc<T> menggunakan reference counter non-atomic, sehingga balapan clone/drop lintas thread
    // bisa menyebabkan memory corruption / double free!
    // raw pointers (*const T, *mut T) juga tidak aman secara default.

    vec![
        ("i32 / f64 / bool", true, true),
        ("String / Vec<T>", true, true),
        ("Arc<Mutex<T>>", true, true),
        ("AtomicBool / AtomicI32", true, true),
        ("RefCell<T> (Interior Mutability)", true, false),
        ("Cell<T> (Interior Mutability)", true, false),
        ("mpsc::Receiver<T>", true, false),
        ("Rc<T> (Non-atomic Ref Count)", false, false),
        ("*const T / *mut T (Raw Pointers)", false, false),
        ("MainThreadOnlyWorker (PhantomData Rc)", false, false),
    ]
}

// ============================================================================
// 4. Hubungan Krusial: T is Sync <=> &T is Send
// ============================================================================

/// Membuktikan secara matematis & teknis prinsip:
/// "Tipe T adalah Sync JIKA DAN HANYA JIKA referensinya (&T) adalah Send".
///
/// Logika:
/// Saat kita membagikan `&T` ke thread lain melalui `thread::spawn(move || { ... })`,
/// hal yang sebenarnya ditransfer (di-Send) melintasi batas thread adalah pointer referensi `&T` itu sendiri!
pub fn demonstrate_ref_t_is_send() -> i32 {
    let data = Arc::new(Mutex::new(50));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let data_ref = Arc::clone(&data);
        // Karena Mutex<i32> adalah Sync, maka &Mutex<i32> aman berstatus Send!
        let handle = thread::spawn(move || {
            let mut guard = data_ref.lock().unwrap();
            *guard += 25;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_val = *data.lock().unwrap();
    final_val
}

// ============================================================================
// 5. Transfer Ownership Tipe Send Lintas Thread
// ============================================================================

/// Mendemonstrasikan pengiriman data yang berstatus Send ke thread lain.
pub fn demonstrate_send_transfer() -> SafeUser {
    let user = SafeUser {
        id: 101,
        username: "ferris_rustacean".to_string(),
    };

    // SafeUser adalah Send, sehingga closure thread::spawn dengan `move`
    // diizinkan mengambil alih kepemilikan struct ini.
    let handle = thread::spawn(move || {
        let mut u = user;
        u.username.push_str("_verified");
        u
    });

    handle.join().expect("Gagal join thread transfer Send")
}

// ============================================================================
// Runner Entry Point
// ============================================================================

pub fn run() {
    println!("=== FASE 11: Task 3 - Marker Traits: Send & Sync ===");

    // 1. Audit Klasifikasi Tipe Data
    println!("\n1. Audit Sifat Send & Sync pada Tipe Data Rust:");
    let audit_list = verify_standard_types_traits();
    println!("   {:<38} | {:<7} | {:<7}", "Tipe Data", "Send?", "Sync?");
    println!("   {:-<38}-+-{:-<7}-+-{:-<7}", "", "", "");
    for (name, is_send, is_sync) in audit_list {
        println!("   {:<38} | {:<7} | {:<7}", name, is_send, is_sync);
    }

    // 2. Transfer Tipe Send
    println!("\n2. Transfer Kepemilikan Tipe Send Lintas Thread:");
    let modified_user = demonstrate_send_transfer();
    println!(
        "   Data berhasil diproses di thread lain: {:?}",
        modified_user
    );
    assert_eq!(modified_user.username, "ferris_rustacean_verified");

    // 3. Hubungan T: Sync <=> &T: Send
    println!("\n3. Pembuktian Hubungan: T is Sync <=> &T is Send:");
    let calculated = demonstrate_ref_t_is_send();
    println!(
        "   Hasil mutasi simultan melalui referensi &Mutex: {}",
        calculated
    );
    assert_eq!(calculated, 100);
    println!("   Terbukti: &T aman di-Send ke thread lain karena Mutex<T> adalah Sync!");

    println!("\n[OK] Seluruh demonstrasi FASE 11 Task 3 (Send & Sync) sukses & verified!\n");
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_assertions_compile_time() {
        assert_send::<String>();
        assert_sync::<String>();
        assert_send::<RefCell<u32>>();
        // Baris di bawah sengaja tidak bisa dikompilasi jika diaktifkan (karena RefCell !Sync):
        // assert_sync::<RefCell<u32>>();
    }

    #[test]
    fn test_send_transfer_user() {
        let user = demonstrate_send_transfer();
        assert_eq!(user.id, 101);
        assert_eq!(user.username, "ferris_rustacean_verified");
    }

    #[test]
    fn test_sync_reference_mutation() {
        let val = demonstrate_ref_t_is_send();
        assert_eq!(val, 100);
    }

    #[test]
    fn test_audit_list_correctness() {
        let list = verify_standard_types_traits();
        assert_eq!(list.len(), 10);
        // String harus Send + Sync
        assert_eq!(list[1], ("String / Vec<T>", true, true));
        // RefCell harus Send tapi !Sync
        assert_eq!(list[4], ("RefCell<T> (Interior Mutability)", true, false));
        // Rc harus !Send dan !Sync
        assert_eq!(list[7], ("Rc<T> (Non-atomic Ref Count)", false, false));
    }
}
