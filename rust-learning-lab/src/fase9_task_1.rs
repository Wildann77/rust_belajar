// Fase 9 - Task 1: Closures Mendalam (Captures, Move Keyword, Fn, FnMut, FnOnce)
// Rujukan: rust_learning_guide.md (Sub-bab 9.1) & rust_execution_tasks.md (L794-L800)

// ----------------------------------------------------------------------------
// 1. Helper Verifikasi Trait Kategori (Fn, FnMut, FnOnce)
// ----------------------------------------------------------------------------
// Hirarki Trait Rust:
// pub trait Fn<Args>: FnMut<Args> { ... }
// pub trait FnMut<Args>: FnOnce<Args> { ... }
//
// Artinya:
// - Tipe yang mengimplementasikan `Fn` otomatis mengimplementasikan `FnMut` dan `FnOnce`.
// - Tipe yang mengimplementasikan `FnMut` otomatis mengimplementasikan `FnOnce`.
// - Tipe yang hanya mengimplementasikan `FnOnce` TIDAK bisa dipanggil sebagai `FnMut` atau `Fn`.

/// Menerima closure berkategori `Fn` (hanya meminjam immutable, aman dipanggil berkali-kali).
pub fn verify_fn<F, R>(f: F) -> R
where
    F: Fn() -> R,
{
    f()
}

/// Menerima closure berkategori `FnMut` (meminjam mutable, memutasi state internal/eksternal).
pub fn verify_fn_mut<F, R>(mut f: F) -> R
where
    F: FnMut() -> R,
{
    f()
}

/// Menerima closure berkategori `FnOnce` (mengonsumsi/memindahkan kepemilikan variabel).
pub fn verify_fn_once<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

// ----------------------------------------------------------------------------
// 2. Closure Tanpa Capture (Zero-sized Environment & Fn Pointer Coercion)
// ----------------------------------------------------------------------------

/// Membuat closure tanpa capture yang menjumlahkan dua bilangan.
///
/// Closure tanpa capture tidak menyimpan state lingkungan (ukuran struct environment = 0 byte).
/// Dapat di-coerce ke tipe function pointer murni `fn(i32, i32) -> i32`.
pub fn create_no_capture_closure() -> impl Fn(i32, i32) -> i32 {
    |a: i32, b: i32| a + b
}

/// Menerima function pointer klasik (fn) membuktikan closure tanpa capture bisa di-coerce.
pub fn call_as_fn_pointer(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

// ----------------------------------------------------------------------------
// 3. Closure Capture Immutable (&T)
// ----------------------------------------------------------------------------

/// Menunjukkan closure meminjam nilai secara immutable dari lingkungan luar.
///
/// Selama closure aktif, variabel sumber tetap bisa dibaca secara bersamaan
/// karena hanya referensi bersama (`&`) yang dipegang oleh closure struct.
pub fn demonstrate_immutable_capture(prefix: &str, items: &[&str]) -> Vec<String> {
    // Closure menangkap `prefix` secara immutable reference: &prefix
    let formatter = |item: &str| format!("{prefix}: {item}");

    // Buktikan `formatter` mengimplementasikan `Fn`
    let mut results = Vec::new();
    for &item in items {
        results.push(formatter(item));
    }

    // Variabel `prefix` masih valid dan bisa dibaca di sini
    results
}

// ----------------------------------------------------------------------------
// 4. Closure Capture Mutable (&mut T)
// ----------------------------------------------------------------------------

/// Struct Counter yang dioperasikan melalui closure mutable.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SimpleCounter {
    pub total: i32,
    pub operations: usize,
}

