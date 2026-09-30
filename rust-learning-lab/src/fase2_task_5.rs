// Fase 2 - Task 5: UTF-8 & String Memory Internals
// Rujukan: rust_learning_guide.md (Bagian 2.5) & rust_execution_tasks.md

pub fn run() {
    println!("=== Fase 2 - Task 5: UTF-8 Internals ===");

    let text = String::from("Rust 🦀");
    println!("Teks target: \"{text}\"");

    // ------------------------------------------------------------------------
    // 1. len() vs chars().count()
    // ------------------------------------------------------------------------
    let byte_length = text.len();
    let char_count = text.chars().count();
    println!("\n1. Perbedaan Panjang Byte vs Karakter:");
    println!("   text.len()          : {byte_length} bytes");
    println!("   text.chars().count(): {char_count} karakter (Unicode Scalar Values)");
    println!("   -> Mengapa berbeda? Karena '🦀' memakan 4 byte di encoding UTF-8!");

    // ------------------------------------------------------------------------
    // 2. bytes() & Iterasi Bytes
    // ------------------------------------------------------------------------
    println!("\n2. Iterasi bytes() (Total {byte_length} byte):");
    for (idx, b) in text.bytes().enumerate() {
        let ch_repr = if b.is_ascii() {
            format!("{:?}", b as char)
        } else {
            String::from("non-ASCII")
        };
        println!("   byte [{idx}]: {b:3} | hex: 0x{b:02X} | {ch_repr}");
    }

    // ------------------------------------------------------------------------
    // 3. chars() & Iterasi Characters
    // ------------------------------------------------------------------------
    println!("\n3. Iterasi chars() (Total {char_count} char):");
    for (idx, c) in text.chars().enumerate() {
        println!(
            "   char [{idx}]: '{c}' (memakan {} byte UTF-8)",
            c.len_utf8()
        );
    }

    // ------------------------------------------------------------------------
    // 4. Slicing pada Boundary Valid
    // ------------------------------------------------------------------------
    println!("\n4. Slicing Boundary Valid:");
    let slice_ascii = &text[0..4]; // "Rust" (byte 0..4)
    let slice_space = &text[0..5]; // "Rust " (byte 0..5)
    let slice_emoji = &text[5..9]; // "🦀" (byte 5..9 persis 4 byte)
    println!("   Slice [0..4] (ASCII) : \"{slice_ascii}\"");
    println!("   Slice [0..5] (Spasi) : \"{slice_space}\"");
    println!("   Slice [5..9] (Emoji) : \"{slice_emoji}\"");

    // ------------------------------------------------------------------------
    // 5. Slicing Tidak Valid (Memotong Karakter Multi-byte)
    // ------------------------------------------------------------------------
    println!("\n5. Eksperimen Slicing Memotong Karakter Multi-byte:");
    // Jika kita langsung menulis: let bad = &text[0..6];
    // Rust akan RUNTIME PANIC:
    // "byte index 6 is not a char boundary; it is inside '🦀' (bytes 5..9) of `Rust 🦀`"

    // Penanganan aman via method .get(range) -> Option<&str>:
    let invalid_range = 0..6; // Memotong emoji 🦀 di byte pertama saja
    match text.get(invalid_range.clone()) {
        Some(valid_slice) => println!("   Slice {:?}: \"{}\"", invalid_range, valid_slice),
        None => println!(
            "   text.get(0..6) mengembalikan None karena index 6 berada di tengah-tengah byte '🦀'!"
        ),
    }

    // Tangkap panic secara terkendali untuk membuktikan behavior runtime panic:
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let panic_result = std::panic::catch_unwind(|| {
        let _bad_slice = &text[0..6];
    });
    std::panic::set_hook(old_hook);

    if panic_result.is_err() {
        println!("   -> Terbukti: Direct slice &text[0..6] memicu PANIC seketika:");
        println!("      \"end byte index 6 is not a char boundary; it is inside '🦀'\"");
        println!("   -> Rust memproteksi integritas UTF-8 agar memori tidak pernah rusak!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_len_vs_chars_count() {
        let text = String::from("Rust 🦀");
        assert_eq!(text.len(), 9);
        assert_eq!(text.chars().count(), 6);
    }

    #[test]
    fn test_valid_slices() {
        let text = String::from("Rust 🦀");
        assert_eq!(&text[0..4], "Rust");
        assert_eq!(&text[5..9], "🦀");
    }

    #[test]
    fn test_safe_get_slice() {
        let text = String::from("Rust 🦀");
        assert_eq!(text.get(0..4), Some("Rust"));
        assert_eq!(text.get(5..9), Some("🦀"));
        assert_eq!(text.get(0..6), None); // invalid boundary
    }

    #[test]
    #[should_panic(expected = "byte index 6 is not a char boundary")]
    fn test_direct_slice_invalid_boundary_panics() {
        let text = String::from("Rust 🦀");
        let _ = &text[0..6]; // panic!
    }
}
