// Fase 2 - Task 3: Borrowing & Non-Lexical Lifetimes (NLL)
// Rujukan: rust_learning_guide.md (Bagian 2.3) & rust_execution_tasks.md

pub fn run() {
    println!("=== Fase 2 - Task 3: Borrowing & NLL ===");

    // ------------------------------------------------------------------------
    // 1. Banyak &T (Multiple Shared / Immutable References)
    // ------------------------------------------------------------------------
    let data = String::from("Rust Concurrency Safe");
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;
    println!("1. Banyak &T (Shared Borrows):");
    println!("   r1: \"{r1}\", r2: \"{r2}\", r3: \"{r3}\"");
    println!("   -> Boleh ada banyak &T aktif bersamaan karena akses hanya baca (read-only).");

    // ------------------------------------------------------------------------
    // 2. Satu &mut T (Single Exclusive / Mutable Reference)
    // ------------------------------------------------------------------------
    let mut score = 100;
    println!("\n2. Satu &mut T (Exclusive Borrow):");
    println!("   score awal: {score}");
    {
        let score_ref = &mut score;
        *score_ref += 25; // dereferencing untuk mutasi nilai
        println!("   score diubah via &mut score: {score_ref}");
    } // score_ref keluar scope, borrow mutable selesai
    println!("   score akhir: {score}");

    // ------------------------------------------------------------------------
    // 3. Coba Konflik &T dengan &mut T (Aliasing XOR Mutability)
    // ------------------------------------------------------------------------
    println!("\n3. Konflik Borrowing (Aturan Emas Borrow Checker):");
    println!("   Aturan: Boleh banyak pembaca (&T) ATAU satu penulis (&mut T), TIDAK keduanya!");
    //
    // Contoh konflik 1: &T bersamaan dengan &mut T
    // let mut text = String::from("Halo");
    // let reader = &text;
    // let writer = &mut text; // COMPILE ERROR: error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable
    // println!("{reader}");
    //
    // Contoh konflik 2: Dua &mut T bersamaan
    // let mut num = 10;
    // let m1 = &mut num;
    // let m2 = &mut num; // COMPILE ERROR: error[E0499]: cannot borrow `num` as mutable more than once at a time
    // *m1 += 1;
    // *m2 += 2;
    println!("   -> Error E0502: Mencegah data race dan dangling pointer saat data dibaca selagi diubah.");
    println!("   -> Error E0499: Mencegah race condition ketika dua penulis memodifikasi memori simultan.");

    // ------------------------------------------------------------------------
    // 4 & 5. Kapan Borrow Selesai & Eksperimen NLL (Non-Lexical Lifetimes)
    // ------------------------------------------------------------------------
    println!("\n4 & 5. Eksperimen NLL (Non-Lexical Lifetimes):");
    let mut message = String::from("Belajar Rust");

    // Immutable borrow dimulai:
    let immut_ref = &message;
    println!("   [Penggunaan terakhir &T] immut_ref: \"{immut_ref}\"");
    // Di Rust modern (NLL), borrow `immut_ref` SELESAI tepat pada baris di atas!
    // Compiler menganalisis bahwa `immut_ref` tidak pernah dipakai lagi di bawah.

    // Sehingga compiler mengizinkan mutable borrow tepat di baris ini:
    let mut_ref = &mut message;
    mut_ref.push_str(" Sangat Menyenangkan!");
    println!("   [Mutable borrow valid via NLL] message: \"{mut_ref}\"");

    // Catatan: Jika baris berikut diaktifkan:
    // println!("{immut_ref}"); // Menghidupkan immut_ref melewati mut_ref!
    // Compiler langsung menolak dengan error[E0502] karena rentang immut_ref bertabrakan dengan mut_ref.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiple_immutable_borrows() {
        let val = 42;
        let r1 = &val;
        let r2 = &val;
        let r3 = &val;
        assert_eq!(*r1, *r2);
        assert_eq!(*r2, *r3);
        assert_eq!(*r1, 42);
    }

    #[test]
    fn test_single_mutable_borrow_modifies_data() {
        let mut text = String::from("Go");
        {
            let m = &mut text;
            m.push_str(" -> Rust");
        }
        assert_eq!(text, "Go -> Rust");
    }

    #[test]
    fn test_nll_allows_sequential_borrows() {
        let mut count = 10;
        let r = &count;
        assert_eq!(*r, 10); // Penggunaan terakhir r

        // NLL melepaskan borrow r di sini:
        let m = &mut count;
        *m += 5;
        assert_eq!(*m, 15);
    }
}
