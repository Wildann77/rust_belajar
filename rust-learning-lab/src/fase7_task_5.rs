// Fase 7 - Task 5: Newtype Pattern & Type Safety
// Rujukan: rust_learning_guide.md (Sub-bab 7.5) & rust_execution_tasks.md (L696-L705)

use std::fmt::{self, Display, Formatter};
use std::mem;

// ----------------------------------------------------------------------------
// 1. Definisi Newtype Structs (UserId & OrderId)
// ----------------------------------------------------------------------------

/// Newtype untuk identifier unik Pengguna (User).
///
/// Membungkus tipe primitif `u64` dalam tuple struct 1 elemen.
/// Memberikan jaminan keamanan tipe kompilasi (compile-time type safety)
/// tanpa overhead performa runtime (Zero-Cost Abstraction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "USR-{:05}", self.0)
    }
}

impl From<u64> for UserId {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

/// Newtype untuk identifier unik Pesanan (Order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(pub u64);

impl OrderId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Display for OrderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ORD-{:06}", self.0)
    }
}

impl From<u64> for OrderId {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

// ----------------------------------------------------------------------------
// 2. Pembuktian Type Safety: Primitive Obsession vs Newtype Pattern
// ----------------------------------------------------------------------------

/// Versi Rentan Bug (Primitive Obsession): Menggunakan `u64` untuk semua parameter ID.
///
/// ⚠️ RISIKO FATAL:
/// Jika pemanggil fungsi secara tidak sengaja menukar urutan argumen `order_id` dan `user_id`,
/// compiler Rust TIDAK AKAN mendeteksi error apapun karena tipe keduanya sama-sama `u64`!
pub fn process_order_unsafe(user_id: u64, order_id: u64) -> String {
    format!("Memproses pesanan ID #{order_id} untuk User #{user_id}")
}

/// Versi Type-Safe (Newtype Pattern): Menggunakan `UserId` dan `OrderId`.
///
/// 🛡️ PROTEKSI MUTLAK:
/// Jika urutan argumen tertukar (`process_order_safe(order_id, user_id)`),
/// compiler Rust LANGSUNG MENOLAK kompilasi dengan error jelas:
/// "expected `UserId`, found `OrderId`".
pub fn process_order_safe(user_id: UserId, order_id: OrderId) -> String {
    format!("Memproses pesanan {order_id} untuk User {user_id}")
}

// ----------------------------------------------------------------------------
// 3. Manfaat Sekunder Newtype: Menembus Batasan Orphan Rule
// ----------------------------------------------------------------------------

/// Aturan Orphan Rule Rust: Trait hanya boleh diimplementasikan jika Trait ATAU Tipe
/// didefinisikan di crate lokal kita.
///
/// Kita DILARANG mengimplementasikan `Display` langsung pada `Vec<String>` karena keduanya
/// berasal dari `std` (eksternal).
///
/// Solusi: Bungkus `Vec<String>` ke dalam Newtype tuple struct lokal!
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagList(pub Vec<String>);

impl Display for TagList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let joined = self.0.join(", ");
        write!(f, "[Tags: {joined}]")
    }
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 7 - Task 5: Newtype Pattern & Type Safety       ===");
    println!("============================================================");

    let user_id = UserId::new(42);
    let order_id = OrderId::new(100889);

    // 1. Primitive Obsession vs Type Safety
    println!("\n1. Mengapa Type Safety Lebih Baik daripada Raw u64?");
    println!("   a. Versi Rentan (u64):");
    let res_unsafe_ok = process_order_unsafe(user_id.raw(), order_id.raw());
    println!("      * Argumen benar : {res_unsafe_ok}");

    // Simulasi human error: parameter tertukar
    let res_unsafe_bug = process_order_unsafe(order_id.raw(), user_id.raw());
    println!("      * Argumen tertukar: {res_unsafe_bug} ❌ (Bug semantik lolos tanpa error!)");

    println!("\n   b. Versi Type-Safe (Newtype):");
    let res_safe = process_order_safe(user_id, order_id);
    println!("      * Eksekusi aman : {res_safe} ✓");
    println!(
        "      * Jika dibalik `process_order_safe(order_id, user_id)`: Ditolak total oleh compiler!"
    );

    // 2. Evaluasi Ukuran Memori (Zero-Cost Abstraction)
    println!("\n2. Analisis Ukuran Memori (Zero-Cost Abstraction):");
    println!(
        "   - Ukuran u64 mentah      : {} bytes",
        mem::size_of::<u64>()
    );
    println!(
        "   - Ukuran UserId (Newtype): {} bytes",
        mem::size_of::<UserId>()
    );
    println!(
        "   - Ukuran OrderId (Newtype): {} bytes",
        mem::size_of::<OrderId>()
    );
    println!(
        "   -> Kesimpulan: Newtype memiliki runtime cost = 0! Pembungkus lenyap saat kompilasi."
    );

    // 3. Menembus Orphan Rule via Newtype
    println!("\n3. Bypass Orphan Rule via Newtype (Display untuk Vec<String>):");
    let tags = TagList(vec![
        "rust".to_string(),
        "type-safety".to_string(),
        "newtype".to_string(),
        "zero-cost".to_string(),
    ]);
    println!("   - Cetak TagList terformat via Display: {tags}");
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_creation_and_values() {
        let u = UserId::new(10);
        assert_eq!(u.raw(), 10);
        assert_eq!(u.0, 10);
        assert_eq!(format!("{u}"), "USR-00010");

        let o = OrderId::from(500);
        assert_eq!(o.raw(), 500);
        assert_eq!(format!("{o}"), "ORD-000500");
    }

    #[test]
    fn test_order_processing_correctness() {
        let u = UserId::new(1);
        let o = OrderId::new(99);
        let out = process_order_safe(u, o);
        assert!(out.contains("ORD-000099"));
        assert!(out.contains("USR-00001"));
    }

    #[test]
    fn test_zero_cost_memory_size() {
        assert_eq!(mem::size_of::<UserId>(), mem::size_of::<u64>());
        assert_eq!(mem::size_of::<OrderId>(), mem::size_of::<u64>());
    }

    #[test]
    fn test_tag_list_orphan_rule_bypass() {
        let tags = TagList(vec!["A".to_string(), "B".to_string()]);
        assert_eq!(format!("{tags}"), "[Tags: A, B]");
    }
}
