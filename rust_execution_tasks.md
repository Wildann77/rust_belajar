# Rust Execution Tasks — Belajar Sambil Praktik

> **Basis kurikulum:** Roadmap Komprehensif Belajar Rust — 15 Fase  
> **Rust Edition:** 2024  
> **Target:** Dari nol → memahami ownership/type system → concurrency/async → advanced Rust → production backend  
> **Metode:** Pelajari konsep → kerjakan task → buat mini-project → jalankan test → review → lanjut fase berikutnya.

---

# Cara Menggunakan Task Ini

## Aturan belajar

- [ ] Jangan hanya membaca kode. Ketik ulang dan jalankan sendiri.
- [ ] Setiap error compiler dianggap sebagai bagian dari materi.
- [ ] Sebelum melihat solusi, coba perbaiki error sendiri.
- [ ] Gunakan `cargo check` sesering mungkin.
- [ ] Setiap fase harus menghasilkan artefak yang bisa dijalankan.
- [ ] Jangan lanjut hanya karena kode "bisa compile"; pastikan kamu bisa menjelaskan **mengapa** kode tersebut diterima compiler.

## Workflow harian

```text
Baca konsep
   ↓
Tulis ulang contoh
   ↓
Eksperimen kecil
   ↓
Sengaja buat error
   ↓
Baca error compiler
   ↓
Perbaiki
   ↓
Buat mini-project
   ↓
Test
   ↓
Catat apa yang dipahami
```

## Status

Gunakan:
- `[ ]` belum dikerjakan
- `[x]` selesai
- `[~]` sedang dipelajari
- `[!]` perlu diulang

---

# FASE 0 — Persiapan Environment

## Target

Memastikan toolchain Rust 2024 siap digunakan.

### Task

- [x] Install Rust melalui `rustup`.
- [x] Pastikan `rustc` tersedia.
- [x] Pastikan `cargo` tersedia.
- [x] Pastikan `rustup` tersedia.
- [x] Pastikan shell sudah mengenali Cargo.
- [x] Buat folder khusus seluruh latihan Rust.

### Command

```bash
rustc --version
cargo --version
rustup --version

rustup show
rustup toolchain list
```

### Project pertama

```bash
cargo new rust-learning-lab
cd rust-learning-lab
cargo run
```

### Checklist pemahaman

- [ ] Bisa menjelaskan perbedaan `rustc`, `cargo`, dan `rustup`.
- [ ] Bisa menjelaskan fungsi `Cargo.toml`.
- [ ] Bisa menemukan `src/main.rs`.
- [ ] Bisa menjelaskan folder `target/`.

### Lulus fase

```bash
cargo check
cargo run
```

berhasil tanpa error.

---

# FASE 1 — Tooling Cargo & Sintaks Inti

## Target

Menguasai syntax dasar Rust dan workflow Cargo.

Materi utama:
- variables
- mutability
- shadowing
- constants
- scalar types
- compound types
- functions
- statements
- expressions
- `if`
- `loop`
- `while`
- `for`

## Task 1 — Variables

- [x] Buat variable immutable.
- [x] Coba ubah immutable variable dan amati compiler error.
- [x] Buat variable `mut`.
- [x] Lakukan shadowing.
- [x] Shadow variable dengan tipe berbeda.
- [x] Buat `const`.

Contoh latihan:

```rust
fn main() {
    let x = 10;
    let mut y = 20;

    y += 5;

    let name = "Rust";
    let name = name.len();

    const MAX_USERS: usize = 1000;

    println!("{x}");
    println!("{y}");
    println!("{name}");
    println!("{MAX_USERS}");
}
```

## Task 2 — Tipe data

- [x] Coba `i8`, `i32`, `i64`.
- [x] Coba `u8`, `u32`, `usize`.
- [x] Coba `f32`, `f64`.
- [x] Coba `bool`.
- [x] Coba `char`.
- [x] Buat tuple.
- [x] Destructure tuple.
- [x] Buat array.
- [x] Akses array menggunakan index.

## Task 3 — Functions

Buat:

```rust
fn add(a: i32, b: i32) -> i32
fn is_even(n: i32) -> bool
fn multiply(a: i32, b: i32) -> i32
```

- [x] Gunakan parameter.
- [x] Gunakan return type.
- [x] Gunakan expression sebagai return.
- [x] Eksperimen dengan `return`.

## Task 4 — Expression vs Statement

- [x] Buat expression block.
- [x] Hilangkan `;` dari return expression.
- [x] Tambahkan `;` dan lihat perubahan error.
- [x] Jelaskan mengapa block `{ ... }` dapat menghasilkan nilai.

