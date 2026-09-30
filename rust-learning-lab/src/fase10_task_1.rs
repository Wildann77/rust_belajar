// Fase 10 - Task 1: Smart Pointer Box<T> (Heap Allocation, Recursive Types, Dereferencing)
// Rujukan: rust_learning_guide.md (Sub-bab 10.1 - 10.4) & rust_execution_tasks.md (L871-L875)

use std::mem::size_of_val;
use std::ops::{Deref, DerefMut};

// ============================================================================
// 1. Heap Allocation & Stack vs Heap Memory Layout
// ============================================================================

/// Struct berukuran besar (1 MB) untuk menguji pemindahan alokasi ke Heap.
///
/// Jika ditaruh langsung di Stack, data sebesar ini rawan menyebabkan
/// stack overflow pada thread dengan batas stack kecil (default 2 MB di Linux).
pub struct LargeBuffer {
    pub data: [u8; 1024 * 1024], // 1 MB
}

impl LargeBuffer {
    /// Membuat buffer besar langsung dialokasikan di Heap via Box.
    pub fn new_boxed() -> Box<Self> {
        // Mengalokasikan vector terlebih dahulu lalu dikonversi ke boxed slice/struct
        // untuk mencegah copy 1 MB di stack frame sementara.
        let byte_vec = vec![0u8; 1024 * 1024];
        let boxed_slice = byte_vec.into_boxed_slice();
        // Verifikasi alokasi berhasil
        assert_eq!(boxed_slice.len(), 1024 * 1024);

        // Alokasi struct langsung di heap
        Box::new(Self {
            data: [7u8; 1024 * 1024],
        })
    }
}

/// Mendemonstrasikan alokasi primitif dan memverifikasi ukuran pointer di stack vs isi di heap.
pub fn demonstrate_heap_allocation() -> (usize, usize, i32) {
    let heap_val: Box<i32> = Box::new(42);

    // Di arsitektur 64-bit:
    // - Ukuran variable `heap_val` di Stack adalah 8 bytes (1 pointer usize)
    // - Ukuran nilai yang ditunjuk di Heap adalah 4 bytes (tipe i32)
    let stack_pointer_size = size_of_val(&heap_val);
    let heap_payload_size = size_of_val(&*heap_val);
    let value = *heap_val;

    (stack_pointer_size, heap_payload_size, value)
}

// ============================================================================
// 2. Recursive Types (Cons List & AST Expression Tree)
// ============================================================================

/// 2.1 Implementasi Klasik: Functional Cons List (Linked List)
///
/// Tanpa `Box`, definisi `enum List<T> { Cons(T, List<T>), Nil }` akan error:
/// `E0072: recursive type has infinite size`.
/// `Box` memecahkan masalah ini dengan memberikan ukuran tetap (usize / 8 byte) di Stack.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    /// Membuat list kosong
    pub fn new() -> Self {
        List::Nil
    }

    /// Menambahkan elemen baru di depan list (O(1))
    pub fn prepend(self, elem: T) -> Self {
        List::Cons(elem, Box::new(self))
    }

    /// Menghitung panjang linked list secara iteratif
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self;
        while let List::Cons(_, next) = current {
            count += 1;
            current = next;
        }
        count
    }

    /// Mengecek apakah list kosong
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }
}

impl<T: Clone> List<T> {
    /// Mengonversi Cons List menjadi Vec<T>
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self;
        while let List::Cons(val, next) = current {
            result.push(val.clone());
            current = next;
        }
        result
    }
}

/// 2.2 Implementasi Dunia Nyata: Abstract Syntax Tree (AST) untuk Evaluasi Matematika
///
/// Representasi pohon biner ekspresi matematika rekursif.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Evaluasi ekspresi rekursif ke nilai hasil akhir.
    /// Mengembalikan Result untuk menangani division by zero secara aman.
    pub fn eval(&self) -> Result<i64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Add(left, right) => Ok(left.eval()? + right.eval()?),
            Expr::Sub(left, right) => Ok(left.eval()? - right.eval()?),
            Expr::Mul(left, right) => Ok(left.eval()? * right.eval()?),
            Expr::Div(left, right) => {
                let divisor = right.eval()?;
                if divisor == 0 {
                    Err(String::from("Peringatan: Pembagian dengan angka nol!"))
                } else {
                    Ok(left.eval()? / divisor)
                }
            }
        }
    }

    /// Menghitung kedalaman maksimum hierarki AST rekursif
    pub fn depth(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
                1 + std::cmp::max(l.depth(), r.depth())
            }
        }
    }
}

