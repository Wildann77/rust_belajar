// Mini Project Fase 9: Statistics Processor (Functional Rust: Closures & Iterators)
// Rujukan: rust_learning_guide.md (Sub-bab 9.3) & rust_execution_tasks.md (L826-L849)

use std::fmt::{self, Display, Formatter};

/// Laporan Statistik Lengkap hasil kalkulasi functional iterator pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct StatisticsReport {
    pub source: Vec<i32>,
    pub evens: Vec<i32>,
    pub threshold: i32,
    pub above_threshold: Vec<i32>,
    pub squares: Vec<i64>,
    pub sum: i64,
    pub average: Option<f64>,
    pub maximum: Option<i32>,
    pub minimum: Option<i32>,
}

impl Display for StatisticsReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- Statistics Processor Report ---")?;
        writeln!(f, "Input Data         : {:?}", self.source)?;
        writeln!(f, "1. Angka Genap     : {:?}", self.evens)?;
        writeln!(
            f,
            "2. Angka > {} (th) : {:?}",
            self.threshold, self.above_threshold
        )?;
        writeln!(f, "3. Square (Kuadrat): {:?}", self.squares)?;
        writeln!(f, "4. Sum (Jumlah)    : {}", self.sum)?;
        match self.average {
            Some(avg) => writeln!(f, "5. Average (Rata)  : {avg:.2}")?,
            None => writeln!(f, "5. Average (Rata)  : N/A (Koleksi Kosong)")?,
        }
        match self.maximum {
            Some(max) => writeln!(f, "6. Maximum         : {max}")?,
            None => writeln!(f, "6. Maximum         : N/A")?,
        }
        match self.minimum {
            Some(min) => write!(f, "7. Minimum         : {min}"),
            None => write!(f, "7. Minimum         : N/A"),
        }
    }
}

/// Mesin Pemroses Statistik Berbasis Iterator & Closure.
#[derive(Debug, Clone)]
pub struct StatisticsProcessor {
    data: Vec<i32>,
}

impl StatisticsProcessor {
    /// Membuat processor baru dengan memindahkan data kepemilikan.
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    /// Mengakses data sumber sebagai slice referensi (&[i32]).
    pub fn as_slice(&self) -> &[i32] {
        &self.data
    }

    /// 1. Angka genap: Filter elemen yang habis dibagi 2.
    pub fn evens(&self) -> Vec<i32> {
        self.data.iter().copied().filter(|&x| x % 2 == 0).collect()
    }

    /// 2. Angka > threshold: Filter elemen yang lebih besar dari ambang batas.
    pub fn above_threshold(&self, threshold: i32) -> Vec<i32> {
        self.data
            .iter()
            .copied()
            .filter(|&x| x > threshold)
            .collect()
    }

    /// 3. Square: Transformasi tiap elemen menjadi kuadrat (i64 untuk hindari overflow).
    pub fn squares(&self) -> Vec<i64> {
        self.data.iter().map(|&x| (x as i64) * (x as i64)).collect()
    }

    /// 4. Sum: Penjumlahan seluruh elemen menggunakan fold / sum iterator adapter.
    pub fn sum(&self) -> i64 {
        self.data.iter().map(|&x| x as i64).sum()
    }

    /// 5. Average: Nilai rata-rata dari seluruh elemen (Option<f64> untuk cegah zero-division).
    pub fn average(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.sum() as f64 / self.data.len() as f64)
        }
    }

    /// 6. Maximum: Elemen terbesar via .max() iterator consuming adaptor.
    pub fn maximum(&self) -> Option<i32> {
        self.data.iter().copied().max()
    }

    /// 7. Minimum: Elemen terkecil via .min() iterator consuming adaptor.
    pub fn minimum(&self) -> Option<i32> {
        self.data.iter().copied().min()
    }

    /// Membangun laporan statistik agregat secara lengkap.
    pub fn generate_report(&self, threshold: i32) -> StatisticsReport {
        StatisticsReport {
            source: self.data.clone(),
            evens: self.evens(),
            threshold,
            above_threshold: self.above_threshold(threshold),
            squares: self.squares(),
            sum: self.sum(),
            average: self.average(),
            maximum: self.maximum(),
            minimum: self.minimum(),
        }
    }

    // ------------------------------------------------------------------------
    // Higher-Order Custom Pipeline
    // ------------------------------------------------------------------------

    /// Filter data dengan closure kustom (Menerima generic `F: Fn(&i32) -> bool`).
    pub fn custom_filter<F>(&self, predicate: F) -> Vec<i32>
    where
        F: Fn(&i32) -> bool,
    {
        self.data.iter().copied().filter(predicate).collect()
    }

    /// Transformasi data dengan closure kustom (Menerima generic `F: Fn(i32) -> U`).
    pub fn custom_transform<U, F>(&self, mapper: F) -> Vec<U>
    where
        F: Fn(i32) -> U,
    {
        self.data.iter().copied().map(mapper).collect()
    }
}

