// Fase 7 - Task 1: Generic Functions & Trait Bounds
// Rujukan: rust_learning_guide.md (Sub-bab 7.1) & rust_execution_tasks.md (L645-L650)

use std::fmt::{Debug, Display};

// ----------------------------------------------------------------------------
// 1. Generic Function: Memilih Nilai Terbesar (find_largest)
// ----------------------------------------------------------------------------

/// Mencari referensi ke elemen dengan nilai terbesar dari sebuah slice generic.
/// 
/// Memerlukan trait bound `PartialOrd` agar operator perbandingan `>` valid untuk tipe `T`.
/// Mengembalikan `Option<&T>`:
/// - `Some(&T)` jika slice memiliki minimal satu elemen.
/// - `None` jika slice kosong (mencegah panic/runtime crash).
pub fn find_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

// ----------------------------------------------------------------------------
// 2. Generic Function: Mencetak Nilai (print_item, print_collection)
// ----------------------------------------------------------------------------

/// Mencetak nilai tunggal generic yang mengimplementasikan trait `Display`.
pub fn print_item<T: Display>(label: &str, item: &T) {
    println!("{label}: {item}");
}

/// Mencetak representasi debug generic yang mengimplementasikan trait `Debug`.
pub fn print_debug_item<T: Debug>(label: &str, item: &T) {
    println!("{label} (Debug): {item:?}");
}

/// Mencetak koleksi generic yang seluruh elemennya mengimplementasikan `Display`.
pub fn print_collection<T: Display>(header: &str, items: &[T]) {
    println!("{header}:");
    if items.is_empty() {
        println!("  (koleksi kosong)");
        return;
    }
    for (idx, item) in items.iter().enumerate() {
        println!("  [{idx}] {item}");
    }
}

// ----------------------------------------------------------------------------
// 3. Generic Function: Mengubah Collection (transform_collection)
// ----------------------------------------------------------------------------

/// Mentransformasikan vector tipe `T` menjadi vector tipe `U` baru menggunakan fungsi/closure generic.
/// 
/// Menggunakan `where clause` untuk keterbacaan trait bounds:
/// - `F: FnMut(T) -> U` : Closure pemetaan yang menerima nilai kepemilikan `T` dan menghasilkan `U`.
pub fn transform_collection<T, U, F>(items: Vec<T>, mut transform_fn: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        result.push(transform_fn(item));
    }
    result
}

/// Mengubah elemen slice `&mut [T]` secara langsung di tempat (*in-place mutation*).
/// 
/// Tidak mengalokasikan memori baru di heap.
pub fn transform_collection_mut<T, F>(items: &mut [T], mut transform_fn: F)
where
    F: FnMut(&mut T),
{
    for item in items.iter_mut() {
        transform_fn(item);
    }
}

// ----------------------------------------------------------------------------
// Tipe Kustom untuk Mendemonstrasikan Generic pada Objek Kompleks
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerScore {
    pub name: String,
    pub score: u32,
}

impl PartialOrd for PlayerScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player '{}' [Score: {}]", self.name, self.score)
    }
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 7 - Task 1: Generic Functions & Trait Bounds    ===");
    println!("============================================================");

    // 1. Generic Function: Memilih Nilai Terbesar
    println!("\n1. Memilih Nilai Terbesar (find_largest):");
    
    // a. Integer slice
    let int_list = [42, 108, 17, 99, 5];
    if let Some(max_int) = find_largest(&int_list) {
        println!("   - Integer terbesar dari {int_list:?}: {max_int}");
    }

    // b. Float slice
    let float_list = [3.14, 2.71, 9.81, 1.41];
    if let Some(max_float) = find_largest(&float_list) {
        println!("   - Float terbesar dari {float_list:?}: {max_float}");
    }

    // c. String slice (&str)
    let words = ["rust", "generics", "monomorphization", "traits"];
    if let Some(max_word) = find_largest(&words) {
        println!("   - Word terbesar secara alfabetis dari {words:?}: \"{max_word}\"");
    }

    // d. Struct Kustom (PlayerScore)
    let leader_board = [
        PlayerScore { name: "Alice".to_string(), score: 320 },
        PlayerScore { name: "Bob".to_string(), score: 550 },
        PlayerScore { name: "Charlie".to_string(), score: 410 },
    ];
    if let Some(champion) = find_largest(&leader_board) {
        println!("   - Skor tertinggi (Custom Struct): {champion}");
    }

    // e. Empty Slice
    let empty_list: [i32; 0] = [];
    let empty_res = find_largest(&empty_list);
    println!("   - Slice kosong: {empty_res:?} (Aman, zero panic)");

    // 2. Generic Function: Mencetak Nilai
    println!("\n2. Mencetak Nilai (print_item & print_collection):");
    print_item("   - Single Item (i32)", &1337);
    print_item("   - Single Item (String)", &"Rust Zero-Cost Abstractions".to_string());
    print_debug_item("   - Debug Item", &vec!["A", "B", "C"]);

    let cities = ["Jakarta", "Bandung", "Surabaya", "Yogyakarta"];
    print_collection("   - Daftar Kota", &cities);

    // 3. Generic Function: Mengubah Collection
    println!("\n3. Mengubah Collection (transform_collection & transform_collection_mut):");

    // a. Mapping T -> U: Mengubah Vec<i32> menjadi Vec<String>
    let raw_numbers = vec![1, 2, 3, 4, 5];
    let formatted_labels: Vec<String> = transform_collection(raw_numbers, |num| {
        format!("Item #{num} (Kuadrat: {})", num * num)
    });
    println!("   - Transformasi Pemetaan (Vec<i32> -> Vec<String>):");
    for label in &formatted_labels {
        println!("     * {label}");
    }

    // b. In-place Mutation: Mengalikan dua setiap elemen &mut [i32]
    let mut scores = vec![10, 20, 30, 40];
    println!("   - In-place mutation sebelum: {scores:?}");
    transform_collection_mut(&mut scores, |val| *val *= 2);
    println!("   - In-place mutation setelah (x2): {scores:?}");
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_integers() {
        let numbers = [12, 45, 2, 89, 34];
        assert_eq!(find_largest(&numbers), Some(&89));
    }

    #[test]
    fn test_find_largest_floats() {
        let floats = [1.5, 9.9, 3.2, 0.1];
        assert_eq!(find_largest(&floats), Some(&9.9));
    }

    #[test]
    fn test_find_largest_strings() {
        let items = ["apple", "zebra", "banana"];
        assert_eq!(find_largest(&items), Some(&"zebra"));
    }

    #[test]
    fn test_find_largest_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(find_largest(&empty), None);
    }

    #[test]
    fn test_find_largest_custom_struct() {
        let players = [
            PlayerScore { name: "Player1".to_string(), score: 100 },
            PlayerScore { name: "Player2".to_string(), score: 500 },
            PlayerScore { name: "Player3".to_string(), score: 250 },
        ];
        let champion = find_largest(&players);
        assert!(champion.is_some());
        assert_eq!(champion.unwrap().name, "Player2");
        assert_eq!(champion.unwrap().score, 500);
    }

    #[test]
    fn test_transform_collection_mapping() {
        let input = vec![1, 2, 3];
        let result: Vec<String> = transform_collection(input, |x| format!("val:{}", x * 10));
        assert_eq!(result, vec!["val:10", "val:20", "val:30"]);
    }

    #[test]
    fn test_transform_collection_mut() {
        let mut data = vec![5, 10, 15];
        transform_collection_mut(&mut data, |x| *x += 1);
        assert_eq!(data, vec![6, 11, 16]);
    }
}