## Task 5 — Control Flow

- [x] `if`
- [x] `if/else`
- [x] `if/else if/else`
- [x] `loop`
- [x] `break`
- [x] `continue`
- [x] loop dengan return value
- [x] loop label
- [x] `while`
- [x] `for`
- [x] range
- [x] `.rev()`

## Mini Project Fase 1 — Calculator CLI

Buat program:

```text
=== Rust Calculator ===
Input angka 1:
Input operator (+ - * /):
Input angka 2:
Hasil:
```

Fitur:
- [x] Addition
- [x] Subtraction
- [x] Multiplication
- [x] Division
- [x] Validasi input sederhana
- [x] Function terpisah untuk operasi

### Lulus fase

- [x] Program bisa dijalankan dengan `cargo run`.
- [x] Minimal 5 function buatan sendiri.
- [x] Bisa menjelaskan expression vs statement tanpa melihat catatan.
- [x] Bisa menjelaskan immutable vs mutable vs shadowing.

---

# FASE 2 — Ownership, Borrowing, Slices & UTF-8

## Target

Memahami alasan utama Rust memiliki borrow checker.

Materi:
- ownership
- scope
- move
- copy
- clone
- references
- mutable references
- borrowing
- NLL
- slices
- `String`
- `&str`
- UTF-8
- `chars()`
- `bytes()`

## Task 1 — Ownership

- [x] Buat `String`.
- [x] Pindahkan `String` ke variable lain.
- [x] Coba gunakan variable lama.
- [x] Baca error compiler.
- [x] Perbaiki menggunakan `.clone()`.
- [x] Uji tipe `Copy`.

## Task 2 — Function & Ownership

Buat:

```rust
fn takes_ownership(s: String)
fn gives_ownership() -> String
fn calculate_length(s: &String) -> usize
```

- [x] Observasi kapan ownership pindah.
- [x] Ubah function agar menggunakan borrow.
- [x] Jelaskan kapan clone diperlukan.

## Task 3 — Borrowing

- [x] Banyak `&T`.
- [x] Satu `&mut T`.
- [x] Coba konflik `&T` dengan `&mut T`.
- [x] Cari tahu kapan borrow selesai.
- [x] Eksperimen NLL.

## Task 4 — Slices

- [x] Array slice.
- [x] String slice.
- [x] Function menerima `&str`.
- [x] Function menerima `&[i32]`.

## Task 5 — UTF-8

Eksperimen:

```rust
let text = String::from("Rust 🦀");
```

- [x] `len()`
- [x] `chars().count()`
- [x] `bytes()`
- [x] iterasi `chars()`
- [x] iterasi `bytes()`
- [x] Coba slicing pada boundary yang valid.
- [x] Coba slicing yang memotong karakter dan amati hasilnya.

## Mini Project Fase 2 — Text Analyzer

Input:

```text
Masukkan teks:
```

Output:
```text
Bytes       : ...
Characters  : ...
Words       : ...
```

Tambahkan:
- [x] Function untuk menghitung byte.
- [x] Function untuk menghitung character.
- [x] Function untuk menghitung word.
- [x] Function menerima `&str`, bukan mengambil ownership jika tidak perlu.

### Lulus fase

- [x] Bisa menjelaskan move vs copy vs clone.
- [x] Bisa menjelaskan `String` vs `&str`.
- [x] Bisa menjelaskan mengapa UTF-8 membuat indexing string tidak sederhana.
- [x] Bisa menjelaskan NLL dengan contoh sendiri.

---

# FASE 3 — Struct, Enum & Pattern Matching

## Target

Menggunakan type system Rust untuk memodelkan domain.

## Task 1 — Struct

- [x] Classic struct.
- [x] Tuple struct.
- [x] Unit-like struct.
- [x] `impl`.
- [x] Associated function.
- [x] Method `&self`.
- [x] Method `&mut self`.
- [x] `Self`.

## Task 2 — Enum

Buat:

```rust
enum Status {
    Todo,
    InProgress,
    Done,
}
```

- [x] Match enum.
- [x] Enum membawa data.
- [x] Struct-like enum variant.
- [x] Tuple-like enum variant.

## Task 3 — Pattern Matching

- [x] `match`
- [x] exhaustive matching
- [x] destructuring
- [x] match guard
- [x] `if let`
- [x] `while let`
- [x] range pattern
- [x] binding

## Mini Project Fase 3 — Task Domain Model

Buat:

```text
Task
├── id
├── title
├── priority
└── status
```

Fitur:
- [x] Constructor.
- [x] Change status.
- [x] Display task.
- [x] Match berdasarkan priority.
- [x] Match berdasarkan status.

