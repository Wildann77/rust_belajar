// Fase 8 - Task 1: Lifetimes Mendalam ('a, Structs, Impls, Elision Rules, 'static)
// Rujukan: rust_learning_guide.md (Sub-bab 8.1 - 8.9) & rust_execution_tasks.md (L728-L773)

use std::fmt::{self, Debug, Display, Formatter};

// ----------------------------------------------------------------------------
// 1. Dasar Lifetime: fn longest<'a>(...) & Eksperimen first<'a>(...)
// ----------------------------------------------------------------------------

/// Menemukan string slice terpanjang di antara dua input.
/// 
/// Mengapa butuh lifetime generic parameter `'a`?
/// Rust borrow checker saat kompilasi tidak tahu cabang mana (`if` atau `else`)
/// yang akan dieksekusi saat runtime. Anotasi `'a` memberitahu compiler bahwa
/// referensi kembalian valid selama irisan masa hidup (intersection/overlap)
/// terpendek antara `x` dan `y`.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// Eksperimen: Mengembalikan referensi pertama (`a`).
/// 
/// Meskipun parameter `b` juga dianotasi dengan `'a`, nilai kembalian hanya berasal dari `a`.
pub fn first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}

// ----------------------------------------------------------------------------
// 2. Multiple Lifetime Parameters ('a dan 'b)
// ----------------------------------------------------------------------------

/// Fungsi dengan dua parameter lifetime independen: `'a` dan `'b`.
/// 
/// Jika hanya menggunakan satu lifetime `'a` untuk `primary` dan `context`,
/// maka masa hidup nilai kembalian akan dibatasi oleh masa hidup `context` yang lebih pendek!
/// Dengan memisahkan `'a` dan `'b`, `context` boleh memiliki scope lokal yang sangat sempit
/// tanpa mengurangi masa hidup referensi kembalian yang bersumber dari `primary`.
pub fn choose_first_with_context<'a, 'b>(primary: &'a str, context: &'b str) -> &'a str {
    let _log = format!("[Context: {context}]");
    primary
}

// ----------------------------------------------------------------------------
// 3. Lifetime pada Struct & Reference sebagai Field
// ----------------------------------------------------------------------------

/// Struct yang memegang referensi sebagai field (`&'a str`).
/// 
/// ATURAN EMAS:
/// Instance dari struct `Parser<'a>` TIDAK BOLEH hidup lebih lama daripada
/// data string yang direferensikan oleh field `source`.
#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub cursor: usize,
}

/// Struct sederhana penyimpan kutipan teks referensial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

// ----------------------------------------------------------------------------
// 4. Lifetime pada Blok Implementasi (`impl<'a>`)
// ----------------------------------------------------------------------------

impl<'a> Parser<'a> {
    /// Membuat instance `Parser` baru dengan lifetime `'a`.
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    /// Mengambil token berikutnya berdasarkan spasi/pemisah.
    /// Mengembalikan slice dari `source` dengan lifetime `'a`.
    pub fn next_token(&mut self) -> Option<&'a str> {
        let remaining = &self.source[self.cursor..];
        let trimmed = remaining.trim_start();
        let leading_spaces = remaining.len() - trimmed.len();
        self.cursor += leading_spaces;

        if self.cursor >= self.source.len() {
            return None;
        }

        let slice = &self.source[self.cursor..];
        let token_len = slice.find(char::is_whitespace).unwrap_or(slice.len());
        let token = &slice[..token_len];
        self.cursor += token_len;
        Some(token)
    }

    /// Mengembalikan karakter pada posisi cursor saat ini (Elision Rule 3).
    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    /// Method dengan referensi input tambahan: mendemonstrasikan Elision Rule 3.
    /// Lifetime kembalian otomatis diikatkan ke `&self`, bukan ke `announcement`.
    pub fn announce_and_get_excerpt(&self, announcement: &str) -> &'a str {
        let _msg = format!("[INFO]: {announcement}");
        self.source
    }
}

// ----------------------------------------------------------------------------
// 5. Pembuktian 3 Aturan Lifetime Elision (Otomatisasi Compiler)
// ----------------------------------------------------------------------------

/// Aturan 1 & 2: Satu input reference otomatis mengisi seluruh output reference.
/// 
/// Versi Elided (implisit tanpa anotasi):
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

