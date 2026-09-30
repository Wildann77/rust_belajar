// Fase 14 - Task 2: Foreign Function Interface (FFI) & ABI Boundaries
// Rujukan: rust_learning_guide.md (FASE 14) & rust_execution_tasks.md (L1184-L1189)
//
// Cakupan Checklist:
// - [x] Pelajari konsep ABI.
// - [x] Pelajari `extern "C"`.
// - [x] Pelajari pemanggilan function C secara konseptual.
// - [x] Identifikasi boundary safe/unsafe.

use std::ffi::{CStr, CString, c_char, c_int};
use std::panic::catch_unwind;

// ============================================================================
// 1. KONSEP ABI (APPLICATION BINARY INTERFACE) & DATA LAYOUT
// ============================================================================

/// Penjelasan pilar-pilar penting dalam Application Binary Interface (ABI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiPillar {
    /// Calling Convention: Aturan penempatan argumen pada register vs stack, dan siapa yang membersihkan stack.
    CallingConvention,
    /// Data Layout & Padding: Keselarasan memori (alignment), urutan byte (endianness), dan ukuran tipe primitif.
    DataLayout,
    /// Name Mangling: Cara compiler mengubah nama fungsi menjadi simbol biner unik di object file.
    NameMangling,
    /// Unwinding & Exception: Cara runtime menangani stack unwinding saat terjadi crash/panic lintas bahasa.
    UnwindingBehavior,
}

impl AbiPillar {
    pub const fn description(&self) -> &'static str {
        match self {
            Self::CallingConvention => {
                "Menentukan register mana yang menampung argumen/return value (misal System V AMD64 ABI vs MS x64)."
            }
            Self::DataLayout => {
                "Rust default layout tidak stabil (bisa di-reorder compiler). #[repr(C)] menjamin layout persis seperti struct C."
            }
            Self::NameMangling => {
                "Rust mengubah nama fungsi (mangling) untuk namespace & generic. #[no_mangle] mempertahankan nama asli untuk linker C."
            }
            Self::UnwindingBehavior => {
                "Panic Rust dilarang keras melintasi batas ABI extern \"C\". Wajib dicegah via std::panic::catch_unwind."
            }
        }
    }
}

/// Struktur data yang kompatibel dengan ABI C.
/// Atribut `#[repr(C)]` mematikan reordering field Rust dan mengikuti aturan padding/alignment C.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CPoint2D {
    pub x: f64,
    pub y: f64,
    pub id: i32,
}

impl CPoint2D {
    pub fn new(x: f64, y: f64, id: i32) -> Self {
        Self { x, y, id }
    }
}

// ============================================================================
// 2. EXPORTING RUST FUNCTIONS DENGAN `extern "C"` & `#[unsafe(no_mangle)]`
// ============================================================================

/// Fungsi yang diekspor menggunakan C calling convention.
/// `#[unsafe(no_mangle)]` memastikan linker dapat menemukan simbol `c_add_integers` tanpa enkripsi nama Rust.
#[unsafe(no_mangle)]
pub extern "C" fn c_add_integers(a: c_int, b: c_int) -> c_int {
    a.saturating_add(b)
}

/// Fungsi C ABI yang menghitung jarak Pythagoras (hipotenusa).
#[unsafe(no_mangle)]
pub extern "C" fn c_calculate_hypotenuse(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

/// Contoh string statis C yang aman untuk dipinjamkan lintas boundary FFI.
static STATUS_OK: &[u8] = b"STATUS_OK\0";
static STATUS_ERR: &[u8] = b"STATUS_ERROR\0";

/// Mengembalikan raw pointer ke string statis null-terminated C.
#[unsafe(no_mangle)]
pub extern "C" fn c_get_status_message(code: c_int) -> *const c_char {
    if code == 0 {
        STATUS_OK.as_ptr() as *const c_char
    } else {
        STATUS_ERR.as_ptr() as *const c_char
    }
}

/// Contoh fungsi FFI yang mengisolasi Rust panic agar tidak membocorkan unwinding ke C runtime.
///
/// # Safety
/// Pemanggil C/Rust wajib memastikan `out_result` adalah pointer valid yang dapat ditulis, atau null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_safe_divide(a: c_int, b: c_int, out_result: *mut c_int) -> c_int {
    if out_result.is_null() {
        return -1; // Error kode: null pointer
    }

    // Mengisolasi panic menggunakan catch_unwind
    let outcome = catch_unwind(|| {
        if b == 0 {
            panic!("Pembagian dengan nol terdeteksi di Rust runtime");
        }
        a / b
    });

    match outcome {
        Ok(val) => {
            unsafe {
                *out_result = val;
            }
            0 // Sukses (exit code C standard)
        }
        Err(_) => -2, // Error kode: panic ditangkap dengan aman
    }
}

// ============================================================================
// 3. IMPORTING C FUNCTIONS (DECLARATIVE C BINDINGS)
// ============================================================================