// ============================================================================
// 3. Dereferencing Box, Deref Trait & Deref Coercion
// ============================================================================

/// Wrapper kustom sederhana untuk memahami implementasi `Deref` di balik layar
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Fungsi yang menerima string slice (`&str`).
/// Digunakan untuk menguji Deref Coercion dari `&Box<String>`.
pub fn greet(name: &str) -> String {
    format!("Halo, {name}!")
}

/// Fungsi yang menerima array slice (`&[i32]`).
/// Digunakan untuk menguji Deref Coercion dari `&Box<Vec<i32>>`.
pub fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// Mendemonstrasikan mutasi nilai di dalam Box melalui dereference mutable
pub fn mutate_boxed_value(boxed: &mut Box<i32>, increment: i32) {
    *boxed.as_mut() += increment; // atau `**boxed += increment;`
}

/// Mendemonstrasikan unboxing: mengeluarkan kepemilikan nilai `T` dari `Box<T>`
pub fn unbox_value<T>(boxed: Box<T>) -> T {
    *boxed // Memindahkan (move) nilai keluar dari heap ke caller
}

// ============================================================================
// 4. Box untuk Trait Objects (Dynamic Dispatch / Heterogeneous Collection)
// ============================================================================

/// Trait untuk komponen yang dapat dirender ke teks
pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Header {
    pub text: String,
}

impl Renderable for Header {
    fn render(&self) -> String {
        format!("# {}", self.text)
    }
}

pub struct Paragraph {
    pub content: String,
}

impl Renderable for Paragraph {
    fn render(&self) -> String {
        format!("<p>{}</p>", self.content)
    }
}

pub struct CodeBlock {
    pub lang: String,
    pub code: String,
}

impl Renderable for CodeBlock {
    fn render(&self) -> String {
        format!("```{}\n{}\n```", self.lang, self.code)
    }
}

/// Merender kumpulan elemen heterogen menggunakan fat pointer `Box<dyn Renderable>`
pub fn render_document(elements: &[Box<dyn Renderable>]) -> String {
    elements
        .iter()
        .map(|el| el.render())
        .collect::<Vec<String>>()
        .join("\n\n")
}

// ============================================================================
// 5. Entry Point Eksekusi Modul
// ============================================================================