impl SimpleCounter {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Mendemonstrasikan closure yang menangkap variabel secara mutable (`&mut T`).
///
/// Closure ini mengimplementasikan `FnMut` dan `FnOnce`, tetapi TIDAK `Fn`.
/// Compiler mengharuskan variabel closure dideklarasikan dengan `mut`.
pub fn demonstrate_mutable_capture(counter: &mut SimpleCounter, steps: &[i32]) {
    // Closure menangkap referensi mutable `counter`
    let mut step_recorder = |amount: i32| {
        counter.total += amount;
        counter.operations += 1;
    };

    for &step in steps {
        // step_recorder membutuhkan &mut self saat dipanggil (FnMut)
        step_recorder(step);
    }
}

// ----------------------------------------------------------------------------
// 5. Keyword `move`: Pemindahan Kepemilikan (Ownership Transfer)
// ----------------------------------------------------------------------------

/// Mendemonstrasikan closure dengan keyword `move` tetapi TIDAK mendestruksi nilai.
///
/// Meskipun kepemilikan data dipindahkan ke dalam closure, closure HANYA membaca data tersebut.
/// Hasilnya: Closure mengimplementasikan `Fn`, `FnMut`, DAN `FnOnce`!
/// Dapat dipanggil berulang kali meskipun menggunakan `move`.
pub fn create_move_reader(prefix: String) -> impl Fn(&str) -> String {
    // `prefix` dipindahkan (moved) ke dalam closure struct
    move |target: &str| format!("{prefix} -> {target}")
}

/// Mendemonstrasikan closure dengan keyword `move` yang MENGONSUMSI kepemilikan nilai.
///
/// Closure memindahkan nilai ke fungsi konsumer (misal `into_iter` atau `drop`).
/// Hasilnya: Closure HANYA mengimplementasikan `FnOnce`, TIDAK BISA dipanggil ulang.
pub fn create_move_consumer(data: Vec<String>) -> impl FnOnce() -> (usize, String) {
    // `data` dipindahkan ke closure, lalu dikonsumsi di dalam eksekusi
    move || {
        let count = data.len();
        // Mengonsumsi Vec menjadi String gabungan (transfer ownership)
        let combined = data.into_iter().collect::<Vec<_>>().join(", ");
        (count, combined)
    }
}

// ----------------------------------------------------------------------------
// 6. Higher-Order Functions (HOF) dengan Generic Trait Bounds
// ----------------------------------------------------------------------------

/// Higher-Order Function untuk transformasi read-only: menerima `F: Fn`.
pub fn transform_elements<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(transform).collect()
}

/// Higher-Order Function untuk agregasi stateful: menerima `F: FnMut`.
pub fn aggregate_stateful<T, S, F>(items: &[T], mut initial: S, mut accumulator: F) -> S
where
    F: FnMut(&mut S, &T),
{
    for item in items {
        accumulator(&mut initial, item);
    }
    initial
}

/// Higher-Order Function untuk eksekusi sekali pakai (resource consumer): menerima `F: FnOnce`.
pub fn execute_and_consume<T, R, F>(resource: T, consumer: F) -> R
where
    F: FnOnce(T) -> R,
{
    consumer(resource)
}

// ----------------------------------------------------------------------------
// 7. Runner Interaktif (Demonstrasi Lengkap)
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== FASE 9: Functional Rust - Task 1 (Closures) ===");

    // 1. Closure tanpa capture
    println!("\n1. Closure Tanpa Capture:");
    let add = create_no_capture_closure();
    println!("   - add(15, 27) = {}", add(15, 27));
    // Coercion ke fn pointer
    let fn_ptr_result = call_as_fn_pointer(|x, y| x * y, 6, 7);
    println!("   - Coercion ke fn pointer: 6 * 7 = {fn_ptr_result}");
    let fn_test = verify_fn(|| "Fn tanpa capture valid");
    println!("   - Verifikasi trait: {fn_test}");

    // 2. Closure capture immutable
    println!("\n2. Closure Capture Immutable (&T):");
    let tag = String::from("TAG");
    let words = vec!["alpha", "beta", "gamma"];
    let tagged = demonstrate_immutable_capture(&tag, &words);
    println!("   - Hasil capture immutable: {:?}", tagged);
    println!("   - Tag di luar closure tetap utuh: \"{tag}\"");

    // 3. Closure capture mutable
    println!("\n3. Closure Capture Mutable (&mut T):");
    let mut counter = SimpleCounter::new();
    let steps = [10, 20, 30];
    demonstrate_mutable_capture(&mut counter, &steps);
    println!(
        "   - Counter setelah mutasi: total = {}, operations = {}",
        counter.total, counter.operations
    );

    // 4. Keyword `move`
    println!("\n4. Keyword `move`:");
    let reader_prefix = String::from("SISTEM");
    let reader = create_move_reader(reader_prefix);
    println!("   - Move-reader call 1: {}", reader("INIT"));
    println!("   - Move-reader call 2: {}", reader("READY"));

    let data_vec = vec![String::from("data1"), String::from("data2")];
    let consumer = create_move_consumer(data_vec);
    let (count, joined) = consumer();
    println!("   - Move-consumer FnOnce result: count = {count}, data = \"{joined}\"");

    // 5. Pembuktian Kategori Trait (Fn, FnMut, FnOnce)
    println!("\n5. Hirarki & Penentuan Trait Closure:");
    let pure_fn = || 100 * 2;
    println!("   - pure_fn lewat verify_fn: {}", verify_fn(pure_fn));
    println!(
        "   - pure_fn lewat verify_fn_mut: {}",
        verify_fn_mut(pure_fn)
    );
    println!(
        "   - pure_fn lewat verify_fn_once: {}",
        verify_fn_once(pure_fn)
    );

    let mut state = 5;
    let mut_fn = || {
        state += 10;
        state
    };
    println!("   - mut_fn lewat verify_fn_mut: {}", verify_fn_mut(mut_fn));

    let owned_string = String::from("Ownership dikonsumsi");
    let once_fn = move || {
        let consumed = owned_string;
        consumed.len()
    };
    println!(
        "   - once_fn lewat verify_fn_once: {}",
        verify_fn_once(once_fn)
    );

    // 6. Higher-Order Functions (HOF)
    println!("\n6. Demonstrasi Higher-Order Functions (HOF):");
    let nums = [1, 2, 3, 4, 5];
    let squares = transform_elements(&nums, |&x| x * x);
    println!("   - HOF transform (Fn): {:?}", squares);

    let sum = aggregate_stateful(&nums, 0, |acc, &x| *acc += x);
    println!("   - HOF aggregate (FnMut): sum = {sum}");

    let res = execute_and_consume(vec![10, 20, 30], |v| v.iter().sum::<i32>());
    println!("   - HOF execute_and_consume (FnOnce): sum = {res}");

    println!("\n[OK] Task 1 Closures selesai & terverifikasi.");
}

