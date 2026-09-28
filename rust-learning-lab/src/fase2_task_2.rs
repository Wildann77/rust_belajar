// Fase 2 - Task 2: Function & Ownership
// Rujukan: rust_learning_guide.md (Bagian 2.1, 2.2, 2.3) & rust_execution_tasks.md

/// Mengambil alih kepemilikan String dari pemanggil.
/// Begitu fungsi selesai, `s` keluar dari scope dan di-drop otomatis.
pub fn takes_ownership(s: String) {
    println!("   [takes_ownership] Menerima \"{s}\". Kepemilikan ada di sini.");
    // s di-drop otomatis di sini!
}

/// Menghasilkan String baru dan mentransfer kepemilikan keluar ke pemanggil fungsi.
pub fn gives_ownership() -> String {
    let some_string = String::from("diberikan_oleh_fungsi");
    println!("   [gives_ownership] Membuat dan memberikan String baru.");
    some_string // ownership ditransfer ke pemanggil
}

/// Menggunakan borrow (&String) tanpa mengambil alih kepemilikan.
/// Nilai hanya dibaca (read-only), pemanggil tetap menjadi owner sah.
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Alternatif lebih fleksibel: menerima string slice (&str)
/// Menerima baik &String maupun &str tanpa memindahkan ownership.
pub fn print_via_borrow(s: &str) {
    println!("   [print_via_borrow] Meminjam: \"{s}\" (tanpa ambil ownership)");
}

pub fn run() {
    println!("=== Fase 2 - Task 2: Function & Ownership ===");

    // 1. Observasi takes_ownership: kepemilikan berpindah (Move ke argumen fungsi)
    let s_orig = String::from("Halo Rust");
    println!("1. Sebelum takes_ownership: s_orig = \"{s_orig}\"");
    takes_ownership(s_orig);
    // s_orig tidak bisa dipakai lagi di sini!
    // println!("{s_orig}"); // COMPILE ERROR: use of moved value: `s_orig`
    println!("   Setelah takes_ownership: s_orig sudah di-drop oleh fungsi penerima.");

    // 2. Observasi gives_ownership: fungsi mentransfer kepemilikan ke variabel pemanggil
    let received_str = gives_ownership();
    println!("2. gives_ownership diterima: received_str = \"{received_str}\"");

    // 3. Menggunakan Borrow (&String) via calculate_length:
    // Kepemilikan TIDAK berpindah.
    let my_text = String::from("Rust 2024");
    let len = calculate_length(&my_text);
    println!("3. calculate_length(&my_text):");
    println!("   Panjang: {len} bytes");
    println!("   my_text tetap valid = \"{my_text}\" (karena hanya dipinjam via &)");

    // 4. Ubah fungsi / gunakan borrow (&str) untuk kenyamanan
    print_via_borrow(&my_text);
    println!("   my_text masih tetap valid setelah dipinjam = \"{my_text}\"");

    // 5. Kapan Clone diperlukan saat passing ke function?
    // Jawab: Ketika function membutuhkan kepemilikan penuh (String),
    // TETAPI pemanggil masih perlu menggunakan variabel aslinya setelah fungsi selesai.
    let preserved_text = String::from("Data Penting");
    println!("5. Kapan clone diperlukan:");
    println!("   Passing clone: takes_ownership(preserved_text.clone())");
    takes_ownership(preserved_text.clone()); // Hanya duplikat heap yang di-move
    println!("   preserved_text asli tetap aman & valid = \"{preserved_text}\"");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_does_not_consume_ownership() {
        let text = String::from("Antigravity");
        let len = calculate_length(&text);
        assert_eq!(len, 11);
        // text masih bisa diakses
        assert_eq!(text, "Antigravity");
    }

    #[test]
    fn test_gives_ownership_transfers_valid_string() {
        let s = gives_ownership();
        assert_eq!(s, "diberikan_oleh_fungsi");
        assert_eq!(s.len(), 21);
    }

    #[test]
    fn test_clone_before_passing_preserves_original() {
        let original = String::from("Tetap Hidup");
        takes_ownership(original.clone());
        assert_eq!(original, "Tetap Hidup");
    }
}