### Lulus fase

- [x] Bisa membuat domain model tanpa meniru contoh.
- [x] Bisa menjelaskan kenapa `match` harus exhaustive.
- [x] Bisa menggunakan `if let` dan `while let`.

---

# FASE 4 — Module System & Code Organization

## Target

Berhenti menulis seluruh aplikasi dalam satu `main.rs`.

Materi:
- package
- crate
- module
- path
- `use`
- `pub`
- `pub(crate)`
- `pub(super)`
- `pub use`
- `lib.rs`
- binary crate
- library crate

## Task

Buat struktur:

```text
task_app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   └── token.rs
│   └── services/
│       └── task_service.rs
└── tests/
```

- [x] Pisahkan model.
- [x] Pisahkan service.
- [x] Pisahkan module auth.
- [x] Gunakan `pub`.
- [x] Gunakan `pub(crate)`.
- [x] Gunakan `use`.
- [x] Coba `pub use` sebagai re-export.
- [x] Tambahkan integration test.

## Mini Project Fase 4 — Modular Task App

Refactor project Fase 3 menjadi multi-module.

### Lulus fase

- [x] Tidak semua logic berada di `main.rs`.
- [x] Bisa menjelaskan package vs crate vs module.
- [x] Bisa membuat library crate sendiri.

---

# FASE 5 — Cargo Tingkat Lanjut & Workspace

## Target

Menguasai Cargo untuk project multi-crate.

## Task 1 — Dependency

- [x] Tambahkan dependency.
- [x] Tambahkan dev-dependency.
- [x] Jalankan `cargo tree`.
- [x] Baca `Cargo.lock`.

## Task 2 — Features

Buat feature:

```toml
[features]
default = []
async_runtime = []
```

- [x] Pahami optional dependency.
- [x] Aktifkan dependency melalui feature.
- [x] Build dengan feature berbeda.

## Task 3 — Profiles

Pelajari:

- [x] `profile.dev`
- [x] `profile.release`
- [x] `opt-level`
- [x] LTO
- [x] debug symbols

## Task 4 — Workspace

Buat:

```text
rust-workspace/
├── Cargo.toml
└── crates/
    ├── core_domain/
    ├── database_adapter/
    └── api_server/
```

- [x] Workspace root.
- [x] Workspace members.
- [x] Workspace dependencies.
- [x] Cross-crate dependency.
- [x] Build seluruh workspace.

### Lulus fase

- [x] Bisa membuat workspace dari nol.
- [x] Bisa menjelaskan manfaat workspace.
- [x] Bisa memakai `cargo tree`, `cargo fmt`, `cargo clippy`, `cargo test`.

---

# FASE 6 — Robust Error Handling & Collections

## Target

Menghilangkan pola error handling yang hanya mengandalkan `unwrap()`.

Materi:
- `Option<T>`
- `Result<T, E>`
- `?`
- `map`
- `map_err`
- `and_then`
- `unwrap_or`
- `unwrap_or_else`
- custom errors
- `std::error::Error`
- Vec
- HashMap
- Entry API

## Task 1 — Option

- [x] `Some`
- [x] `None`
- [x] `match`
- [x] `if let`
- [x] `map`
- [x] `and_then`
- [x] `unwrap_or`

## Task 2 — Result

- [x] `Ok`
- [x] `Err`
- [x] `match`
- [x] `?`
- [x] error propagation

## Task 3 — Custom Error

Buat:

```rust
enum AppError {
    NotFound(String),
    InvalidInput(String),
    Io(std::io::Error),
}
```

- [x] `Debug`
- [x] `Display`
- [x] `Error`
- [x] Konversi error
- [x] Propagation dengan `?`

## Task 4 — Collections

- [x] `Vec<T>`
- [x] `Vec::with_capacity`
- [x] `push`
- [x] `get`
- [x] `retain`
- [x] `HashMap`
- [x] `entry`
- [x] `or_insert`
- [x] `and_modify`

## Mini Project Fase 6 — CLI Task Manager v1

Fitur:
- [x] Add task.
- [x] List task.
- [x] Delete task.
- [x] Find task.
- [x] Error ketika task tidak ditemukan.
- [x] Tidak memakai `unwrap()` pada jalur input utama.

### Lulus fase

- [x] Bisa menjelaskan kapan menggunakan `Option` dan kapan `Result`.
- [x] Bisa memakai `?`.
- [x] Bisa membuat custom error sederhana.
- [x] Bisa memakai HashMap Entry API.

---

# FASE 7 — Generics, Traits & Advanced Trait System

## Target

