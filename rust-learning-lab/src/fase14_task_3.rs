// Fase 14 - Task 3: Declarative Macro System (macro_rules!)
// Rujukan: rust_learning_guide.md (FASE 14) & rust_execution_tasks.md (L1191-L1198)
//
// Cakupan Checklist:
// - [x] `macro_rules!`
// - [x] Pattern matching macro.
// - [x] Repetition `$(...)*`.
// - [x] Expression fragment.
// - [x] Item fragment.
// - [x] Buat macro sederhana.

use std::collections::HashMap;

// ============================================================================
// 1. MACRO FRAGMENTS & PATTERN MATCHING PENJELASAN
// ============================================================================

/// Klasifikasi fragmen penentu (designators) pada declarative macros Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroFragmentKind {
    /// expr: Ekspresi Rust yang menghasilkan nilai (contoh: 1 + 2, "teks", func()).
    Expression,
    /// item: Deklarasi tingkat atas seperti fn, struct, enum, trait, impl.
    Item,
    /// ident: Nama pengidentifikasi variabel, fungsi, struct, atau field.
    Identifier,
    /// ty: Tipe data konkret atau generic (contoh: i32, Vec<String>, Option<T>).
    Type,
    /// pat: Pola pencocokan (pattern matching arms).
    Pattern,
    /// block: Blok kode berbalut kurung kurawal `{ ... }`.
    Block,
}

impl MacroFragmentKind {
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Expression => {
                "Cocok dengan ekspresi kode apa pun yang menghasilkan nilai (expr)."
            }
            Self::Item => "Cocok dengan item bahasa tingkat atas (fn, struct, enum, impl, trait).",
            Self::Identifier => "Cocok dengan nama simbol pengidentifikasi mentah (ident).",
            Self::Type => "Cocok dengan notasi tipe data Rust yang valid (ty).",
            Self::Pattern => "Cocok dengan ekspresi pola destructuring pada match / if let (pat).",
            Self::Block => {
                "Cocok dengan blok statement yang diawali dan diakhiri kurung kurawal (block)."
            }
        }
    }
}

// ============================================================================
// 2. IMPLEMENTASI MACROS DECLARATIVE (macro_rules!)
// ============================================================================

/// Macro 1: Demonstrasi Repetition `$(...)*` dan Expression Fragment (`$x:expr`).
/// Menjumlahkan sekumpulan ekspresi angka dengan fleksibilitas trailing comma.
#[macro_export]
macro_rules! calculate_sum {
    // Pola 1: Tanpa argumen -> menghasilkan 0
    () => {
        0
    };
    // Pola 2: Satu atau banyak ekspresi dipisahkan koma, opsional trailing comma
    ( $( $x:expr ),+ $(,)? ) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )+
            total
        }
    };
}

