// Fase 9 - Task 2: Lazy Iterators, Iterator Adaptors, dan Consuming Adaptors
// Rujukan: rust_learning_guide.md (Sub-bab 9.2) & rust_execution_tasks.md (L802-L825)

// ----------------------------------------------------------------------------
// 1. Tiga Metode Penghasil Iterator: iter(), iter_mut(), dan into_iter()
// ----------------------------------------------------------------------------

/// 1. `.iter()`: Meminjam elemen secara immutable (&T).
///
/// Koleksi asli tidak berubah dan tidak dipindahkan kepemilikannya.
pub fn demonstrate_iter(items: &[i32]) -> Vec<i32> {
    // .iter() menghasilkan iterator dengan Item = &i32
    items.iter().map(|&x| x * 2).collect()
}

/// 2. `.iter_mut()`: Meminjam elemen secara mutable (&mut T).
///
/// Memungkinkan mutasi langsung elemen di dalam koleksi secara in-place.
pub fn demonstrate_iter_mut(items: &mut [i32], addition: i32) {
    // .iter_mut() menghasilkan iterator dengan Item = &mut i32
    for item in items.iter_mut() {
        *item += addition;
    }
}

/// 3. `.into_iter()`: Mengonsumsi koleksi dan mengambil kepemilikan elemen (T).
///
/// Setelah dipanggil, koleksi asli dipindahkan (moved) dan tidak bisa diakses lagi.
pub fn demonstrate_into_iter(items: Vec<String>) -> Vec<String> {
    // .into_iter() memindahkan kepemilikan tiap elemen String ke dalam iterator
    items
        .into_iter()
        .map(|s| format!("[Processed: {s}]"))
        .collect()
}

// ----------------------------------------------------------------------------
// 2. Iterator Pipeline: iter() -> filter() -> map() -> take() -> collect()
// ----------------------------------------------------------------------------

/// Membangun pipeline lazy:
/// numbers.iter().filter(...).map(...).take(...).collect()
///
/// Sifat penting:
/// Iterator di Rust bersifat LAZY (malas). Tidak ada komputasi yang dieksekusi
/// sampai method consuming adaptor terminal seperti `.collect()` dipanggil!
pub fn build_lazy_pipeline(
    numbers: &[i32],
    is_even: bool,
    multiplier: i32,
    limit: usize,
) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| if is_even { x % 2 == 0 } else { x % 2 != 0 }) // Filter predikat
        .map(|&x| x * multiplier) // Transformasi nilai
        .take(limit) // Batasi jumlah elemen
        .collect() // Terminal: kumpulkan hasil
}

// ----------------------------------------------------------------------------
// 3. Consuming Adaptors: find, any, all, fold, collect
// ----------------------------------------------------------------------------

/// `.find()`: Mencari elemen pertama yang memenuhi kriteria predikat.
///
/// Bersifat short-circuiting: berhenti mengevaluasi begitu elemen ditemukan.
/// Mengembalikan Option<&T> jika dari .iter().
pub fn find_first_gt<'a>(numbers: &'a [i32], threshold: i32) -> Option<&'a i32> {
    numbers.iter().find(|&&x| x > threshold)
}

/// `.any()` dan `.all()`: Predikat boolean pada iterator.
///
/// Bersifat short-circuiting:
/// - `any`: berhenti dan kembalikan true begitu ada 1 elemen cocok.
/// - `all`: berhenti dan kembalikan false begitu ada 1 elemen tidak cocok.
pub fn verify_predicates(numbers: &[i32], min_val: i32) -> (bool, bool) {
    let has_negative = numbers.iter().any(|&x| x < 0);
    let all_above_min = numbers.iter().all(|&x| x >= min_val);
    (has_negative, all_above_min)
}

/// `.fold()`: Mengakumulasi seluruh elemen menjadi satu nilai tunggal.
///
/// Menggunakan state awal (accumulator) dan closure penggabung:
/// `fold(init, |acc, item| ...)`
pub fn calculate_stats_with_fold(numbers: &[i32]) -> (i32, i64) {
    // 1. Hitung total penjumlahan via fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);

    // 2. Hitung total perkalian via fold
    let product = numbers.iter().fold(1i64, |acc, &x| acc * (x as i64));

    (sum, product)
}

// ----------------------------------------------------------------------------
// 4. Custom Iterator (Implementasi Trait Iterator Mandiri)
// ----------------------------------------------------------------------------

/// Struct penghitung Fibonacci kustom pembukti kesederhanaan trait Iterator di Rust.
#[derive(Debug, Clone)]
pub struct Fibonacci {
    pub curr: u64,
    pub next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

// Hanya butuh mengimplementasikan method `next(&mut self) -> Option<Item>`
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        let result = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

// ----------------------------------------------------------------------------
// 5. Runner Interaktif (Demonstrasi Lengkap)
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== FASE 9: Functional Rust - Task 2 (Iterators) ===");

    let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 1. iter()
    println!("\n1. Menggunakan .iter() [Immutable Borrow]:");
    let doubled = demonstrate_iter(&source);
    println!("   - Source tetap utuh: {:?}", source);
    println!("   - Hasil doubled: {:?}", doubled);