pub fn run() {
    println!("=== FASE 10 TASK 1: SMART POINTER BOX<T> ===");

    // 1. Heap Allocation & Memory Footprint
    println!("\n1. Heap Allocation & Ukuran Memori:");
    let (stack_sz, heap_sz, val) = demonstrate_heap_allocation();
    println!("   - Nilai dalam Box: {val}");
    println!("   - Ukuran variabel Box di Stack: {stack_sz} bytes (pointer 64-bit)");
    println!("   - Ukuran payload nilai di Heap: {heap_sz} bytes");

    let big_box = LargeBuffer::new_boxed();
    println!(
        "   - LargeBuffer (1 MB) sukses dialokasikan di Heap. Byte pertama: {}",
        big_box.data[0]
    );

    // 2. Recursive Types: Cons List
    println!("\n2. Recursive Type: Cons List (Linked List):");
    let list: List<i32> = List::new().prepend(30).prepend(20).prepend(10);
    println!("   - List berhasil dibentuk: {:?}", list);
    println!(
        "   - Panjang list: {} (is_empty: {})",
        list.len(),
        list.is_empty()
    );
    println!("   - Representasi Vec: {:?}", list.to_vec());

    // 3. Recursive Types: AST Expression Tree
    println!("\n3. Recursive Type: Abstract Syntax Tree (AST):");
    // Ekspresi: ((10 + 5) * 4) - (50 / 2) = (15 * 4) - 25 = 60 - 25 = 35
    let expr = Expr::Sub(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Number(10)),
                Box::new(Expr::Number(5)),
            )),
            Box::new(Expr::Number(4)),
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Number(50)),
            Box::new(Expr::Number(2)),
        )),
    );

    println!("   - Kedalaman pohon ekspresi AST: {}", expr.depth());
    match expr.eval() {
        Ok(result) => println!("   - Hasil evaluasi AST ((10+5)*4) - (50/2) = {result}"),
        Err(err) => println!("   - Error: {err}"),
    }

    // Uji proteksi error divide-by-zero
    let zero_div_expr = Expr::Div(Box::new(Expr::Number(100)), Box::new(Expr::Number(0)));
    println!("   - Uji pembagian dengan nol: {:?}", zero_div_expr.eval());

    // 4. Dereferencing & Deref Coercion
    println!("\n4. Dereferencing & Deref Coercion:");
    let mut boxed_num = Box::new(100);
    println!("   - Nilai awal: {}", *boxed_num);
    mutate_boxed_value(&mut boxed_num, 50);
    println!("   - Nilai setelah mutasi via deref: {}", *boxed_num);

    let unboxed = unbox_value(boxed_num);
    println!("   - Nilai setelah unboxing (move out): {unboxed}");

    // Deref coercion: Box<String> -> &String -> &str
    let boxed_name = Box::new(String::from("Rustacean"));
    println!("   - Deref coercion string: {}", greet(&boxed_name));

    // Deref coercion: Box<Vec<i32>> -> &Vec<i32> -> &[i32]
    let boxed_vec = Box::new(vec![1, 2, 3, 4, 5]);
    println!("   - Deref coercion slice: sum = {}", sum_slice(&boxed_vec));

    // Custom MyBox Deref
    let mut my_b = MyBox::new(String::from("Hello"));
    assert_eq!(*my_b, "Hello");
    my_b.push_str(" World");
    println!("   - Custom MyBox DerefMut hasil: {}", *my_b);

    // 5. Box<dyn Trait> (Dynamic Dispatch)
    println!("\n5. Box<dyn Trait> Heterogeneous Collection:");
    let doc: Vec<Box<dyn Renderable>> = vec![
        Box::new(Header {
            text: String::from("Dokumentasi Smart Pointer"),
        }),
        Box::new(Paragraph {
            content: String::from("Box<T> adalah smart pointer paling sederhana di Rust."),
        }),
        Box::new(CodeBlock {
            lang: String::from("rust"),
            code: String::from("let b = Box::new(42);"),
        }),
    ];
    let output = render_document(&doc);
    println!("   Rendered document:\n{output}");

    println!("\n[OK] Task Fase 10 (Box) selesai & terverifikasi.");
}

// ============================================================================
// Unit Tests Komprehensif
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_allocation_sizes() {
        let (ptr_sz, payload_sz, val) = demonstrate_heap_allocation();
        assert_eq!(ptr_sz, std::mem::size_of::<usize>());
        assert_eq!(payload_sz, 4);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_cons_list_operations() {
        let empty: List<i32> = List::new();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert_eq!(empty.to_vec(), Vec::<i32>::new());

        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_ast_evaluation() {
        // (10 + 20) * 3 = 90
        let expr = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Number(10)),
                Box::new(Expr::Number(20)),
            )),
            Box::new(Expr::Number(3)),
        );
        assert_eq!(expr.eval().unwrap(), 90);
        assert_eq!(expr.depth(), 3);

        // Division by zero
        let div_zero = Expr::Div(Box::new(Expr::Number(10)), Box::new(Expr::Number(0)));
        assert!(div_zero.eval().is_err());
    }

    #[test]
    fn test_deref_and_mutation() {
        let mut b = Box::new(10);
        *b += 5;
        assert_eq!(*b, 15);

        mutate_boxed_value(&mut b, 10);
        assert_eq!(*b, 25);

        let extracted = unbox_value(b);
        assert_eq!(extracted, 25);
    }

    #[test]
    fn test_deref_coercion() {
        let boxed_str = Box::new(String::from("World"));
        assert_eq!(greet(&boxed_str), "Halo, World!");

        let boxed_nums = Box::new(vec![10, 20, 30]);
        assert_eq!(sum_slice(&boxed_nums), 60);
    }

    #[test]
    fn test_custom_mybox() {
        let b = MyBox::new(50);
        assert_eq!(*b, 50);

        let mut b_str = MyBox::new(String::from("Rust"));
        b_str.push_str(" Lab");
        assert_eq!(*b_str, "Rust Lab");
    }

    #[test]
    fn test_boxed_trait_objects() {
        let elements: Vec<Box<dyn Renderable>> = vec![
            Box::new(Header {
                text: "Judul".into(),
            }),
            Box::new(Paragraph {
                content: "Isi".into(),
            }),
        ];
        let doc = render_document(&elements);
        assert!(doc.contains("# Judul"));
        assert!(doc.contains("<p>Isi</p>"));
    }
}
