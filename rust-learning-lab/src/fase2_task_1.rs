// Fase 2 - Task 1: Ownership (Move, Clone, Copy)
// Rujukan: rust_learning_guide.md (Bagian 2.1 & 2.2)

pub fn run() {
    println!("=== Fase 2 - Task 1: Ownership ===");

    // 1. Buat String (alokasi di heap)
    let s1 = String::from("Rustacean");
    println!("1. String dibuat: s1 = \"{s1}\"");

    // 2. Pindahkan String ke variable lain (Move Semantics)
    // Pointer, length, capacity disalin ke s2; kepemilikan heap berpindah ke s2.
    // Variabel s1 dibatalkan (invalidated) oleh Rust.
    let s2 = s1;
    println!("2. Ownership dipindahkan (Move): s2 = \"{s2}\"");

    // 3 & 4. Coba gunakan variable lama & amati pesan error compiler:
    // Jika baris di bawah di-uncomment, compiler menolak:
    //
    // println!("{s1}");
    //           ^^^^ value borrowed here after move
    // error[E0382]: borrow of moved value: `s1`
    // note: move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    println!("3 & 4. Variable s1 sudah tidak valid (moved). Compiler cegah double free error (E0382).");

    // 5. Perbaiki menggunakan .clone() (Deep copy di Heap)
    let s3 = s2.clone();
    println!("5. Perbaikan dengan .clone():");
    println!("   s2 tetap valid = \"{s2}\"");
    println!("   s3 alokasi baru = \"{s3}\"");

    // 6. Uji tipe Copy (Primitive Stack Types)
    // Tipe primitif mengimplementasikan trait Copy, disalin secara bitwise di stack.
    let x: i32 = 42;
    let y = x; // Copy otomatis (bukan move)
    println!("6. Uji tipe Copy:");
    println!("   Integer: x = {x}, y = {y} (keduanya tetap valid)");

    let is_rust_fast: bool = true;
    let flag_copy = is_rust_fast;
    println!("   Bool: is_rust_fast = {is_rust_fast}, flag_copy = {flag_copy}");

    let tuple_copy: (i32, f64, char) = (100, 3.14, '🦀');
    let tuple_clone = tuple_copy; // Tuple primitif juga Copy
    println!("   Tuple Copy: tuple_copy = {:?}, tuple_clone = {:?}", tuple_copy, tuple_clone);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone_creates_independent_heap_data() {
        let original = String::from("hello");
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.as_ptr() == cloned.as_ptr(), false, "Buffer heap harus berada di alamat berbeda");
    }

    #[test]
    fn test_copy_types_remain_valid() {
        let a = 10;
        let b = a;
        assert_eq!(a, 10);
        assert_eq!(b, 10);

        let t = (1, 2.5, true);
        let t2 = t;
        assert_eq!(t, (1, 2.5, true));
        assert_eq!(t2, (1, 2.5, true));
    }
}