// Mengimpor fungsi pustaka standar C (libc) yang selalu ditautkan pada platform target.
// Pada Rust 2024 Edition, blok extern wajib diawali dengan keyword `unsafe`.
unsafe extern "C" {
    /// Fungsi libc `abs`: menghitung nilai mutlak bilangan bulat.
    pub fn abs(x: c_int) -> c_int;

    /// Fungsi libc `strlen`: menghitung panjang string null-terminated.
    pub fn strlen(s: *const c_char) -> usize;
}

// ============================================================================
// 4. IDENTIFIKASI BOUNDARY SAFE / UNSAFE (THE ADAPTER PATTERN)
// ============================================================================

/// Error representasi pada jembatan FFI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiBridgeError {
    NullPointerReceived,
    InteriorNullInRustString,
    InvalidUtf8String,
    PanicCaughtInBoundary,
}

impl std::fmt::Display for FfiBridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullPointerReceived => write!(f, "Pointer null diterima pada FFI boundary"),
            Self::InteriorNullInRustString => {
                write!(f, "String Rust mengandung byte null '\\0' di tengah teks")
            }
            Self::InvalidUtf8String => write!(f, "C string tidak berisi urutan UTF-8 yang valid"),
            Self::PanicCaughtInBoundary => write!(f, "Panic terisolasi pada batas eksekusi FFI"),
        }
    }
}

impl std::error::Error for FfiBridgeError {}

/// Safe wrapper di atas pemanggilan fungsi C `strlen`.
/// Menjamin Rust `&str` dikonversi dengan aman menjadi `CString` null-terminated
/// sebelum diserahkan ke pointer mentah libc.
pub fn safe_c_strlen(text: &str) -> Result<usize, FfiBridgeError> {
    // 1. Validasi safe: pastikan tidak ada interior null byte
    let c_string = CString::new(text).map_err(|_| FfiBridgeError::InteriorNullInRustString)?;

    // 2. Transisi boundary unsafe: panggil libc function
    // SAFETY: c_string dijamin valid, non-null, dan null-terminated.
    let len = unsafe { strlen(c_string.as_ptr()) };

    Ok(len)
}

/// Boundary wrapper untuk membaca string null-terminated C menjadi Rust `String`.
///
/// # Safety
/// Pemanggil wajib memastikan:
/// 1. `ptr` menunjuk ke string C null-terminated yang valid di memori aktif, atau bernilai null.
/// 2. Memori tidak dimutasi secara bersamaan oleh thread lain selama pembacaan.
pub unsafe fn safe_read_c_string(ptr: *const c_char) -> Result<String, FfiBridgeError> {
    if ptr.is_null() {
        return Err(FfiBridgeError::NullPointerReceived);
    }

    // SAFETY:
    // 1. Null check telah dipastikan di atas.
    // 2. Kontrak fungsi memastikan string valid dan null-terminated.
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        c_str
            .to_str()
            .map(|s| s.to_string())
            .map_err(|_| FfiBridgeError::InvalidUtf8String)
    }
}

/// Safe wrapper untuk memanggil fungsi kalkulasi C ABI.
pub fn safe_compute_distance(x: f64, y: f64) -> f64 {
    // Memanggil fungsi extern "C" Rust yang diekspor
    c_calculate_hypotenuse(x, y)
}

/// Safe wrapper untuk operasi pembagian dengan isolasi panic C ABI.
pub fn safe_divide_via_c_abi(a: i32, b: i32) -> Result<i32, FfiBridgeError> {
    let mut result: c_int = 0;
    // SAFETY: &mut result valid, aligned, dan menunjuk ke memori stack aktif.
    let code = unsafe { c_safe_divide(a, b, &mut result as *mut c_int) };

    match code {
        0 => Ok(result),
        -1 => Err(FfiBridgeError::NullPointerReceived),
        -2 => Err(FfiBridgeError::PanicCaughtInBoundary),
        _ => Err(FfiBridgeError::PanicCaughtInBoundary),
    }
}

// ============================================================================
// 5. RUNNER DEMONSTRASI (pub fn run())
// ============================================================================