Memahami abstraction Rust tanpa mengandalkan inheritance.

Materi:
- generics
- trait
- trait bounds
- `where`
- static dispatch
- dynamic dispatch
- `dyn Trait`
- monomorphization
- associated types
- supertraits
- orphan rule
- newtype pattern

## Task 1 — Generic Function

Buat function generic untuk:
- [x] memilih nilai terbesar.
- [x] mencetak nilai.
- [x] mengubah collection.

## Task 2 — Trait

Buat:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

- [x] Implementasikan pada 2 struct.
- [x] Default implementation.
- [x] Trait bound.

## Task 3 — Dispatch

Buat dua versi:

```rust
fn process<T: Summary>(item: &T)
fn process(item: &dyn Summary)
```

- [x] Bandingkan static dispatch.
- [x] Bandingkan dynamic dispatch.
- [x] Gunakan `Box<dyn Summary>`.

## Task 4 — Associated Types

Buat trait:

```rust
trait Repository {
    type Item;
    type Error;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error>;
}
```

- [x] Definisikan trait dengan associated types `type Item` dan `type Error`.
- [x] Implementasikan pada struct konkret (misal `UserRepository`).
- [x] Bandingkan associated types vs generic parameters (`trait Repository<Item, Error>`).

## Task 5 — Newtype

Buat:

```rust
struct UserId(u64);
struct OrderId(u64);
```

- [x] Jelaskan mengapa type safety lebih baik daripada memakai `u64` untuk semuanya.

## Mini Project Fase 7 — Repository Abstraction

Buat:

```text
Repository trait
     ↓
InMemoryRepository
     ↓
MockRepository
```

### Lulus fase

- [x] Bisa menjelaskan generic vs trait.
- [x] Bisa menjelaskan static vs dynamic dispatch.
- [x] Bisa menggunakan associated type.
- [x] Paham orphan rule secara konsep.

---

# FASE 8 — Lifetimes Mendalam

## Target

Memahami lifetime sebagai hubungan validitas reference, bukan sebagai "memperpanjang umur object".

## Task

- [x] `fn longest<'a>(...)`.
- [x] Multiple lifetime parameter.
- [x] Lifetime pada struct.
- [x] Lifetime pada `impl`.
- [x] Lifetime elision.
- [x] `'static`.
- [x] Reference sebagai field.
- [x] Generic + lifetime.
- [x] Trait + lifetime.

## Eksperimen

Buat beberapa versi:

```rust
fn first<'a>(a: &'a str, b: &'a str) -> &'a str
```

dan:

```rust
struct Parser<'a> {
    source: &'a str,
}
```
 
- [x] Sengaja buat dangling-reference scenario.
- [x] Baca error borrow checker.
- [x] Perbaiki tanpa `.clone()` jika memungkinkan.

### Lulus fase

- [x] Bisa menjelaskan apa fungsi `'a`.
- [x] Bisa menjelaskan bahwa lifetime annotation tidak memperpanjang umur object.
- [x] Bisa menjelaskan lifetime elision.
- [x] Bisa membuat struct yang menyimpan reference.

---

# FASE 9 — Functional Rust

## Target

Menggunakan closure dan iterator secara idiomatic.

Materi:
- closures
- `Fn`
- `FnMut`
- `FnOnce`
- `move`
- iterator
- `map`
- `filter`
- `take`
- `fold`
- `collect`

## Task 1 — Closures

- [ ] Closure tanpa capture.
- [ ] Closure capture immutable.
- [ ] Closure capture mutable.
- [ ] `move`.
- [ ] Tentukan apakah closure menjadi `Fn`, `FnMut`, atau `FnOnce`.

## Task 2 — Iterator

Buat pipeline:

```rust
numbers
    .iter()
    .filter(...)
    .map(...)
    .take(...)
    .collect()
```

- [ ] `iter()`
- [ ] `iter_mut()`
- [ ] `into_iter()`
- [ ] `map`
- [ ] `filter`
- [ ] `find`
- [ ] `any`
- [ ] `all`
- [ ] `fold`
- [ ] `collect`

## Mini Project Fase 9 — Statistics Processor

Input:

```text
[10, 20, 11, 30, 40, 21, 50]
```

Output:
- [ ] angka genap
- [ ] angka > threshold
- [ ] square
- [ ] sum
- [ ] average
- [ ] maximum
- [ ] minimum

### Lulus fase

- [ ] Bisa membedakan `iter()`, `iter_mut()`, `into_iter()`.
- [ ] Bisa menjelaskan laziness iterator.
- [ ] Bisa menjelaskan Fn/FnMut/FnOnce dengan contoh sendiri.