/// Versi Eksplisit ekuivalen yang dipahami oleh compiler:
pub fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

// ----------------------------------------------------------------------------
// 6. Lifetime Khusus: `'static`
// ----------------------------------------------------------------------------

/// String literal selalu bertipe `&'static str` karena disimpan di segmen data binary.
pub const GLOBAL_SYSTEM_NAME: &'static str = "RUST_LEARNING_SYSTEM_V2";

/// Verifikasi trait bound `T: 'static`.
/// 
/// Artinya: Tipe `T` TIDAK mengandung referensi non-static (hanya tipe owned
/// seperti `String`, `i32`, atau referensi `&'static`).
pub fn verify_static_bound<T: Display + 'static>(val: T) -> String {
    format!("[Static Validated]: {val}")
}

// ----------------------------------------------------------------------------
// 7. Kombinasi Tingkat Lanjut: Generic Tipe + Lifetime
// ----------------------------------------------------------------------------

/// Struct yang menggabungkan parameter generic tipe `T` dan generic lifetime `'a`.
#[derive(Debug)]
pub struct AnnotatedItem<'a, T> {
    pub item: &'a T,
    pub note: &'a str,
}

impl<'a, T: Display> Display for AnnotatedItem<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Catatan: {})", self.item, self.note)
    }
}

/// Fungsi generik dengan trait bound, generic type `T`, dan lifetime `'a`.
pub fn find_first_match<'a, T: PartialEq>(items: &'a [T], target: &T) -> Option<&'a T> {
    for item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

/// Menggabungkan generic lifetime `'a` dengan generic type `T` bertrait bound `Display`.
pub fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    let _log = format!("Pengumuman: {ann}");
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// ----------------------------------------------------------------------------
// 8. Kombinasi Tingkat Lanjut: Trait + Lifetime
// ----------------------------------------------------------------------------

/// Trait yang memiliki parameter lifetime `'a`.
pub trait TextTokenizer<'a> {
    fn tokenize(&'a mut self) -> Vec<&'a str>;
}

impl<'a> TextTokenizer<'a> for Parser<'a> {
    fn tokenize(&'a mut self) -> Vec<&'a str> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }
}

/// Trait yang mengembalikan referensi dengan lifetime `'a`.
pub trait Highlightable<'a> {
    fn get_highlight(&self) -> &'a str;
}

impl<'a> Highlightable<'a> for Excerpt<'a> {
    fn get_highlight(&self) -> &'a str {
        self.part
    }
}

// ----------------------------------------------------------------------------
// 9. Eksperimen Dangling Reference & Analisis Borrow Checker
// ----------------------------------------------------------------------------