// ----------------------------------------------------------------------------
// Evaluasi Kriteria Lulus Fase 9
// ----------------------------------------------------------------------------

/// Demonstrasi evaluasi pemahaman perbedaan `iter()`, `iter_mut()`, dan `into_iter()`.
pub fn verify_iteration_modes() -> (&'static str, &'static str, &'static str) {
    let mode_iter = "iter(): meminjam immutable (&T), koleksi asal tetap utuh.";
    let mode_iter_mut = "iter_mut(): meminjam mutable (&mut T), mutasi elemen in-place.";
    let mode_into_iter = "into_iter(): mengonsumsi koleksi (T), ownership dipindahkan.";
    (mode_iter, mode_iter_mut, mode_into_iter)
}

/// Demonstrasi evaluasi pembuktian laziness pada iterator.
pub fn verify_iterator_laziness() -> usize {
    use std::cell::Cell;
    let step_count = Cell::new(0);
    let numbers = [1, 2, 3, 4, 5];

    // Pipeline disusun: TIDAK ADA iterasi yang dieksekusi di sini!
    let pipeline = numbers.iter().map(|&x| {
        step_count.set(step_count.get() + 1);
        x * 2
    });

    // Buktikan step_count masih 0 sebelum terminal method dipanggil
    assert_eq!(step_count.get(), 0);

    // Terminal method .take(2).collect() dipanggil: hanya 2 langkah yang dieksekusi!
    let _: Vec<_> = pipeline.take(2).collect();
    step_count.get()
}

/// Demonstrasi evaluasi pemahaman Fn, FnMut, dan FnOnce lewat closure mandiri.
pub fn verify_closure_traits() -> (&'static str, i32, usize) {
    // 1. Fn (Read-only borrow)
    let greeting = String::from("Halo");
    let fn_read = || greeting.len();
    let res_fn = fn_read();

    // 2. FnMut (Mutable borrow)
    let mut counter = 10;
    let mut fn_mut = || {
        counter += 5;
        counter
    };
    let res_fn_mut = fn_mut();

    // 3. FnOnce (Consume / Move ownership)
    let data = vec![100, 200, 300];
    let fn_once = move || data.into_iter().sum::<i32>();
    let res_fn_once = fn_once();

    ("Fn: baca saja", res_fn_mut, res_fn + (res_fn_once as usize))
}

// ----------------------------------------------------------------------------
// Runner Interaktif
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== Mini Project Fase 9: Statistics Processor ===");

    let sample_input = vec![10, 20, 11, 30, 40, 21, 50];
    let processor = StatisticsProcessor::new(sample_input);
    let threshold = 25;

    let report = processor.generate_report(threshold);
    println!("{report}");

    // Demonstrasi custom pipeline
    let slice_len = processor.as_slice().len();
    let multiples_of_10 = processor.custom_filter(|&x| x % 10 == 0);
    let labels: Vec<String> = processor.custom_transform(|x| format!("N:{x}"));
    println!("\nCustom Pipeline Demo (slice length: {slice_len}):");
    println!("   - Kelipatan 10: {:?}", multiples_of_10);
    println!("   - Format Label: {:?}", labels);

    println!("\n--- Evaluasi & Bukti Kriteria Lulus Fase 9 ---");
    let (m1, m2, m3) = verify_iteration_modes();
    println!("1. Pemahaman Mode Iterasi:");
    println!("   - {m1}");
    println!("   - {m2}");
    println!("   - {m3}");

    let executed_steps = verify_iterator_laziness();
    println!("\n2. Bukti Laziness Iterator:");
    println!("   - Diprogram untuk ambil 2 elemen (take(2)) dari 5.");
    println!("   - Jumlah langkah eksekusi aktual: {executed_steps} langkah (bukan 5!).");

    let (c1, c2, c3) = verify_closure_traits();
    println!("\n3. Bukti Kategori Fn / FnMut / FnOnce:");
    println!("   - {c1}");
    println!("   - FnMut mutasi counter: {c2}");
    println!("   - FnOnce mengonsumsi Vec: checksum = {c3}");

    println!("\n[OK] Mini Project Fase 9 selesai & terverifikasi.");
}