---

# FASE 10 — Smart Pointers & Interior Mutability

## Target

Memahami bagaimana Rust mengelola ownership yang lebih kompleks.

Materi:
- `Box<T>`
- `Deref`
- `Drop`
- `Rc<T>`
- `RefCell<T>`
- `Arc<T>`
- `Mutex<T>`
- `RwLock<T>`
- atomics
- interior mutability

## Task

### Box

- [ ] Heap allocation.
- [ ] Recursive type.
- [ ] Dereference `Box`.

### Rc

- [ ] Shared ownership single-thread.
- [ ] `Rc::clone`.
- [ ] `strong_count`.

### RefCell

- [ ] `borrow`.
- [ ] `borrow_mut`.
- [ ] Sengaja buat double mutable borrow.
- [ ] Amati runtime panic.

### Arc

- [ ] Clone `Arc`.
- [ ] Share value antar-thread.

### Mutex / RwLock

- [ ] Shared mutable state.
- [ ] Lock.
- [ ] Guard.
- [ ] Scope lock.
- [ ] Read lock vs write lock.

## Mini Project Fase 10 — Shared Counter

Buat:
- [ ] `Arc<Mutex<i32>>`
- [ ] 10 worker tasks/threads
- [ ] Setiap worker increment counter
- [ ] Join semuanya
- [ ] Pastikan hasil deterministic

### Lulus fase

- [ ] Bisa menjelaskan perbedaan `Rc` vs `Arc`.
- [ ] Bisa menjelaskan `RefCell` vs `Mutex`.
- [ ] Bisa menjelaskan kapan memilih `Mutex` dan `RwLock`.

---

# FASE 11 — Concurrency

## Target

Memahami concurrency native Rust sebelum masuk async.

Materi:
- OS threads
- `thread::spawn`
- `JoinHandle`
- `move`
- channels
- `mpsc`
- shared state
- `Send`
- `Sync`
- race safety

## Task 1 — Threads

- [ ] Spawn 2 thread.
- [ ] Spawn banyak thread.
- [ ] Return value dari thread.
- [ ] `join()`.

## Task 2 — Channels

- [ ] `mpsc::channel`.
- [ ] Clone transmitter.
- [ ] Multi producer.
- [ ] Single consumer.
- [ ] `drop(tx)` untuk menutup channel.

## Task 3 — Send & Sync

- [ ] Cari tipe yang `Send`.
- [ ] Cari tipe yang tidak `Send`.
- [ ] Cari tipe yang `Sync`.
- [ ] Pahami hubungan `&T` dan `Send`.

## Mini Project Fase 11 — Worker Pool CLI

Arsitektur:

```text
Main
 │
 ├── Job Channel
 │
 ├── Worker 1
 ├── Worker 2
 ├── Worker 3
 └── Worker 4
        │
        ▼
    Result Channel
```

Fitur:
- [ ] Kirim job.
- [ ] Worker memproses job.
- [ ] Worker mengirim hasil.
- [ ] Main mengumpulkan hasil.
- [ ] Graceful completion.

### Lulus fase

- [ ] Bisa membedakan concurrency dan parallelism.
- [ ] Bisa menjelaskan `Send` dan `Sync`.
- [ ] Bisa membuat worker pool sederhana.

---

# FASE 12 — Modern Async Rust & Tokio

## Target

Memahami async bukan hanya sebagai "thread yang lebih cepat".

Materi:
- `Future`
- `async`
- `.await`
- executor
- runtime
- task
- `tokio::spawn`
- `JoinHandle`
- `tokio::select!`
- `spawn_blocking`
- timeout
- cancellation
- channels
- `Pin`
- `Waker`

## Task 1 — Future Mental Model

- [ ] Jelaskan mengapa Future bersifat lazy.
- [ ] Pahami `poll`.
- [ ] Pahami executor.
- [ ] Pahami `Waker`.
- [ ] Pahami secara konseptual mengapa `Pin` ada.

## Task 2 — Tokio

Buat project baru:

```bash
cargo new tokio-lab
cd tokio-lab

cargo add tokio --features full
```

- [ ] `#[tokio::main]`
- [ ] `async fn`
- [ ] `.await`
- [ ] `tokio::spawn`
- [ ] `JoinHandle`
- [ ] `tokio::join!`
- [ ] `tokio::select!`
- [ ] `tokio::time::sleep`
- [ ] `timeout`
- [ ] `spawn_blocking`

## Task 3 — Concurrent Tasks

Buat 10 task:

```text
Task 1
Task 2
...
Task 10
```

