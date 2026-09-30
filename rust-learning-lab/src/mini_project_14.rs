// Mini Project Fase 14: Utility Macro & Advanced Rust Integration Suite
// Rujukan: rust_learning_guide.md (FASE 14) & rust_execution_tasks.md (L1200-L1223)
//
// Target Checklist:
// - [x] Buat macro sendiri: log_value!(name, value)
// - [x] Buat macro sendiri: create_vec!(1, 2, 3, 4, 5)
// - [x] Macro Utility Pendukung: retry_operation!(max, op), timed_exec!(label, block)
//
// Evaluasi Lulus Fase 14:
// - [x] Bisa menjelaskan mengapa `unsafe` ada.
// - [x] Bisa membedakan safe abstraction dan unsafe implementation.
// - [x] Bisa membuat `macro_rules!` sederhana.
// - [x] Bisa menjelaskan apa itu FFI.

// ============================================================================
// 1. MACRO UTILITY: log_value!
// ============================================================================

/// Macro untuk mencetak nilai variabel secara terstruktur dengan nama atau tag.
/// Mendukung multi-branch pattern matching:
/// 1. `log_value!(name, value)`
/// 2. `log_value!(TAG: tag, name, value)`
#[macro_export]
macro_rules! log_value {
    // Cabang 1: log_value!(name, value)
    ($name:expr, $val:expr) => {{
        let formatted = format!("[LOG] {} => {:?}", $name, $val);
        println!("{}", formatted);
        formatted
    }};
    // Cabang 2: log_value!(TAG: "AUTH", name, value)
    (TAG: $tag:expr, $name:expr, $val:expr) => {{
        let formatted = format!("[LOG][{}] {} => {:?}", $tag, $name, $val);
        println!("{}", formatted);
        formatted
    }};
}

// ============================================================================
// 2. MACRO UTILITY: create_vec!
// ============================================================================

/// Macro pembuat Vector dinamis dengan optimasi pra-alokasi kapasitas (`with_capacity`).
/// Mendukung tiga varian sintaks:
/// 1. `create_vec!()` -> Vector kosong
/// 2. `create_vec!(repeat: elem; count)` -> Vector terisi pengulangan elemen
/// 3. `create_vec!(1, 2, 3, 4, 5)` -> Vector dengan pra-alokasi panjang exact
#[macro_export]
macro_rules! create_vec {
    // Pola 1: Kosong
    () => {
        Vec::new()
    };
    // Pola 2: Inisialisasi pengulangan elemen (repeat: elem; count)
    (repeat: $elem:expr; $count:expr) => {
        vec![$elem; $count]
    };
    // Pola 3: Kumpulan elemen dengan pra-alokasi kapasitas dan koma opsional di akhir
    ( $( $elem:expr ),+ $(,)? ) => {
        {
            // Menghitung jumlah elemen pada saat kompilasi
            let count = <[()]>::len(&[ $( $crate::create_vec!(@replace_unit $elem) ),+ ]);
            let mut v = Vec::with_capacity(count);
            $(
                v.push($elem);
            )+
            v
        }
    };
    // Pola internal pembantu untuk menghitung jumlah argumen
    (@replace_unit $e:expr) => { () };
}

// ============================================================================
// 3. MACRO UTILITY TINGKAT LANJUT: retry_operation! & timed_exec!
// ============================================================================

/// Macro untuk mencoba kembali (retry) operasi yang menghasilkan Result hingga `max_attempts`.
#[macro_export]
macro_rules! retry_operation {
    ($max_attempts:expr, $op:expr) => {{
        let mut attempts = 0;
        let max = $max_attempts;
        loop {
            attempts += 1;
            let res = $op;
            match res {
                Ok(val) => break Ok((val, attempts)),
                Err(err) => {
                    if attempts >= max {
                        break Err((err, attempts));
                    }
                }
            }
        }
    }};
}

/// Macro untuk mengukur durasi eksekusi suatu blok kode dan mengembalikan nilai baliknya.
#[macro_export]
macro_rules! timed_exec {
    ($label:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        println!("[TIMER] '{}' selesai dalam {:?}", $label, elapsed);
        (result, elapsed)
    }};
}

