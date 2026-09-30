// Fase 14 - Task 1: Unsafe Rust & Raw Pointers
// Rujukan: rust_learning_guide.md (FASE 14) & rust_execution_tasks.md (L1177-L1182)
//
// Cakupan Checklist:
// - [x] Buat raw pointer `*const T`.
// - [x] Buat raw pointer `*mut T`.
// - [x] Dereference dalam `unsafe`.
// - [x] Jelaskan invariant yang harus dijaga programmer.

use std::ptr;
use std::slice;

// ============================================================================
// 1. DOKUMENTASI INVARIANT KESELAMATAN MEMORI (PROGRAMMER INVARIANTS)
// ============================================================================

/// Representasi invariant yang wajib dijamin programmer saat menggunakan `unsafe`.
/// Pelanggaran salah satu invariant di bawah berujung pada Undefined Behavior (UB).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyInvariant {
    /// Invariant 1: Pointer tidak boleh null saat didereferensi.
    NonNull,
    /// Invariant 2: Alamat memori harus selaras (aligned) sesuai `std::mem::align_of::<T>()`.
    ProperAlignment,
    /// Invariant 3: Memori valid, dialokasikan, dan tidak dangling (lifetime belum berakhir).
    ValidAllocationAndLifetime,
    /// Invariant 4: Memori telah terinisialisasi dengan representasi bit yang valid untuk tipe T.
    ProperInitialization,
    /// Invariant 5: Tidak melanggar aturan aliasing (tidak boleh ada &mut bersamaan dengan pointer lain ke data yang sama).
    NoAliasingViolation,
    /// Invariant 6: Operasi pointer arithmetic tidak boleh keluar dari batas alokasi buffer.
    BoundsIntegrity,
}

impl SafetyInvariant {
    /// Penjelasan ringkas untuk setiap invariant keamanan memori.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::NonNull => {
                "Pointer tidak boleh NULL (0x0). Dereferensi null seketika memicu segfault / UB."
            }
            Self::ProperAlignment => {
                "Alamat pointer harus kelipatan dari align_of::<T>(). Pointer misaligned memicu panic hardware atau crash arsitektur."
            }
            Self::ValidAllocationAndLifetime => {
                "Alamat harus menunjuk ke alokasi memori aktif yang belum di-deallocate (menghindari use-after-free)."
            }
            Self::ProperInitialization => {
                "Memori harus memiliki nilai bit valid sebelum dibaca sebagai tipe data konkret (menghindari pembacaan uninitialized memory)."
            }
            Self::NoAliasingViolation => {
                "Mematuhi aturan borrow checker: jika ada akses tulis (*mut T), tidak boleh ada pembacaan aktif lain pada region tersebut."
            }
            Self::BoundsIntegrity => {
                "Pointer arithmetic (add/offset) tidak boleh melompat melebihi batas buffer memori yang dialokasikan."
            }
        }
    }
}

// ============================================================================
// 2. DASAR RAW POINTERS: PEMBUATAN & DEREFERENSI
// ============================================================================

/// Demonstrasi pembuatan raw pointer `*const T` dan `*mut T`.
/// Catatan: Pembuatan raw pointer adalah operasi SAFE (tidak butuh blok `unsafe`).
pub fn create_raw_pointers(value: &mut i32) -> (*const i32, *mut i32) {
    // 1. Pembuatan klasik menggunakan casting referensi (`as *const T`, `as *mut T`)
    let const_ptr: *const i32 = value as *const i32;
    let mut_ptr: *mut i32 = value as *mut i32;

    (const_ptr, mut_ptr)
}

/// Demonstrasi pembuatan pointer dengan macro modern `ptr::addr_of!` dan `ptr::addr_of_mut!`.
/// Pendekatan ini direkomendasikan pada Rust modern karena tidak membuat referensi perantara.
pub fn create_raw_pointers_modern(value: &mut i32) -> (*const i32, *mut i32) {
    let const_ptr = ptr::addr_of!(*value);
    let mut_ptr = ptr::addr_of_mut!(*value);
    (const_ptr, mut_ptr)
}

/// Membaca nilai dari raw pointer `*const T`.
///
/// # Safety
/// Pemanggil wajib memastikan:
/// 1. `ptr` bukan null.
/// 2. `ptr` aligned dan menunjuk ke instance valid dari `T`.
/// 3. Memori belum di-free.
pub unsafe fn read_raw_pointer<T: Copy>(ptr: *const T) -> T {
    // Dereferensi raw pointer WAJIB di dalam blok / fungsi unsafe
    unsafe { *ptr }
}

/// Mengubah nilai pada alamat memori yang ditunjuk `*mut T`.
///
/// # Safety
/// Pemanggil wajib memastikan:
/// 1. `ptr` bukan null dan aligned.
/// 2. `ptr` menunjuk ke alokasi valid dengan izin tulis.
/// 3. Tidak ada referensi lain yang sedang aktif membaca/menulis memori ini (aliasing check).
pub unsafe fn write_raw_pointer<T>(ptr: *mut T, new_value: T) {
    // Dereferensi mutasi raw pointer
    unsafe {
        *ptr = new_value;
    }
}