    // 2. iter_mut()
    println!("\n2. Menggunakan .iter_mut() [Mutable Borrow In-Place]:");
    let mut mutable_data = vec![10, 20, 30];
    println!("   - Data sebelum: {:?}", mutable_data);
    demonstrate_iter_mut(&mut mutable_data, 5);
    println!("   - Data setelah (+5 in-place): {:?}", mutable_data);

    // 3. into_iter()
    println!("\n3. Menggunakan .into_iter() [Ownership Transfer]:");
    let words = vec![
        String::from("rust"),
        String::from("iterator"),
        String::from("zero-cost"),
    ];
    let processed_words = demonstrate_into_iter(words);
    println!("   - Hasil into_iter: {:?}", processed_words);

    // 4. Pipeline: iter() -> filter() -> map() -> take() -> collect()
    println!("\n4. Pipeline Lazy: filter -> map -> take -> collect:");
    let pipeline_res = build_lazy_pipeline(&source, true, 3, 3);
    println!("   - Input numbers: {:?}", source);
    println!("   - Filter Genap, Kali 3, Ambil 3: {:?}", pipeline_res);

    // 5. find()
    println!("\n5. Consuming Adaptor .find():");
    let found = find_first_gt(&source, 7);
    println!("   - Angka pertama > 7: {:?}", found);

    // 6. any() dan all()
    println!("\n6. Predikat .any() dan .all():");
    let (has_neg, all_ge_zero) = verify_predicates(&source, 0);
    println!("   - Mengandung angka negatif? {has_neg}");
    println!("   - Semua angka >= 0? {all_ge_zero}");

    // 7. fold()
    println!("\n7. Reduksi .fold():");
    let sample = [1, 2, 3, 4, 5];
    let (sum, prod) = calculate_stats_with_fold(&sample);
    println!("   - Data: {:?}", sample);
    println!("   - Sum via fold: {sum}");
    println!("   - Product via fold: {prod}");

    // 8. Custom Iterator Fibonacci
    println!("\n8. Custom Iterator (Fibonacci):");
    let fib_first_8: Vec<u64> = Fibonacci::new().take(8).collect();
    println!("   - 8 angka pertama Fibonacci: {:?}", fib_first_8);

    println!("\n[OK] Task 2 Iterators selesai & terverifikasi.");
}

// ----------------------------------------------------------------------------
// Unit Tests Komprehensif
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_immutable_borrow() {
        let nums = vec![1, 2, 3];
        let res = demonstrate_iter(&nums);
        assert_eq!(res, vec![2, 4, 6]);
        // nums tetap bisa dibaca setelah .iter()
        assert_eq!(nums.len(), 3);
    }

    #[test]
    fn test_iter_mut_in_place_modification() {
        let mut nums = vec![5, 15, 25];
        demonstrate_iter_mut(&mut nums, 10);
        assert_eq!(nums, vec![15, 25, 35]);
    }

    #[test]
    fn test_into_iter_ownership_consumption() {
        let strings = vec![String::from("alpha"), String::from("beta")];
        let processed = demonstrate_into_iter(strings);
        assert_eq!(
            processed,
            vec![
                String::from("[Processed: alpha]"),
                String::from("[Processed: beta]")
            ]
        );
    }

    #[test]
    fn test_lazy_pipeline_filter_map_take_collect() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Genap: [2, 4, 6, 8, 10] -> Kali 10: [20, 40, 60, 80, 100] -> Take 3: [20, 40, 60]
        let res = build_lazy_pipeline(&numbers, true, 10, 3);
        assert_eq!(res, vec![20, 40, 60]);

        // Ganjil: [1, 3, 5, 7, 9] -> Kali 2: [2, 6, 10, 14, 18] -> Take 2: [2, 6]
        let odd_res = build_lazy_pipeline(&numbers, false, 2, 2);
        assert_eq!(odd_res, vec![2, 6]);
    }

    #[test]
    fn test_find_adaptor() {
        let numbers = [10, 25, 30, 45, 50];
        assert_eq!(find_first_gt(&numbers, 25), Some(&30));
        assert_eq!(find_first_gt(&numbers, 100), None);
    }

    #[test]
    fn test_any_and_all_adaptors() {
        let positive = [1, 2, 3, 4, 5];
        let (has_neg, all_ge_zero) = verify_predicates(&positive, 0);
        assert!(!has_neg);
        assert!(all_ge_zero);

        let mixed = [1, -2, 3];
        let (has_neg_mixed, all_ge_zero_mixed) = verify_predicates(&mixed, 0);
        assert!(has_neg_mixed);
        assert!(!all_ge_zero_mixed);
    }

    #[test]
    fn test_fold_sum_and_product() {
        let items = [1, 2, 3, 4];
        let (sum, prod) = calculate_stats_with_fold(&items);
        assert_eq!(sum, 10);
        assert_eq!(prod, 24);

        let empty: [i32; 0] = [];
        let (sum_empty, prod_empty) = calculate_stats_with_fold(&empty);
        assert_eq!(sum_empty, 0);
        assert_eq!(prod_empty, 1);
    }

    #[test]
    fn test_custom_fibonacci_iterator() {
        let fibs: Vec<u64> = Fibonacci::new().take(6).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5]);
    }
}