Masing-masing:
- [ ] sleep berbeda.
- [ ] menghasilkan result.
- [ ] di-join oleh main.

## Task 4 — Timeout

- [ ] Request simulasi 100ms.
- [ ] Timeout 50ms.
- [ ] Timeout 500ms.
- [ ] Handle keduanya menggunakan `Result`.

## Task 5 — Cancellation

- [ ] Buat worker background.
- [ ] Tambahkan signal shutdown.
- [ ] Hentikan worker secara graceful.

## Mini Project Fase 12 — Async Job Processor

```text
API/Main
   ↓
Job Channel
   ↓
Async Worker
   ↓
Processing
   ↓
Result Channel
```

### Lulus fase

- [ ] Bisa menjelaskan `Future`.
- [ ] Bisa menjelaskan perbedaan OS thread vs Tokio task.
- [ ] Bisa menggunakan `spawn`.
- [ ] Bisa menggunakan `select!`.
- [ ] Bisa menjelaskan kapan harus menggunakan `spawn_blocking`.

---

# FASE 13 — Testing & Quality Assurance

## Target

Membuat Rust code yang dapat dipercaya dan mudah direfactor.

Materi:
- unit test
- integration test
- doc test
- async test
- assertions
- test organization
- linting
- formatting
- benchmarking

## Task

- [ ] Buat unit test.
- [ ] Buat integration test di `tests/`.
- [ ] Buat documentation test.
- [ ] Buat async test dengan `#[tokio::test]`.
- [ ] `assert_eq!`
- [ ] `assert_ne!`
- [ ] `assert!`
- [ ] Test success case.
- [ ] Test error case.
- [ ] Test edge case.

## Quality commands

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Mini Project Fase 13 — Test Suite Task Manager

Target:
- [ ] CRUD tests.
- [ ] Error tests.
- [ ] Concurrency tests.
- [ ] Async tests.
- [ ] Integration tests.

### Lulus fase

Semua command berikut lolos:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

---

# FASE 14 — Advanced Rust

## Target

Mengenal batas kemampuan safe Rust dan metaprogramming.

Materi:
- unsafe
- raw pointers
- unsafe functions
- unsafe traits
- static mut / modern alternatives
- union
- FFI
- declarative macros
- procedural macros

## Task 1 — Unsafe

- [ ] Buat raw pointer `*const T`.
- [ ] Buat raw pointer `*mut T`.
- [ ] Dereference dalam `unsafe`.
- [ ] Jelaskan invariant yang harus dijaga programmer.

## Task 2 — FFI

- [ ] Pelajari konsep ABI.
- [ ] Pelajari `extern "C"`.
- [ ] Pelajari pemanggilan function C secara konseptual.
- [ ] Identifikasi boundary safe/unsafe.

## Task 3 — Macros

- [ ] `macro_rules!`
- [ ] Pattern matching macro.
- [ ] Repetition `$(...)*`.
- [ ] Expression fragment.
- [ ] Item fragment.
- [ ] Buat macro sederhana.

## Mini Project Fase 14 — Utility Macro

Buat macro sendiri, misalnya:

```rust
log_value!(name, value);
```

atau:

```rust
create_vec!(1, 2, 3, 4, 5);
```

### Lulus fase

- [ ] Bisa menjelaskan mengapa `unsafe` ada.
- [ ] Bisa membedakan safe abstraction dan unsafe implementation.
- [ ] Bisa membuat `macro_rules!` sederhana.
- [ ] Bisa menjelaskan apa itu FFI.

> Catatan: Fase ini tidak berarti semua project harus menggunakan `unsafe`. Tujuannya adalah memahami boundary dan trade-off.

---

# FASE 15 — Production Backend + Capstone

## Target

Menggabungkan seluruh materi menjadi REST API modern dengan Rust Edition 2024.

Stack:

```text
Rust 2024
   ↓
Axum
   ↓
Tokio
   ↓
Serde
   ↓
Repository abstraction
   ↓
Thread-safe state
   ↓
Error mapping
   ↓
Tracing
   ↓
Graceful shutdown
   ↓
Docker
```

Roadmap sumber menargetkan Axum + Tokio + Serde + repository pattern + graceful shutdown sebagai capstone backend. 

---

## FASE 15.1 — Project Initialization

```bash
cargo new task-service-api
cd task-service-api
```

Pastikan:

```toml
[package]
name = "task_service_api"
version = "0.1.0"
edition = "2024"
```

### Tambahkan dependency

```bash
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
cargo add serde_json
cargo add tracing
cargo add tracing-subscriber --features env-filter
```

- [ ] `cargo check`
- [ ] `cargo run`

---

# FASE 15.2 — Domain Model