/// Fungsi utama runner demonstrasi untuk modul Fase 14 Task 2.
pub fn run() {
    println!("=== FASE 14 Task 2: Foreign Function Interface (FFI) & ABI ===");

    // 1. Konsep Dasar ABI
    println!("1. Empat Pilar Utama ABI (Application Binary Interface):");
    let pillars = [
        AbiPillar::CallingConvention,
        AbiPillar::DataLayout,
        AbiPillar::NameMangling,
        AbiPillar::UnwindingBehavior,
    ];
    for (i, p) in pillars.iter().enumerate() {
        println!("   [{}] {:?}: {}", i + 1, p, p.description());
    }

    // 2. Layout Memori C vs Rust
    println!("\n2. Kompatibilitas Layout Memori (#[repr(C)]):");
    let pt = CPoint2D::new(12.5, 24.0, 101);
    println!("   Struct CPoint2D: x={}, y={}, id={}", pt.x, pt.y, pt.id);
    println!(
        "   Ukuran memori (size_of) : {} bytes",
        std::mem::size_of::<CPoint2D>()
    );
    println!(
        "   Alignment (align_of)    : {} bytes",
        std::mem::align_of::<CPoint2D>()
    );

    // 3. Memanggil Fungsi C Eksternal (Libc)
    println!("\n3. Pemanggilan Fungsi Libc via unsafe extern \"C\":");
    let negative_val: c_int = -42;
    // SAFETY: abs() adalah fungsi murni dari pustaka standar C (libc)
    let abs_val = unsafe { abs(negative_val) };
    println!("   C libc abs({}) -> {}", negative_val, abs_val);

    // 4. Safe Boundary: String Conversion (Rust &str <-> C CStr/CString)
    println!("\n4. Safe Boundary Pattern: Konversi String Rust & C:");
    let sample_text = "Rust FFI Berhasil!";
    match safe_c_strlen(sample_text) {
        Ok(len) => println!(
            "   [Safe C-strlen] String \"{}\" -> Panjang: {} bytes",
            sample_text, len
        ),
        Err(e) => println!("   [Error]: {}", e),
    }

    // Membaca string C statis melalui safe wrapper
    let msg_ptr_ok = c_get_status_message(0);
    let msg_ptr_err = c_get_status_message(1);
    // SAFETY: msg_ptr_ok dan msg_ptr_err menunjuk ke byte string statis null-terminated yang valid.
    let ok_str = unsafe { safe_read_c_string(msg_ptr_ok) }.unwrap_or_default();
    let err_str = unsafe { safe_read_c_string(msg_ptr_err) }.unwrap_or_default();
    println!("   Status Code 0 -> {}", ok_str);
    println!("   Status Code 1 -> {}", err_str);

    // 5. Ekspor Rust ke C ABI & Isolasi Panic
    println!("\n5. Ekspor Rust ke C ABI & Isolasi Panic (catch_unwind):");
    let sum = c_add_integers(25, 17);
    println!("   c_add_integers(25, 17) via extern \"C\" = {}", sum);

    let hypotenuse = safe_compute_distance(3.0, 4.0);
    println!("   safe_compute_distance(3.0, 4.0) = {:.2}", hypotenuse);

    // Uji safe divide normal
    match safe_divide_via_c_abi(100, 4) {
        Ok(val) => println!("   safe_divide_via_c_abi(100, 4) = {}", val),
        Err(e) => println!("   Gagal: {}", e),
    }

    // Uji safe divide dengan pembagian nol (panic tertangkap, tidak crash)
    match safe_divide_via_c_abi(100, 0) {
        Ok(_) => panic!("Pembagian dengan nol seharusnya gagal!"),
        Err(e) => println!(
            "   safe_divide_via_c_abi(100, 0) -> [Expected Error Ditangkap]: {}",
            e
        ),
    }

    println!("\n[OK] Fase 14 Task 2 FFI demonstrasi selesai.");
}

// ============================================================================
// 6. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repr_c_point_layout() {
        let pt = CPoint2D::new(1.0, 2.0, 7);
        assert_eq!(pt.x, 1.0);
        assert_eq!(pt.y, 2.0);
        assert_eq!(pt.id, 7);
        assert!(std::mem::size_of::<CPoint2D>() >= 20);
    }

    #[test]
    fn test_c_add_integers() {
        assert_eq!(c_add_integers(10, 32), 42);
        assert_eq!(c_add_integers(-10, 10), 0);
    }

    #[test]
    fn test_c_calculate_hypotenuse() {
        let dist = c_calculate_hypotenuse(6.0, 8.0);
        assert!((dist - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_libc_abs_import() {
        unsafe {
            assert_eq!(abs(-999), 999);
            assert_eq!(abs(0), 0);
            assert_eq!(abs(42), 42);
        }
    }

    #[test]
    fn test_safe_c_strlen_success() {
        let res = safe_c_strlen("Halo Rust FFI");
        assert_eq!(res.unwrap(), 13);
    }

    #[test]
    fn test_safe_c_strlen_rejects_interior_null() {
        let bad_string = "Halo\0Dunia";
        let res = safe_c_strlen(bad_string);
        assert_eq!(res.unwrap_err(), FfiBridgeError::InteriorNullInRustString);
    }

    #[test]
    fn test_safe_read_c_string() {
        let msg_ok = c_get_status_message(0);
        let msg_err = c_get_status_message(99);

        unsafe {
            assert_eq!(safe_read_c_string(msg_ok).unwrap(), "STATUS_OK");
            assert_eq!(safe_read_c_string(msg_err).unwrap(), "STATUS_ERROR");

            let null_res = safe_read_c_string(std::ptr::null());
            assert_eq!(null_res.unwrap_err(), FfiBridgeError::NullPointerReceived);
        }
    }

    #[test]
    fn test_safe_divide_via_c_abi() {
        assert_eq!(safe_divide_via_c_abi(50, 2).unwrap(), 25);
        assert_eq!(
            safe_divide_via_c_abi(50, 0).unwrap_err(),
            FfiBridgeError::PanicCaughtInBoundary
        );
    }

    #[test]
    fn test_smoke_runner() {
        run();
    }
}