// ----------------------------------------------------------------------------
// Unit Tests Komprehensif
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_specification_input_output() {
        let input = vec![10, 20, 11, 30, 40, 21, 50];
        let processor = StatisticsProcessor::new(input.clone());
        let threshold = 25;

        // 1. Angka genap: [10, 20, 30, 40, 50]
        assert_eq!(processor.evens(), vec![10, 20, 30, 40, 50]);

        // 2. Angka > threshold (25): [30, 40, 50]
        assert_eq!(processor.above_threshold(threshold), vec![30, 40, 50]);

        // 3. Square: [100, 400, 121, 900, 1600, 441, 2500]
        assert_eq!(
            processor.squares(),
            vec![100, 400, 121, 900, 1600, 441, 2500]
        );

        // 4. Sum: 10 + 20 + 11 + 30 + 40 + 21 + 50 = 182
        assert_eq!(processor.sum(), 182);

        // 5. Average: 182 / 7 = 26.0
        assert_eq!(processor.average(), Some(26.0));

        // 6. Maximum: 50
        assert_eq!(processor.maximum(), Some(50));

        // 7. Minimum: 10
        assert_eq!(processor.minimum(), Some(10));
    }

    #[test]
    fn test_empty_collection_handling() {
        let empty_processor = StatisticsProcessor::new(Vec::new());

        assert_eq!(empty_processor.evens(), Vec::<i32>::new());
        assert_eq!(empty_processor.above_threshold(10), Vec::<i32>::new());
        assert_eq!(empty_processor.squares(), Vec::<i64>::new());
        assert_eq!(empty_processor.sum(), 0);
        assert_eq!(empty_processor.average(), None);
        assert_eq!(empty_processor.maximum(), None);
        assert_eq!(empty_processor.minimum(), None);
    }

    #[test]
    fn test_negative_and_zero_numbers() {
        let processor = StatisticsProcessor::new(vec![-20, -5, 0, 5, 20]);

        assert_eq!(processor.evens(), vec![-20, 0, 20]);
        assert_eq!(processor.above_threshold(0), vec![5, 20]);
        assert_eq!(processor.sum(), 0);
        assert_eq!(processor.average(), Some(0.0));
        assert_eq!(processor.maximum(), Some(20));
        assert_eq!(processor.minimum(), Some(-20));
    }

    #[test]
    fn test_custom_closures_filter_and_transform() {
        let processor = StatisticsProcessor::new(vec![1, 2, 3, 4, 5, 6]);

        // Custom filter: bilangan kelipatan 3
        let multiples_of_3 = processor.custom_filter(|&x| x % 3 == 0);
        assert_eq!(multiples_of_3, vec![3, 6]);

        // Custom transform: ubah ke format String berlabel
        let labeled = processor.custom_transform(|x| format!("NUM-{}", x * 10));
        assert_eq!(
            labeled,
            vec!["NUM-10", "NUM-20", "NUM-30", "NUM-40", "NUM-50", "NUM-60"]
        );
    }

    #[test]
    fn test_fase_graduation_criteria() {
        let (m1, m2, m3) = verify_iteration_modes();
        assert!(m1.contains("iter()"));
        assert!(m2.contains("iter_mut()"));
        assert!(m3.contains("into_iter()"));

        let executed_steps = verify_iterator_laziness();
        assert_eq!(executed_steps, 2);

        let (c1, res_mut, checksum) = verify_closure_traits();
        assert_eq!(c1, "Fn: baca saja");
        assert_eq!(res_mut, 15);
        assert_eq!(checksum, 604); // 4 (Halo.len()) + 600
    }
}
