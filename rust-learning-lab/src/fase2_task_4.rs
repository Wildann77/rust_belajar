// Fase 2 - Task 4: Slices (Array Slices &[T] & String Slices &str)
// Rujukan: rust_learning_guide.md (Bagian 2.4) & rust_execution_tasks.md

/// Menemukan kata pertama dalam sebuah string slice.
/// Menerima `&str` membuat fungsi ini kompatibel dengan `&String`, `&str`, maupun literal `"..."`.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s // jika tidak ada spasi, seluruh string adalah kata pertama
}

/// Menghitung jumlah seluruh elemen pada slice integer.
/// Menerima `&[i32]` membuat fungsi dapat menerima array statis `&[i32; N]`, slice `&arr[1..4]`, maupun `&Vec<i32>`.
pub fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// Mencari nilai maksimum dari sebuah slice integer.
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().max()
}

pub fn run() {
    println!("=== Fase 2 - Task 4: Slices ===");

    // ------------------------------------------------------------------------
    // 1. Array Slice (&[T])
    // ------------------------------------------------------------------------
    let numbers = [10, 20, 30, 40, 50];
    println!("1. Array Slice:");
    println!("   Array penuh: {:?}", numbers);

    // Fat pointer di stack: (pointer ke elemen index 1, panjang = 3)
    let middle_slice: &[i32] = &numbers[1..4];
    println!("   Slice &numbers[1..4]: {:?}", middle_slice);
    println!("   Panjang slice: {}", middle_slice.len());

    let first_two: &[i32] = &numbers[..2];
    let from_three: &[i32] = &numbers[2..];
    let whole_slice: &[i32] = &numbers[..];
    println!("   Slice &numbers[..2]: {:?}", first_two);
    println!("   Slice &numbers[2..]: {:?}", from_three);
    println!("   Slice &numbers[..] : {:?}", whole_slice);

    // ------------------------------------------------------------------------
    // 2. String Slice (&str)
    // ------------------------------------------------------------------------
    let sentence = String::from("Rust Pemrograman Sistem");
    println!("\n2. String Slice:");
    println!("   String asal: \"{sentence}\"");

    // Slicing string berdasarkan range byte
    let word1: &str = &sentence[0..4];   // "Rust"
    let word2: &str = &sentence[5..16];  // "Pemrograman"
    let word3: &str = &sentence[17..];   // "Sistem"
    println!("   Slice [0..4]  : \"{word1}\"");
    println!("   Slice [5..16] : \"{word2}\"");
    println!("   Slice [17..]  : \"{word3}\"");

    // ------------------------------------------------------------------------
    // 3. Function Menerima &str (Idiomatis & Deref Coercion)
    // ------------------------------------------------------------------------
    println!("\n3. Function Menerima &str:");
    let sample_string = String::from("Antigravity IDE Google");
    let sample_literal = "Halo Dunia Rust";

    // Deref coercion: &String otomatis diubah menjadi &str
    let first1 = first_word(&sample_string);
    let first2 = first_word(sample_literal);
    let first3 = first_word(&sample_string[12..]); // slice dari string

    println!("   first_word(&String): \"{first1}\"");
    println!("   first_word(literal): \"{first2}\"");
    println!("   first_word(&slice) : \"{first3}\"");

    // ------------------------------------------------------------------------
    // 4. Function Menerima &[i32]
    // ------------------------------------------------------------------------
    println!("\n4. Function Menerima &[i32]:");
    let total_all = sum_slice(&numbers);
    let total_mid = sum_slice(&numbers[1..4]); // [20, 30, 40]
    let max_val = find_max(&numbers[1..4]);

    println!("   sum_slice(&numbers)      : {total_all}");
    println!("   sum_slice(&numbers[1..4]): {total_mid}");
    println!("   find_max(&numbers[1..4]) : {:?}", max_val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("Hello World"), "Hello");
        assert_eq!(first_word("Rustacean"), "Rustacean");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_sum_slice() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_slice(&arr), 15);
        assert_eq!(sum_slice(&arr[0..3]), 6);
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn test_find_max() {
        let arr = [10, 50, 30, 90, 20];
        assert_eq!(find_max(&arr), Some(90));
        assert_eq!(find_max(&arr[0..3]), Some(50));
        assert_eq!(find_max(&[]), None);
    }
}
