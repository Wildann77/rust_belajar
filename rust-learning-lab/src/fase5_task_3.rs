// Fase 5 - Task 3: Cargo Profiles & Binary Optimization
// Rujukan: rust_learning_guide.md (Sub-bab 5.3) & rust_execution_tasks.md (L500-L509)

use std::time::Instant;

/// Representasi konfigurasi profil build Cargo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSettings {
    pub name: &'static str,
    pub opt_level: u8,
    pub lto: &'static str,
    pub has_debug_symbols: bool,
    pub codegen_units: u32,
    pub purpose: &'static str,
}

/// Mendeteksi profil kompilasi yang sedang aktif saat runtime
pub fn active_profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "dev (Debug)"
    } else {
        "release (Release)"
    }
}

/// Mengembalikan metadata profil yang aktif berdasarkan conditional compilation
pub fn inspect_current_profile() -> ProfileSettings {
    if cfg!(debug_assertions) {
        ProfileSettings {
            name: "dev",
            opt_level: 0,
            lto: "off",
            has_debug_symbols: true,
            codegen_units: 256,
            purpose: "Waktu kompilasi cepat dan kemudahan inspeksi debugging",
        }
    } else {
        ProfileSettings {
            name: "release",
            opt_level: 3,
            lto: "thin",
            has_debug_symbols: false,
            codegen_units: 1,
            purpose: "Kecepatan eksekusi runtime maksimal dan ukuran binary minimal",
        }
    }
}

/// Algoritma Collatz Conjecture untuk demonstrasi performa komputasi.
/// Menghitung angka dengan langkah terpanjang di antara 1..=limit.
pub fn compute_collatz_max_steps(limit: u64) -> (u64, u32) {
    let mut max_num = 1;
    let mut max_steps = 0;

    for i in 1..=limit {
        let mut n = i;
        let mut steps = 0;

        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                // Cegah overflow dengan saturating / wrapping logic standar
                n = match n.checked_mul(3).and_then(|val| val.checked_add(1)) {
                    Some(val) => val,
                    None => break,
                };
            }
            steps += 1;
        }

        if steps > max_steps {
            max_steps = steps;
            max_num = i;
        }
    }

    (max_num, max_steps)
}

pub fn run() {
    println!("=== Fase 5 - Task 3: Cargo Profiles & Binary Optimization ===");

    let current = inspect_current_profile();
    println!("1. Profil Build Saat Ini:");
    println!("   Nama Profil     : {}", current.name);
    println!("   Opt-Level       : {}", current.opt_level);
    println!("   LTO             : {}", current.lto);
    println!("   Debug Symbols   : {}", current.has_debug_symbols);
    println!("   Codegen Units   : {}", current.codegen_units);
    println!("   Tujuan          : {}", current.purpose);

    println!("\n2. Konsep Inti 5 Parameter Profil Cargo:");
    println!(
        "   - profile.dev    : Profil bawaan untuk `cargo build` & `cargo test` (fokus kecepatan kompilasi)."
    );
    println!(
        "   - profile.release: Profil bawaan untuk `cargo build --release` (fokus kecepatan runtime)."
    );
    println!(
        "   - opt-level      : 0 (tanpa optimasi) s/d 3 (optimasi maksimum), 's'/'z' (ukuran binary)."
    );
    println!("   - LTO (Link-Time): Optimasi lintas-crate oleh linker ('off', 'thin', 'fat').");
    println!("   - debug / strip  : Mengatur keberadaan symbol debug di binary executable.");

    println!("\n3. Uji Beban Komputasi (Collatz Sequence 1..=200,000):");
    let start = Instant::now();
    let (num, steps) = compute_collatz_max_steps(200_000);
    let duration = start.elapsed();

    println!(
        "   Angka {} menghasilkan rantai terpanjang: {} langkah",
        num, steps
    );
    println!(
        "   Waktu komputasi [{}]: {:.2?}",
        active_profile_name(),
        duration
    );
    if cfg!(debug_assertions) {
        println!(
            "   [Catatan] Jalankan `cargo run --release` untuk merasakan akselerasi opt-level=3 + LTO!"
        );
    } else {
        println!(
            "   [Catatan] Mode release aktif: optimasi LLVM memaksimalkan register & inlining."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_detection() {
        let name = active_profile_name();
        assert!(!name.is_empty());

        let settings = inspect_current_profile();
        if cfg!(debug_assertions) {
            assert_eq!(settings.name, "dev");
            assert_eq!(settings.opt_level, 0);
            assert!(settings.has_debug_symbols);
        } else {
            assert_eq!(settings.name, "release");
            assert_eq!(settings.opt_level, 3);
            assert!(!settings.has_debug_symbols);
        }
    }

    #[test]
    fn test_collatz_computation_known_values() {
        // Untuk rentang 1..=10, angka 9 memiliki langkah terbanyak (19 langkah)
        let (num, steps) = compute_collatz_max_steps(10);
        assert_eq!(num, 9);
        assert_eq!(steps, 19);
    }
}