/// Memverifikasi apakah suatu pointer memenuhi keselarasan (alignment) tipe T.
pub fn is_pointer_aligned<T>(ptr: *const T) -> bool {
    let align = std::mem::align_of::<T>();
    (ptr as usize).is_multiple_of(align)
}

// ============================================================================
// 3. PRAKTEK SAFE ABSTRACTION: CUSTOM SPLIT_AT_MUT
// ============================================================================

/// Memecah sebuah slice mutable menjadi dua slice mutable yang tidak tumpang tindih (disjoint).
///
/// Mengapa butuh `unsafe`?
/// Compiler Rust tidak dapat membuktikan bahwa `&mut slice[..mid]` dan `&mut slice[mid..]`
/// berasal dari bagian memori yang berbeda. Tanpa raw pointer, compiler akan menolak
/// karena dianggap meminjam mutable ganda dari sumber data yang sama.
///
/// Fungsi ini adalah "Safe Abstraction":
/// Seluruh invariant divalidasi terlebih dahulu (boundary check), baru masuk blok `unsafe`.
pub fn custom_split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(
        mid <= len,
        "Index mid ({}) melebihi panjang slice ({})",
        mid,
        len
    );

    let ptr: *mut T = slice.as_mut_ptr();

    // SAFETY:
    // 1. `ptr` valid dan aligned karena berasal dari `slice` yang valid.
    // 2. `mid <= len`, sehingga pointer arithmetic `ptr.add(mid)` tetap berada dalam batas alokasi.
    // 3. Dua slice yang dihasilkan bersifat disjoint (tidak tumpang tindih),
    //    sehingga invariant eksklusivitas `&mut` tetap terjaga sepenuhnya.
    unsafe {
        let left = slice::from_raw_parts_mut(ptr, mid);
        let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        (left, right)
    }
}

// ============================================================================
// 4. PRAKTEK: POINTER ARITHMETIC & RAW BUFFER MANIPULATION
// ============================================================================

/// Struktur pembaca buffer biner langsung berbasis raw pointer.
pub struct RawBufferInspector<'a> {
    data: &'a [u8],
}

impl<'a> RawBufferInspector<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Membaca u32 dari offset tertentu menggunakan raw pointer arithmetic.
    pub fn read_u32_le_at(&self, offset: usize) -> Option<u32> {
        let size = std::mem::size_of::<u32>();
        if offset + size > self.data.len() {
            return None;
        }

        let base_ptr: *const u8 = self.data.as_ptr();

        // SAFETY:
        // 1. Boundary check telah dilakukan di atas (`offset + size <= data.len()`).
        // 2. `base_ptr.add(offset)` aman dan valid dalam alokasi `self.data`.
        // 3. Kita menggunakan `ptr::read_unaligned` untuk mencegah UB jika offset tidak 4-byte aligned!
        unsafe {
            let target_ptr = base_ptr.add(offset) as *const u32;
            let val = ptr::read_unaligned(target_ptr);
            Some(u32::from_le(val))
        }
    }
}

// ============================================================================
// 5. RUNNER DEMONSTRASI (pub fn run())
// ============================================================================