Buat:

```text
Task
├── id
├── title
├── status
└── priority
```

Enum:

```text
TaskStatus
├── Todo
├── InProgress
└── Done

Priority
├── Low
├── Medium
└── High
```

Task:
- [ ] `Serialize`
- [ ] `Deserialize`
- [ ] `Debug`
- [ ] `Clone`
- [ ] `PartialEq`
- [ ] DTO create
- [ ] DTO update

---

# FASE 15.3 — Repository Layer

Buat:

```rust
trait TaskRepository {
    fn create(...);
    fn find_all(...);
    fn find_by_id(...);
    fn update_status(...);
    fn delete(...);
}
```

Implementasi:

```text
TaskRepository
      │
      ▼
InMemoryTaskRepo
      │
      ▼
HashMap<u64, Task>
```

Tambahkan:

```text
Arc
 +
RwLock
```

Task:
- [ ] Create.
- [ ] Read all.
- [ ] Read one.
- [ ] Update.
- [ ] Delete.
- [ ] Not found.
- [ ] Invalid input.

---

# FASE 15.4 — Error Architecture

Buat:

```text
ApiError
├── NotFound
└── BadRequest
```

Implement:

```rust
IntoResponse
```

Task:
- [ ] Map domain error → HTTP status.
- [ ] 400 Bad Request.
- [ ] 404 Not Found.
- [ ] JSON error response.
- [ ] Konsistenkan format response.

---

# FASE 15.5 — Axum Router

Endpoint minimum:

```text
GET    /api/tasks
GET    /api/tasks/{id}
POST   /api/tasks
PATCH  /api/tasks/{id}/status
DELETE /api/tasks/{id}
```

Task:

- [ ] Buat Router.
- [ ] Buat State.
- [ ] Buat handler GET all.
- [ ] Buat handler GET by id.
- [ ] Buat handler POST.
- [ ] Buat handler PATCH.
- [ ] Buat handler DELETE.
- [ ] Test dengan curl/Postman/HTTP client.

---

# FASE 15.6 — Tokio Runtime

Task:

- [ ] `#[tokio::main]`.
- [ ] `TcpListener`.
- [ ] `async/await`.
- [ ] Shared state.
- [ ] Async handler.
- [ ] Graceful shutdown.

---

# FASE 15.7 — Logging & Observability

Gunakan:

```text
tracing
tracing-subscriber
```

Task:
- [ ] Log startup.
- [ ] Log request penting.
- [ ] Log error.
- [ ] Gunakan level `info`.
- [ ] Gunakan level `error`.
- [ ] Gunakan environment filter.

---

# FASE 15.8 — Testing Backend

Minimal:

```text
Unit tests
Integration tests
Handler/API tests
Error tests
```

Checklist:

- [ ] Create task.
- [ ] Get all tasks.
- [ ] Get existing task.
- [ ] Get missing task.
- [ ] Update task.
- [ ] Delete task.
- [ ] Invalid request.
- [ ] Concurrent access.

---

# FASE 15.9 — Graceful Shutdown

Implement:

```text
Ctrl+C
   ↓
shutdown_signal()
   ↓
stop accepting new work
   ↓
finish active work
   ↓
shutdown
```

Checklist:

- [ ] Ctrl+C terdeteksi.
- [ ] Server tidak langsung crash.
- [ ] Log shutdown.
- [ ] Active request diberi kesempatan selesai.

---

# FASE 15.10 — Docker

Buat multi-stage build:

```text
Builder image
    ↓
cargo build --release
    ↓
Runtime image
    ↓
Rust binary
```

Checklist:
- [ ] Dockerfile.
- [ ] Build image.
- [ ] Run container.
- [ ] Expose port 3000.
- [ ] Test endpoint dari host.
- [ ] Pastikan application bisa shutdown.

> Jangan mengejar ukuran image tertentu sebelum build benar-benar reproducible. Ukuran akhir dapat berubah tergantung toolchain, target, linker, dan binary.

---

# FASE 15.11 — Refactor Production Structure

Refactor dari satu file menjadi:

```text
src/
├── main.rs
├── lib.rs
├── domain/
│   ├── mod.rs
│   └── task.rs
├── repository/
│   ├── mod.rs
│   └── in_memory.rs
├── handlers/
│   ├── mod.rs
│   └── tasks.rs
├── errors.rs
└── state.rs
```

Checklist:
- [ ] Domain terpisah.
- [ ] Repository terpisah.
- [ ] Handler terpisah.
- [ ] Error terpisah.
- [ ] State terpisah.
- [ ] `main.rs` hanya bootstrap application.

---