/// Penjelasan Simulasi Dangling Reference (Kenapa Ditolak Compiler):
/// 
/// ```rust
/// // ❌ CONTOH ERROR 1: Mengembalikan referensi data lokal yang segera di-drop
/// fn create_dangling_reference() -> &str {
///     let s = String::from("halo lokal");
///     &s // ERROR: returns a value referencing data owned by the current function
/// }      // `s` di-drop di sini! Referensi yang dikembalikan akan menunjuk memori sampah.
/// 
/// // ❌ CONTOH ERROR 2: Struct menyimpan referensi ke variabel lokal scope sempit
/// fn struct_dangling_scenario() {
///     let parser: Parser;
///     {
///         let temporary_string = String::from("data sementara");
///         parser = Parser { source: &temporary_string, cursor: 0 };
///     } // ERROR: `temporary_string` di-drop di sini saat scope blok berakhir!
///     println!("{:?}", parser.source); // `parser` hidup lebih lama dari data yang dipinjamnya.
/// }
/// ```
/// 
/// SOLUSI BEBAS `.clone()`:
/// 1. Transfer kepemilikan utuh (return owned `String` alih-alih `&str`).
/// 2. Pastikan owner data hidup di scope luar (caller scope), bukan di dalam local scope.
pub fn safe_lifetime_retention(source: &str, start: usize, len: usize) -> Option<&str> {
    if start + len <= source.len() {
        Some(&source[start..start + len])
    } else {
        None
    }
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 8: Lifetimes Mendalam ('a, Struct, Impl, Static) ===");
    println!("============================================================");

    // 1. fn longest<'a> & first<'a>
    println!("\n1. Demonstrasi fn longest<'a> dan first<'a>:");
    let string1 = String::from("Bahasa Pemrograman Rust");
    let string2 = "Modern 2024";
    let result_longest = longest(&string1, string2);
    let result_first = first(&string1, string2);
    println!("   - String 1        : \"{string1}\" (len: {})", string1.len());
    println!("   - String 2        : \"{string2}\" (len: {})", string2.len());
    println!("   - Hasil longest() : \"{result_longest}\"");
    println!("   - Hasil first()   : \"{result_first}\"");

    // 2. Multiple Lifetime Parameters ('a vs 'b)
    println!("\n2. Multiple Lifetime Parameters ('a, 'b):");
    let primary_doc = String::from("Dokumen Rahasia Negara");
    let selected_doc;
    {
        let short_lived_context = String::from("Audit Sesi #8812");
        selected_doc = choose_first_with_context(&primary_doc, &short_lived_context);
        println!("   - Konteks aktif di inner scope: \"{short_lived_context}\"");
    }
    // selected_doc tetap valid meskipun short_lived_context sudah di-drop!
    println!("   - Hasil di outer scope: \"{selected_doc}\" ✓ (Bebas dari batasan masa hidup konteks)");

    // 3. Lifetime pada Struct & Impl (Parser<'a>)
    println!("\n3. Lifetime pada Struct dan Impl (Parser<'a>):");
    let raw_payload = String::from("POST /api/v1/auth/login HTTP/1.1");
    let mut parser = Parser::new(&raw_payload);
    println!("   - Sumber Teks      : \"{}\"", parser.source);
    println!("   - Token 1 (Method) : {:?}", parser.next_token().unwrap());
    println!("   - Token 2 (Path)   : {:?}", parser.next_token().unwrap());
    println!("   - Token 3 (Proto)  : {:?}", parser.next_token().unwrap());
    println!("   - Token 4 (Habis)  : {:?}", parser.next_token());
    println!("   - Peek sisa string : {:?}", parser.peek());
    println!("   - Method announce  : \"{}\"", parser.announce_and_get_excerpt("Parsing Header OK"));

    // 4. Lifetime Elision Demonstration
    println!("\n4. Pembuktian Tiga Aturan Lifetime Elision:");
    let phrase = "Zero-Cost Abstraction in Rust";
    let elided_res = first_word(phrase);
    let explicit_res = first_word_explicit(phrase);
    println!("   - Frasa Asli       : \"{phrase}\"");
    println!("   - first_word (Elision Rule 1 & 2): \"{elided_res}\"");
    println!("   - first_word_explicit          : \"{explicit_res}\"");

    // 5. 'static Lifetime & Trait Bound
    println!("\n5. Analisis Lifetime 'static:");
    println!("   - Konstanta Binary Global       : \"{GLOBAL_SYSTEM_NAME}\" (&'static str)");
    let owned_string = String::from("Data Dinamis di Heap");
    let static_validated_1 = verify_static_bound(GLOBAL_SYSTEM_NAME);
    let static_validated_2 = verify_static_bound(owned_string);
    println!("   - {static_validated_1}");
    println!("   - {static_validated_2} (String owned memenuhi trait bound T: 'static)");

    // 6. Generic + Lifetime & Trait + Lifetime
    println!("\n6. Generic Tipe + Lifetime & Trait + Lifetime:");
    let score = 98.75;
    let note = "Nilai evaluasi borrow checker sempurna";
    let annotated = AnnotatedItem { item: &score, note };
    println!("   - AnnotatedItem<f64>: {annotated}");

    let int_array = [10, 20, 30, 40, 50];
    let query_val = 30;
    let found = find_first_match(&int_array, &query_val);
    println!("   - find_first_match pada array: {:?}", found);

    let ann_res = longest_with_announcement("Alpha", "BetaGamma", 2026);
    println!("   - longest_with_announcement: \"{ann_res}\"");

    let excerpt = Excerpt { part: "Rust guarantees memory safety without garbage collection" };
    println!("   - Trait Highlightable: \"{}\"", excerpt.get_highlight());

    let mut query_parser = Parser::new("SELECT name FROM users");
    let tokens = query_parser.tokenize();
    println!("   - Trait TextTokenizer: {:?}", tokens);

    // 7. Evaluasi Dangling Reference & Validasi Kriteria Lulus Fase 8
    println!("\n7. Evaluasi Pemahaman Lifetime & Kriteria Lulus Fase 8:");
    let retained_slice = safe_lifetime_retention("Penyimpanan Memori Aman", 0, 11);
    println!("   - safe_lifetime_retention: {:?}", retained_slice);
    println!("   [x] Fungsi 'a: Menandai hubungan validitas antar referensi bagi borrow checker.");
    println!("   [x] Non-extending: Lifetime tidak memperpanjang umur memori objek yang dipinjam.");
    println!("   [x] Lifetime Elision: 3 aturan deterministik yang mengotomatisasi anotasi.");
    println!("   [x] Struct Reference: Struct Parser<'a> dan Excerpt<'a> valid selama sumber referensi hidup.");
}

// ----------------------------------------------------------------------------
// Unit Tests Komprehensif
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_behavior() {
        let s1 = "Singkat";
        let s2 = "Sangat Panjang Sekali";
        assert_eq!(longest(s1, s2), s2);
        assert_eq!(longest(s2, s1), s2);

        let same_a = "Sama";
        let same_b = "Beda";
        assert_eq!(longest(same_a, same_b), same_a);
    }

    #[test]
    fn test_first_behavior() {
        let a = "Utama";
        let b = "Cadangan";
        assert_eq!(first(a, b), "Utama");
    }

    #[test]
    fn test_multiple_lifetime_scope_independence() {
        let outer_primary = String::from("Data Utama Abadi");
        let result;
        {
            let inner_ctx = String::from("Scope Pendek");
            result = choose_first_with_context(&outer_primary, &inner_ctx);
            assert_eq!(result, "Data Utama Abadi");
        }
        // Pastikan result masih valid setelah inner_ctx lenyap
        assert_eq!(result, "Data Utama Abadi");
    }

    #[test]
    fn test_parser_and_tokenizer() {
        let query = String::from("SELECT * FROM users WHERE active = 1");
        let mut parser = Parser::new(&query);

        assert_eq!(parser.next_token(), Some("SELECT"));
        assert_eq!(parser.next_token(), Some("*"));
        assert_eq!(parser.next_token(), Some("FROM"));
        assert_eq!(parser.next_token(), Some("users"));
        assert_eq!(parser.next_token(), Some("WHERE"));
        assert_eq!(parser.next_token(), Some("active"));
        assert_eq!(parser.next_token(), Some("="));
        assert_eq!(parser.next_token(), Some("1"));
        assert_eq!(parser.next_token(), None);
    }

    #[test]
    fn test_parser_trait_tokenizer() {
        let source_code = String::from("let mut x = 42;");
        let mut parser = Parser::new(&source_code);
        let tokens: Vec<&str> = parser.tokenize();
        assert_eq!(tokens, vec!["let", "mut", "x", "=", "42;"]);
    }

    #[test]
    fn test_elision_equivalence() {
        let sentence = "Rustacean sejati menulis kode aman";
        assert_eq!(first_word(sentence), "Rustacean");
        assert_eq!(first_word_explicit(sentence), "Rustacean");
        assert_eq!(first_word(sentence), first_word_explicit(sentence));
    }

    #[test]
    fn test_static_bound_with_owned_and_literal() {
        let literal: &'static str = "Literal Statis";
        let owned = String::from("Owned Heap Data");

        assert!(verify_static_bound(literal).contains("Literal Statis"));
        assert!(verify_static_bound(owned).contains("Owned Heap Data"));
    }

    #[test]
    fn test_generic_and_lifetime_combination() {
        let numbers = vec![100, 200, 300, 400];
        let target = 300;
        let found = find_first_match(&numbers, &target);
        assert_eq!(found, Some(&300));

        let not_found = find_first_match(&numbers, &999);
        assert_eq!(not_found, None);

        let item = 42;
        let note = "Angka Keberuntungan";
        let ann = AnnotatedItem { item: &item, note };
        assert_eq!(format!("{ann}"), "42 (Catatan: Angka Keberuntungan)");
    }

    #[test]
    fn test_safe_lifetime_slice_retention() {
        let data = "Kompilasi Rust Cepat";
        let slice = safe_lifetime_retention(data, 0, 9);
        assert_eq!(slice, Some("Kompilasi"));

        let out_of_bounds = safe_lifetime_retention(data, 10, 50);
        assert_eq!(out_of_bounds, None);
    }
}