/// Fungsi utama runner demonstrasi untuk modul Fase 14 Task 1.
pub fn run() {
    println!("=== FASE 14 Task 1: Unsafe Rust & Raw Pointers ===");

    // 1. Invariant edukasi
    println!("1. Invariant Keamanan Memori yang Wajib Dijaga Programmer:");
    let invariants = [
        SafetyInvariant::NonNull,
        SafetyInvariant::ProperAlignment,
        SafetyInvariant::ValidAllocationAndLifetime,
        SafetyInvariant::ProperInitialization,
        SafetyInvariant::NoAliasingViolation,
        SafetyInvariant::BoundsIntegrity,
    ];
    for (i, inv) in invariants.iter().enumerate() {
        println!("   [{}] {:?}: {}", i + 1, inv, inv.description());
    }

    // 2. Pembuatan dan Dereferensi Raw Pointer
    println!("\n2. Pembuatan Raw Pointer (*const T & *mut T):");
    let mut number: i32 = 42;

    // Safe context: membuat pointer
    let (c_ptr, m_ptr) = create_raw_pointers(&mut number);
    let (modern_c_ptr, modern_m_ptr) = create_raw_pointers_modern(&mut number);

    println!("   Alamat *const i32 : {:p}", c_ptr);
    println!("   Alamat *mut i32   : {:p}", m_ptr);
    println!("   Pointer Aligned   : {}", is_pointer_aligned(c_ptr));
    println!(
        "   Modern AddrOf Ptr : {:p} / {:p}",
        modern_c_ptr, modern_m_ptr
    );

    // Unsafe context: dereferensi
    println!("\n3. Dereferensi dan Mutasi dalam Blok unsafe:");
    unsafe {
        // Pembacaan via *const T
        let read_val = read_raw_pointer(c_ptr);
        println!("   [Read] Nilai awal via *const i32: {}", read_val);

        // Penulisan via *mut T
        write_raw_pointer(m_ptr, 100);
        println!("   [Write] Nilai setelah ditulis via *mut i32: {}", *c_ptr);
    }
    println!("   [Verify] Nilai asli pada variabel aman: {}", number);

    // 4. Safe Abstraction: custom_split_at_mut
    println!("\n4. Safe Abstraction: custom_split_at_mut (Disjoint Mutable Slices):");
    let mut dataset = [10, 20, 30, 40, 50, 60];
    println!("   Buffer awal: {:?}", dataset);

    let (left, right) = custom_split_at_mut(&mut dataset, 3);
    // Mutasi kedua slice secara independen tanpa konflik borrow checker
    for item in left.iter_mut() {
        *item *= 2;
    }
    for item in right.iter_mut() {
        *item += 5;
    }

    println!("   Left slice  (* 2): {:?}", left);
    println!("   Right slice (+ 5): {:?}", right);
    println!("   Hasil akhir array: {:?}", dataset);

    // 5. Pointer Arithmetic & Read Unaligned
    println!("\n5. Raw Pointer Arithmetic & Unaligned Reading:");
    let payload: [u8; 8] = [0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0xAA, 0xBB, 0xCC];
    let inspector = RawBufferInspector::new(&payload);

    // Offset 1 berisi 0xDEADBEEF dalam format Little Endian
    if let Some(val) = inspector.read_u32_le_at(1) {
        println!("   U32 dibaca dari offset 1: 0x{:08X}", val);
    }

    // 6. Null Pointer Detection
    println!("\n6. Deteksi Null Pointer:");
    let null_ptr: *const i32 = ptr::null();
    println!("   Is null_ptr null? {}", null_ptr.is_null());
    println!("   Pointer null dilarang keras didereferensi (Safe Guard).");

    println!("\n[OK] Fase 14 Task 1 Unsafe demonstrasi selesai.");
}

// ============================================================================
// 6. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointer_creation_and_reading() {
        let mut val = 999;
        let (c_ptr, m_ptr) = create_raw_pointers(&mut val);

        assert!(!c_ptr.is_null());
        assert!(!m_ptr.is_null());
        assert!(is_pointer_aligned(c_ptr));

        unsafe {
            assert_eq!(read_raw_pointer(c_ptr), 999);
            write_raw_pointer(m_ptr, 1234);
            assert_eq!(read_raw_pointer(c_ptr), 1234);
        }
        assert_eq!(val, 1234);
    }

    #[test]
    fn test_addr_of_modern_pointers() {
        let mut x: i32 = 777;
        let (c_ptr, m_ptr) = create_raw_pointers_modern(&mut x);

        unsafe {
            assert_eq!(*c_ptr, 777);
            *m_ptr = 888;
            assert_eq!(*c_ptr, 888);
        }
        assert_eq!(x, 888);
    }

    #[test]
    fn test_custom_split_at_mut_correctness() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let (left, right) = custom_split_at_mut(&mut arr, 4);

        assert_eq!(left, &mut [1, 2, 3, 4]);
        assert_eq!(right, &mut [5, 6, 7]);

        left[0] = 100;
        right[0] = 500;

        assert_eq!(arr, [100, 2, 3, 4, 500, 6, 7]);
    }

    #[test]
    fn test_custom_split_at_mut_boundaries() {
        let mut arr = [10, 20];
        // Split di index 0
        let (left, right) = custom_split_at_mut(&mut arr, 0);
        assert_eq!(left.len(), 0);
        assert_eq!(right.len(), 2);

        // Split di index len
        let (left2, right2) = custom_split_at_mut(&mut arr, 2);
        assert_eq!(left2.len(), 2);
        assert_eq!(right2.len(), 0);
    }

    #[test]
    #[should_panic(expected = "melebihi panjang slice")]
    fn test_custom_split_at_mut_out_of_bounds() {
        let mut arr = [1, 2];
        custom_split_at_mut(&mut arr, 5);
    }

    #[test]
    fn test_raw_buffer_inspector_unaligned_read() {
        let bytes: [u8; 8] = [0x00, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00];
        let inspector = RawBufferInspector::new(&bytes);

        let val = inspector.read_u32_le_at(1);
        assert_eq!(val, Some(0x00001234));

        let out_of_bounds = inspector.read_u32_le_at(6);
        assert_eq!(out_of_bounds, None);
    }

    #[test]
    fn test_null_pointer_handling() {
        let null_const: *const u32 = ptr::null();
        let null_mut: *mut u32 = ptr::null_mut();

        assert!(null_const.is_null());
        assert!(null_mut.is_null());
    }

    #[test]
    fn test_smoke_runner() {
        run();
    }
}