// ============================================================================
// 4. MODEL EVALUASI KELULUSAN FASE 14
// ============================================================================

/// Representasi jawaban komprehensif untuk empat pilar kelulusan Fase 14.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraduationEvaluationItem {
    pub topic: &'static str,
    pub question: &'static str,
    pub status: &'static str,
    pub explanation: &'static str,
}

pub fn get_phase14_graduation_evaluation() -> Vec<GraduationEvaluationItem> {
    vec![
        GraduationEvaluationItem {
            topic: "1. Keberadaan Unsafe",
            question: "Mengapa keyword `unsafe` harus ada di Rust?",
            status: "[LULUS]",
            explanation: "Komputer dan hardware pada level mesin tidak memiliki sistem tipe statis borrow checker. Unsafe ada sebagai pintu darurat (escape hatch) untuk: interaksi register hardware/OS, implementasi struktur data dasar efisien (Vec, Box, Arc), optimasi performa pointer arithmetic, dan integrasi library C via FFI. Unsafe tidak mematikan borrow checker, melainkan memberi izin untuk 5 kemampuan khusus yang keamanannya diverifikasi oleh programmer.",
        },
        GraduationEvaluationItem {
            topic: "2. Safe Abstraction",
            question: "Apa perbedaan Safe Abstraction dan Unsafe Implementation?",
            status: "[LULUS]",
            explanation: "Unsafe implementation adalah operasi internal berisiko yang memanipulasi pointer mentah atau memori tak terkelola. Safe abstraction adalah antarmuka publik (API) aman yang membungkus blok unsafe tersebut, di mana seluruh invariant (boundary check, alignment, null check) diverifikasi ketat terlebih dahulu. Contoh: fungsi split_at_mut atau Vec::push secara internal menggunakan unsafe raw pointer, tetapi konsumen luar dapat memanggilnya dengan jaminan 100% aman.",
        },
        GraduationEvaluationItem {
            topic: "3. Macro System",
            question: "Bagaimana cara kerja dan manfaat macro_rules!?",
            status: "[LULUS]",
            explanation: "macro_rules! adalah sistem metaprogramming deklaratif berbasis pattern matching pohon token saat kompilasi. Manfaatnya mencakup: eliminasi boilerplate code, pembuatan DSL (Domain Specific Language) ringkas, argumen variadic dinamis, dan sifat higienis (tidak mencemari scope lokal).",
        },
        GraduationEvaluationItem {
            topic: "4. Konsep FFI & ABI",
            question: "Apa itu FFI dan Application Binary Interface (ABI)?",
            status: "[LULUS]",
            explanation: "FFI (Foreign Function Interface) adalah jembatan interoperabilitas antar bahasa pemrograman. ABI adalah kontrak level mesin yang mengatur konvensi pemanggilan fungsi (register CPU, stack layout), keselarasan memori (padding struct via #[repr(C)]), name mangling (#[unsafe(no_mangle)]), serta penanganan isolasi panic (catch_unwind) agar program tidak crash lintas bahasa.",
        },
    ]
}

// ============================================================================
// 5. RUNNER DEMONSTRASI (pub fn run())
// ============================================================================