// ----------------------------------------------------------------------------
// Unit Tests Komprehensif
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_no_capture() {
        let add = create_no_capture_closure();
        assert_eq!(add(10, 20), 30);
        assert_eq!(add(-5, 5), 0);

        // Uji coercion ke function pointer
        let result = call_as_fn_pointer(|x, y| x - y, 50, 15);
        assert_eq!(result, 35);

        // Closure tanpa capture memenuhi Fn, FnMut, dan FnOnce
        assert_eq!(verify_fn(|| 42), 42);
        assert_eq!(verify_fn_mut(|| 42), 42);
        assert_eq!(verify_fn_once(|| 42), 42);
    }

    #[test]
    fn test_closure_capture_immutable() {
        let prefix = "ID";
        let items = ["001", "002", "003"];
        let result = demonstrate_immutable_capture(prefix, &items);

        assert_eq!(result, vec!["ID: 001", "ID: 002", "ID: 003"]);
        // Immutable borrow tidak menghalangi pembacaan ulang prefix
        assert_eq!(prefix, "ID");
    }

    #[test]
    fn test_closure_capture_mutable() {
        let mut counter = SimpleCounter::new();
        demonstrate_mutable_capture(&mut counter, &[5, 15, -2]);

        assert_eq!(counter.total, 18);
        assert_eq!(counter.operations, 3);
    }

    #[test]
    fn test_move_closure_reusable_as_fn() {
        let greeting = String::from("Halo");
        let greeter = create_move_reader(greeting);

        // Membuktikan closure `move` yang hanya membaca tetap mengimplementasikan Fn (bisa dipanggil berkali-kali)
        assert_eq!(greeter("Dunia"), "Halo -> Dunia");
        assert_eq!(greeter("Rust"), "Halo -> Rust");
    }

    #[test]
    fn test_move_closure_consumed_as_fn_once() {
        let items = vec![String::from("A"), String::from("B"), String::from("C")];
        let consumer = create_move_consumer(items);

        let (len, combined) = consumer();
        assert_eq!(len, 3);
        assert_eq!(combined, "A, B, C");
        // consumer() tidak bisa dipanggil lagi di sini (FnOnce dikonsumsi)
    }

    #[test]
    fn test_trait_hierarchy_and_bounds() {
        // 1. Fn memenuhi FnMut dan FnOnce
        let pure = || 99;
        assert_eq!(verify_fn(pure), 99);
        assert_eq!(verify_fn_mut(pure), 99);
        assert_eq!(verify_fn_once(pure), 99);

        // 2. FnMut memenuhi FnOnce
        let mut val = 10;
        let mut_closure = || {
            val *= 2;
            val
        };
        assert_eq!(verify_fn_mut(mut_closure), 20);

        // 3. FnOnce mengonsumsi objek non-Copy
        let non_copy = String::from("drop-me");
        let once_closure = || {
            let _owned = non_copy; // moves non_copy
        };
        verify_fn_once(once_closure);
    }

    #[test]
    fn test_higher_order_functions() {
        let data = [1, 2, 3, 4];

        // Fn bound
        let doubled = transform_elements(&data, |&x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8]);

        // FnMut bound
        let product = aggregate_stateful(&data, 1, |acc, &x| *acc *= x);
        assert_eq!(product, 24);

        // FnOnce bound
        let sum_result = execute_and_consume(data.to_vec(), |v| v.iter().sum::<i32>());
        assert_eq!(sum_result, 10);
    }
}