# CAPSTONE FINAL

## Nama

**Task Service API — Rust Edition 2024**

## Requirement minimum

### Language

- [ ] Ownership
- [ ] Borrowing
- [ ] Lifetimes
- [ ] Struct
- [ ] Enum
- [ ] Pattern matching
- [ ] Generics
- [ ] Traits
- [ ] Collections

### Architecture

- [ ] Modules
- [ ] Repository abstraction
- [ ] DTO
- [ ] Error architecture
- [ ] Shared application state

### Async

- [ ] Tokio runtime
- [ ] Async handler
- [ ] Spawn/task concept
- [ ] Graceful shutdown

### Backend

- [ ] Axum
- [ ] REST endpoint
- [ ] JSON
- [ ] Serde
- [ ] HTTP error mapping

### Quality

- [ ] Unit tests
- [ ] Integration tests
- [ ] Async tests
- [ ] `cargo fmt`
- [ ] `cargo clippy`
- [ ] `cargo test`
- [ ] Release build

### Deployment

- [ ] Docker image
- [ ] Container running
- [ ] API accessible
- [ ] Graceful shutdown tested

---

# FINAL SELF-ASSESSMENT

Nilai dirimu dari 0–2 untuk setiap kemampuan:

```text
0 = belum paham
1 = bisa mengikuti contoh
2 = bisa membuat sendiri
```

## Core Rust

- [ ] Variables
- [ ] Types
- [ ] Functions
- [ ] Expressions
- [ ] Control flow

## Ownership

- [ ] Ownership
- [ ] Move
- [ ] Copy
- [ ] Clone
- [ ] Borrowing
- [ ] Mutable borrowing
- [ ] NLL
- [ ] Slices
- [ ] UTF-8

## Type System

- [ ] Struct
- [ ] Enum
- [ ] Pattern matching
- [ ] Generics
- [ ] Traits
- [ ] Associated types
- [ ] `dyn Trait`
- [ ] Lifetimes

## Memory

- [ ] Box
- [ ] Rc
- [ ] RefCell
- [ ] Arc
- [ ] Mutex
- [ ] RwLock
- [ ] Atomics

## Concurrency

- [ ] Threads
- [ ] Channels
- [ ] Send
- [ ] Sync

## Async

- [ ] Future
- [ ] async/await
- [ ] Tokio
- [ ] spawn
- [ ] JoinHandle
- [ ] select!
- [ ] timeout
- [ ] cancellation
- [ ] spawn_blocking

## Advanced

- [ ] Unsafe
- [ ] Raw pointers
- [ ] FFI
- [ ] macro_rules!

## Backend

- [ ] Axum
- [ ] Serde
- [ ] REST API
- [ ] Error mapping
- [ ] Tracing
- [ ] Graceful shutdown
- [ ] Docker

---

# RULE SEBELUM NAIK FASE

Jangan lanjut jika kamu masih tidak bisa menjelaskan minimal:

```text
Apa konsepnya?
Mengapa Rust membutuhkannya?
Bagaimana compiler/runtime memperlakukannya?
Apa error yang biasa terjadi?
Kapan konsep tersebut digunakan?
```

Untuk konsep sulit, gunakan pola:

```text
Konsep
  ↓
Contoh sederhana
  ↓
Contoh salah
  ↓
Compiler error
  ↓
Perbaikan
  ↓
Mini-project
  ↓
Refactor
  ↓
Test
```

---

# PROJECT PROGRESSION

```text
Fase 1
Calculator CLI
    ↓
Fase 2
Text Analyzer
    ↓
Fase 3
Task Domain Model
    ↓
Fase 4
Modular Task App
    ↓
Fase 5
Multi-crate Workspace
    ↓
Fase 6
CLI Task Manager
    ↓
Fase 7
Repository Abstraction
    ↓
Fase 8
Lifetime-driven Components
    ↓
Fase 9
Statistics Processor
    ↓
Fase 10
Shared Counter
    ↓
Fase 11
Worker Pool
    ↓
Fase 12
Async Job Processor
    ↓
Fase 13
Full Test Suite
    ↓
Fase 14
Macro / Unsafe Lab
    ↓
Fase 15
Production Task Service API
```

---

# Target Akhir

Setelah seluruh checklist selesai, targetnya bukan sekadar:

> "Saya bisa menulis syntax Rust."

Tetapi:

> "Saya dapat membaca error borrow checker, memilih ownership model, merancang tipe dengan trait, mengorganisasi crate/module, menangani error secara eksplisit, membangun concurrent/async code, menulis test, serta membuat REST API Rust yang terstruktur dan dapat diuji."