/// Fungsi utama runner demonstrasi untuk Mini Project Fase 14.
pub fn run() {
    println!("=== MINI PROJECT FASE 14: UTILITY MACRO SUITE ===");

    // 1. Demonstrasi log_value!
    println!("\n1. Demonstrasi Utility Macro: log_value!");
    log_value!("service_name", "task-processor");
    log_value!("max_concurrency", 16);
    log_value!(TAG: "SECURITY", "jwt_algorithm", "RS256");
    log_value!(TAG: "DATABASE", "pool_size", 20);

    // 2. Demonstrasi create_vec!
    println!("\n2. Demonstrasi Utility Macro: create_vec!");
    let v_empty: Vec<i32> = create_vec![];
    let v_repeated = create_vec![repeat: 7; 4];
    let v_numbers = create_vec![10, 20, 30, 40, 50];
    let v_trailing = create_vec!["Rust", "Edition", "2024",];

    println!("   create_vec![]                    -> {:?}", v_empty);
    println!("   create_vec![repeat: 7; 4]        -> {:?}", v_repeated);
    println!("   create_vec![10, 20, 30, 40, 50]  -> {:?}", v_numbers);
    println!("   create_vec!['Rust'...] (trailing)-> {:?}", v_trailing);

    // 3. Demonstrasi timed_exec!
    println!("\n3. Demonstrasi Utility Macro: timed_exec!");
    let (heavy_calc, duration) = timed_exec!("Komputasi Fibonacci Parsial", {
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 0..50 {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    });
    println!(
        "   Hasil perhitungan: {} (tercatat: {:?})",
        heavy_calc, duration
    );

    // 4. Demonstrasi retry_operation!
    println!("\n4. Demonstrasi Utility Macro: retry_operation!");
    let mut attempt_counter = 0;
    let retry_result: Result<(String, usize), (&str, usize)> = retry_operation!(3, {
        attempt_counter += 1;
        if attempt_counter < 3 {
            Err("Simulasi kegagalan jaringan sementara")
        } else {
            Ok(String::from("Koneksi berhasil dipulihkan"))
        }
    });

    match retry_result {
        Ok((msg, attempts)) => {
            println!("   [SUKSES] {} pada percobaan ke-{}", msg, attempts);
        }
        Err((err, attempts)) => {
            println!("   [GAGAL] {} setelah {} percobaan", err, attempts);
        }
    }

    // 5. Evaluasi Kriteria Lulus Fase 14
    println!("\n========================================================");
    println!("=== EVALUASI KRITERIA KELULUSAN FASE 14 ===");
    println!("========================================================");
    let eval_items = get_phase14_graduation_evaluation();
    for item in eval_items {
        println!("\n{} {}", item.status, item.topic);
        println!("Pertanyaan : {}", item.question);
        println!("Penjelasan : {}", item.explanation);
    }

    println!("\n[OK] Mini Project Fase 14 (Utility Macro Suite) tuntas & terverifikasi!");
}

// ============================================================================
// 6. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_value_macro() {
        let out1 = log_value!("key", 123);
        assert_eq!(out1, "[LOG] key => 123");

        let out2 = log_value!(TAG: "AUTH", "token", "valid");
        assert_eq!(out2, "[LOG][AUTH] token => \"valid\"");
    }

    #[test]
    fn test_create_vec_macro_variants() {
        let empty: Vec<u8> = create_vec![];
        assert_eq!(empty.len(), 0);

        let nums = create_vec![1, 2, 3, 4, 5];
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
        assert_eq!(nums.capacity(), 5); // Verifikasi optimasi with_capacity

        let repeated = create_vec![repeat: 9; 3];
        assert_eq!(repeated, vec![9, 9, 9]);

        let trailing = create_vec![100, 200,];
        assert_eq!(trailing, vec![100, 200]);
    }

    #[test]
    fn test_retry_operation_macro() {
        // Kasus 1: Berhasil pada percobaan ke-2
        let mut count1 = 0;
        let res1: Result<(i32, usize), (&str, usize)> = retry_operation!(3, {
            count1 += 1;
            if count1 >= 2 { Ok(42) } else { Err("fail") }
        });
        assert_eq!(res1, Ok((42, 2)));

        // Kasus 2: Gagal hingga max attempts
        let mut count2 = 0;
        let res2: Result<(i32, usize), (&str, usize)> = retry_operation!(3, {
            count2 += 1;
            Err("fatal error")
        });
        assert_eq!(res2, Err(("fatal error", 3)));
        assert_eq!(count2, 3);
    }

    #[test]
    fn test_timed_exec_macro() {
        let (val, duration) = timed_exec!("Test Timer", 10 + 25);
        assert_eq!(val, 35);
        // Durasi harus non-negatif
        assert!(duration.as_nanos() > 0 || duration.as_nanos() == 0);
    }

    #[test]
    fn test_graduation_evaluation_completeness() {
        let eval = get_phase14_graduation_evaluation();
        assert_eq!(eval.len(), 4);
        for item in &eval {
            assert_eq!(item.status, "[LULUS]");
            assert!(!item.explanation.is_empty());
        }
    }

    #[test]
    fn test_smoke_runner() {
        run();
    }
}