/// Macro 2: Pattern Matching Macro dengan Sintaks Custom Key-Value.
/// Membuat `HashMap` secara instan menggunakan format `key => value`.
#[macro_export]
macro_rules! make_map {
    // Pola kosong: inisialisasi map kosong
    () => {
        std::collections::HashMap::new()
    };
    // Pola dengan pasangan key => value berulang
    ( $( $k:expr => $v:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

/// Macro 3: Multi-Branch Pattern Matching Macro (Logging Terstruktur).
/// Menunjukkan bagaimana satu macro dapat memiliki banyak cabang pattern matching.
#[macro_export]
macro_rules! log_event {
    // Cabang A: Pesan info sederhana
    (INFO: $msg:expr) => {
        format!("[INFO] {}", $msg)
    };
    // Cabang B: Peringatan dengan konteks error code
    (WARN: $code:expr, $msg:expr) => {
        format!("[WARN][Code: {}] {}", $code, $msg)
    };
    // Cabang C: Log metrik data berpasangan
    (METRIC: $name:expr => $val:expr) => {
        format!("[METRIC] {}={:.2}", $name, $val as f64)
    };
}

/// Macro 4: Demonstrasi Item Fragment (`$it:item`).
/// Menerima deklarasi item utuh (seperti `struct` atau `fn`) dan membungkusnya
/// dengan metadata atribut atau trait helper otomatis.
#[macro_export]
macro_rules! wrap_with_debug_item {
    (
        $it:item
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        $it
    };
}

// Menggunakan wrap_with_debug_item! untuk membangkitkan struct DomainItem
wrap_with_debug_item!(
    /// Struct yang di-generate via macro dengan Item Fragment ($it:item).
    pub struct GeneratedItem {
        pub id: u64,
        pub label: &'static str,
        pub score: f64,
    }
);

/// Macro 5: Demonstrasi Ident & Type Fragment untuk Boilerplate Generator.
/// Menghasilkan struct entity dengan getter otomatis.
#[macro_export]
macro_rules! define_metric_pair {
    ($struct_name:ident { $field1:ident: $type1:ty, $field2:ident: $type2:ty }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $struct_name {
            pub $field1: $type1,
            pub $field2: $type2,
        }

        impl $struct_name {
            pub fn new($field1: $type1, $field2: $type2) -> Self {
                Self { $field1, $field2 }
            }

            pub fn get_primary(&self) -> &$type1 {
                &self.$field1
            }

            pub fn get_secondary(&self) -> &$type2 {
                &self.$field2
            }
        }
    };
}

// Menggunakan define_metric_pair! untuk membuat struct ResponseMetric
define_metric_pair!(ResponseMetric {
    duration_ms: u64,
    status_code: u16
});

// ============================================================================
// 3. FUNGSI PEMBANTU ANALISIS & DOMAIN
// ============================================================================

/// Menghitung statistik kumpulan angka menggunakan macro `calculate_sum!`.
pub fn compute_metrics(numbers: &[i32]) -> (i32, f64) {
    if numbers.is_empty() {
        return (0, 0.0);
    }

    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }
    let avg = sum as f64 / numbers.len() as f64;
    (sum, avg)
}

/// Menghasilkan peta konfigurasi aplikasi via macro `make_map!`.
pub fn get_default_system_config() -> HashMap<&'static str, &'static str> {
    make_map!(
        "env" => "production",
        "app_name" => "rust-learning-lab",
        "version" => "1.0.0",
        "log_level" => "INFO",
    )
}

// ============================================================================
// 4. RUNNER DEMONSTRASI (pub fn run())
// ============================================================================

/// Fungsi utama runner demonstrasi untuk modul Fase 14 Task 3.
pub fn run() {
    println!("=== FASE 14 Task 3: Declarative Macro System (macro_rules!) ===");

    // 1. Teori Fragmen Macro
    println!("1. Jenis Fragmen Penentu (Designators) pada Declarative Macro:");
    let fragments = [
        MacroFragmentKind::Expression,
        MacroFragmentKind::Item,
        MacroFragmentKind::Identifier,
        MacroFragmentKind::Type,
        MacroFragmentKind::Pattern,
        MacroFragmentKind::Block,
    ];
    for (i, frag) in fragments.iter().enumerate() {
        println!("   [{}] {:?}: {}", i + 1, frag, frag.description());
    }

    // 2. Demo Repetition & Expression Fragment
    println!("\n2. Macro calculate_sum! (Repetition $(...)* & Expression Fragment):");
    let empty_sum = calculate_sum!();
    let single_sum = calculate_sum!(42);
    let multi_sum = calculate_sum!(10, 20, 30, 40, 50);
    let trailing_comma_sum = calculate_sum!(1, 2, 3, 4,);

    println!("   calculate_sum!()              = {}", empty_sum);
    println!("   calculate_sum!(42)            = {}", single_sum);
    println!("   calculate_sum!(10, 20... 50)  = {}", multi_sum);
    println!("   calculate_sum!(1, 2, 3, 4,)   = {}", trailing_comma_sum);

    // 3. Demo Pattern Matching & Custom Separator (make_map!)
    println!("\n3. Macro make_map! (Pattern Matching $k:expr => $v:expr):");
    let user_scores = make_map!(
        "Alice" => 95,
        "Bob"   => 88,
        "Carol" => 92,
    );
    println!("   HashMap berhasil dibuat dari sintaks macro:");
    for (name, score) in &user_scores {
        println!("   -> {} : {}", name, score);
    }

    let default_config = get_default_system_config();
    println!("   Total item config default: {}", default_config.len());

    // 4. Demo Multi-Branch Pattern Matching (log_event!)
    println!("\n4. Macro log_event! (Multi-Branch Pattern Matching):");
    let log1 = log_event!(INFO: "Server berhasil booting di port 8080");
    let log2 = log_event!(WARN: 404, "Endpoint resource tidak ditemukan");
    let log3 = log_event!(METRIC: "cpu_usage_percent" => 37.5);

    println!("   Branch A: {}", log1);
    println!("   Branch B: {}", log2);
    println!("   Branch C: {}", log3);

    // 5. Demo Item Fragment ($it:item)
    println!("\n5. Macro wrap_with_debug_item! (Item Fragment $it:item):");
    let item = GeneratedItem {
        id: 101,
        label: "Primary Sensor",
        score: 99.8,
    };
    println!("   Instance GeneratedItem (Derive Debug otomatis dari macro):");
    println!("   -> {:?}", item);

    // 6. Demo Identifier & Type Fragment (define_metric_pair!)
    println!("\n6. Macro define_metric_pair! (Ident & Type Fragments):");
    let metric = ResponseMetric::new(145, 200);
    println!(
        "   Metric instance: duration={}ms, status={}",
        metric.get_primary(),
        metric.get_secondary()
    );

    println!("\n[OK] Fase 14 Task 3 Macros demonstrasi selesai.");
}

// ============================================================================
// 5. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum_macro() {
        assert_eq!(calculate_sum!(), 0);
        assert_eq!(calculate_sum!(5), 5);
        assert_eq!(calculate_sum!(1, 2, 3, 4, 5), 15);
        assert_eq!(calculate_sum!(10, 20,), 30);
    }

    #[test]
    fn test_make_map_macro() {
        let empty: HashMap<String, i32> = make_map!();
        assert!(empty.is_empty());

        let map = make_map!(
            1 => "one",
            2 => "two",
            3 => "three",
        );
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_log_event_branches() {
        let info = log_event!(INFO: "Koneksi stabil");
        assert_eq!(info, "[INFO] Koneksi stabil");

        let warn = log_event!(WARN: 500, "Database timeout");
        assert_eq!(warn, "[WARN][Code: 500] Database timeout");

        let metric = log_event!(METRIC: "latency_ms" => 12.34);
        assert_eq!(metric, "[METRIC] latency_ms=12.34");
    }

    #[test]
    fn test_generated_item_from_macro() {
        let item1 = GeneratedItem {
            id: 1,
            label: "Test",
            score: 85.0,
        };
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_define_metric_pair_macro() {
        let resp = ResponseMetric::new(250, 404);
        assert_eq!(*resp.get_primary(), 250);
        assert_eq!(*resp.get_secondary(), 404);
    }

    #[test]
    fn test_compute_metrics_helper() {
        let nums = [10, 20, 30];
        let (sum, avg) = compute_metrics(&nums);
        assert_eq!(sum, 60);
        assert!((avg - 20.0).abs() < 1e-6);

        let empty: [i32; 0] = [];
        let (sum0, avg0) = compute_metrics(&empty);
        assert_eq!(sum0, 0);
        assert_eq!(avg0, 0.0);
    }

    #[test]
    fn test_smoke_runner() {
        run();
    }
}
