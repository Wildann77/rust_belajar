# Roadmap Komprehensif Belajar Rust: Dari Nol Hingga Production Backend (15 Fase)
> **Edisi Modern**: Rust Edition 2024 (Baseline Standar Rust 1.90+)
> **Integrasi Materi**: Seluruh Transkrip Dasar 3 Jam + Arsitektur Sistem, Advanced Type System, Tokio Async Runtime, & Axum Production Web API.

---

## Peta Jalan 15 Fase (Curriculum Blueprint)

```text
[BAGIAN I: CORE LANGUAGE FUNDAMENTALS]
├── FASE 1: Tooling Cargo & Sintaks Inti (Variables, Types, Functions, Expressions, Control Flow)
├── FASE 2: Ownership, Borrowing, Slices, & UTF-8 Memory Internals (NLL Model)
├── FASE 3: Rust Type System (Structs, Enums, & Advanced Pattern Matching)
├── FASE 4: Module System & Code Organization (Packages, Crates, Modules, Visibility)
└── FASE 5: Cargo Tingkat Lanjut & Workspace Management (Features, Workspaces, Profiles)

[BAGIAN II: ERROR HANDLING, COLLECTIONS, & ABSTRAKSI TIPE]
├── FASE 6: Robust Error Handling (Option, Result, ?, Error Combinators, Custom Error)
├── FASE 7: Generics, Traits, & Advanced Trait System (Associated Types, dyn Trait, Orphan Rule)
├── FASE 8: Lifetimes Mendalam ('a, Structs, Impls, Elision Rules, 'static)
└── FASE 9: Functional Rust (Closures Fn/FnMut/FnOnce, Lazy Iterators, Adapters)

[BAGIAN III: MEMORY, CONCURRENCY, & ASYNCHRONOUS SYSTEMS]
├── FASE 10: Smart Pointers & Interior Mutability (Box, Rc, RefCell, Arc, Mutex, RwLock, Atomics)
├── FASE 11: Concurrency (Threads OS, mpsc Channels, Send & Sync Guarantees, Race-Safety)
├── FASE 12: Modern Async Rust & Tokio Runtime (Future, Pin, Waker, Task Concurrency, select!)
└── FASE 13: Testing & Quality Assurance (Unit, Integration, Documentation, Async Tests)

[BAGIAN IV: ADVANCED RUST & PRODUCTION WEB BACKEND]
├── FASE 14: Advanced Rust (Unsafe Memory Invariants, Raw Pointers, Macros System)
└── FASE 15: Production Backend Ecosystem & Capstone Project (Axum REST API, Serde, Docker, Graceful Shutdown)
```

---

# BAGIAN I: CORE LANGUAGE FUNDAMENTALS

## FASE 1: Tooling Cargo & Sintaks Inti

### 1.1 Toolchain Resmi & Verifikasi (Context7 MCP Validated)
Instalasi compiler `rustc` dan manager `cargo` melalui installer resmi **rustup**:
```bash
# Download dan instal rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Muat environment Cargo ke terminal aktif
source "$HOME/.cargo/env"

# Cek versi toolchain
rustc --version
cargo --version
rustup --version
```

### 1.2 Cargo Command Workflow
| Perintah | Deskripsi Teknis |
|---|---|
| `cargo new <nama> --bin` | Menginisialisasi package binary baru dengan `src/main.rs` |
| `cargo new <nama> --lib` | Menginisialisasi library crate baru dengan `src/lib.rs` |
| `cargo check` | Memeriksa validitas tipe data dan borrowing tanpa kompilasi binary (sangat cepat) |
| `cargo build` | Mengompilasi kode ke mode debug di `target/debug/` |
| `cargo build --release` | Mengompilasi binary teroptimasi penuh (LTO, vectorization) di `target/release/` |
| `cargo run` | Kompilasi bertahap lalu langsung mengeksekusi binary |
| `cargo test` | Menjalankan seluruh harness test (unit, integration, doc-tests) |
| `cargo clippy` | Linter resmi Rust untuk audit idiomatis dan performa |
| `cargo fmt` | Memformat kode sesuai standar resmi RFC Rust |

---

### 1.3 Variable Bindings, Mutability, & Shadowing
Di Rust, variabel diikat (*bound*) ke nilai.
- **Koreksi Konseptual Presisi**: `let` immutable secara default **bukan** otomatis berarti "jaminan thread safety". Immutability mencegah mutasi tidak disengaja dalam satu scope. Jaminan thread safety di Rust ditegakkan secara menyeluruh melalui kombinasi model ownership, borrowing, tipe data, serta trait penanda `Send` dan `Sync`.

```rust
fn main() {
    // 1. Immutable binding: tidak dapat diubah setelah diinisialisasi
    let x = 5;
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // 2. Mutable binding: izin eksplisit untuk mutasi nilai pada lokasi memori yang sama
    let mut y = 10;
    println!("y awal: {}", y);
    y = 20; // Valid
    println!("y baru: {}", y);

    // 3. Shadowing: mendeklarasikan ulang variabel dengan nama yang sama menggunakan keyword `let`
    // Variabel lama ditimpa (shadowed). Tipe data dan mutability boleh berganti!
    let spaces = "   ";        // Tipe: &str
    let spaces = spaces.len(); // Tipe: usize (shadowed)
    println!("Jumlah spasi: {}", spaces);

    // 4. Constants: selalu dievaluasi saat compile-time, wajib tipe data eksplisit, tidak bisa di-shadow
    const MAX_BUFFER_SIZE: usize = 1024 * 64;
    println!("Buffer size: {}", MAX_BUFFER_SIZE);
}
```

#### Komparasi: Shadowing vs `mut`

| Aspek | Shadowing (`let x; let x;`) | Mutable (`let mut x; x = ...;`) |
|---|---|---|
| **Mekanisme** | Buat binding variabel **baru** | Pakai variabel & lokasi memori **sama** |
| **Ganti Tipe Data** | **Boleh** (contoh: `&str` $\to$ `usize`) | **Dilarang keras** (tipe terkunci) |
| **Status Immutability** | Tetap immutable setelah re-bind | Tetap mutable sepanjang scope |

> [!NOTE]
> **Tujuan & Manfaat Shadowing**:
> 1. **Transformasi tanpa polusi nama**: Hindari nama redundan seperti `data_str` lalu `data_len`. Cukup gunakan kembali nama variabel yang sama.
> 2. **Menjaga immutability**: Variabel hasil transformasi tetap immutable, aman dari modifikasi tak disengaja.
> 3. **Proteksi tipe data**: Mengubah tipe via `mut` memicu compiler error `E0308 (mismatched types)`:
>    ```rust
>    let mut name = "Rust";
>    name = name.len(); // ERROR [E0308]: expected `&str`, found `usize`
>    ```

---

### 1.4 Sistem Tipe Data (Scalar vs Compound)
Rust adalah bahasa bertipe statis (*statically typed*) dengan *type inference* yang kuat.

#### Scalar Types (Alokasi Stack, Ukuran Pasti)
- **Integer Bertanda (*Signed*)**: `i8`, `i16`, `i32` (default), `i64`, `i128`, `isize` (pointer size arsitektur CPU target).
- **Integer Tak Bertanda (*Unsigned*)**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (digunakan untuk indexing memory dan koleksi).
- **Floating-Point**: `f32`, `f64` (default, presisi ganda IEEE-754).
- **Boolean**: `bool` (`true` atau `false`, ukuran tepat 1 byte).
- **Character**: `char` (ukuran tepat 4 byte, merepresentasikan Unicode Scalar Value dari `U+0000` s/d `U+D7FF` dan `U+E000` s/d `U+10FFFF`). Literal menggunakan tanda petik tunggal (`'🦀'`).

#### Compound Types
- **Tuples**: Menyatukan beberapa nilai dengan tipe heterogen, panjang tetap di stack.
  ```rust
  let point: (i32, f64, char) = (10, 3.14, 'P');
  let (x, y, label) = point; // Destructuring pattern
  let direct_access = point.1; // Mengakses index 1 (3.14)
  ```
- **Arrays**: Kumpulan elemen homogen dengan panjang pasti yang dialokasikan di Stack.
  ```rust
  let buffer: [u8; 4] = [0, 1, 2, 3];
  let zeros = [0u8; 1024]; // Mengisi 1024 byte dengan angka 0
  let first = buffer[0];
  ```

---

### 1.5 Functions (Anatomi, Parameter, & Return Values)
Fungsi dideklarasikan menggunakan kata kunci `fn`. Konvensi penamaan fungsi di Rust adalah **`snake_case`**.

#### Karakteristik & Aturan Inti:
1. **Anotasi Tipe Parameter Wajib**: Rust sengaja tidak melakukan *type inference* pada parameter fungsi. Setiap parameter wajib memiliki tipe data eksplisit demi kejelasan kontrak API dan kompilasi modular.
2. **Return Type (`-> Type`)**: Ditulis setelah parameter menggunakan panah tipis (`->`). Jika fungsi tidak mengembalikan nilai, tipe kembaliannya secara implisit adalah unit type `()`.
3. **Tail Expression vs Keyword `return`**:
   - **Tail Expression (Idiomatik Rust)**: Baris evaluasi terakhir tanpa titik koma (`;`) otomatis menjadi nilai kembalian.
   - **Keyword `return`**: Digunakan untuk *early exit* (keluar lebih awal berdasarkan kondisi sebelum akhir fungsi) atau return eksplisit.

```rust
// 1. Parameter dan return type dengan tail expression (tanpa titik koma)
fn add(a: i32, b: i32) -> i32 {
    a + b // Expression: hasil langsung di-return
}

// 2. Fungsi dengan return boolean
fn is_even(n: i32) -> bool {
    n % 2 == 0 // Expression menghasilkan bool
}

// 3. Eksperimen: Early return vs Tail expression
fn multiply(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0; // Early exit menggunakan keyword `return` eksplisit
    }
    a * b // Tail expression untuk alur normal
}
```

> [!NOTE]
> **Mengapa `println!` Menggunakan Tanda Seru (`!`)? (Macro Invocation)**
> - **Pembeda Macro vs Fungsi Biasa**: Tanda `!` menandakan bahwa kita sedang memanggil **Macro**, bukan fungsi biasa (`fn`).
> - **Arity Fleksibel (Variadic)**: Fungsi biasa di Rust memiliki parameter dengan jumlah dan tipe kaku. Macro seperti `println!` atau `format!` dapat menerima format string beserta jumlah argumen bebas sesuai kebutuhan.
> - **Ekspansi Compile-Time**: Kode macro diekspansi (*metaprogramming*) menjadi kode Rust murni saat compile-time. Compiler memverifikasi validitas placeholder format `{}` sebelum program dijalankan tanpa penalti performa runtime.
> - *(Detail pembuatan custom macro dibahas pada [Fase 14: Macro System](#142-macro-system))*

---

### 1.6 Statement vs Expression (Koreksi Presisi)
- **Expression**: Bagian kode yang dievaluasi dan **menghasilkan suatu nilai**.
- **Statement**: Instruksi yang melakukan suatu aksi atau deklarasi tanpa menghasilkan nilai (mengembalikan tipe *unit* `()`).
- Tanda titik koma (`;`) mengubah expression menjadi *expression statement*, membuang nilai evaluasinya dan mengembalikan `()`.

```rust
fn statement_expression_demo() {
    // let x = 5; adalah statement deklarasi (tidak menghasilkan nilai)
    let x = 5;

    // Blok kode {} adalah expression yang menghasilkan nilai 6
    let y = {
        let internal = 1;
        internal + x // Expression menghasilkan 6
    };
    println!("y: {}", y);
}
```

---

### 1.7 Control Flow Lengkap
Rust menyediakan struktur kendali alur yang ekspresif, aman, dan berorientasi expression.

#### 1. Percabangan: `if`, `else`, & `else if`
- **Kondisi Eksplisit `bool` (No Truthy/Falsy)**:
  - Di bahasa seperti JavaScript, Python, atau C, nilai angka selain 0 (`5`) atau string non-kosong otomatis dianggap "truthy" (`true`).
  - Rust **menolak keras konversi implisit ini**. Kondisi pada `if` **wajib** menghasilkan tipe data `bool` murni (`true` atau `false`).
  ```rust
  let count = 5;
  // if count { ... }      // COMPILE ERROR [E0308]: expected `bool`, found `integer`
  if count != 0 { ... }   // BENAR: evaluasi eksplisit menjadi bool
  ```
- **`if` sebagai Expression (Pengganti Ternary `? :`)**:
  - Rust sengaja **tidak memiliki operator ternary (`? :`)** seperti bahasa C/JS.
  - Sebagai gantinya, struktur `if ... else` di Rust adalah sebuah **expression** yang dapat langsung mengembalikan nilai ke dalam binding variabel.
  ```rust
  let is_admin = true;
  // Nilai evaluasi blok (tanpa semicolon) langsung di-assign ke variabel `role`
  let role = if is_admin { "Administrator" } else { "User Biasa" };
  ```
- **Invarian Homogen (Keseragaman Tipe Data Antar Cabang)**:
  - Rust adalah bahasa bertipe statis (*statically typed*); compiler harus menentukan tepat satu tipe pasti untuk setiap variabel saat compile-time.
  - Jika `if` digunakan sebagai expression, setiap cabang (`if`, `else if`, `else`) **wajib mengembalikan tipe data yang identik**.
  ```rust
  let lulus = true;
  // SALAH: cabang IF return integer (i32), cabang ELSE return string slice (&str)
  // let hasil = if lulus { 100 } else { "Gagal" }; // COMPILE ERROR [E0308]

  // BENAR: semua cabang mengembalikan tipe data seragam (&str)
  let hasil = if lulus { "Lulus (100)" } else { "Gagal (0)" };
  ```

#### 2. Perulangan: `loop`, `break`, & `continue`
Rust menyediakan `loop` murni tanpa syarat henti bawaan, di mana kendali perulangan diatur sepenuhnya menggunakan `break`, `continue`, dan label.

- **Unconditional `loop` vs `while true`**:
  - `loop` adalah perulangan tak terbatas asli dari Rust (pengganti `while (true)` di bahasa lain).
  - **Optimasi Compiler**: Compiler Rust mengetahui secara pasti bahwa blok `loop` berjalan terus menerus hingga menemui `break`. Hal ini memungkinkan compiler melakukan analisis *definite assignment* (memastikan variabel pasti terisi nilai) tanpa peringatan uninitialized variable.
  ```rust
  loop {
      println!("Berjalan terus menerus...");
      break; // Wajib ada penghenti agar tidak infinite loop
  }
  ```

- **Alur `continue` dan `break`**:
  - `continue`: Melewatkan (*skip*) sisa instruksi pada iterasi saat ini, lalu langsung melompat ke awal iterasi berikutnya.
  - `break`: Menghentikan eksekusi perulangan saat itu juga dan langsung keluar dari blok loop.
  ```rust
  let mut step = 0;
  loop {
      step += 1;
      if step % 2 != 0 {
          continue; // Lewati angka ganjil, lanjut ke iterasi berikutnya
      }
      println!("Angka genap: {step}");
      if step >= 6 {
          break; // Berhenti total saat mencapai 6
      }
  }
  ```

- **`break` dengan Return Value (Loop sebagai Expression)**:
  - Sama seperti blok `{}` dan `if`, perulangan `loop` di Rust adalah sebuah **expression** yang dapat mengembalikan nilai.
  - Nilai dioperasikan setelah keyword `break` menggunakan sintaks: `break <nilai>;`. Nilai tersebut langsung ditampung oleh variabel di luar loop.
  - **Use Case Nyata**: Polling data hingga siap, retry koneksi database/API sampai sukses, atau pencarian elemen dalam array.
  ```rust
  let mut counter = 0;
  let hasil_komputasi = loop {
      counter += 1;
      if counter == 5 {
          break counter * 10; // Mengembalikan nilai 50 keluar ke variabel
      }
  };
  println!("Hasil: {hasil_komputasi}"); // Output: 50
  ```

- **Loop Labels (`'label`) untuk Disambiguasi Nested Loops**:
  - Masalah umum perulangan bersarang (*nested loops*) di bahasa lain adalah `break` hanya keluar dari satu level loop terdalam, sehingga sering membutuhkan variabel flag bantuan (`let mut stop = false;`).
  - Di Rust, setiap loop dapat diberi label pengenal diawali tanda petik tunggal (`'nama_label:`).
  - Dengan memanggil `break 'nama_label;` atau `continue 'nama_label;`, kita dapat langsung mengontrol loop luar dari dalam perulangan terdalam secara presisi.
  ```rust
  let mut outer = 0;
  'outer_loop: loop {
      let mut inner = 0;
      loop {
          if inner == 2 {
              break; // Hanya keluar dari inner loop
          }
          if outer == 1 {
              break 'outer_loop; // Langsung keluar menembus 'outer_loop!
          }
          inner += 1;
      }
      outer += 1;
  }
  ```

#### 3. `while` Loop (Perulangan Bersyarat)
- Berjalan berulang kali selama kondisi evaluasi menghasilkan `true`.
- Evaluasi predikat boolean dilakukan di awal setiap siklus iterasi. Jika kondisi sudah bernilai `false` sejak awal, blok di dalamnya dilewati sepenuhnya.
- **Kapan pakai `while` vs `loop`?**: Gunakan `while` saat ada kondisi terminasi predikat yang jelas. Gunakan `loop` jika butuh perulangan tak terbatas atau mengembalikan nilai via `break <nilai>`.
```rust
let mut countdown = 3;
while countdown > 0 {
    println!("Hitung mundur: {countdown}");
    countdown -= 1;
}
```

#### 4. `for` Loop, Range, & `.rev()` (Pendekatan Idiomatik Rust)
Rust sengaja **menghilangkan gaya perulangan C tradisional** (`for (int i = 0; i < n; i++)`). Seluruh `for` loop di Rust menggunakan abstraksi **Iterator**.

- **Keunggulan Teknis (Zero-Cost Safety)**:
  - **Eliminasi Bug Off-by-One**: Tidak ada risiko salah menulis operator batas (`<` vs `<=`).
  - **Bebas Bounds Checking Overhead**: Iterator langsung menavigasi elemen internal tanpa compiler harus mengecek batas array (*index bound checking*) di setiap putaran siklus runtime.

- **Range Eksklusif (`start..end`)**:
  - Mengiterasi dari `start` hingga `end - 1` (angka ujung `end` **tidak diikutsertakan**).
  - Contoh: `0..3` menghasilkan urutan: `0, 1, 2`. Sangat cocok untuk mengakses index array berukuran 3.
  ```rust
  for i in 0..3 {
      println!("Index eksklusif: {i}"); // 0, 1, 2
  }
  ```

- **Range Inklusif (`start..=end`)**:
  - Menggunakan tanda sama dengan (`=`) setelah titik ganda.
  - Mengiterasi dari `start` sampai tepat `end` (angka ujung `end` **diikutsertakan**).
  - Contoh: `1..=3` menghasilkan urutan: `1, 2, 3`.
  ```rust
  for num in 1..=3 {
      println!("Angka inklusif: {num}"); // 1, 2, 3
  }
  ```

- **Pembalikan Urutan Iterator via `.rev()`**:
  - Method `.rev()` membalik urutan pembacaan iterator dari belakang ke depan.
  - **Aturan Sintaks Tanda Kurung**: Wajib membungkus range dalam tanda kurung `(1..=3).rev()` karena aturan prioritas operator (*operator precedence*) agar method `.rev()` dipanggil pada objek Range.
  - **Efisiensi Memori**: Range mengimplementasikan trait `DoubleEndedIterator`. Pembalikan urutan terjadi langsung pada level iterator pointer tanpa alokasi array baru di memori.
  ```rust
  for num in (1..=3).rev() {
      println!("Mundur: {num}"); // 3, 2, 1
  }
  ```

- **Iterasi Langsung pada Koleksi (Array / Slice)**:
  ```rust
  let fruits = ["Apel", "Pisang", "Jeruk"];
  for fruit in fruits {
      println!("Buah: {fruit}");
  }
  ```

---

#### Kode Demonstrasi Lengkap Control Flow (Sinkron dengan `task_5.rs`)

```rust
fn control_flow_demo() {
    // 1. if statement
    let score = 85;
    if score >= 80 {
        println!("1. if: Score lulus kualifikasi ({score})");
    }

    // 2. if/else statement
    let is_admin = false;
    if is_admin {
        println!("2. if/else: Akses admin diberikan");
    } else {
        println!("2. if/else: Akses biasa (bukan admin)");
    }

    // 3. if/else if/else sebagai expression (semua cabang wajib mengembalikan tipe data homogen)
    let marks = 75;
    let grade = if marks >= 90 {
        "A"
    } else if marks >= 75 {
        "B"
    } else {
        "C"
    };
    println!("3. if/else if/else expression: marks={marks} -> grade={grade}");

    // 4. loop, continue, dan break
    let mut step = 0;
    println!("4. loop, continue, break:");
    loop {
        step += 1;
        if step % 2 != 0 {
            continue; // Lewati angka ganjil, lompat ke iterasi berikutnya
        }
        println!("   step genap: {step}");
        if step >= 6 {
            break; // Keluar loop saat mencapai 6
        }
    }

    // 5. loop dengan return value (expression)
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // Mengembalikan nilai 50 keluar dari loop
        }
    };
    println!("5. loop dengan return value: {loop_result}");

    // 6. Loop label ('label) untuk disambiguasi nested loops
    let mut outer_count = 0;
    'outer_loop: loop {
        let mut inner_count = 0;
        loop {
            inner_count += 1;
            if inner_count == 2 {
                break; // Keluar dari inner loop saja
            }
            if outer_count == 1 {
                break 'outer_loop; // Keluar langsung menembus 'outer_loop!
            }
        }
        outer_count += 1;
    }
    println!("6. Loop label: berhasil break 'outer_loop saat outer_count={outer_count}");

    // 7. while loop (kondisional)
    let mut countdown = 3;
    println!("7. while loop:");
    while countdown > 0 {
        println!("   hitung mundur: {countdown}");
        countdown -= 1;
    }

    // 8. for loop dengan range eksklusif
    println!("8. for loop & range (0..3):");
    for i in 0..3 {
        println!("   index: {i}"); // 0, 1, 2
    }

    // 9. for loop dengan range inklusif dan .rev()
    println!("9. for loop & range inklusif .rev() (1..=3):");
    for num in (1..=3).rev() {
        println!("   mundur: {num}"); // 3, 2, 1
    }
}
```

---

### 1.8 Mini Project Fase 1: Calculator CLI

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/mini_project_1.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/mini_project_1.rs):

Mini project kalkulator CLI memadukan konsep **Functions**, **Expressions vs Statements**, **Enum Result Error Handling**, dan **Control Flow (`match`)**.

#### 1. Arsitektur & Modularitas Fungsi
```rust
pub fn add(a: f64, b: f64) -> f64 { a + b }
pub fn subtract(a: f64, b: f64) -> f64 { a - b }
pub fn multiply(a: f64, b: f64) -> f64 { a * b }

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Pembagian dengan nol tidak valid"))
    } else {
        Ok(a / b)
    }
}
```

#### 2. Rangkuman Kelulusan Fase 1
| Konsep Inti | Ringkasan Teknis |
|---|---|
| **Expression vs Statement** | Expression dievaluasi menghasilkan nilai (contoh: `a + b`, blok `match`). Statement adalah aksi/deklarasi tanpa nilai return (menghasilkan unit `()`). Titik koma (`;`) mengubah expression menjadi statement. |
| **Immutable vs Mutable vs Shadowing** | Immutable (`let x`): proteksi nilai default. Mutable (`let mut x`): alokasi memori sama, isi dapat diubah in-place, tipe terkunci. Shadowing (`let x; let x;`): binding variabel baru timpa nama lama, tipe data dan mutability boleh berubah. |

---

## FASE 2: Ownership, Borrowing, Slices, & UTF-8 Memory Internals

### 2.1 Model Memori: 3 Aturan Emas Ownership & RAII
1. **Aturan 1 — Setiap nilai memiliki Owner**: Setiap nilai data di Rust memiliki variabel pemilik yang disebut **owner**.
2. **Aturan 2 — Hanya satu Owner pada satu waktu**: Tidak boleh ada dua variabel yang bersamaan menjadi owner sah atas resource data yang sama.
3. **Aturan 3 — Drop otomatis saat keluar Scope (RAII pattern)**: Saat owner keluar dari scope kurung kurawal (`}`), Rust otomatis memanggil fungsi internal `drop` untuk membebaskan memori heap seketika. Tidak ada Garbage Collector (GC), tidak ada runtime pause, dan tidak ada kebocoran memori (*memory leak*).

```rust
fn scope_demo() {
    {
        let s = String::from("hello"); // Alokasi buffer memori di Heap, pointer di Stack
        // Gunakan s...
    } // s keluar dari scope: Rust memanggil drop(s) otomatis. Memori heap bebas seketika.
}
```

#### Anatomi Memori `String`: Stack vs Heap
Tipe dinamis seperti `String` terbagi menjadi dua bagian:
- **Stack**: Berisi 3 word berukuran tetap (24 byte pada arsitektur 64-bit):
  1. `ptr`: Pointer penunjuk alamat memori buffer di Heap.
  2. `len`: Jumlah byte teks yang saat ini terisi.
  3. `capacity`: Total kapasitas buffer yang dialokasikan di Heap.
- **Heap**: Tempat buffer memori dinamis yang sesungguhnya menyimpan byte teks UTF-8.

```text
       STACK (s1)                     HEAP
┌───────────┬────────┐         ┌───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr       │ 0x1000 ├────────►│ R │ u │ s │ t │ a │ c │ e │ a │ n │
├───────────┼────────┤         └───┴───┴───┴───┴───┴───┴───┴───┴───┘
│ len       │   9    │           0   1   2   3   4   5   6   7   8
├───────────┼────────┤
│ capacity  │   9    │
└───────────┴────────┘
```

---

### 2.2 Move vs Copy vs Clone

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/fase2_task_1.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase2_task_1.rs):

#### 1. Move Semantics (Tipe Heap Tanpa Trait `Copy`)
Ketika variabel tipe heap seperti `String` di-assign ke variabel baru (`let s2 = s1;`):
- Rust **hanya menyalin metadata di stack** (`ptr`, `len`, `capacity`). Data byte di Heap **tidak disalin** (sangat cepat, $O(1)$).
- **Kepemilikan dialihkan (Move)** seutuhnya ke `s2`.
- Variabel lama (`s1`) **seketika dibatalkan (*invalidated*)**.

```text
       STACK (s1: INVALID)            HEAP
┌───────────┬────────┐         ┌───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr       │ 0x1000 ├- - - - ─│ R │ u │ s │ t │ a │ c │ e │ a │ n │
│ len / cap │  ...   │         └───┴───┴───┴───┴───┴───┴───┴───┴───┘
└───────────┴────────┘           ▲
       STACK (s2: OWNER)         │
┌───────────┬────────┐           │
│ ptr       │ 0x1000 ├───────────┘
├───────────┼────────┤
│ len       │   9    │
├───────────┼────────┤
│ capacity  │   9    │
└───────────┴────────┘
```

> [!CAUTION]
> **Mengapa Rust Meng-invalidate `s1`? (Mencegah Double Free Bug)**  
> Jika `s1` dan `s2` sama-sama dibiarkan aktif, saat keduanya keluar dari scope `}`, Rust akan memanggil `drop()` dua kali untuk alamat heap yang sama (`0x1000`). Ini dinamakan **Double Free Vulnerability**—sumber bug keamanan memory corruption di bahasa seperti C/C++. Rust membasmi bug ini di masa kompilasi (*compile-time*).

Jika mencoba mengakses `s1` setelah kepemilikan dipindah:
```rust
let s1 = String::from("Rustacean");
let s2 = s1; // Ownership berpindah ke s2
println!("{s1}"); // COMPILE ERROR!
```
Compiler langsung menolak dengan pesan error presisi:
```text
error[E0382]: borrow of moved value: `s1`
  --> src/fase2_task_1.rs
   |
   | let s1 = String::from("Rustacean");
   |     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
   | let s2 = s1;
   |          -- value moved here
   | println!("{s1}");
   |           ^^^^ value borrowed here after move
```

---

#### 2. Clone Semantics (Deep Copy di Heap)
Jika ingin menduplikasi isi data heap seutuhnya sehingga kedua variabel memiliki data independen yang sama-sama valid, gunakan method `.clone()` dari trait `Clone`:

```rust
let s2 = String::from("Rustacean");
let s3 = s2.clone(); // Alokasi buffer heap baru dibuat!
```

```text
       STACK (s2)                     HEAP (Alamat: 0x1000)
┌───────────┬────────┐         ┌───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr       │ 0x1000 ├────────►│ R │ u │ s │ t │ a │ c │ e │ a │ n │
├───────────┼────────┤         └───┴───┴───┴───┴───┴───┴───┴───┴───┘
│ len: 9    │ cap: 9 │
└───────────┴────────┘
       STACK (s3)                     HEAP (Alamat: 0x2500 - Baru!)
┌───────────┬────────┐         ┌───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr       │ 0x2500 ├────────►│ R │ u │ s │ t │ a │ c │ e │ a │ n │
├───────────┼────────┤         └───┴───┴───┴───┴───┴───┴───┴───┴───┘
│ len: 9    │ cap: 9 │
└───────────┴────────┘
```
- **Karakteristik**: Menghasilkan salinan identik dengan alokasi heap terpisah (`s2.as_ptr() != s3.as_ptr()`).
- **Trade-off**: Operasi mahal ($O(n)$ memori & waktu alokasi). Hindari clone berlebihan jika cukup meminjam (*borrowing*) lewat referensi (`&s2`).

---

#### 3. Copy Semantics (Stack Types)
Tipe data yang ukurannya diketahui pasti saat compile-time disimpan seutuhnya di Stack dan mengimplementasikan trait `Copy`:
- **Tipe yang memiliki Trait `Copy`**:
  - Seluruh tipe integer (`i8`, `i32`, `i64`, `u8`, `usize`, dsb.)
  - Tipe floating point (`f32`, `f64`)
  - Tipe boolean (`bool`)
  - Tipe karakter (`char`)
  - Tipe Tuple yang seluruh anggotanya bertipe `Copy` (contoh: `(i32, f64, char)`)
- **Mekanisme**:
  Saat di-assign (`let y = x;`), Rust melakukan duplikasi bitwise langsung di stack. Karena data berada di stack tanpa alokasi heap, proses ini sangat cepat. Variabel lama **tetap valid** dan tidak di-invalidate!

```rust
let x: i32 = 42;
let y = x; // Bitwise copy murah di Stack
println!("x = {x}, y = {y}"); // Keduanya tetap valid!

let tuple_copy = (100, 3.14, '🦀');
let tuple_clone = tuple_copy; // Tuple primitif otomatis Copy
println!("tuple_copy: {:?}, tuple_clone: {:?}", tuple_copy, tuple_clone);
```

> [!IMPORTANT]
> **Aturan Trait: `Copy` vs `Drop`**  
> Tipe apa pun yang mengimplementasikan trait `Drop` (mengelola resource heap, file handle, socket jaringan) **dilarang keras** oleh compiler untuk mengimplementasikan trait `Copy`.  
> Trait `Copy` hanya diperuntukkan bagi tipe data trivial murni stack yang tidak membutuhkan pembersihan destruktor khusus saat keluar scope.

#### Komparasi Ringkas: Move vs Copy vs Clone

| Aspek | Move | Copy | Clone |
|---|---|---|---|
| **Lokasi Data** | Heap + Stack | Stack saja | Heap + Stack |
| **Mekanisme** | Copy metadata stack, pindah owner | Bitwise duplicate di stack | Alokasi buffer heap baru |
| **Status Variabel Lama** | **Invalidated** (tidak bisa dipakai) | **Tetap valid** | **Tetap valid** |
| **Biaya Kinerja** | Sangat murah ($O(1)$) | Sangat murah ($O(1)$) | Mahal ($O(n)$ heap allocation) |
| **Trait Terlibat** | Default jika bukan `Copy` | `std::marker::Copy` | `std::clone::Clone` |
| **Contoh Tipe** | `String`, `Vec<T>`, `Box<T>` | `i32`, `f64`, `bool`, `char` | Tipe apa pun ber-`#[derive(Clone)]` |

---

### 2.2.1 Ownership & Functions (Task 2)
Aturan perpindahan kepemilikan berlaku identik saat data dioper ke dalam fungsi atau dikembalikan sebagai return value:

1. **Mengoper Value ke Fungsi (Move Semantics)**:
   ```rust
   fn takes_ownership(s: String) {
       println!("{s}");
   } // s keluar dari scope pemanggil dan fungsi, di-drop seketika!
   ```
   Variabel di pemanggil tidak bisa digunakan lagi setelah argumen dioper.

2. **Mengembalikan Value dari Fungsi**:
   ```rust
   fn gives_ownership() -> String {
       let some_string = String::from("hello");
       some_string // ownership ditransfer ke pemanggil fungsi
   }
   ```

3. **Meminjam (*Borrowing*) via Referensi (`&String` / `&str`)**:
   Alih-alih mentransfer kepemilikan bolak-balik (yang merepotkan), gunakan tanda ampersand `&` untuk membaca data tanpa mengambil alih ownership.
   ```rust
   fn calculate_length(s: &String) -> usize {
       s.len()
   } // s hanya referensi; data heap asli milik pemanggil TIDAK di-drop saat fungsi selesai.
   ```

4. **Kapan Clone Diperlukan pada Fungsi?**:
   Gunakan `.clone()` saat:
   - Fungsi tujuan mutlak membutuhkan *ownership* penuh (misal: disimpan ke dalam struktur data lain atau di-move ke thread background).
   - Namun, pemanggil fungsi masih harus tetap menggunakan variabel aslinya secara independen.
   ```rust
   takes_ownership(s_original.clone()); // s_original tetap valid!
   ```

---

### 2.3 Borrowing & Non-Lexical Lifetimes (NLL)

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/fase2_task_3.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase2_task_3.rs):

Alih-alih mentransfer kepemilikan (*move*), data dapat dipinjam (*borrowed*) menggunakan referensi dengan tanda ampersand (`&`). Borrowing memungkinkan kode membaca atau memodifikasi data tanpa mengorbankan kepemilikan.

#### 2 Aturan Emas Borrow Checker (Aliasing XOR Mutability)
Pada sembarang titik waktu dalam program, Anda hanya diperbolehkan memiliki **salah satu** dari dua kondisi ini:
1. **Banyak referensi shared/immutable (`&T`)** aktif bersamaan, **ATAU**
2. **Tepat satu referensi exclusive/mutable (`&mut T`)** aktif.

> [!NOTE]
> **Mengapa Aturan Ini Ada? (Mencegah Data Race di Waktu Kompilasi)**  
> *Data Race* terjadi jika dua pointer mengakses alamat memori yang sama bersamaan, minimal satu melakukan operasi tulis (*write*), dan tidak ada sinkronisasi.  
> Rumus Rust: `Aliasing + Mutability = Data Race`. Dengan membatasi bahwa referensi boleh *Aliasing* (banyak `&T`) ATAU *Mutable* (satu `&mut T`), Data Race dicegah 100% pada saat kompilasi!

---

#### 1. Banyak Shared Reference (`&T`)
Selama data tidak dimutasi, banyak pihak boleh membaca data secara bersamaan tanpa saling mengganggu.

```rust
let data = String::from("Rust Concurrency Safe");
let r1 = &data;
let r2 = &data;
let r3 = &data;
println!("{r1}, {r2}, {r3}"); // Valid! Semua pembaca aman
```

---

#### 2. Satu Exclusive Reference (`&mut T`)
Jika ingin memodifikasi data melalui referensi, gunakan `&mut`. Rust menjamin hanya ada tepat satu akses tulis aktif.

```rust
let mut score = 100;
{
    let score_ref = &mut score;
    *score_ref += 25; // Menggunakan dereference (*) untuk mengubah nilai
    println!("Score baru: {score_ref}");
} // score_ref keluar scope, pinjaman mutable selesai
println!("Score akhir: {score}");
```

---

#### 3. Konflik Borrowing & Deteksi Compiler

##### Konflik 1: `&T` Bersamaan dengan `&mut T`
Jika mencoba membuat mutable borrow saat immutable borrow masih aktif:

```rust
let mut text = String::from("Halo");
let reader = &text;       // Immutable borrow dimulai
let writer = &mut text;   // COMPILE ERROR!
println!("{reader}");     // reader masih aktif di sini
```
Compiler langsung menolak dengan error `E0502`:
```text
error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable
  --> src/fase2_task_3.rs
   |
   | let reader = &text;
   |              ----- immutable borrow occurs here
   | let writer = &mut text;
   |              ^^^^^^^^^ mutable borrow occurs here
   | println!("{reader}");
   |           -------- immutable borrow later used here
```
*Mengapa dilarang?* Jika `writer` mengubah atau mengalokasikan ulang buffer `text`, alamat memori yang sedang dibaca oleh `reader` bisa menjadi rusak (*dangling pointer*).

##### Konflik 2: Dua `&mut T` Bersamaan
```rust
let mut num = 10;
let m1 = &mut num;
let m2 = &mut num; // COMPILE ERROR!
*m1 += 1;
*m2 += 2;
```
Compiler menolak dengan error `E0499`:
```text
error[E0499]: cannot borrow `num` as mutable more than once at a time
  --> src/fase2_task_3.rs
   |
   | let m1 = &mut num;
   |          -------- first mutable borrow occurs here
   | let m2 = &mut num;
   |          ^^^^^^^^ second mutable borrow occurs here
   | *m1 += 1;
   | -------- first borrow later used here
```

---

#### 4 & 5. Kapan Borrow Selesai & Non-Lexical Lifetimes (NLL)

Secara historis (Rust sebelum edisi 2018), masa hidup (*lifetime*) sebuah pinjaman terikat secara kaku ke kurung kurawal scope leksikal (`{ ... }`). 

Sejak hadirnya **Non-Lexical Lifetimes (NLL)**, compiler menganalisis grafik kontrol aliran (*control flow graph*). **Lifetime sebuah borrow selesai tepat pada titik terakhir ia digunakan (*last use*)**, bukan di akhir kurung kurawal scope!

##### Visualisasi Garis Waktu NLL:
```text
let mut msg = String::from("Rust");
let r = &msg;            ──┐
                           │ Rentang Aktif Borrow `&T`
println!("{r}");         ──┘ (PENGGUNAAN TERAKHIR `r` -> Borrow SELESAI di sini!)
                           
let m = &mut msg;        ──┐ (VALID! Compiler tahu `r` sudah mati)
m.push_str(" 2024");       │ Rentang Aktif Borrow `&mut T`
println!("{m}");         ──┘
```

##### Contoh Kode Eksperimen NLL:
```rust
let mut message = String::from("Belajar Rust");

let immut_ref = &message;
println!("immut_ref: {immut_ref}"); // Penggunaan terakhir &T. Borrow r SELESAI di baris ini!

// Compiler mengizinkan mutable borrow berikut karena immut_ref sudah tidak aktif:
let mut_ref = &mut message;
mut_ref.push_str(" Sangat Menyenangkan!");
println!("mut_ref: {mut_ref}");

// PENTING: Jika baris ini diaktifkan kembali di bawah mut_ref:
// println!("{immut_ref}");
// Maka rentang hidup immut_ref diperpanjang melewati mut_ref -> langsung memicu error E0502!
```

#### Komparasi: Shared (`&T`) vs Mutable (`&mut T`) Reference

| Fitur | Shared Reference (`&T`) | Mutable Reference (`&mut T`) |
|---|---|---|
| **Izin Akses** | Read-Only (Hanya Baca) | Read-Write (Baca & Ubah) |
| **Batas Jumlah Simultan** | Tidak terbatas ($N$ referensi) | Tepat **satu** (1 referensi) |
| **Trait `Copy`** | **Mengimplementasikan `Copy`** (bisa diduplikasi murah) | **TIDAK `Copy`** (re-borrow atau move) |
| **Tujuan Keamanan** | Menjamin data tidak berubah saat dibaca | Menjamin tidak ada pembaca/penulis lain saat data dimutasi |

---

### 2.4 Slices: Array Slices `&[T]` & String Slices `&str` (Task 4)

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/fase2_task_4.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase2_task_4.rs):

Slice adalah referensi ke deret elemen berurutan yang bersebelahan dalam sebuah koleksi (*contiguous sequence*), tanpa mengambil kepemilikan (*ownership*) dari data aslinya.

#### Model Memori Slice: "Fat Pointer"
Secara internal di memori Stack, sebuah slice adalah **Fat Pointer** berukuran 2 word (16 byte pada arsitektur 64-bit):
1. **`ptr`**: Pointer ke elemen pertama dari potongan data.
2. **`len`**: Jumlah elemen (untuk array slice) atau jumlah byte (untuk string slice).

```text
Array di Stack: [10, 20, 30, 40, 50]
                 0   1   2   3   4

Fat Pointer Slice: let slice = &numbers[1..4];
┌───────────┬────────┐         
│ ptr       │ 0x1004 ├────────► [20, 30, 40]
├───────────┼────────┤            1   2   3
│ len       │   3    │
└───────────┴────────┘
```

---

#### 1. Array Slice (`&[T]`)
Array slice meminjam sebagian atau seluruh elemen array:

```rust
let numbers = [10, 20, 30, 40, 50];

let middle: &[i32] = &numbers[1..4]; // [20, 30, 40] (panjang = 3)
let first_two      = &numbers[..2];  // [10, 20]
let from_index_two = &numbers[2..];  // [30, 40, 50]
let whole_array    = &numbers[..];   // [10, 20, 30, 40, 50]
```

- **Rentang (*Range Syntax*)**: `[start..end]` bersifat *half-open* (inklusif `start`, eksklusif `end`).
- **Keamanan Batas (*Bounds Checking*)**: Mengakses range di luar ukuran array memicu *runtime panic* aman, mencegah exploit buffer overflow / undefined behavior.

---

#### 2. String Slice (`&str`)
String slice adalah fat pointer yang menunjuk ke deret byte teks UTF-8 valid:
- Dapat menunjuk ke buffer di Heap milik `String`.
- Dapat menunjuk ke area memori read-only (*rodata*) dari program binary (seperti string literal `"..."` bertipe `&'static str`).

```rust
let sentence = String::from("Rust Pemrograman Sistem");

let word1: &str = &sentence[0..4];  // "Rust" (byte 0 sampai 3)
let word2: &str = &sentence[5..16]; // "Pemrograman"
let word3: &str = &sentence[17..];  // "Sistem" sampai akhir
```

##### Tanya Jawab Kritis: Mengapa Hasil Slicing Harus `&str`, Bukan `&String`?

Pertanyaan mendasar pemula: *"Variabel `sentence` bertipe `String`, dan kita meminjamnya dengan tanda `&`. Mengapa tipe hasilnya bukan `&String` melainkan `&str`?"*

###### 1. Analogi: Buku Utuh vs Kaca Pembesar (Bookmark)
- **`String`**: **Buku fisik utuh** yang kamu miliki di rak (memiliki sampul, daftar isi, kapasitas buffer di Heap).
- **`&String`**: **Meminjam seluruh buku fisik itu** milik teman tanpa mengubahnya. `&String` menuntut adanya satu kesatuan objek `String` lengkap di stack & heap.
- **`&str` (Slice)**: **Kaca pembesar / bookmark** untuk membaca Bab 1 (halaman 1–4) langsung di buku aslinya. Kamu **tidak mencetak buku baru** dan **tidak merobek kertas**.

###### 2. Diagram Memori: Mengapa `&String` Mustahil untuk Potongan Substring?
```text
       STACK                                     HEAP
sentence (String utuh)
┌───────────┬────────┐                     ┌───┬───┬───┬───┬───┬───┬───┐
│ ptr       │ 0x1000 ├────────────────────►│ R │ u │ s │ t │   │ P │ … │
├───────────┼────────┤                     └───┴───┴───┴───┴───┴───┴───┘
│ len: 23   │ cap: 23│                       ▲               ▲
└───────────┴────────┘                       │               │
                                             │               │
word1 (&str - Fat Pointer)                   │               │
┌───────────┬────────┐                       │               │
│ ptr       │ 0x1000 ├───────────────────────┘ (mulai index 0)│
├───────────┼────────┤                                       │
│ len       │   4    │ (panjang hanya 4 byte "Rust")         │
└───────────┴────────┘                                       │
                                                             │
word2 (&str - Fat Pointer)                                   │
┌───────────┬────────┐                                       │
│ ptr       │ 0x1005 ├───────────────────────────────────────┘ (mulai index 5)
├───────────┼────────┤
│ len       │   11   │ (panjang hanya 11 byte "Pemrograman")
└───────────┴────────┘
```

###### 3. Tiga Alasan Arsitektur Memori:
1. **Zero-Cost Abstraction (Tidak Ada Objek `String` Baru)**:
   Kata `"Rust"` hanyalah 4 byte di dalam buffer memori `sentence`. Rust tidak membuat objek `String` baru di heap ($O(1)$ waktu & RAM). Karena objek `String` baru tidak pernah dibuat, maka **tidak ada objek yang bisa ditunjuk oleh tipe `&String`**.
2. **`&str` Membawa Metadata Panjangnya Sendiri**:
   Metadata `sentence` mencatat panjang total `23`. Potongan `word1` butuh informasi panjangnya sendiri (`4`). `&str` adalah fat pointer mandiri di stack yang memegang `(ptr_ke_awal_potongan, len_potongan)`.
3. **`str` Adalah DST (*Dynamically Sized Type*)**:
   Deret byte teks di memori tanpa batasan ukuran compile-time bertipe `str`. Tipe DST tidak bisa dialokasikan langsung di stack (`let x: str;` dilarang compiler), sehingga wajib diakses lewat referensi fat pointer `&str`.

###### Komparasi Cepat: `String` vs `&String` vs `&str`

| Aspek | `String` | `&String` | `&str` |
|---|---|---|---|
| **Definisi** | Pemilik data teks heap dinamis | Referensi pinjaman ke **seluruh** objek `String` | Referensi jendela intip (*slice*) ke deret byte teks |
| **Bisa Mewakili Potongan?** | Tidak (harus alokasi baru) | **Tidak bisa** (selalu mengacu struct `String` utuh) | **Bisa** (menunjuk range byte mana pun) |
| **Sumber Data** | Heap saja | Objek `String` di stack | Heap `String`, Stack, atau rodata binary literal |
| **Ukuran di Stack** | 3 word (`ptr`, `len`, `cap`) | 1 word (pointer biasa) | 2 word (`ptr`, `len` - Fat Pointer) |

---

#### 3. Function Menerima `&str` (Deref Coercion)
Dalam Rust yang idiomatis, jika fungsi hanya membaca teks string, gunakan parameter `&str`, bukan `&String`.

```rust
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

> [!TIP]
> **Kekuatan Deref Coercion**:
> Fungsi yang menerima `&str` dapat menerima:
> 1. String literal langsung: `first_word("Hello World")`
> 2. Variabel `&String`: `first_word(&my_string)` (otomatis di-coerce ke `&str`)
> 3. Potongan substring: `first_word(&my_string[5..])`
> Jika parameter bertipe `&String`, fungsi tersebut HANYA bisa menerima referensi ke `String` utuh.

---

#### 4. Function Menerima `&[i32]` (Generic Slice)
Prinsip serupa berlaku untuk array slice. Alih-alih mengikat fungsi ke array berukuran tetap (seperti `&[i32; 5]`) atau `&Vec<i32>`, terima `&[i32]`:

```rust
pub fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

pub fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().max()
}
```
Fungsi di atas fleksibel menerima potongan array mana pun:
- `sum_slice(&numbers)` (seluruh array)
- `sum_slice(&numbers[1..4])` (potongan array)
- `sum_slice(&vec_data)` (bisa juga dari dynamic Vector)

---

#### 5. Keamanan Borrow Checker pada Slice
Slice adalah bentuk peminjaman (*borrowing*). Selama slice aktif menunjuk ke sebuah data, data asal **tidak boleh dimutasi**:

```rust
let mut s = String::from("hello world");
let word = first_word(&s); // immutable borrow &s dimulai

// s.clear(); // COMPILE ERROR! error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
println!("Kata pertama: {word}"); // word masih digunakan di sini
```
Compiler mencegah bug klasik di mana string dikosongkan namun pointer slice masih menunjuk ke memori lama (*dangling reference*).

---

### 2.5 String Internals & UTF-8 Memory Guarantees (Task 5)

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/fase2_task_5.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase2_task_5.rs):

Rust membedakan tiga representasi terkait teks dan byte:
- **`String`**: Buffer byte UTF-8 dinamis yang dialokasikan di Heap (bisa bertambah panjang, memiliki ownership).
- **`&str` (*string slice*)**: View referensi fat pointer `(ptr, len)` ke deret byte UTF-8 valid tanpa alokasi baru.
- **`&[u8]`**: Slice raw bytes murni tanpa validasi encoding apa pun.

---

#### Anatomi UTF-8: 1 Karakter TIDAK Selalu 1 Byte
UTF-8 adalah format encoding dengan panjang variabel (*variable-length encoding*):
- Karakter ASCII (`a`–`z`, `0`–`9`, spasi): **1 byte**
- Karakter Latin beraksen, Cyrillic, Arab, Yunani: **2 byte**
- Karakter Asia (Mandarin, Jepang Kanji, Korea): **3 byte**
- Emoji & simbol khusus: **4 byte**

#### Eksperimen Memori: `let text = String::from("Rust 🦀");`

Berikut adalah pemetaan internal per byte di memori:

```text
Karakter:   'R'    'u'    's'    't'    ' '   <-------- '🦀' (4 Byte) -------->
Byte Index:  0      1      2      3      4      5        6        7        8
Hex Byte:   0x52   0x75   0x73   0x74   0x20   0xF0     0x9F     0xA6     0x80
Kategori:  [--------- ASCII (1 byte) --------] [----- UTF-8 Multi-byte ------]
```

- **`text.len()` $\rightarrow$ 9 bytes**: Mengembalikan jumlah **byte**, bukan jumlah huruf! Operasi ini $O(1)$ cepat karena langsung membaca field metadata `len`.
- **`text.chars().count()` $\rightarrow$ 6 karakter**: Mengembalikan jumlah *Unicode Scalar Value*. Operasi ini $O(n)$ karena Rust harus menelusuri tiap byte untuk membedakan batas karakter.

---

#### Iterasi: `chars()` vs `bytes()`

```rust
let text = String::from("Rust 🦀");

// 1. Iterasi Karakter (Unicode Scalar Value)
for c in text.chars() {
    println!("Char: '{c}' (ukuran: {} byte)", c.len_utf8());
}

// 2. Iterasi Raw Byte (u8)
for b in text.bytes() {
    println!("Byte: {b} (0x{b:02X})");
}
```

---

#### Slicing Boundary: Valid vs Invalid

Rust melarang *direct indexing* (seperti `text[0]`) karena operasi $O(1)$ tidak mungkin dilakukan secara akurat pada encoding variabel seperti UTF-8. Pengambilan substring dilakukan dengan slicing range byte:

##### 1. Boundary yang Valid
Range index byte harus jatuh tepat pada batas awal dan akhir karakter UTF-8:
```rust
let slice_ascii = &text[0..4]; // "Rust" (byte 0 sampai 3)
let slice_emoji = &text[5..9]; // "🦀" (tepat 4 byte dari index 5 sampai 8)
```

##### 2. Bahaya Boundary yang Tidak Valid (Panic Runtime)
Jika mencoba memotong di tengah-tengah multi-byte sequence (misal index 6 yang membelah emoji 🦀):
```rust
let bad_slice = &text[0..6]; // RUNTIME PANIC!
```
Compiler Rust tidak bisa mendeteksi range dinamis di compile time, sehingga runtime Rust memicu *controlled panic*:
```text
thread 'main' panicked:
end byte index 6 is not a char boundary; it is inside '🦀' (bytes 5..9 of string)
```

> [!CAUTION]
> **Mengapa Rust Harus Panic?**  
> Tipe `&str` memiliki jaminan absolut (*type invariant*): **Isinya selalu merupakan string UTF-8 yang valid**. Jika pemotongan sembarangan diizinkan, string akan menghasilkan data korup (*invalid byte sequence*) yang dapat merusak terminal, parser JSON, database, dan memicu celah keamanan.

##### 3. Penanganan Aman Tanpa Panic: `.get(range)`
Dalam kode produksi, gunakan method `.get(range)` yang mengembalikan `Option<&str>`:
```rust
match text.get(0..6) {
    Some(s) => println!("Slice: {s}"),
    None => println!("Boundary tidak valid, dicegah tanpa panic!"), // dieksekusi
}
```

---

### 2.6 Mini Project Fase 2: Text Analyzer

Selaras dengan implementasi praktikum di [`rust-learning-lab/src/mini_project_2.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/mini_project_2.rs):

Mini project ini memadukan konsep **Borrowing (`&str`)**, **Slice**, dan **Karakteristik Memori UTF-8** untuk menganalisis statistik teks tanpa menyalin alokasi memori heap (*zero-cost abstraction*).

#### 1. Arsitektur & Signature Fungsi
Fungsi-fungsi analyzer tidak mengambil kepemilikan (*ownership*) dari teks pemanggil, melainkan hanya meminjam melalui string slice (`&str`):

```rust
// 1. Menghitung jumlah raw byte di memori UTF-8 (O(1))
pub fn count_bytes(s: &str) -> usize {
    s.len()
}

// 2. Menghitung jumlah karakter Unicode Scalar Values (O(n))
pub fn count_characters(s: &str) -> usize {
    s.chars().count()
}

// 3. Menghitung jumlah kata berdasarkan pemisah whitespace
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// 4. Agregasi statistik teks dalam bentuk Tuple
pub fn analyze_text(s: &str) -> (usize, usize, usize) {
    (count_bytes(s), count_characters(s), count_words(s))
}
```

#### 2. Perilaku Eksekusi & Bukti UTF-8
Contoh ketika teks `"Rust 🦀"` dianalisis:
```text
=== Mini Project Fase 2: Text Analyzer ===
Masukkan teks: Rust 🦀
Bytes       : 9
Characters  : 6
Words       : 2
```
- **Bytes (9)**: `'R'`(1) + `'u'`(1) + `'s'`(1) + `'t'`(1) + `' '`(1) + `'🦀'`(4 byte) = 9 byte.
- **Characters (6)**: 5 karakter ASCII + 1 karakter Emoji.
- **Words (2)**: Terdiri dari token `"Rust"` dan `"🦀"`.

---

#### 3. Deep Dive: Mengapa Argumen `"Masukkan teks: "` adalah Reference Tanpa Simbol `&`?

Perhatikan pemanggilan fungsi di [`mini_project_2.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/mini_project_2.rs#L50):
```rust
let input = read_line_or_default("Masukkan teks: ", "Belajar Rust 🦀 sangat menyenangkan!");
```

Sedangkan signature fungsinya:
```rust
fn read_line_or_default(prompt: &str, default: &str) -> String
```

**Pertanyaan Kritis Pemula:** *“Kenapa `"Masukkan teks: "` cocok dengan parameter `&str`, padahal di depannya tidak ditulis tanda `&`?”*

**Jawaban & Cara Kerja Memori:**
Di Rust, semua teks yang ditulis di dalam tanda kutip ganda `"..."` disebut **String Literal**:
1. **Otomatis `&'static str`**: Compiler Rust secara implisit menetapkan tipe data untuk setiap string literal sebagai `&'static str`.
2. **Tersimpan di Segmen Binary (Read-Only Data)**: Teks literal dikompilasi langsung ke memori program biner, bukan dialokasikan di Heap saat runtime.
3. **Reference Bawaan**: Karena string literal sejatinya adalah fat pointer (alamat di biner + panjang byte), maka `"Masukkan teks: "` secara otomatis sudah merupakan reference (`&str`). Menulis `&"Masukkan teks: "` bersifat redundan.

**Korelasi Ownership pada `input`:**
- Fungsi `read_line_or_default` mengembalikan tipe **`String`** (tanpa tanda `&`).
- Maka variabel `let input` bertindak sebagai **OWNER** sah atas alokasi buffer baru di memori Heap.
- Saat memanggil `analyze_text(&input)`, simbol `&` wajib ditulis agar data hanya dipinjam (*borrow*), bukan dipindahkan (*move*), di mana compiler melakukan **Deref Coercion** otomatis dari `&String` menjadi `&str`.

---

#### 4. Rangkuman Kelulusan Fase 2

| Konsep Inti | Ringkasan Teknis |
|---|---|
| **Move vs Copy vs Clone** | **Move**: Mengalihkan kepemilikan pointer heap ke owner baru, owner lama di-*invalidate* ($O(1)$).<br>**Copy**: Duplikasi bit Stack otomatis untuk tipe primitif bertrait `Copy`.<br>**Clone**: Deep copy seluruh alokasi data Heap ($O(n)$) secara eksplisit. |
| **`String` vs `&str`** | **`String`**: Tipe owned dinamis di Heap, mutable, memiliki `ptr`, `len`, `capacity`.<br>**`&str`**: Tipe borrowed slice (fat pointer: `ptr` + `len`), read-only window ke data string (literal binary, heap `String`, dll). |
| **Kenapa Indexing String Dilarang (`text[i]`)** | UTF-8 menggunakan panjang variabel 1–4 byte per karakter. Akses index langsung berisiko jatuh di tengah-tengah sequence byte karakter multi-byte (*boundary violation*), merusak invariant validitas UTF-8. Mencegah bug ini secara aman membutuhkan scan $O(n)$ via `.chars().nth(i)`. |
| **Non-Lexical Lifetimes (NLL)** | Borrow checker Rust mengakhiri masa hidup (*lifetime*) sebuah reference pada titik terakhir referensi tersebut digunakan dalam kode (*last use*), bukan menunggu sampai akhir kurung kurawal scope (`}`). Ini memungkinkan mutable borrow baru dibuat tepat setelah shared borrow selesai dibaca. |

---

## FASE 3: Rust Type System (Structs, Enums, & Advanced Pattern Matching)

### 3.1 Structs: Classic, Tuple, Unit, & Methods

Struct adalah tipe data bentukan kustom (*custom data type*) yang mengelompokkan beberapa nilai terkait ke dalam satu kesatuan bermakna.

Berbeda dari bahasa pemrograman berorientasi objek tradisional (OOP) seperti Java atau C++:
- **Rust memisahkan Data dan Perilaku (*Behavior*)**: Data didefinisikan murni di dalam `struct`, sedangkan perilaku / fungsinya didefinisikan terpisah di dalam blok `impl` (*implementation*).
- Tidak ada pewarisan kelas (*class inheritance*). Rust menggunakan komposisi dan *traits*.

---

#### 1. Tiga Ragam Struct di Rust

Rust menyediakan 3 jenis struct sesuai kebutuhan perancangan domain:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        RAGAM STRUCT DI RUST                            │
├─────────────────────┬──────────────────────────┬───────────────────────┤
│   Classic Struct    │       Tuple Struct       │   Unit-like Struct    │
│   (Named Fields)    │     (Indexed Fields)     │    (Zero-Sized ZST)   │
├─────────────────────┼──────────────────────────┼───────────────────────┤
│ struct UserAccount  │ struct ColorRgb          │ struct AdminPrivilege;│
│ { id: u64, ... }    │ (u8, u8, u8);            │                       │
│ Akses: acc.id       │ Akses: color.0           │ Akses: Tanpa field    │
│ Domain Model Utama  │ Koordinat / Newtype Type │ Marker / Type-State   │
└─────────────────────┴──────────────────────────┴───────────────────────┘
```

1. **Classic Struct (Named Fields)**:
   - Setiap field memiliki nama dan tipe data yang eksplisit.
   - Ideal untuk memodelkan entitas bisnis/domain (contoh: `UserAccount`, `Order`, `Product`).
   - Mendukung **Field Init Shorthand** (jika nama variabel sama dengan nama field: `id` alih-alih `id: id`).
   - Mendukung **Struct Update Syntax** (`..base_account`) untuk membuat instansiasi baru dengan menyalin nilai field yang tersisa.

2. **Tuple Struct (Indexed Fields & Newtype Pattern)**:
   - Field tidak diberi nama, melainkan diakses melalui indeks angka numerik: `.0`, `.1`, `.2`.
   - Sangat ampuh untuk **Newtype Pattern** (membungkus tipe primitif agar *type-safe*, misal `Kilometers(f64)` vs `Miles(f64)` sehingga tidak sengaja tertukar saat kalkulasi).

3. **Unit-like Struct (Zero-Sized Type / ZST)**:
   - Struct tanpa field sama sekali (`struct AdminPrivilege;`).
   - Memiliki ukuran **0 byte** di memori (`std::mem::size_of::<T>() == 0`).
   - Digunakan sebagai *marker trait*, kontrol hak akses pada *type-state pattern*, atau mendefinisikan behavior statis tanpa beban memori.

---

#### 2. Blok `impl`: Mengapa Rust Butuh `self` Eksplisit?

Di bahasa pemrograman berorientasi objek lain (seperti Java, C++, atau JavaScript), terdapat kata kunci gaib `this` yang secara otomatis diselipkan compiler di balik layar ke dalam method.

Rust memilih prinsip **Explict over Implicit** (eksplisit lebih baik daripada gaib). Method di Rust sebenarnya adalah **fungsi biasa**:
- Yang membedakan "fungsi biasa" dengan "method" hanyalah **parameter pertamanya**: jika parameter pertamanya bernama `self`, maka Rust mengizinkan fungsi tersebut dipanggil menggunakan notasi titik (*dot syntax*): `account.deposit(50.0)`.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        ANATOMI BLOK IMPL                               │
├───────────────────────────────────┬────────────────────────────────────┤
│        Associated Function        │               Method               │
│           (Tanpa self)            │        (Ada parameter self)        │
├───────────────────────────────────┼────────────────────────────────────┤
│ fn new(...) -> Self               │ fn is_solvent(&self) -> bool       │
│ fn from_db(...) -> Self           │ fn deposit(&mut self, amount: f64) │
│                                   │ fn close_account(self) -> String   │
│ Dipanggil via Nama Tipe:          │ Dipanggil via Instance Objek:      │
│ UserAccount::new(...)             │ account.deposit(...)               │
└───────────────────────────────────┴────────────────────────────────────┘
```

> [!NOTE]
> **Di Balik Layar (Desugaring Notasi Titik):**  
> Saat menulis:
> ```rust
> account.deposit(100.0);
> ```
> Rust compiler secara internal mengubahnya (*desugar*) menjadi pemanggilan fungsi biasa:
> ```rust
> UserAccount::deposit(&mut account, 100.0);
> ```
> Notasi titik hanyalah gula sintaksis (*syntactic sugar*) agar kode lebih mudah dan nyaman dibaca!

---

#### 3. Perbedaan Kritis: `Self` (Kapital) vs `self` (Kecil)

Ini perbedaan mendasar yang paling sering membingungkan pemula:

| Simbol | Apa itu? | Analogi | Contoh di Kode |
|---|---|---|---|
| **`Self`** (S Kapital) | **Tipe Data** (Type Alias untuk struct yang di-impl) | **Cetak Biru / Resep Kue** | `pub fn new(...) -> Self` |
| **`self`** (s kecil) | **Nilai / Instance Objek** yang sedang menjalankan fungsi | **Kue Nyata Hasil Cetakan** | `pub fn deposit(&mut self, ...)` |

Di dalam blok `impl UserAccount`:
- Menulis return type `-> Self` sama persis artinya dengan menulis `-> UserAccount`.
- Menulis konstruktor `Self { ... }` sama persis artinya dengan menulis `UserAccount { ... }`.
- Keuntungannya: Jika suatu saat nama struct diganti dari `UserAccount` menjadi `Account`, seluruh isi blok `impl` yang menggunakan `Self` tidak perlu diubah satu per satu!

---

#### 4. Empat Variasi Pemanggilan `self` & Hubungannya dengan Ownership

Karena Rust mengelola memori melalui sistem **Ownership & Borrowing**, perlakuan terhadap `self` terbagi menjadi 4 variasi sesuai hak akses yang dibutuhkan:

##### A. Tanpa `self` (Associated Function / Constructor)
- **Definisi**: Fungsi di dalam `impl` yang tidak mencantumkan `self` pada parameternya.
- **Karakteristik**: Fungsi ini milik Tipe-nya secara umum, bukan milik objek tertentu (mirip *static method* di Java/C#).
- **Tujuan Utama**: Konstruktor (*factory pattern*) untuk membuat instance baru dari nol.
- **Pemanggilan**: Menggunakan operator `::` langsung dari nama tipe:
  ```rust
  let acc = UserAccount::new(1, "boyblanco", "boy@example.com");
  ```

##### B. `&self` (Immutable Borrow — "Hanya Numpang Baca")
- **Bentuk Asli (Desugared)**: `self: &Self`.
- **Karakteristik**: Meminjam instance hanya untuk keperluan baca (*read-only*).
- **Ownership**: Objek **tidak berpindah (no move)**. Setelah method selesai, objek asli tetap utuh dan masih dapat digunakan di baris berikutnya.
- **Analogi**: Membaca buku di perpustakaan. Kamu membaca isi lembarannya, tetapi tidak mencoret-coret atau membawa pulang buku tersebut.
- **Contoh**: Getter, kalkulasi saldo, format tampilan:
  ```rust
  pub fn is_solvent(&self) -> bool {
      self.balance >= 0.0 // Hanya membaca nilai self.balance
  }
  ```

##### C. `&mut self` (Mutable Borrow — "Pinjam dan Boleh Ubah")
- **Bentuk Asli (Desugared)**: `self: &mut Self`.
- **Karakteristik**: Meminjam instance dengan hak modifikasi nilai internal (*read-write*).
- **Syarat Mutlak**: Variabel instance pemanggil **wajib dideklarasikan dengan kata kunci `mut`** (`let mut acc = ...;`).
- **Ownership**: Objek **tidak berpindah**, tetapi nilainya dimutasi langsung di lokasi memori yang sama (*in-place mutation*).
- **Analogi**: Menyerahkan formulir biodata ke loket untuk diperbarui nomor teleponnya, lalu formulir dikembalikan lagi ke tanganmu.
- **Contoh**: Menambah saldo (*deposit*), menarik uang (*withdraw*), mematikan akun (*deactivate*):
  ```rust
  pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
      self.balance += amount; // Mengubah field internal
      Ok(self.balance)
  }
  ```

##### D. `self` (By Value / Move — "Makan / Ambil Alih Hak Milik")
- **Bentuk Asli (Desugared)**: `self: Self`.
- **Karakteristik**: Mengambil alih kepemilikan penuh (*ownership*) dari objek pemanggil ke dalam method.
- **Konsekuensi Kritis**: Begitu method ini selesai dijalankan, instance `self` **langsung di-drop dan dihancurkan dari RAM**. Variabel aslinya di sisi pemanggil menjadi **invalid / hangus** (*moved*).
- **Analogi**: Memakan kue atau membakar surat rahasia. Begitu proses selesai, kue atau suratnya sudah musnah dan tidak bisa dipakai lagi.
- **Kapan Digunakan?**:
  1. **Konversi / Transformasi Tipe**: Misal mengubah struct menjadi data mentah (`into_bytes()`, `into_inner()`).
  2. **Penutupan / Destruksi Permanen**: Menutup rekening atau memutus koneksi socket sehingga tidak mungkin dipakai lagi secara tidak sengaja.
  3. **State Machine Transitions**: Mengubah status `DraftPost` menjadi `PublishedPost` di mana versi draft tidak boleh eksis lagi.
- **Contoh**:
  ```rust
  pub fn close_account(self) -> String {
      format!("Akun #{} milik '{}' resmi ditutup dan dibersihkan dari memori.", self.id, self.username)
  } // 'self' keluar dari scope dan di-drop seketika di sini!
  ```

---

#### 5. Rangkuman Cheat Sheet `self`

| Sintaks Singkat | Bentuk Asli (Desugared) | Hak Akses | Status Objek Setelah Method Selesai | Kebutuhan Variabel Pemanggil | Cara Panggil |
|---|---|---|---|---|---|
| *(tanpa self)* | *(tidak ada)* | Tidak pegang instance | Belum dibuat / Terpisah | Bebas | `UserAccount::new(...)` |
| **`&self`** | `self: &Self` | Read-only (Baca saja) | **Tetap Utuh & Valid** | `let acc = ...;` | `acc.is_solvent()` |
| **`&mut self`** | `self: &mut Self` | Read + Write (Ubah nilai) | **Tetap Utuh (Nilai Berubah)** | `let mut acc = ...;` | `acc.deposit(50.0)` |
| **`self`** | `self: Self` | Move (Ambil Hak Milik) | **Musnah / Hangus (Di-drop)** | Bebas (akan dipindah) | `acc.close_account()` |

---

#### 6. Kode Lengkap & Selaras Lab (`fase3_task_1.rs`)

Berikut implementasi lengkap yang mengintegrasikan seluruh ragam struct dan keempat variasi `self` di atas:

```rust
// ==========================================
// 1. Classic Struct (Named Fields)
// ==========================================
#[derive(Debug, Clone, PartialEq)]
pub struct UserAccount {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub balance: f64,
}

// ==========================================
// 2. Tuple Struct & Newtype Pattern
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilometers(pub f64);

// ==========================================
// 3. Unit-like Struct (Zero-Sized Type / ZST)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPrivilege;

// ==========================================
// 4. Implementasi `impl` untuk UserAccount
// ==========================================
impl UserAccount {
    // A. Associated Function (Constructor) -> Mengembalikan `Self`
    // Dipanggil via UserAccount::new(...)
    pub fn new(id: u64, username: &str, email: &str) -> Self {
        // Field init shorthand: id langsung terisi dari argumen id
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            active: true,
            balance: 0.0,
        }
    }

    pub fn with_initial_balance(id: u64, username: &str, email: &str, initial_balance: f64) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            active: true,
            balance: initial_balance,
        }
    }

    // B. Method membaca (&self) -> Meminjam data tanpa mutasi (Read-only)
    pub fn is_solvent(&self) -> bool {
        self.balance >= 0.0
    }

    pub fn display_summary(&self) -> String {
        format!(
            "[Account #{}] User: '{}' ({}) | Active: {} | Balance: ${:.2}",
            self.id, self.username, self.email, self.active, self.balance
        )
    }

    // C. Method mutasi (&mut self) -> Mengubah data internal (Read-write)
    pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Nominal deposit harus lebih besar dari 0".to_string());
        }
        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Nominal penarikan harus lebih besar dari 0".to_string());
        }
        if self.balance < amount {
            return Err(format!(
                "Saldo tidak mencukupi: saldo saat ini ${:.2}, ditarik ${:.2}",
                self.balance, amount
            ));
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    // D. Method consuming (self) -> Mengambil kepemilikan (Move)
    // Setelah fungsi ini dipanggil, instance struct di-drop dari memori!
    pub fn close_account(self) -> String {
        format!(
            "Akun #{} milik '{}' resmi ditutup dan dihapus dari memori.",
            self.id, self.username
        )
    }
}

// ==========================================
// 5. Implementasi Tuple & Unit Struct
// ==========================================
impl ColorRgb {
    pub fn black() -> Self {
        Self(0, 0, 0)
    }

    pub fn white() -> Self {
        Self(255, 255, 255)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

impl Kilometers {
    pub fn to_miles(&self) -> f64 {
        self.0 * 0.621371
    }
}

impl AdminPrivilege {
    pub fn can_delete_users(&self) -> bool {
        true
    }
}
```

> [!TIP]
> **Kapan Memilih Tuple Struct vs Classic Struct?**  
> Gunakan Classic Struct saat field memiliki makna mandiri yang butuh kejelasan nama (contoh: `street`, `city`, `zip_code`).  
> Gunakan Tuple Struct saat nama field sudah sangat jelas dari posisinya (contoh: koordinat `Point(x, y)`) atau untuk Newtype wrapper 1 elemen demi keamanan kompilasi (*type safety*).

---

### 3.2 Enums: Data Variants, Tagged Unions, & Memory Layout

Enum (*Enumeration*) di Rust adalah tipe data aljabar (*Algebraic Data Type* / *Sum Type*) yang memungkinkan suatu nilai menjadi salah satu dari beberapa kemungkinan varian.

Berbeda dari enum di bahasa C atau Java biasa yang sekadar konstanta bilangan bulat (*integer constants*), **Enum di Rust dapat membawa tipe data berbeda di setiap variannya**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        RAGAM VARIANT ENUM RUST                         │
├─────────────────────┬──────────────────────────┬───────────────────────┤
│  Unit-like Variant  │    Tuple-like Variant    │  Struct-like Variant  │
│     (Tanpa Data)    │      (Data Berurutan)    │     (Named Fields)    │
├─────────────────────┼──────────────────────────┼───────────────────────┤
│ Status::Todo        │ TaskEvent::              │ TaskEvent::           │
│ TaskEvent::Created  │ AssignedTo(String)       │ CommentAdded {        │
│                     │ StatusChanged(from, to)  │   author: String,     │
│                     │ LoggedHours(f64)         │   content: String,    │
│                     │                          │   is_internal: bool   │
│                     │                          │ }                     │
└─────────────────────┴──────────────────────────┴───────────────────────┘
```

---

#### 1. Basic Enum (`Status`)

Bentuk paling sederhana adalah enum tanpa payload data tambahan, cocok untuk memodelkan *state* atau status:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
```

Seperti halnya `struct`, enum di Rust juga dapat memiliki blok `impl` untuk mendefinisikan method dan fungsi:

```rust
impl Status {
    pub fn default_status() -> Self {
        Self::Todo
    }

    // Method mencocokkan varian dengan exhaustive match
    pub fn label(&self) -> &'static str {
        match self {
            Status::Todo => "Menunggu Dikerjakan (TODO)",
            Status::InProgress => "Sedang Berjalan (IN PROGRESS)",
            Status::Done => "Selesai (DONE)",
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Status::Done)
    }

    pub fn next(&self) -> Option<Status> {
        match self {
            Status::Todo => Some(Status::InProgress),
            Status::InProgress => Some(Status::Done),
            Status::Done => None,
        }
    }
}
```

---

#### 2. Enum Membawa Data (*Data-Bearing Variants*)

Keunggulan utama Rust adalah kemampuan menyematkan data berbeda di setiap varian:

1. **Unit-like Variant**: Tidak membawa data apapun (`TaskEvent::Created`).
2. **Tuple-like Variant**: Membawa data tanpa nama field, diakses berdasarkan posisi urutan argumen (`TaskEvent::AssignedTo(String)`, `TaskEvent::StatusChanged(Status, Status)`).
3. **Struct-like Variant**: Membawa data dengan nama field yang eksplisit diapit kurung kurawal `{ ... }` (`TaskEvent::CommentAdded { author, content, is_internal }`).

---

#### 3. Di Balik Layar: Memory Layout & *Tagged Union*

Di tingkat biner memori (RAM), enum di Rust diimplementasikan sebagai **Tagged Union** (atau *Discriminated Union*):

```
┌──────────────────────────────────────────────────────────────┐
│                  MEMORY LAYOUT ENUM DI RAM                   │
├─────────────────────┬────────────────────────────────────────┤
│ Tag / Discriminant  │        Payload Data (Union Area)       │
│      (1-8 byte)     │  (Sebesar ukuran variant yang terbesar)│
└─────────────────────┴────────────────────────────────────────┘
```

- **Tag / Discriminant**: Angka integer kecil (biasanya 1 byte: `0`, `1`, `2`, ...) yang memberi tahu compiler varian mana yang sedang aktif saat ini.
- **Payload Data Area**: Karena satu instance hanya bisa berupa **satu varian pada satu waktu**, ukuran memorinya dialokasikan sebesar **varian terbesar** di antara seluruh opsi.

```rust
// Contoh ukuran memori:
let status_size = std::mem::size_of::<Status>();       // 1 byte (cukup tag 0, 1, 2)
let event_size  = std::mem::size_of::<TaskEvent>();    // 56 bytes (tag + string/fields terbesar)
```

> [!IMPORTANT]
> **Optimasi Compiler: Null Pointer Optimization (NPO)**  
> Jika sebuah enum seperti `Option<&T>` atau `Option<Box<T>>` membungkus sebuah pointer yang dijamin tidak pernah bernilai nol (*non-null pointer*), Rust memanfaatkan nilai alamat bit `0x0` sebagai penanda varian `None`.  
> Hasilnya: `std::mem::size_of::<Option<&T>>() == std::mem::size_of::<&T>()` (**0 overhead memori tambahan!**).

---

#### 4. Kode Lengkap & Selaras Lab (`fase3_task_2.rs`)

Berikut implementasi lengkap dari laboratorium:

```rust
// ==========================================
// 1. Basic Enum: Status
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn default_status() -> Self {
        Self::Todo
    }

    pub fn label(&self) -> &'static str {
        match self {
            Status::Todo => "Menunggu Dikerjakan (TODO)",
            Status::InProgress => "Sedang Berjalan (IN PROGRESS)",
            Status::Done => "Selesai (DONE)",
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Status::Done)
    }

    pub fn next(&self) -> Option<Status> {
        match self {
            Status::Todo => Some(Status::InProgress),
            Status::InProgress => Some(Status::Done),
            Status::Done => None,
        }
    }
}

// ==========================================
// 2. Data-bearing Enum: TaskEvent
// ==========================================
#[derive(Debug, Clone, PartialEq)]
pub enum TaskEvent {
    // A. Unit-like Variant
    Created,

    // B. Tuple-like Variant
    AssignedTo(String),
    StatusChanged(Status, Status),
    LoggedHours(f64),

    // C. Struct-like Variant
    CommentAdded {
        author: String,
        content: String,
        is_internal: bool,
    },
    Rescheduled {
        new_deadline: String,
        reason: String,
    },
}

impl TaskEvent {
    pub fn describe(&self) -> String {
        match self {
            TaskEvent::Created => "Event: Task baru saja dibuat.".to_string(),

            // Match Tuple-like variant
            TaskEvent::AssignedTo(user) => {
                format!("Event: Task dialihkan penanggung jawabnya ke '{user}'.")
            }
            TaskEvent::StatusChanged(from, to) => {
                format!(
                    "Event: Status berubah dari '{}' -> '{}'.",
                    from.label(),
                    to.label()
                )
            }
            TaskEvent::LoggedHours(hours) => {
                format!("Event: Waktu kerja dicatat sebesar {hours:.1} jam.")
            }

            // Match Struct-like variant
            TaskEvent::CommentAdded {
                author,
                content,
                is_internal,
            } => {
                let badge = if *is_internal { "[Internal]" } else { "[Public]" };
                format!("Event: Komentar baru dari {author} {badge}: \"{content}\"")
            }
            TaskEvent::Rescheduled {
                new_deadline,
                reason,
            } => {
                format!("Event: Jadwal diundur ke {new_deadline}. Alasan: {reason}")
            }
        }
    }
}
```

---

### 3.3 Dasar-Dasar `match` Control Flow

Bagi yang terbiasa dengan bahasa seperti C, C++, Java, atau JavaScript, kita sering menggunakan pernyataan `switch-case` atau rentetan panjang `if-else if-else` untuk mengecek banyak kemungkinan.

Di Rust, peran tersebut digantikan dan ditingkatkan secara revolusioner oleh **`match`**.

---

#### 1. Apa Itu `match` dan Mengapa Bukan `switch`?

`match` adalah mekanisme kendali alur (*control flow*) yang membandingkan sebuah nilai target dengan serangkaian **pola (patterns)** secara berurutan dari atas ke bawah. Cabang pertama yang polanya cocok akan langsung dieksekusi.

**Mengapa Rust Meninggalkan `switch` Tradisional?**
1. **Tidak Ada Bug *Fallthrough***: Pada `switch` konvensional di C/Java, jika kamu lupa menulis kata `break;`, kode akan "bocor" (*fall through*) dan mengeksekusi *case* di bawahnya secara tidak sengaja. Di Rust, **tidak butuh kata kunci `break`**; cabang yang cocok langsung selesai dan keluar.
2. **`match` adalah Sebuah Expression**: `match` bukan sekadar *statement* (instruksi kosong), melainkan **ekspresi yang menghasilkan nilai kembalian (*return value*)**. Hasilnya bisa langsung ditampung ke dalam variabel.
3. **Pemeriksaan Menyeluruh (*Exhaustive*)**: Compiler menjamin tidak ada satu pun kemungkinan nilai yang terlewat. Jika ada yang lupa ditangani, kode **gagal dikompilasi**.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        ANALOGI CARA KERJA MATCH                        │
├────────────────────────────────────────────────────────────────────────┤
│                       [ Nilai Masukan: 2 ]                             │
│                                │                                       │
│          ┌─────────────────────▼─────────────────────┐                 │
│          │  Pola 1: `1`  ───► Tidak Cocok            │                 │
│          ├───────────────────────────────────────────┤                 │
│          │  Pola 2: `2`  ───► COCOK! ──► Eksekusi    │                 │
│          ├───────────────────────────────────────────┤                 │
│          │  Pola 3: `_`  ───► (Dilewati)             │                 │
│          └───────────────────────────────────────────┘                 │
└────────────────────────────────────────────────────────────────────────┘
```

---

#### 2. Anatomi Sintaksis `match`

Setiap baris pilihan di dalam `match` disebut sebagai **Match Arm** (lengan cabang):

```rust
match nilai_target {
    pola_1 => aksi_1,
    pola_2 => aksi_2,
    _ => aksi_default,
}
```

Anatomi tiap arm terdiri dari 4 bagian:
1. **Pola (*Pattern*)**: Nilai atau bentuk yang ingin dicocokkan (contoh: `Status::Todo`, `1`, `"admin"`).
2. **Panah Gemuk (`=>`)**: Pemisah antara pola pengujian dan kode yang akan dijalankan (*fat arrow*).
3. **Aksi / Ekspresi (*Expression*)**: Kode yang dieksekusi jika pola cocok. Jika lebih dari satu baris, gunakan kurung kurawal `{ ... }`.
4. **Tanda Koma (`,`)**: Pemisah antar arm cabang. Koma wajib ditulis untuk ekspresi satu baris.

```rust
let nomor_hari = 3;

let nama_hari = match nomor_hari {
    1 => "Senin",
    2 => "Selasa",
    3 => "Rabu",
    4 => "Kamis",
    5 => "Jumat",
    6 | 7 => "Akhir Pekan (Sabtu / Minggu)", // Operator | untuk multiple pattern
    _ => "Nomor hari tidak valid",           // Wildcard catch-all
};

println!("Hari ke-{nomor_hari} adalah {nama_hari}");
```

---

#### 3. Aturan Kritis: `match` adalah Expression

Karena `match` adalah ekspresi, nilainya dapat langsung disimpan ke dalam `let`:

> [!IMPORTANT]
> **Aturan Keseragaman Tipe (Type Consistency):**  
> Seluruh cabang (*arms*) di dalam sebuah ekspresi `match` **wajib mengembalikan tipe data yang persis sama!**  
> Kamu tidak boleh mengembalikan `String` di cabang pertama tetapi mengembalikan angka `i32` di cabang kedua.

```rust
let role = "admin";

// ✅ Benar: Semua cabang menghasilkan tipe &'static str
let deskripsi = match role {
    "admin" => "Akses seluruh sistem",
    "member" => "Akses pengguna terdaftar",
    _ => "Akses publik",
};

// ❌ Salah (Compile Error!): Cabang mengembalikan tipe data berbeda
// let hasil = match role {
//     "admin" => 100,       // i32
//     _ => "tidak valid",   // &str -> ERROR E0308 (mismatched types)
// };
```

---

#### 4. Perbandingan Langsung: `if-else if-else` vs `match`

Kapan kita sebaiknya menggunakan `match` alih-alih `if-else`?

| Kriteria | `if-else` | `match` |
|---|---|---|
| **Fokus Evaluasi** | Kondisi boolean umum (`x > 5 && is_active`) | Mencocokkan bentuk data, nilai diskrit, atau varian enum |
| **Kelelahan Compiler** | Jika ada kondisi terlewat, compiler diam saja (bisa muncul bug) | **Wajib Exhaustive**: Compiler melarang ada kasus terlewat |
| **Dukungan Destructuring** | Terbatas (harus manual di dalam blok) | **Bawaan lahir**: Otomatis membongkar isi tuple/struct/enum |
| **Keterbacaan Kode** | Menjadi berantakan jika banyak cabang (`else if` panjang) | Sangat rapi, deklaratif, dan mudah dipindai mata |

---

#### 5. Pola Tangkap-Semua: Wildcard (`_`)

Dalam tipe data dengan kemungkinan tak terbatas (seperti integer `u32` atau string `&str`), kita tidak mungkin menulis semua angka dari 0 sampai 4 miliar. 

Rust menyediakan pola **Wildcard** menggunakan simbol garis bawah (`_`):
- `_` berarti: *"Cocokkan nilai apa pun selain pola yang sudah ditulis di atasnya."*
- Pola `_` selalu diletakkan di **paling bawah** sebagai penutup penyelamat (*fallback*).

```rust
let kode_status_http = 404;

match kode_status_http {
    200 => println!("OK - Sukses"),
    400 => println!("Bad Request"),
    404 => println!("Not Found - Halaman tidak ditemukan"),
    500 => println!("Internal Server Error"),
    _ => println!("Kode status HTTP lain: {kode_status_http}"),
}
```

---

#### 6. Kode Mandiri: Basic Match Sederhana (`demo_basic_match`)

Berikut contoh fungsi mandiri yang mengimplementasikan dasar-dasar `match`:

```rust
// Fungsi evaluasi angka dadu menggunakan basic match
pub fn demo_basic_match(dice: u8) -> &'static str {
    // 1. match mengevaluasi nilai argumen 'dice'
    // 2. Setiap arm mengembalikan string literal &'static str
    match dice {
        1 => "Satu (Paling Rendah)",
        2 | 3 => "Dua atau Tiga (Rendah)",       // Multiple pattern dengan operator '|'
        4 | 5 => "Empat atau Lima (Sedang)",
        6 => "Enam (Tertinggi)",
        _ => "Bukan angka dadu standar",          // Wildcard fallback penutup
    }
}

fn main() {
    let roll = 3;
    let hasil = demo_basic_match(roll);
    println!("Hasil lemparan dadu {roll}: {hasil}");
    // Output: Hasil lemparan dadu 3: Dua atau Tiga (Rendah)
}
```

---

### 3.4 Advanced Pattern Matching: Destructuring, Guards, Range, & Bindings

Setelah memahami dasar-dasar `match`, sekarang kita melangkah ke fitur-fitur tingkat lanjut yang menjadikan pattern matching di Rust sangat kuat dalam memproses struktur data kompleks:

```
┌────────────────────────────────────────────────────────────────────────┐
│                      FITUR ADVANCED PATTERN MATCHING                   │
├─────────────────────┬──────────────────────────┬───────────────────────┤
│    Destructuring    │       Match Guard        │      Binding (@)      │
│ Membongkar isi      │ Syarat boolean runtime   │ Tes range sekaligus   │
│ struct / enum / data│ tambahan via `if`        │ ikat nilai ke variabel│
├─────────────────────┼──────────────────────────┼───────────────────────┤
│    Range Pattern    │          if let          │       while let       │
│ Uji rentang inklusif│ Tangani tepat 1 pola     │ Loop iterasi selama   │
│ angka: `0..=50`     │ tanpa boilerplate match  │ pola masih cocok      │
└─────────────────────┴──────────────────────────┴───────────────────────┘
```

---

#### 1. Destructuring (Membongkar Struktur Data)

Pattern matching memungkinkan kamu membongkar data bersarang (*nested data*) secara langsung pada baris pola tanpa perlu mengakses field satu per satu:

1. **Destructuring Struct di dalam Enum**:
   ```rust
   AppCommand::MoveTo(Coordinate { x, y }) => {
       println!("Koordinat x: {x}, y: {y}");
   }
   ```
2. **Destructuring Struct-like Variant**:
   ```rust
   AppCommand::SendMessage { sender, content } => {
       println!("Dari {sender}: {content}");
   }
   ```

---

#### 2. Match Guard (`if <condition>`)

Match guard adalah syarat boolean tambahan yang ditempelkan setelah pola menggunakan kata kunci `if`. Cabang ini hanya akan dieksekusi jika pola cocok **DAN** kondisi `if` bernilai `true`:

```rust
AppCommand::SendMessage { sender, content } if content.trim().is_empty() => {
    format!("Pesan kosong dari '{sender}' diabaikan.")
}
```

> [!WARNING]
> **Match Guard Tidak Menjamin Exhaustiveness!**  
> Compiler Rust tidak dapat memprediksi nilai runtime dari ekspresi boolean di dalam `if`. Oleh karena itu, arm yang memiliki match guard **tidak dihitung** oleh compiler untuk memenuhi syarat *exhaustive*. Kamu wajib menyediakan arm penutup (fallback) tanpa guard untuk varian terkait!

---

#### 3. Range Pattern (`start..=end`) & Binding (`@`)

1. **Range Pattern**:
   Menguji apakah suatu nilai numerik atau karakter berada di dalam rentang inklusif (`..=`):
   ```rust
   match score {
       90..=100 => "A (Istimewa)",
       80..=89  => "B (Baik)",
       _        => "Lainnya",
   }
   ```

2. **Binding Operator (`@`)**:
   Seringkali kita ingin memastikan suatu nilai masuk ke dalam rentang tertentu, **sekaligus menyimpan nilai tersebut ke variabel baru** agar bisa dipakai di dalam blok eksekusi:
   ```rust
   // Variabel 'vol' mengikat nilai asli u8 yang lolos pengujian rentang 0..=30
   AppCommand::SetVolume(vol @ 0..=30) => {
       format!("Volume rendah disetel ke: {vol}%")
   }
   ```

---

#### 4. Idiom Ringkas: `if let` dan `while let`

Untuk kasus sederhana di mana kamu tidak membutuhkan percabangan lengkap seluruh varian:

1. **`if let` (Hanya Peduli 1 Pola)**:
   Menghindari boilerplate `match` ketika hanya ingin mengekstrak satu nilai tertentu:
   ```rust
   if let AppCommand::MoveTo(Coordinate { x, y }) = cmd {
       println!("Koordinat terdeteksi: ({x}, {y})");
   } // Varian lainnya diabaikan secara elegan
   ```

2. **`while let` (Loop Selama Pola Cocok)**:
   Sangat populer untuk menguras antrean (*draining a queue/stack*):
   ```rust
   let mut queue = vec![cmd1, cmd2, cmd3];
   while let Some(command) = queue.pop() {
       process_command(&command);
   } // Loop otomatis berhenti seketika queue kosong (pop() menghasilkan None)
   ```

---

#### 5. Rangkuman Cheat Sheet Seluruh Fitur Pattern Matching

| Fitur | Contoh Sintaks | Fungsi / Kapan Digunakan |
|---|---|---|
| **Literal Match** | `Status::Todo => ...` | Mencocokkan nilai atau varian yang sudah pasti sama persis. |
| **Multiple Patterns** | `'a' \| 'A' => ...` | Mencocokkan salah satu dari beberapa kemungkinan pola (*or pattern*). |
| **Wildcard** | `_ => ...` | Menangkap seluruh nilai sisa (*catch-all*) untuk memastikan exhaustive. |
| **Destructuring** | `Point { x, y } => ...` | Membongkar field struct/tuple langsung ke variabel lokal. |
| **Range Pattern** | `1..=10 => ...` | Menguji apakah angka/karakter berada di dalam rentang inklusif. |
| **Binding `@`** | `x @ 1..=10 => ...` | Menguji rentang sekaligus mengikat nilainya ke variabel `x`. |
| **Match Guard** | `x if x % 2 == 0 => ...` | Menambahkan syarat logika boolean dinamis pada arm. |
| **`if let`** | `if let Some(val) = opt { ... }` | Pintasan ringkas jika hanya ingin mengecek 1 varian saja. |
| **`while let`** | `while let Some(i) = iter.next() { ... }` | Perulangan selama hasil fungsi masih cocok dengan polanya. |

---

#### 6. Kode Lengkap & Selaras Lab (`fase3_task_3.rs`)

Berikut implementasi lengkap yang memadukan seluruh teknik pattern matching:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Moderator,
    Member(u32), // Membawa level reputasi (1..=100)
    Guest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    Quit,
    MoveTo(Coordinate),
    SendMessage { sender: String, content: String },
    SetVolume(u8), // Volume suara 0..=100
}

// 1. Match, Destructuring, Range, Binding @, & Match Guard
pub fn process_command(cmd: &AppCommand) -> String {
    match cmd {
        // A. Match Varian Sederhana
        AppCommand::Quit => "Aplikasi ditutup (Quit).".to_string(),

        // B. Destructuring Struct & Binding @ pada Range
        AppCommand::MoveTo(Coordinate { x: x @ 0..=50, y }) => {
            format!("Berpindah ke area aman pojok kiri: x={x}, y={y}")
        }
        AppCommand::MoveTo(Coordinate { x, y }) => {
            format!("Berpindah ke koordinat target: x={x}, y={y}")
        }

        // C. Destructuring Field Bernama & Match Guard
        AppCommand::SendMessage { sender, content } if content.trim().is_empty() => {
            format!("Pesan kosong dari '{sender}' diabaikan.")
        }
        AppCommand::SendMessage { sender, content } => {
            format!("Pesan dari {sender}: \"{content}\"")
        }

        // D. Range Pattern & Binding @
        AppCommand::SetVolume(vol @ 0..=30) => {
            format!("Volume diatur rendah: {vol}%")
        }
        AppCommand::SetVolume(vol @ 31..=70) => {
            format!("Volume diatur sedang: {vol}%")
        }
        AppCommand::SetVolume(vol @ 71..=100) => {
            format!("Volume diatur tinggi: {vol}% (Peringatan pendengaran!)")
        }
        AppCommand::SetVolume(vol) => {
            format!("Volume {vol}% melebihi batas aman 100%!")
        }
    }
}

// 2. Evaluasi Role dengan Match Guard & Binding @
pub fn classify_role(role: &UserRole) -> String {
    match role {
        UserRole::Admin => "Akses Penuh (Super Admin)".to_string(),
        UserRole::Moderator => "Akses Moderasi Konten".to_string(),

        // Literal Pattern
        UserRole::Member(0) => "Member Belum Terverifikasi (Level 0)".to_string(),

        // Range Pattern dengan Binding @
        UserRole::Member(level @ 1..=20) => {
            format!("Member Pemula (Level {level})")
        }
        UserRole::Member(level @ 21..=70) => {
            format!("Member Aktif (Level {level})")
        }
        // Match Guard
        UserRole::Member(level) if *level > 70 => {
            format!("Member Veteran / Elit (Level {level} - Hak Voting)")
        }
        // Fallback untuk menjamin exhaustiveness
        UserRole::Member(level) => {
            format!("Member Khusus (Level {level})")
        }

        UserRole::Guest => "Akses Tamu (Read-only)".to_string(),
    }
}

// 3. Range Pattern Mandiri
pub fn classify_grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A (Istimewa)",
        80..=89  => "B (Baik)",
        70..=79  => "C (Cukup)",
        0..=69   => "D (Perlu Perbaikan)",
        _        => "Skor Tidak Valid (> 100)",
    }
}

// 4. if let
pub fn inspect_move(cmd: &AppCommand) -> Option<(i32, i32)> {
    if let AppCommand::MoveTo(Coordinate { x, y }) = cmd {
        Some((*x, *y))
    } else {
        None
    }
}

// 5. while let
pub fn process_queue(queue: &mut Vec<AppCommand>) -> Vec<String> {
    let mut logs = Vec::new();
    while let Some(cmd) = queue.pop() {
        logs.push(process_command(&cmd));
    }
    logs
}
```

---

### 3.5 Mini Project Fase 3: Task Domain Model

Mini project ini memadukan seluruh materi di Fase 3: **Struct**, **Enum**, **Methods (`impl`, `&self`, `&mut self`)**, serta **Pattern Matching (`match`, `if let`, `while let`)** ke dalam sebuah perancangan model domain nyata (*Domain-Driven Design / DDD*).

```text
Task
├── id: u64
├── title: String
├── priority: Priority (Low, Medium, High, Critical)
└── status: TaskStatus (Todo, InProgress, Review, Done)
```

---

#### 1. Arsitektur Domain Model

Dalam arsitektur backend Rust modern, kita tidak menggunakan *class inheritance* bertingkat seperti di Java/C#. Kita memisahkan domain menjadi **Entitas Struct** dan **State Enum**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        TASK DOMAIN MODEL                               │
├────────────────────────────────────────────────────────────────────────┤
│ struct Task                                                            │
│ ├── id: u64                                                            │
│ ├── title: String                                                      │
│ ├── priority: Priority  ──► [Low | Medium | High | Critical]           │
│ └── status: TaskStatus  ──► [Todo -> InProgress -> Review -> Done]     │
└────────────────────────────────────────────────────────────────────────┘
```

1. **Enum `Priority`**: Memodelkan tingkat urgensi tugas. Menentukan SLA pengerjaan.
2. **Enum `TaskStatus`**: Memodelkan mesin status (*finite state machine*) siklus hidup tugas dari awal dibuat (`Todo`) hingga siap rilis (`Done`).
3. **Struct `Task`**: Entitas utama yang menggabungkan seluruh data di atas, menyediakan konstruktor aman, dan method evaluasi berbasis pattern matching.

---

#### 2. Konsep Kunci Lulus Fase 3

1. **Pemodelan Domain Tanpa Tiruan**:
   Domain model di Rust dirancang secara murni menggunakan kombinasi *Product Type* (`struct`) untuk atribut yang ada bersamaan dan *Sum Type* (`enum`) untuk status yang saling eksklusif (*mutually exclusive*).
2. **Kewajiban Exhaustive Matching**:
   Jika di masa depan tim backend menambahkan status baru (misal `TaskStatus::Blocked` atau `TaskStatus::Archived`), compiler Rust akan langsung memunculkan error di setiap method `match_status`. Ini mencegah bug *"status baru lupa ditangani"* yang sangat sering terjadi di bahasa pemrograman dinamis.
3. **Pemanfaatan `if let` dan `while let`**:
   - `if let`: Menyaring subset task tertentu (misal mengambil hanya task berstatus `Priority::Critical` tanpa boilerplate `match`).
   - `while let`: Menguras antrean (*pipeline drain*) untuk mengeksekusi transisi status task satu per satu hingga antrean kosong.

---

#### 3. Kode Lengkap & Selaras Lab (`mini_project_3.rs`)

Berikut implementasi lengkap dari laboratorium:

```rust
// ==========================================
// 1. Enum Priority & TaskStatus
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub fn badge(&self) -> &'static str {
        match self {
            Priority::Low => "[LOW]",
            Priority::Medium => "[MED]",
            Priority::High => "[HIGH]",
            Priority::Critical => "[CRITICAL]",
        }
    }

    pub fn urgency_label(&self) -> &'static str {
        match self {
            Priority::Low => "Rendah - Backlog",
            Priority::Medium => "Sedang - Sprint Reguler",
            Priority::High => "Tinggi - Pekan Ini",
            Priority::Critical => "Kritis - Hotfix Segera",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
}

impl TaskStatus {
    pub fn badge(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "[TODO]",
            TaskStatus::InProgress => "[IN PROGRESS]",
            TaskStatus::Review => "[IN REVIEW]",
            TaskStatus::Done => "[DONE]",
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self, TaskStatus::Done)
    }

    // Transisi siklus hidup task
    pub fn next(&self) -> Option<TaskStatus> {
        match self {
            TaskStatus::Todo => Some(TaskStatus::InProgress),
            TaskStatus::InProgress => Some(TaskStatus::Review),
            TaskStatus::Review => Some(TaskStatus::Done),
            TaskStatus::Done => None,
        }
    }
}

// ==========================================
// 2. Struct Task (Domain Model)
// ==========================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
}

impl Task {
    // 2a. Constructor
    pub fn new(id: u64, title: &str, priority: Priority) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status: TaskStatus::Todo,
        }
    }

    pub fn with_status(id: u64, title: &str, priority: Priority, status: TaskStatus) -> Self {
        Self {
            id,
            title: title.to_string(),
            priority,
            status,
        }
    }

    // 2b. Change status
    pub fn change_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    pub fn advance_status(&mut self) -> Result<TaskStatus, &'static str> {
        match self.status.next() {
            Some(next_status) => {
                self.status = next_status;
                Ok(self.status)
            }
            None => Err("Task sudah mencapai status final [DONE] dan tidak dapat dimajukan lagi"),
        }
    }

    // 2c. Display task
    pub fn display(&self) -> String {
        format!(
            "#{:<3} {:<10} {:<13} {}",
            self.id,
            self.priority.badge(),
            self.status.badge(),
            self.title
        )
    }

    // 2d. Match berdasarkan priority
    pub fn match_priority(&self) -> &'static str {
        match self.priority {
            Priority::Low => "Dapat dikerjakan saat ada waktu luang (Backlog).",
            Priority::Medium => "Prioritas standar, harus selesai dalam sprint berjalan.",
            Priority::High => "Prioritas tinggi, butuh perhatian khusus pekan ini.",
            Priority::Critical => "Mendesak! Blokir rilis sampai masalah ini terselesaikan.",
        }
    }

    // 2e. Match berdasarkan status
    pub fn match_status(&self) -> &'static str {
        match self.status {
            TaskStatus::Todo => "Task tersimpan di antrean dan belum dimulai.",
            TaskStatus::InProgress => "Sedang aktif dikerjakan oleh software engineer.",
            TaskStatus::Review => "Kode telah disubmit, sedang menunggu QA & Code Review.",
            TaskStatus::Done => "Pekerjaan tuntas, lolos verifikasi, dan siap rilis.",
        }
    }
}

// ==========================================
// 3. Helper Functions: `if let` & `while let`
// ==========================================

// Menggunakan `if let` untuk menyaring task berstatus Critical
pub fn filter_critical_tasks(tasks: &[Task]) -> Vec<&Task> {
    let mut critical_list = Vec::new();
    for task in tasks {
        if let Priority::Critical = task.priority {
            critical_list.push(task);
        }
    }
    critical_list
}

// Menggunakan `while let` untuk memproses antrean task pipeline
pub fn drain_task_pipeline(pipeline: &mut Vec<Task>) -> Vec<String> {
    let mut execution_logs = Vec::new();
    while let Some(mut task) = pipeline.pop() {
        let prev_status = task.status;
        let _ = task.advance_status();
        execution_logs.push(format!(
            "Task #{}: {} diproses dari {:?} -> {:?}",
            task.id, task.title, prev_status, task.status
        ));
    }
    execution_logs
}
```

---

## FASE 4: Module System & Code Organization

Rust menyediakan sistem pengorganisasian kode modular hierarkis yang sangat ketat dan aman: **Packages -> Crates -> Modules -> Paths**. 

Dengan sistem ini, Anda tidak perlu lagi menumpuk ribuan baris kode di dalam satu file `main.rs`. Kode dipecah menjadi modul-modul terpisah dengan batasan hak akses (*visibility encapsulation*) yang terverifikasi secara matematis oleh compiler saat proses kompilasi.

---

### 4.1 Konsep Inti: Packages, Crates, Modules, & Paths

Mari kita bedah perbedaan fundamental dari empat tingkatan organisasi kode di Rust:

| Tingkatan | Apa Itu? | Didefinisikan Oleh | Peran & Karakteristik |
|---|---|---|---|
| **Package** | Bundle proyek software lengkap | File `Cargo.toml` | Mengatur metadata, dependensi, dan build script. Boleh memiliki 0 atau 1 library crate, serta 0 atau lebih binary crate. |
| **Crate** | Unit kompilasi terkecil di Rust (*tree of modules*) | `src/lib.rs` atau `src/main.rs` | Dihasilkan compiler `rustc`. Crate menghasilkan binary executable (`.exe` / ELF) atau library (`.rlib`). |
| **Module** | Pengelompokan kode hierarkis di dalam crate | Kata kunci `mod` | Membagi ruang nama (*namespaces*), mengelompokkan fungsionalitas, dan mengontrol privasi (*private/public*). |
| **Path** | Alamat penunjuk lokasi suatu item | `crate::...`, `super::...`, `self::...` | Cara menavigasi dan merujuk struct, enum, trait, atau fungsi antar modul. |

```text
Package (rust-learning-lab / Cargo.toml)
│
├── Library Crate (src/lib.rs -> nama crate: rust_learning_lab)
│   ├── mod models (src/models.rs)
│   ├── mod auth   (src/auth/mod.rs)
│   │   └── mod token (src/auth/token.rs)
│   └── mod services (src/services/mod.rs)
│       └── mod task_service (src/services/task_service.rs)
│
├── Binary Crate (src/main.rs -> executable bin)
│   └── Mengonsumsi library crate: `use rust_learning_lab::...`
│
└── Integration Test Crates (tests/*.rs)
    └── Mengonsumsi library crate dari luar layaknya pengguna pihak ketiga
```

---

### 4.2 Binary Crate vs Library Crate dalam Satu Package

Dalam proyek profesional, pola standar arsitektur Rust memisahkan antara **Library Crate** (`src/lib.rs`) dan **Binary Crate** (`src/main.rs`):

1. **Library Crate (`src/lib.rs`)**:
   - Berisi seluruh *core business logic*, model entitas, modul autentikasi, dan services.
   - Tidak memiliki fungsi `fn main()`.
   - Nama crate diturunkan dari nama package di `Cargo.toml` (tanda minus `-` otomatis diubah menjadi underscore `_`, contoh: `rust-learning-lab` menjadi `rust_learning_lab`).
   - Dapat diimpor oleh siapa saja: oleh binary crate lokal, oleh integration tests di folder `tests/`, maupun oleh aplikasi lain jika dipublikasikan ke `crates.io`.

2. **Binary Crate (`src/main.rs`)**:
   - Hanya berperan sebagai *entry point* tipis (*thin wrapper* / CLI runner).
   - Memiliki `fn main()`.
   - Mengimpor domain logic dari library crate via `use rust_learning_lab::...;`.

#### Analogi Dunia Nyata: Restoran, Dapur, & Etalase Depan
Untuk memudahkan pemahaman:
- **Dapur & Ruang Racik Belakang** (`src/models.rs`, `src/auth/token.rs`, `src/services/task_service.rs`): Tempat seluruh data dan logika dimasak.
- **Library Crate (`src/lib.rs`)**: Gedung restoran resmi beserta etalase kasir depan. Tempat mendaftarkan ruangan apa saja yang sah diakui (`pub mod`) dan memajang menu jadi (`pub use`).
- **Binary Crate (`src/main.rs`)**: Sopir pengantar / pembeli yang menyalakan mesin, menekan tombol order, dan mengeksekusi aplikasi.

---

### 4.3 Struktur File & Sub-Modul yang Diimplementasikan

Berikut struktur modular nyata yang kita bangun pada proyek `rust-learning-lab`:

```text
rust-learning-lab/
├── Cargo.toml                          # Manifes Package
├── src/
│   ├── lib.rs                          # Root Library Crate (deklarasi modul & pub use)
│   ├── main.rs                         # Root Binary Crate (runner aplikasi)
│   ├── models.rs                       # Modul Domain Models (Task, Priority, TaskStatus)
│   ├── auth/                           # Modul Autentikasi (folder)
│   │   ├── mod.rs                      # Entry point modul auth & re-export
│   │   └── token.rs                    # Sub-modul token & claims
│   ├── services/                       # Modul Services (folder)
│   │   ├── mod.rs                      # Entry point modul services
│   │   └── task_service.rs             # Business logic task lifecycle
│   └── mini_project_4.rs               # Demo runner modular task app
└── tests/
    └── task_app_integration_test.rs    # Integration test crate independen
```

#### Gaya Penamaan Modul di Rust:
Rust mendukung dua gaya peletakan file modul:
- **Gaya Tradisional / Mod.rs**: Folder `auth/` dengan `auth/mod.rs` sebagai entry point, dan `auth/token.rs` sebagai sub-modul.
- **Gaya Rust 2018+**: File `auth.rs` sejajar dengan folder `auth/token.rs`.
> Keduanya didukung penuh. Proyek ini mendemonstrasikan pola `mod.rs` yang sangat rapi untuk pengelompokan modul kompleks.

---

### 4.4 Visibility Modifiers & Privacy Rules (Enkapsulasi Data)

Secara default, **semua item di Rust berstatus PRIVATE murni**. Item private hanya dapat diakses oleh file/modul tempat item tersebut dideklarasikan beserta anak modulnya (*submodules*).

Rust menyediakan 4 level visibilitas:

| Modifier | Ruang Lingkup Hak Akses | Kapan Digunakan? |
|---|---|---|
| *(tanpa keyword)* | **Private**: Hanya file/modul saat ini | Menyembunyikan detail implementasi, hashing salt, atau field internal sensitif. |
| `pub(super)` | Terlihat khusus oleh modul **parent setingkat di atasnya** | Method internal yang hanya boleh dipanggil oleh modul induk (misal `auth/mod.rs` ke `auth/token.rs`). |
| `pub(crate)` | Terlihat oleh **seluruh modul di dalam crate yang sama** | Berbagi struktur antar modul (misal `services` membaca catatan internal `models`), tetapi tersembunyi total dari konsumen luar library! |
| `pub` | **Public**: Terbuka bebas untuk siapapun | API publik yang stabil dan ditujukan untuk digunakan oleh pemakai library. |

#### Studi Kasus Enkapsulasi Nyata pada Proyek:

```rust
// File: src/auth/token.rs
pub struct Claims {
    pub sub: String,                   // 1. Publik: Username boleh dibaca siapa saja
    pub role: String,                  // 2. Publik: Role boleh diperiksa publik
    pub(crate) session_id: u64,        // 3. pub(crate): Hanya service dalam crate ini yang tahu session ID
    secret_salt: String,               // 4. Private: Rahasia internal file token.rs ini saja!
}

impl Claims {
    // Dipanggil khusus oleh modul parent (src/auth/mod.rs)
    pub(super) fn internal_session_key(&self) -> String {
        format!("AUTH_SUPER_{}_{}", self.session_id, self.sub)
    }

    // Dipanggil oleh modul mana saja di crate rust_learning_lab
    pub(crate) fn is_session_active(&self) -> bool {
        self.session_id > 0
    }
}
```

---

### 4.5 Paths & Ergonomi Import: `use` dan `pub use` (Facade Pattern)

Bagi programmer yang baru mempelajari Rust, pertanyaan yang paling sering muncul adalah:
> *"Kenapa di `src/lib.rs` kita menulis `pub mod auth;` DAN juga menulis `pub use auth::...;`? Kenapa harus dua-duanya?"*

Mari kita bedah perbedaan krusial keduanya:

#### 1. Peran `mod` / `pub mod` (Mendaftarkan File ke Rust)
Di bahasa seperti Python atau JavaScript, jika kita membuat file baru, file itu bisa langsung di-`import`. **Di Rust TIDAK BISA.**

Compiler Rust (`rustc`) sangat disiplin:
1. Rust **hanya membaca satu pintu gerbang utama**, yaitu `src/lib.rs` (atau `src/main.rs`).
2. Jika Anda membuat file baru (misal `src/models.rs`) tetapi tidak menuliskan `mod models;` di `src/lib.rs`, maka bagi Rust file tersebut **dianggap tidak pernah ada!**
3. **`mod models;`**: Memerintahkan Rust: *"Tolong baca dan kompilasi file `src/models.rs`."*
4. **`pub mod models;`**: Ada kata `pub`. Artinya: *"Izinkan orang luar library untuk melihat dan mengakses modul models ini."*

#### 2. Peran `pub use` (Memajang Item ke Etalase Depan / Re-export)
Setelah file terdaftar dengan `pub mod`, muncul masalah ergonomi: pemakai kode harus mengetik path yang panjang dan dalam:

```rust
// ❌ Melelahkan dan membocorkan detail hierarki internal:
use rust_learning_lab::models::Task;
use rust_learning_lab::models::Priority;
use rust_learning_lab::auth::token::Claims;
use rust_learning_lab::services::task_service::TaskService;
```

Dengan menambahkan `pub use` di `src/lib.rs` (**Facade Pattern**):
```rust
// File: src/lib.rs
pub mod auth;
pub mod models;
pub mod services;

// Re-export: Memajang item dari ruangan dalam ke pintu depan crate:
pub use auth::{authenticate, Claims};
pub use models::{Priority, Task, TaskStatus};
pub use services::TaskService;
```

Konsumen luar (seperti `src/main.rs` dan file di `tests/`) sekarang cukup mengimpor dengan **1 baris super bersih**:
```rust
// ✅ Ergonomis, bersih, dan profesional:
use rust_learning_lab::{authenticate, Claims, Priority, Task, TaskService, TaskStatus};
```

#### 3. Diagram Alur Kerja Modularitas

```text
[ File Asli di Dalam Folder ]
src/models.rs                  ---> struct Task, Priority, TaskStatus
src/auth/token.rs              ---> struct Claims
src/services/task_service.rs   ---> struct TaskService
       │
       ▼  (Didaftarkan & Dipajang oleh src/lib.rs)
src/lib.rs
  ├── pub mod models;          (Mendaftarkan file models.rs agar dikompilasi)
  └── pub use models::Task;    (Memajang Task di etalase depan crate)
       │
       ▼  (Dikonsumsi dengan Nyaman)
src/main.rs & tests/
  └── use rust_learning_lab::Task;  (Langsung dipakai tanpa peduli struktur folder!)
```

#### Ringkasan 1 Kalimat:
- **`pub mod`**: **Mendaftarkan file** agar diakui dan dikompilasi oleh compiler Rust.
- **`pub use`**: **Memajang item ke etalase depan** agar pemakai tidak perlu mengetik path modul yang panjang dan rumit.

---

### 4.6 Integration Tests di Rust (`tests/`)

Di Rust, unit tests ditaruh di dalam file yang sama (`#[cfg(test)] mod tests`), sedangkan **Integration Tests** ditaruh di direktori khusus `tests/` di root proyek:

1. Setiap file `.rs` di dalam direktori `tests/` dikompilasi oleh Cargo sebagai **crate terpisah**.
2. Integration test **TIDAK BISA** mengakses item private atau item `pub(crate)` dari library Anda.
3. Test ini memastikan bahwa API publik (`pub`) yang Anda rancang benar-benar bekerja mulus dari sudut pandang pemakai eksternal.

Contoh verifikasi integration test nyata (`tests/task_app_integration_test.rs`):
```rust
use rust_learning_lab::{authenticate, Claims, Priority, Task, TaskService, TaskStatus};

#[test]
fn test_task_service_workflow_and_reexports() {
    let mut service = TaskService::new("EnterpriseTaskHub");
    let admin_claims = authenticate("root:admin").unwrap();
    let dev_claims = authenticate("budi:developer").unwrap();

    let task_id = service.create_task("Mitigasi Bug", Priority::Critical, &admin_claims).unwrap();
    
    // Verifikasi role boundary: Developer dilarang memajukan status task Critical
    assert!(service.advance_task(task_id, &dev_claims).is_err());
    
    // Admin diizinkan memajukan status task Critical
    assert_eq!(service.advance_task(task_id, &admin_claims).unwrap(), TaskStatus::InProgress);
}
```

Jalankan integration test dengan perintah:
```bash
cargo test --test task_app_integration_test
```
Seluruh skenario pengujian modularitas lolos 100% tanpa error!

---

## FASE 5: Cargo Tingkat Lanjut & Workspace Management

### 5.1 Manajemen Dependensi Modern: `[dependencies]`, `[dev-dependencies]`, `cargo tree`, & `Cargo.lock`

Sistem manajemen paket Rust melalui **Cargo** adalah salah satu paket manajer paling aman, deterministik, dan modern di dunia rekayasa perangkat lunak. Untuk membangun aplikasi enterprise, Anda wajib memahami klasifikasi dependensi serta cara kerja resolusinya.

---

#### 1. Klasifikasi Dependensi di `Cargo.toml`

File `Cargo.toml` mendefinisikan *manifest* proyek. Terdapat pemisahan tegas antara dependensi produksi dan dependensi pengembangan:

```toml
[package]
name = "rust-learning-lab"
version = "0.1.0"
edition = "2024"

# Dependensi Utama (Production Runtime)
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Dependensi Khusus Pengujian & Benchmark (Development Only)
[dev-dependencies]
pretty_assertions = "1.4"
```

##### A. `[dependencies]` (Normal Dependencies)
- **Peran**: Crate eksternal yang menjadi bagian dari runtime logic aplikasi atau library.
- **Kompilasi**: Dikompilasi dan di-link langsung ke dalam file binary hasil build (`cargo build` dan `cargo build --release`).
- **Contoh**: `serde` untuk serialisasi/deserialisasi struktur data, `serde_json` untuk manipulasi payload JSON, `tokio` untuk runtime async.

##### B. `[dev-dependencies]` (Development/Testing Dependencies)
- **Peran**: Crate pembantu yang **hanya** digunakan dalam pengujian (`cargo test`), benchmark (`cargo bench`), atau modul demonstrasi (`examples/`).
- **Keuntungan Arsitektural**: Sama sekali **tidak dimasukkan ke dalam binary rilis produksi**. Hasil binary tetap ramping, waktu kompilasi rilis tetap cepat, dan risiko celah keamanan (attack surface) di lingkungan produksi berkurang drastis.
- **Contoh**: `pretty_assertions` untuk perbandingan unit test dengan visual diff warna-warni yang jelas saat terjadi kegagalan (test failure), `criterion` untuk analisis benchmark statistik.

---

#### 2. Visualisasi Pohon Dependensi via `cargo tree`

Dalam proyek nyata, dependensi yang Anda pasang hampir selalu memiliki dependensi turunan (*transitive dependencies*). Cargo menyediakan perintah analisis pohon dependensi:

```bash
cargo tree
```

Hasil eksekusi pada proyek kita:
```text
rust-learning-lab v0.1.0 (/mnt/.../rust-learning-lab)
├── serde v1.0.229
│   ├── serde_core v1.0.229
│   └── serde_derive v1.0.229 (proc-macro)
│       ├── proc-macro2 v1.0.107
│       │   └── unicode-ident v1.0.26
│       ├── quote v1.0.47
│       │   └── proc-macro2 v1.0.107 (*)
│       └── syn v3.0.6
│           ├── proc-macro2 v1.0.107 (*)
│           ├── quote v1.0.47 (*)
│           └── unicode-ident v1.0.26
└── serde_json v1.0.151
    ├── itoa v1.0.18
    ├── memchr v2.8.3
    ├── serde_core v1.0.229
    └── zmij v1.0.23
[dev-dependencies]
└── pretty_assertions v1.4.1
    ├── diff v0.1.13
    └── yansi v1.0.1
```

##### Cara Membaca Notasi `cargo tree`:
1. **`├──` dan `└──`**: Mengindikasikan hierarki parent-child. Misalnya, `serde_json` menarik sub-crate transitif seperti `itoa`, `memchr`, dan `zmij`.
2. **`(*)` (Deduplication Marker)**: Menandakan bahwa sub-pohon crate tersebut sudah ditampilkan secara lengkap di cabang sebelumnya, sehingga tidak digambar ulang demi efisiensi visual.
3. **`(proc-macro)`**: Menandakan crate macro prosedural yang dieksekusi oleh compiler pada fase pre-processing (misal `serde_derive`).
4. **Blok `[dev-dependencies]`**: Menampilkan cabang dependensi yang hanya aktif saat mode pengujian (`cargo test`).

---

#### 3. Anatomi dan Peran Kritis `Cargo.lock`

Banyak pemula bingung membedakan antara `Cargo.toml` dan `Cargo.lock`. Keduanya memiliki filosofi yang bertolak belakang:

| Karakteristik | `Cargo.toml` | `Cargo.lock` |
| :--- | :--- | :--- |
| **Pencipta** | Ditulis manual oleh Pengembang (*Human-edited*) | Dihasilkan otomatis oleh Cargo (*Machine-generated*) |
| **Tujuan** | Menyatakan **niat & rentang versi** (*Intent / SemVer Requirement*) | Mencatat **realitas versi eksak** (*Exact State Resolution*) |
| **Format Versi** | `serde = "1.0"` (artinya: $\ge 1.0.0, < 2.0.0$) | `version = "1.0.229"` (pasti versi 1.0.229) |
| **Integritas** | Hanya nama crate dan semver | Menyimpan **Checksum SHA-256** kriptografis tiap paket |
| **Aturan Git** | **Wajib di-commit** pada semua proyek | **Wajib di-commit** untuk aplikasi binary (`main.rs`), opsional untuk library murni |

##### Cuplikan Isi Riil `Cargo.lock`:
```toml
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "serde"
version = "1.0.229"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6fe0b4f3a49d115814030617156a29e97be6a90ce6280910707c7636449a5635"
dependencies = [
 "serde_core",
 "serde_derive",
]

[[package]]
name = "pretty_assertions"
version = "1.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3ae130e2f271fbc2ac3a40fb1d07180839cdbbe443c7a27e1e3c13c5cac0116d"
dependencies = [
 "diff",
 "yansi",
]
```

##### Mengapa `Cargo.lock` Sangat Penting?
1. **Reproducible Builds**: Jika rekan tim Anda atau server CI/CD menjalankan `cargo build`, Cargo membaca `Cargo.lock` sehingga versi dependensi yang diunduh 100% identik hingga ke bit terakhir. Masalah klise *"di laptop saya jalan, di server error"* dicegah secara tuntas.
2. **Perlindungan Terhadap Supply Chain Attack**: Checksum SHA-256 memvalidasi bahwa paket yang diunduh dari crates.io tidak dimanipulasi di tengah jalan (tamper-proof).

---

#### 4. Kode Implementasi Laboratorium: `src/fase5_task_1.rs`

Berikut adalah implementasi modul `fase5_task_1.rs` yang mengintegrasikan crate `serde`, `serde_json`, dan dev-dependency `pretty_assertions`:

```rust
// File: src/fase5_task_1.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub is_production: bool,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub version_req: String,
    pub kind: DependencyKind,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

impl PackageMeta {
    pub fn new(name: &str, version: &str, edition: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            edition: edition.to_string(),
            is_production: true,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dep(&mut self, name: &str, version_req: &str, kind: DependencyKind) {
        self.dependencies.push(DependencyInfo {
            name: name.to_string(),
            version_req: version_req.to_string(),
            kind,
        });
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

pub fn run() {
    println!("=== Fase 5 - Task 1: Dependency Management ===");

    let mut pkg = PackageMeta::new("rust-learning-lab", "0.1.0", "2024");
    pkg.add_dep("serde", "1.0", DependencyKind::Normal);
    pkg.add_dep("serde_json", "1.0", DependencyKind::Normal);
    pkg.add_dep("pretty_assertions", "1.4", DependencyKind::Dev);

    println!("1. Package: {} v{} (Edition {})", pkg.name, pkg.version, pkg.edition);

    // Serialisasi Struct -> JSON String via serde_json
    let json_output = pkg.to_json().expect("Gagal serialisasi ke JSON");
    println!("2. Hasil Serialisasi JSON:\n{json_output}");

    // Deserialisasi JSON String -> Struct via serde_json
    let parsed_pkg = PackageMeta::from_json(&json_output).expect("Gagal deserialisasi");
    println!("3. Verifikasi parsed == original: {}", parsed_pkg == pkg);
}

#[cfg(test)]
mod tests {
    use super::*;
    // Dev-dependency: pretty_assertions hanya di-link pada tahap pengujian
    use pretty_assertions::assert_eq;

    #[test]
    fn test_package_meta_serialization_cycle() {
        let mut pkg = PackageMeta::new("rust-learning-lab", "0.1.0", "2024");
        pkg.add_dep("serde", "1.0", DependencyKind::Normal);
        pkg.add_dep("pretty_assertions", "1.4", DependencyKind::Dev);

        let json = pkg.to_json().expect("Serialisasi gagal");
        let restored = PackageMeta::from_json(&json).expect("Deserialisasi gagal");

        assert_eq!(pkg, restored);
    }
}
```

---

### 5.2 Fitur Modular & Optional Dependencies: `[features]`, `dep:`, dan Conditional Compilation

Dalam pengembangan pustaka (*library*) maupun aplikasi skala besar, menyertakan seluruh dependensi secara *default* akan mengakibatkan binary membengkak (*binary bloat*) dan waktu kompilasi yang lama. Cargo menyediakan mekanisme **Feature Flags** untuk mengaktifkan kode dan dependensi hanya saat dibutuhkan (*Zero-Cost Modularity*).

---

#### 1. Konfigurasi `Cargo.toml`: Optional Dependencies & Feature Flags

Untuk membuat dependensi bersifat opsional, tambahkan atribut `optional = true`. Selanjutnya, deklarasikan blok `[features]` untuk mengontrol aktivasinya:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Dependensi opsional: tidak akan diunduh/dikompilasi jika fitur terkait tidak aktif
tokio = { version = "1.0", optional = true, features = ["rt", "macros"] }

[features]
# Fitur yang otomatis aktif jika pengguna tidak menentukan flags
default = []

# Fitur kustom yang mengaktifkan dependensi tokio via sintaks modern dep: (Rust 2021/2024)
async_runtime = ["dep:tokio"]
```

##### Mengapa Sintaks `dep:tokio` Penting?
- Pada edisi Rust sebelum 2021, mendeklarasikan `tokio = { optional = true }` otomatis membuat fitur terselubung bernama `tokio`. Ini sering membingungkan karena nama fitur bertabrakan dengan nama crate.
- Pada Rust Edition 2021/2024, disarankan menggunakan format eksplisit `dep:nama_crate`. Dengan cara ini, nama fitur (`async_runtime`) terpisah secara bersih dari nama crate internal (`tokio`).

---

#### 2. Conditional Compilation di Kode Rust (`#[cfg]` & `cfg!`)

Kode Rust dapat mendeteksi keberadaan fitur yang aktif pada saat kompilasi menggunakan dua cara:

##### A. Atribut `#[cfg(feature = "...")]`
Digunakan pada level item (fungsi, struct, modul, atau blok kode). Kode yang tidak memenuhi kondisi tidak akan dikompilasi ke dalam binary sama sekali:

```rust
// Hanya dikompilasi jika feature "async_runtime" aktif
#[cfg(feature = "async_runtime")]
pub async fn execute_async_task(id: u64, name: &str) -> String {
    format!("[ASYNC RUNTIME] Task #{id} '{name}' via tokio")
}

// Fallback: hanya dikompilasi jika feature "async_runtime" TIDAK aktif
#[cfg(not(feature = "async_runtime"))]
pub fn execute_sync_task(id: u64, name: &str) -> String {
    format!("[SYNC RUNTIME] Task #{id} '{name}' blocking")
}
```

##### B. Macro `cfg!(feature = "...")`
Menghasilkan nilai boolean `true` atau `false` saat runtime/compile-time untuk percabangan logika ringan:

```rust
if cfg!(feature = "async_runtime") {
    println!("Asynchronous runtime terdeteksi aktif.");
} else {
    println!("Berjalan dalam synchronous fallback mode.");
}
```

---

#### 3. Perintah Build & Eksekusi dengan Feature Berbeda

Cargo memungkinkan kita menguji berbagai kombinasi fitur:

| Perintah | Deskripsi Efek Kompilasi |
|---|---|
| `cargo build` | Menggunakan fitur `default`. Crate `tokio` **tidak dikompilasi**. Binary berukuran minimal. |
| `cargo build --features async_runtime` | Mengaktifkan fitur `async_runtime`. Cargo otomatis mengunduh dan mengompilasi crate `tokio`. |
| `cargo build --no-default-features` | Mematikan seluruh fitur default (berguna jika `default` memiliki kumpulan fitur dasar). |
| `cargo build --all-features` | Mengaktifkan seluruh fitur yang dideklarasikan di `Cargo.toml`. |

---

#### 4. Kode Implementasi Laboratorium: `src/fase5_task_2.rs`

Berikut implementasi nyata sistem adaptif multi-feature yang diterapkan pada lab:

```rust
// File: src/fase5_task_2.rs

/// Mengembalikan status runtime berdasarkan feature flag aktif saat kompilasi
pub fn runtime_status() -> &'static str {
    #[cfg(feature = "async_runtime")]
    {
        "Mode: Asynchronous Runtime (Aktif via feature flag 'async_runtime' & crate 'tokio')"
    }
    #[cfg(not(feature = "async_runtime"))]
    {
        "Mode: Synchronous / Blocking Standar (Feature flag 'async_runtime' tidak aktif)"
    }
}

pub fn is_async_enabled() -> bool {
    cfg!(feature = "async_runtime")
}

/// Fungsi dispatcher utama yang otomatis beradaptasi dengan fitur yang dikompilasi
pub fn process_task(id: u64, name: &str) -> String {
    #[cfg(feature = "async_runtime")]
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Gagal menginisialisasi runtime tokio");

        rt.block_on(async {
            format!("[ASYNC RUNTIME] Task #{id} '{name}' diproses via tokio non-blocking")
        })
    }

    #[cfg(not(feature = "async_runtime"))]
    {
        format!("[SYNC RUNTIME] Task #{id} '{name}' diproses secara sekuensial (blocking)")
    }
}

pub fn run() {
    println!("=== Fase 5 - Task 2: Cargo Features & Optional Dependencies ===");
    println!("1. Status Runtime: {}", runtime_status());
    println!("2. Eksekusi: {}", process_task(101, "Database Ping"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_detection_and_execution() {
        let result = process_task(1, "Test Workload");
        assert!(result.contains("Task #1 'Test Workload'"));

        if cfg!(feature = "async_runtime") {
            assert!(is_async_enabled());
            assert!(result.contains("[ASYNC RUNTIME]"));
        } else {
            assert!(!is_async_enabled());
            assert!(result.contains("[SYNC RUNTIME]"));
        }
    }
}
```

---

### 5.3 Cargo Profiles & Binary Optimization: `profile.dev`, `profile.release`, `opt-level`, LTO, dan Debug Symbols

Compiler Rust (`rustc`) dibantu oleh backend pengoptimalan **LLVM**. Cargo menyediakan sistem **Profiles** yang memungkinkan pengembang mengatur kompromi (*trade-off*) antara **kecepatan kompilasi** (saat *development*) dan **kecepatan eksekusi runtime serta ukuran binary** (saat *production*).

---

#### 1. Lima Parameter Kunci Pengendali Profil

Berikut adalah 5 parameter fundamental di dalam konfigurasi profil Cargo:

| Parameter | Pilihan Nilai | Makna & Pengaruh Arsitektural |
|---|---|---|
| **`opt-level`** | `0`, `1`, `2`, `3`, `"s"`, `"z"` | Mengatur agresivitas optimasi compiler LLVM: <br>• `0`: Tanpa optimasi (kompilasi instan). <br>• `3`: Optimasi loop, inlining fungsi, dan SIMD vectorization maksimum. <br>• `"s"` / `"z"`: Mengoptimalkan ukuran binary sekecil mungkin (bagus untuk Embedded / WebAssembly). |
| **`debug`** | `true`, `false`, `0`, `1`, `2` | Mengontrol pembuatan *debug symbols* (tabel alamat memori dan nama fungsi untuk GDB/LLDB serta pesan panic backtrace). |
| **`lto`** | `false` (`"off"`), `"thin"`, `"fat"` | **Link-Time Optimization**: Mengizinkan linker mengoptimalkan dan melakukan inlining kode lintas batas-batas crate independen (sangat ampuh pada proyek modular). |
| **`codegen-units`** | Angka bulat $\ge 1$ (misal `1` s/d `256`) | Jumlah potongan paralel yang diproses LLVM: <br>• Nilai besar (misal 256 di dev): Memanfaatkan multi-core CPU untuk kompilasi secepat kilat. <br>• Nilai `1` (di release): LLVM memproses seluruh kode sebagai satu kesatuan sehingga inlining cross-module berjalan 100% optimal. |
| **`strip`** | `false`, `"none"`, `"debuginfo"`, `true` (`"symbols"`) | Membuang simbol debug dari berkas biner akhir hasil rilis untuk memangkas ukuran biner secara drastis sebelum didistribusikan ke server produksi. |

---

#### 2. Perbandingan Profil Bawaan: `profile.dev` vs `profile.release`

Cargo secara otomatis mengaktifkan dua profil bawaan ini berdasarkan perintah yang Anda jalankan di terminal:

| Karakteristik | `[profile.dev]` (`cargo build`) | `[profile.release]` (`cargo build --release`) |
|---|---|---|
| **Fokus Utama** | Kecepatan Iterasi Kompilasi & Kemudahan Debug | Performa Eksekusi Runtime Maksimal & Binary Ramping |
| **Nilai `opt-level`** | `0` (Tidak ada optimasi) | `3` (Optimasi tertinggi) |
| **Nilai `debug`** | `true` (Simbol debug lengkap) | `false` / `strip = "debuginfo"` |
| **Nilai `lto`** | `off` (Linker standar) | `"thin"` atau `"fat"` |
| **`codegen-units`** | `256` (Paralelisme multi-core tinggi) | `1` (Satu kesatuan utuh) |
| **`debug_assertions`** | **Aktif** (`assert!`, bounds-checking ketat) | **Nonaktif** (Lewati assertion demi kecepatan) |
| **Ukuran Binary** | Lebih besar (menyimpan symbol table) | Sangat kecil & terkompresi |
| **Waktu Eksekusi** | Lebih lambat ($\sim 3\times - 10\times$ lebih lambat) | Sangat kencang (kecepatan bahasa C/C++) |

---

#### 3. Konfigurasi Nyata di `Cargo.toml` Laboratorium

Berikut konfigurasi profil yang kita terapkan pada `rust-learning-lab/Cargo.toml`:

```toml
# Profil Pengembangan (Development / Debug)
[profile.dev]
opt-level = 0        # Kompilasi cepat tanpa beban optimasi
debug = true         # Sertakan debug symbols lengkap untuk debugging

# Profil Produksi (Release)
[profile.release]
opt-level = 3        # Optimasi maksimum (kecepatan eksekusi)
lto = "thin"         # Link-Time Optimization lintas crate
codegen-units = 1    # Unit tunggal untuk maksimalkan inline LLVM
strip = "debuginfo"  # Buang debug info untuk pangkas ukuran binary
```

---

#### 4. Kode Implementasi Laboratorium: `src/fase5_task_3.rs`

Modul ini mendeteksi profil secara dinamis menggunakan macro `cfg!(debug_assertions)` serta menguji kecepatan eksekusi kalkulasi berbobot (algoritma Collatz Conjecture 1..=200,000 angka):

```rust
// File: src/fase5_task_3.rs
use std::time::Instant;

pub fn active_profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "dev (Debug)"
    } else {
        "release (Release)"
    }
}

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
    println!("Profil Aktif: {}", active_profile_name());

    let start = Instant::now();
    let (num, steps) = compute_collatz_max_steps(200_000);
    let duration = start.elapsed();

    println!("Hasil: Angka {} menghasilkan {} langkah", num, steps);
    println!("Waktu Komputasi: {:.2?}", duration);
}
```

##### Bukti Hasil Benchmark Nyata:
- **`cargo run` (Dev Mode - `opt-level=0` & `debug=true`)**: Waktu eksekusi = **199.61ms**.
- **`cargo run --release` (Release Mode - `opt-level=3` & `LTO=thin`)**: Waktu eksekusi = **69.46ms**.
> **Terbukti:** Optimasi profil release menghasilkan percepatan komputasi hampir **$3\times$ lipat lebih cepat** berkat *inlining*, registrasi variabel pada CPU registers, dan eliminasi branch overhead oleh LLVM!

---

### 5.4 Multi-Crate Workspace Architecture

Untuk aplikasi berskala enterprise atau monorepo modern, mengelola banyak crate terpisah dalam repositori yang sama dilakukan menggunakan fitur **Cargo Workspace**. Workspace mengikat beberapa package menjadi satu kesatuan build dengan direktori `target/` bersama dan file `Cargo.lock` tunggal yang konsisten.

---

#### 1. Keuntungan Arsitektural Multi-Crate Workspace

| Keuntungan | Penjelasan Arsitektural |
|---|---|
| **Single `Cargo.lock`** | Menjamin seluruh sub-crate menggunakan versi dependensi eksternal yang 100% identik tanpa konflik versi antar library. |
| **Shared `target/` Directory** | Hasil kompilasi (objek biner pihak ketiga seperti `serde`) hanya dikompilasi satu kali dan dipakai bersama oleh seluruh sub-crate, menghemat disk dan memangkas waktu build drastis. |
| **Workspace Inheritance** | Versi package, license, dan dependensi eksternal diatur terpusat di root via `workspace = true`. |
| **Strict Boundary & Decoupling** | Pemisahan tegas domain logic (`core_domain`), adapter penyimpanan (`database_adapter`), dan runner aplikasi (`api_server`). |
| **Unified Tooling** | Seluruh pengujian, formatting, dan linting dapat dijalankan serentak dengan satu perintah (`cargo test`, `cargo fmt`, `cargo clippy`). |

---

#### 2. Struktur Direktori Nyata Proyek `rust-workspace/`

Berikut arsitektur folder yang kita bangun pada proyek `rust-workspace`:

```text
rust-workspace/
├── Cargo.toml                          # Workspace Root Manifest
├── Cargo.lock                          # Single Lockfile untuk seluruh sub-crate
└── crates/
    ├── core_domain/                    # Library Crate: Entitas murni & Business Logic
    │   ├── Cargo.toml
    │   └── src/lib.rs                  # Struct User, Enum UserRole
    ├── database_adapter/               # Library Crate: Storage Engine
    │   ├── Cargo.toml                  # Depends: core_domain
    │   └── src/lib.rs                  # Struct DatabaseAdapter
    └── api_server/                     # Binary Crate: Entry Point & Controller
        ├── Cargo.toml                  # Depends: core_domain, database_adapter
        └── src/main.rs                 # fn main() & HTTP handler simulation
```

---

#### 3. Konfigurasi Workspace Root Manifest (`Cargo.toml`)

Pada root workspace, deklarasikan bagian `[workspace]`, daftarkan `members`, dan atur metadata serta dependensi bersama:

```toml
# File: rust-workspace/Cargo.toml
[workspace]
resolver = "3"
members = [
    "crates/core_domain",
    "crates/database_adapter",
    "crates/api_server",
]

# Metadata Bersama (Diwariskan ke semua sub-crate)
[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["Software Engineer <dev@enterprise.internal>"]

# Manajemen Dependensi Terpusat (Workspace Dependencies)
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

#### 4. Sub-Crates & Cross-Crate Dependency

Sub-crate mengadopsi konfigurasi root menggunakan flag `workspace = true` dan saling merujuk melalui path relatif:

##### A. Sub-Crate `core_domain` (Domain Entities)
```toml
# File: crates/core_domain/Cargo.toml
[package]
name = "core_domain"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { workspace = true }
```

```rust
// File: crates/core_domain/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Engineer,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

impl User {
    pub fn new(id: u64, username: &str, email: &str, role: UserRole) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            role,
        }
    }
}
```

##### B. Sub-Crate `database_adapter` (Cross-Crate Path Dependency)
Crate ini bergantung pada `core_domain` melalui path relatif:
```toml
# File: crates/database_adapter/Cargo.toml
[package]
name = "database_adapter"
version.workspace = true
edition.workspace = true

[dependencies]
core_domain = { path = "../core_domain" } # Cross-crate dependency!
serde = { workspace = true }
```

##### C. Sub-Crate `api_server` (Binary Runner)
Crate ini mengintegrasikan kedua sub-crate sebelumnya:
```toml
# File: crates/api_server/Cargo.toml
[package]
name = "api_server"
version.workspace = true
edition.workspace = true

[dependencies]
core_domain = { path = "../core_domain" }
database_adapter = { path = "../database_adapter" }
serde = { workspace = true }
serde_json = { workspace = true }
```

---

#### 5. Penggunaan Perintah Cargo pada Workspace

Dari direktori root workspace (`rust-workspace/`), Anda dapat menjalankan seluruh perkakas Cargo secara terpadu:

```bash
# 1. Memeriksa kompilasi seluruh sub-crate sekaligus
cargo check --workspace

# 2. Menjalankan seluruh test pada semua sub-crate
cargo test --workspace

# 3. Menjalankan linter clippy untuk seluruh proyek
cargo clippy --workspace

# 4. Format seluruh kode sumber otomatis sesuai standar Rust
cargo fmt

# 5. Visualisasi pohon dependensi antar crate di workspace
cargo tree

# 6. Menjalankan binary crate tertentu
cargo run --bin api_server
```

##### Bukti Hasil Eksekusi `cargo run --bin api_server`:
```text
============================================================
=== Multi-Crate Workspace: api_server Starting...        ===
============================================================
1. Mendaftarkan user via cross-crate logic:
   Payload API Response User 1:
{
  "id": 1,
  "username": "alice",
  "email": "alice@enterprise.com",
  "role": "Admin"
}
2. Query semua user dari database_adapter:
   Total user di database: 2
   - [ENGINEER] #2 - bob (bob@enterprise.com)
   - [ADMIN]    #1 - alice (alice@enterprise.com)
```

---

# BAGIAN II: ERROR HANDLING, COLLECTIONS, & ABSTRAKSI TIPE

## FASE 6: Robust Error Handling & Collections

Rust secara fundamental menolak konsep `null` pointer dan perkecualian runtime tak tertangani (*unhandled runtime exceptions*). Masalah null pointer—yang oleh penemunya, Sir Tony Hoare, disebut sebagai *"The Billion-Dollar Mistake"*—dieliminasi total pada tingkat kompilasi. 

Rust menggantinya dengan pendekatan penanganan eksplisit berbasis sistem tipe aljabar (*algebraic data types*), yaitu enum `Option<T>` untuk ketiadaan nilai dan `Result<T, E>` untuk kemungkinan kegagalan operasi.

---

### 6.1 Fondasi Robust Error Handling: Enum `Option<T>` (Eliminasi Null)

Dalam bahasa tradisional seperti C/C++, Java, atau JavaScript, sebuah variabel referensi bisa bernilai `null` / `undefined`. Jika programmer lupa memeriksa kondisi null sebelum mengakses properti atau memanggil metode, program akan mengalami crash fatal (`NullPointerException`, `Segmentation fault`, dsb).

Rust tidak memiliki `null`. Sebagai gantinya, pustaka standar Rust (`std`) mendefinisikan enum `Option<T>` yang diimpor otomatis ke dalam *prelude*:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

- **`None`**: Mengindikasikan ketiadaan nilai (mirip konsep null, namun bertipe eksplisit aman).
- **`Some(T)`**: Membungkus nilai konkret bertipe `T`.

Karena `Option<T>` dan `T` adalah tipe yang berbeda, compiler **tidak akan pernah mengizinkan** Anda memperlakukan `Option<T>` seolah-olah itu adalah `T`. Anda diwajibkan secara eksplisit memeriksa dan membuka bungkus (*unwrap*) nilai tersebut.

---

#### 1. Cara Ekstraksi Nilai: `match` vs `if let`

##### A. Exhaustive Pattern Matching (`match`)
Pola paling fundamental dan ketat di Rust. Compiler menjamin bahwa kedua cabang (`Some` dan `None`) wajib ditangani. Jika ada cabang yang terlewat, kode gagal dikompilasi.

```rust
let user_opt = repo.find_by_id(1);

let greeting = match user_opt {
    Some(user) => format!("Halo, {}! (ID: {})", user.username, user.id),
    None => "User tidak ditemukan dalam sistem!".to_string(),
};
```

##### B. Concise Branching (`if let`)
Jika Anda hanya tertarik pada kasus `Some` dan ingin mengabaikan kasus `None` tanpa keharusan menulis boilerplate cabang kosong, gunakan sintaks ergonomis `if let`:

```rust
if let Some(ref email) = user.email {
    println!("Email terdaftar: {}", email);
} else {
    println!("Email belum didaftarkan");
}
```

---

#### 2. Functional Combinators: `.map()` vs `.and_then()`

Alih-alih selalu menggunakan `match` yang panjang untuk operasi sederhana, Rust menyediakan metode combinator fungsional tingkat tinggi:

##### A. Transformasi Elemen: `.map()`
Digunakan untuk mentransformasikan nilai di dalam `Some(T)` menjadi `Option<U>` menggunakan fungsi atau closure `FnOnce(T) -> U`. Jika nilainya `None`, closure tidak akan dieksekusi dan hasilnya tetap `None`.

```rust
// Mengubah Option<String> -> Option<String> (uppercase) tanpa bongkar manual
let uppercase_email = user.email.as_ref().map(|e| e.to_uppercase());
```

##### B. Chaining & Flattening: `.and_then()` (Monadic Bind / Flat Map)
Ketika operasi pemetaan itu sendiri mengembalikan `Option` lain (`FnOnce(T) -> Option<U>`), pemanggilan `.map()` biasa akan menghasilkan struktur bersarang (*nested*) `Option<Option<U>>`.

`.and_then()` secara otomatis meratakan (*flatten*) hirarki tersebut sehingga tetap menghasilkan `Option<U>` tunggal. Sangat berguna untuk pengecekan berantai (*chain of lookups*):

```rust
// Mencari kode pos user yang berada di dalam field opsional bertingkat:
// user_opt: Option<&UserProfile> -> address: Option<Address> -> postal_code: Option<String>
let postal_code = user_opt
    .and_then(|u| u.address.as_ref())
    .and_then(|addr| addr.postal_code.clone());
```

---

#### 3. Defensive Fallback: Nilai Cadangan via `.unwrap_or()`

Seringkali pemula tergoda memanggil `.unwrap()`. Namun, memanggil `.unwrap()` pada varian `None` akan langsung memicu **fatal panic** dan mematikan thread program!

Untuk kode produksi yang andal (*robust*), selalu sediakan nilai cadangan aman (*safe fallback*) menggunakan `.unwrap_or()` atau `.unwrap_or_else()`:

```rust
// Aman: jika email None, gunakan fallback default tanpa risiko crash fatal
let display_email = user.email.as_deref().unwrap_or("guest@system.local");
```

---

#### 4. Bedah Arsitektural Macro `#[derive(...)]` pada Struct Domain

Mengapa struct domain seperti `Address` dan `UserProfile` selalu diawali dengan `#[derive(Debug, Clone, PartialEq, Eq)]`? Mengapa sintaks ini sangat sering muncul di berbagai kode Rust?

##### A. Masalah: Struct Baru adalah "Benda Asing" bagi Compiler
Rust menolak paradigma *inheritance* (pewarisan kelas) OOP. Semua fungsionalitas dan perilaku objek dimodelkan secara modular melalui **Trait**. 

Secara default, saat Anda membuat struct baru:
1. Rust **tidak tahu cara mencetak** isinya ke terminal via `println!("{:?}", user)` $\rightarrow$ memerlukan implementasi trait `std::fmt::Debug`.
2. Rust **tidak tahu cara menduplikasi** data via `.clone()` $\rightarrow$ memerlukan implementasi trait `Clone`.
3. Rust **tidak tahu cara membandingkan** kesamaan nilai via operator `==` atau makro `assert_eq!` $\rightarrow$ memerlukan implementasi trait `PartialEq`.

##### B. Tanpa `derive`: Boilerplate Manual yang Repetitif & Melelahkan
Jika Rust tidak menyediakan jalan pintas, programmer terpaksa menulis implementasi trait manual yang panjang untuk setiap struct:
```rust
// Tanpa derive: harus tulis manual hanya untuk bisa di-print {:?}
impl std::fmt::Debug for UserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserProfile")
            .field("id", &self.id)
            .field("username", &self.username)
            .finish()
    }
}

// Tanpa derive: harus tulis manual hanya agar bisa dibandingkan pakai ==
impl PartialEq for UserProfile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.username == other.username
    }
}
```

##### C. Solusi Modern: `#[derive(...)]` (Compile-Time Code Generation)
Macro atribut `#[derive(...)]` memerintahkan compiler Rust untuk **secara otomatis meng-generate seluruh kode boilerplate di atas saat proses kompilasi**. Proses ini terjadi di tahap kompilasi (*compile-time*) sehingga memiliki sifat **Zero-Cost Abstraction** (tanpa overhead runtime).

Tabel fungsi trait umum di dalam `#[derive(...)]`:
| Trait di `derive` | Peran & Kemampuan | Contoh Pemanggilan Nyata |
| :--- | :--- | :--- |
| **`Debug`** | Mengizinkan format cetak inspeksi debugging (`{:?}` / `{:#?}`). | `println!("{:?}", user);` atau `dbg!(user);` |
| **`Clone`** | Mengizinkan duplikasi nilai struct secara eksplisit ke alokasi memori baru. | `let user_copy = user.clone();` |
| **`PartialEq`** | Mengizinkan komparasi kesamaan antar-instance via operator `==` dan `!=`. | `if u1 == u2` atau `assert_eq!(u1, u2);` di unit test |
| **`Eq`** | Menegaskan relasi kesetaraan mutlak (*equivalence relation*, di mana `a == a` selalu `true`). | Syarat wajib sebagai key di `HashMap` atau `HashSet` |
| **`Default`** | Menyediakan nilai default standar saat instansiasi tanpa parameter. | `let u = UserProfile::default();` |
| **`Copy`** | Duplikasi implisit bit-by-bit di stack (khusus struct yang seluruh field-nya tipe primitif kecil tanpa alokasi heap). | Assignment `let b = a;` tanpa memindahkan kepemilikan (*move ownership*) |

---

#### 5. Kode Implementasi Terintegrasi (`src/fase6_task_1.rs`)

Selaras dengan checklist target Fase 6 Task 1, berikut adalah implementasi domain nyata profil pengguna:

```rust
// Fase 6 - Task 1: Robust Error Handling — Option<T>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub city: String,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
    pub address: Option<Address>,
}

pub struct UserRepository {
    users: Vec<UserProfile>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: vec![
                UserProfile {
                    id: 1,
                    username: "alice".to_string(),
                    email: Some("alice@example.com".to_string()),
                    address: Some(Address {
                        city: "Jakarta".to_string(),
                        postal_code: Some("10110".to_string()),
                    }),
                },
                UserProfile {
                    id: 2,
                    username: "bob".to_string(),
                    email: Some("bob@rustacean.org".to_string()),
                    address: Some(Address {
                        city: "Bandung".to_string(),
                        postal_code: None,
                    }),
                },
                UserProfile {
                    id: 3,
                    username: "charlie".to_string(),
                    email: None,
                    address: None,
                },
            ],
        }
    }

    // 1 & 2. Demonstrasi Some dan None
    pub fn find_by_id(&self, id: u64) -> Option<&UserProfile> {
        self.users.iter().find(|u| u.id == id)
    }
}

// 3. match exhaustive
pub fn format_user_greeting(user_opt: Option<&UserProfile>) -> String {
    match user_opt {
        Some(user) => format!("Halo, {}! (ID: {})", user.username, user.id),
        None => "User tidak ditemukan dalam sistem!".to_string(),
    }
}

// 4. if let branching
pub fn check_has_email(user: &UserProfile) -> String {
    if let Some(ref email) = user.email {
        format!("Email terdaftar: {}", email)
    } else {
        "Email belum didaftarkan".to_string()
    }
}

// 5. map combinator
pub fn get_uppercase_email(user: &UserProfile) -> Option<String> {
    user.email.as_ref().map(|e| e.to_uppercase())
}

// 6. and_then chaining & flattening
pub fn get_user_postal_code(user_opt: Option<&UserProfile>) -> Option<String> {
    user_opt
        .and_then(|u| u.address.as_ref())
        .and_then(|addr| addr.postal_code.clone())
}

// 7. unwrap_or safe fallback
pub fn get_email_or_default<'a>(user: &'a UserProfile, default_email: &'a str) -> &'a str {
    user.email.as_deref().unwrap_or(default_email)
}
```

##### Hasil Eksekusi Output Terminal:
```text
=== Fase 6 - Task 1: Robust Error Handling — Option<T> ===
Konsep Inti: Eliminasi null pointer exception via enum Option<T> (Some & None)

1. Pembuatan & Pencarian via Option (Some & None):
   - ID 1  : Some("alice")
   - ID 99 : None

2. Ekstraksi Exhaustive via `match`:
   - match ID 1  -> Halo, alice! (ID: 1)
   - match ID 99 -> User tidak ditemukan dalam sistem!

3. Ekstraksi Ringkas via `if let`:
   - Alice   -> Email terdaftar: alice@example.com
   - Charlie -> Email belum didaftarkan

4. Transformasi Nilai via `.map()`:
   - Alice upper email -> Some("ALICE@EXAMPLE.COM")
   - Charlie upper email -> None

5. Rantai Pengecekan Bersarang via `.and_then()` (Flat Map):
   - ID 1 (Lengkap)     postal_code: Some("10110")
   - ID 2 (Tanpa Pos)   postal_code: None
   - ID 3 (Tanpa Alamat)postal_code: None
   - ID 99 (None User)  postal_code: None

6. Fallback Aman via `.unwrap_or()` (Anti-Crash):
   - Bob email     -> bob@rustacean.org
   - Charlie email -> guest@system.local
```

---

### 6.2 Eksekusi Terkendali via `Result<T, E>` & Operator `?` (Error Propagation)

Di kebanyakan bahasa pemrograman (seperti Java, Python, JavaScript, atau C#), error ditangani melalui mekanisme *exceptions* (`try-catch-finally`). Namun, exception memiliki kelemahan mendasar:
1. **Tidak Eksplisit**: Tanda tangan fungsi (*function signature*) seringkali menyembunyikan fakta bahwa fungsi tersebut bisa melempar error tak terduga (*hidden runtime crashes*).
2. **Overhead Tinggi**: Pembuatan stack trace pada exception membebani alokasi CPU dan memori saat runtime.

Rust menolak *exceptions* untuk error yang dapat dipulihkan (*recoverable errors*). Rust mewajibkan setiap operasi yang berpotensi gagal mengembalikan enum `Result<T, E>`.

---

#### 1. Struktur Internal Enum `Result<T, E>`

Didefinisikan di dalam pustaka standar Rust (`std`) dan diimpor otomatis ke *prelude*:

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- **`Ok(T)`**: Menunjukkan operasi sukses dan membungkus data hasil bertipe `T`.
- **`Err(E)`**: Menunjukkan operasi gagal dan membungkus informasi/alasan kegagalan bertipe `E`.

Karena bertipe aljabar tegas, compiler memaksa Anda menangani kemungkinan `Err` sebelum diizinkan menggunakan nilai `T`.

---

#### 2. Penanganan Manual: Exhaustive `match`

Cara paling dasar untuk mengekstrak `Result` adalah dengan pattern matching `match`. Kedua cabang (`Ok` dan `Err`) wajib dicakup:

```rust
match parse_amount("1500.50") {
    Ok(amount) => println!("Nominal valid: Rp{:.2}", amount),
    Err(err_msg) => eprintln!("Error validasi: {}", err_msg),
}
```

---

#### 3. Operator `?` (The Try Operator) & Mekanisme Early-Return

Menulis blok `match` berulang-ulang untuk setiap fungsi yang bisa gagal akan membuat kode menjadi sangat bertele-tele (*callback hell* atau tumpukan indentasi).

Rust menyediakan sintaks ergonomis tingkat tinggi: **Operator `?`**.

```rust
let amount = parse_amount(amount_str)?;
```

##### Cara Kerja Operator `?`:
1. Jika ekspresi bernilai `Ok(nilai)`, operator `?` **otomatis membuka bungkusnya** dan mengembalikan `nilai` tersebut ke variabel.
2. Jika ekspresi bernilai `Err(error)`, operator `?` **seketika menghentikan eksekusi fungsi saat itu juga (*early-return*)** dan langsung mengembalikan `Err(error)` ke fungsi pemanggil (*caller*).

> **Syarat Wajib**: Operator `?` hanya bisa digunakan di dalam fungsi yang tipe return-nya kompatibel (misalnya fungsi tersebut juga mengembalikan `Result<_, E>`).

---

#### 4. Konsep Error Propagation (Perambatan Error Terkendali)

Error propagation adalah pola di mana error tidak langsung ditangani di tempat terjadinya, melainkan diteruskan ke tingkat yang lebih tinggi (*caller*) yang memiliki wewenang untuk mengambil keputusan (misal: menampilkan pesan ke user, mencatat audit log, atau mengembalikan HTTP status 400).

Perhatikan alur perambatan pada pipeline transaksi:
```text
[Input Nominal]  ──> parse_amount()?    ──(Gagal)──> [Return Err Langsung]
                          │ (Sukses)
[Akun Bank]      ──> fetch_account()?   ──(Gagal)──> [Return Err Langsung]
                          │ (Sukses)
[Pemotongan]     ──> debit_account()?   ──(Gagal)──> [Return Err Langsung]
                          │ (Sukses)
                   [Return Ok(Receipt)]
```

Dengan operator `?`, seluruh alur validasi dan pengecekan di atas dapat ditulis bersih secara linear tanpa satupun blok `try-catch` bertumpuk.

---

#### 5. Kode Implementasi Terintegrasi (`src/fase6_task_2.rs`)

Selaras dengan checklist target Fase 6 Task 2:

```rust
// Fase 6 - Task 2: Robust Error Handling — Result<T, E> & Operator ?
#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub id: u64,
    pub owner: String,
    pub balance: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionReceipt {
    pub transaction_id: &'static str,
    pub account_id: u64,
    pub amount: f64,
    pub remaining_balance: f64,
}

// 1 & 2. Pembuatan Ok(T) dan Err(E)
pub fn parse_amount(raw: &str) -> Result<f64, String> {
    let trimmed = raw.trim();
    let amount = match trimmed.parse::<f64>() {
        Ok(val) => val,
        Err(_) => return Err(format!("Nominal '{trimmed}' bukan format angka yang valid")),
    };

    if amount <= 0.0 {
        Err("Nominal transaksi harus bernilai lebih dari 0".to_string())
    } else {
        Ok(amount)
    }
}

pub fn fetch_account(id: u64) -> Result<BankAccount, String> {
    match id {
        101 => Ok(BankAccount { id: 101, owner: "Alice".to_string(), balance: 5000.0 }),
        102 => Ok(BankAccount { id: 102, owner: "Bob".to_string(), balance: 250.0 }),
        unknown_id => Err(format!("Akun dengan ID {unknown_id} tidak ditemukan")),
    }
}

pub fn debit_account(account: &mut BankAccount, amount: f64) -> Result<f64, String> {
    if account.balance < amount {
        Err(format!("Saldo tidak mencukupi! Tersedia: Rp{:.2}, diminta: Rp{:.2}", account.balance, amount))
    } else {
        account.balance -= amount;
        Ok(account.balance)
    }
}

// 3. match exhaustive
pub fn format_amount_inspection(raw: &str) -> String {
    match parse_amount(raw) {
        Ok(amount) => format!("[VALID] Nominal: Rp{:.2}", amount),
        Err(err_msg) => format!("[GAGAL] Validasi gagal: {}", err_msg),
    }
}

// 4 & 5. Operator ? dan Error Propagation
pub fn process_payment(account_id: u64, amount_str: &str) -> Result<TransactionReceipt, String> {
    let amount = parse_amount(amount_str)?;
    let mut account = fetch_account(account_id)?;
    let remaining = debit_account(&mut account, amount)?;

    Ok(TransactionReceipt {
        transaction_id: "TXN-2026-001",
        account_id: account.id,
        amount,
        remaining_balance: remaining,
    })
}
```

##### Hasil Eksekusi Output Terminal:
```text
=== Fase 6 - Task 2: Robust Error Handling — Result<T, E> & Operator ? ===
Konsep Inti: Penanganan kemungkinan gagal secara eksplisit via Ok & Err serta operator ?

1. Validasi Input via `match` (Ok vs Err):
   - [VALID] Nominal: Rp1500.50
   - [GAGAL] Validasi gagal: Nominal transaksi harus bernilai lebih dari 0
   - [GAGAL] Validasi gagal: Nominal 'seribu_rupiah' bukan format angka yang valid

2. Eksekusi Pipeline Pembayaran Sukses (Operator ?):
   [✓] Transaksi Berhasil! ID: TXN-2026-001, Akun: 101, Debet: Rp1200.00, Sisa: Rp3800.00

3. Propagasi Error Tahap 1: Format Nominal Tidak Valid:
   [Propagated Err] Nominal 'abc' bukan format angka yang valid

4. Propagasi Error Tahap 2: Akun Tidak Terdaftar:
   [Propagated Err] Akun dengan ID 999 tidak ditemukan

5. Propagasi Error Tahap 3: Saldo Akun Kurang:
   [Propagated Err] Saldo tidak mencukupi! Tersedia: Rp250.00, diminta: Rp1000.00
```

---

### 6.3 Custom Error Types: Anatomi, Trait Hierarchy, & Error Conversion

Pada kode pemula atau prototipe, seringkali kita tergoda menggunakan string sebagai tipe error (misal `Result<T, String>`). Namun, di level sistem dan backend produksi enterprise, pola tersebut **dihindari** karena:
1. **Tidak Type-Safe**: Caller tidak bisa menggunakan `match` untuk membedakan secara terstruktur apakah error disebabkan oleh *not found*, *invalid input*, atau *kegagalan koneksi*.
2. **Tidak Terintegrasi**: Tipe string biasa tidak memenuhi kontrak `std::error::Error`, sehingga tidak bisa digunakan dengan ekosistem logging, tracing, dan middleware framework.

Pendekatan idiomatik di Rust adalah mendefinisikan **Custom Enum Error**.

---

#### 1. Anatomi Enum `AppError`

Enum mendeskripsikan secara exhaustive semua kemungkinan kegagalan yang dapat terjadi di domain aplikasi:

```rust
use std::io;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),      // Resource yang dicari tidak ada
    InvalidInput(String),  // Validasi parameter gagal
    Io(io::Error),         // Kegagalan I/O dari sistem operasi/file
}
```

---

#### 2. Tiga Trait Fondasi Standar Error

Sebuah tipe kustom dianggap sebagai "Error Resmi" di Rust jika mengimplementasikan 3 trait berikut:

| Trait | Peran & Konsumen | Cara Implementasi |
| :--- | :--- | :--- |
| **`std::fmt::Debug`** | Untuk keperluan logging internal pengembang (`tracing`, `log::error!`, atau format `{:?}`). | Cukup pasang `#[derive(Debug)]`. |
| **`std::fmt::Display`** | Untuk pesan yang disajikan ke pengguna akhir (*user-facing*) atau response HTTP body. | Tulis manual via `impl fmt::Display for AppError`. |
| **`std::error::Error`** | Kontrak standar pustaka Rust. Menyediakan metode `.source()` untuk menelusuri rantai akar penyebab error (*error causal chain*). | `impl std::error::Error for AppError`. |

```rust
// Implementasi Display
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Data Tidak Ditemukan: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Input Tidak Valid: {msg}"),
            AppError::Io(err) => write!(f, "Kesalahan I/O Sistem: {err}"),
        }
    }
}

// Implementasi std::error::Error dengan causal chain (source)
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err), // Io menyimpan error asli dari sistem operasi
            _ => None,
        }
    }
}
```

---

#### 3. Konversi Error Otomatis via Trait `From<T>`

Ketika fungsi kita memanggil fungsi pustaka standar yang menghasilkan tipe error berbeda (misal `std::io::Error`), operator `?` **tidak akan bisa bekerja** jika tipe error tersebut tidak kompatibel dengan return function kita.

Rust menyelesaikan masalah ini secara elegan: **Operator `?` di belakang layar memanggil `From::from(err)`**.

Dengan mengimplementasikan trait `From<io::Error>` untuk `AppError`:
```rust
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}
```
Maka kode pembacaan file:
```rust
let content = std::fs::read_to_string(path)?;
```
Akan **secara otomatis dikonversi** dari `std::io::Error` menjadi `AppError::Io(...)` tanpa perlu konversi manual yang bertele-tele!

---

#### 4. Kode Implementasi Terintegrasi (`src/fase6_task_3.rs`)

Selaras dengan checklist target Fase 6 Task 3:

```rust
// Fase 6 - Task 3: Robust Error Handling — Custom Error
use std::fmt;
use std::fs;
use std::io;

// 1. Custom Enum Error
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    Io(io::Error),
}

// 2. Trait Display
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Data Tidak Ditemukan: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Input Tidak Valid: {msg}"),
            AppError::Io(err) => write!(f, "Kesalahan I/O Sistem: {err}"),
        }
    }
}

// 3. Trait std::error::Error & source chain
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

// 4. Konversi Error via From
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

// 5. Propagation dengan ?
pub fn validate_config_filename(filename: &str) -> Result<&str, AppError> {
    let trimmed = filename.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("Nama file konfigurasi tidak boleh kosong".to_string()));
    }
    if !trimmed.ends_with(".conf") && !trimmed.ends_with(".json") {
        return Err(AppError::InvalidInput(format!(
            "Ekstensi file '{trimmed}' tidak didukung (harus .conf atau .json)"
        )));
    }
    Ok(trimmed)
}

pub fn read_config_entry(path: &str, target_key: &str) -> Result<String, AppError> {
    let valid_path = validate_config_filename(path)?;
    let content = fs::read_to_string(valid_path)?; // Otomatis terkonversi io::Error -> AppError

    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == target_key {
                return Ok(v.trim().to_string());
            }
        }
    }

    Err(AppError::NotFound(format!("Key '{target_key}' tidak ditemukan di '{valid_path}'")))
}
```

##### Hasil Eksekusi Output Terminal:
```text
=== Fase 6 - Task 3: Robust Error Handling — Custom Error ===
Konsep Inti: Custom enum error dengan trait Debug, Display, Error, From, dan ?

1. Kasus Sukses (Valid Input, Valid I/O, Key Ditemukan):
   [✓] Nilai config ditemukan: database_url = postgres://localhost:5432/app

2. Kasus Gagal 1: Validasi Input (AppError::InvalidInput):
   - Display : Input Tidak Valid: Ekstensi file 'invalid_format.txt' tidak didukung (harus .conf atau .json)
   - Debug   : InvalidInput("Ekstensi file 'invalid_format.txt' tidak didukung (harus .conf atau .json)")

3. Kasus Gagal 2: I/O File Hilang (AppError::Io via `?` & `From`):
   - Display : Kesalahan I/O Sistem: No such file or directory (os error 2)
   - Debug   : Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
   - Source  : No such file or directory (os error 2)

4. Kasus Gagal 3: Key Tidak Ada (AppError::NotFound):
   - Display : Data Tidak Ditemukan: Key 'secret_api_key' tidak ditemukan di '/tmp/app_fase6_test.conf'
   - Debug   : NotFound("Key 'secret_api_key' tidak ditemukan di '/tmp/app_fase6_test.conf'")
```

---

### 6.4 Collections Tingkat Lanjut: `Vec<T>` Internals & `HashMap` Entry API

Koleksi data (*collections*) di Rust dialokasikan secara dinamis di memori heap. Memahami cara kerja memori dan API yang aman sangat penting untuk mencegah degradasi performa dan fatal panic di backend.

---

#### 1. `Vec<T>`: Anatomi Memori, Kapasitas, & Safe Access

Sebuah `Vec<T>` di stack terdiri dari 3 kata mesin (*three words of memory*):
1. **Pointer**: Alamat memori heap tempat elemen pertama disimpan.
2. **Length (`len`)**: Jumlah elemen yang saat ini aktif di dalam vector.
3. **Capacity (`capacity`)**: Jumlah alokasi ruang maksimum di heap sebelum vector dipaksa melakukan *re-alokasi*.

```text
Stack Buffer:
[ Pointer (8B) ] ──> Heap Buffer: [ Item 1 | Item 2 | Item 3 | (Kosong) | (Kosong) ]
[ Length: 3    ]                  |─────── len: 3 ──────────|
[ Capacity: 5  ]                  |─────────────── capacity: 5 ────────────────────|
```

##### A. Optimasi Performa via `Vec::with_capacity(n)`
Jika Anda membuat vector kosong biasa (`Vec::new()`) dan menambahkan 10.000 elemen via `.push()`, Rust akan melakukan re-alokasi berulang kali: mengalokasikan buffer baru 2x lebih besar, menyalin semua data lama, dan mendealokasikan buffer lama.

Dengan `Vec::with_capacity(10_000)`, buffer heap dialokasikan **hanya 1 kali** di awal, menghemat siklus CPU dan mencegah fragmentasi memori.

```rust
let mut inventory = Vec::with_capacity(100);
inventory.push(product);
```

##### B. Akses Index Aman: `.get(index)` vs `vec[index]`
- `vec[index]`: Jika `index >= len`, program langsung mengalami **fatal panic (crash runtime)**.
- `vec.get(index)`: Mengembalikan `Option<&T>` (`Some(&item)` jika ada, `None` jika di luar batas). Tidak akan pernah crash!

```rust
// Aman: mengembalikan None jika index 99 tidak ada
let item = inventory.get(99); 
```

##### C. In-Place Filtering via `.retain(predicate)`
Alih-alih membuat vector baru via `.filter().collect()`, metode `.retain()` menyaring elemen langsung di tempat (*in-place mutation*), mempertahankan urutan, dan membuang elemen yang tidak lolos tanpa satu pun alokasi heap baru:

```rust
// Membuang semua produk yang stoknya 0 secara in-place
inventory.retain(|p| p.stock > 0);
```

---

#### 2. `HashMap<K, V>` & Ergonomi Entry API

`HashMap` bawaan Rust menggunakan algoritma **SipHash 1-3** untuk melindungi aplikasi web dari serangan *HashDoS* (Denial of Service via collision).

##### Masalah Anti-Pattern: Double Hash Lookup
Pemula dari bahasa lain sering menulis kode pencatatan frekuensi seperti ini:
```rust
// ❌ ANTI-PATTERN: Menghitung hash 2x untuk key yang sama
if map.contains_key(&key) {
    *map.get_mut(&key).unwrap() += 1; // Lookup ke-2
} else {
    map.insert(key, 1);               // Lookup ke-2
}
```

##### Solusi Idiomatik: Entry API (Single Lookup / O(1))
Entry API memungkinkan Anda menghitung hash **hanya satu kali**, lalu mengambil keputusan apakah key tersebut sudah ada (*Occupied*) atau belum (*Vacant*):

- **`.entry(key)`**: Mengembalikan enum `Entry` yang menunjuk ke slot hash bucket.
- **`.and_modify(|val| ...)`**: Menjalankan closure untuk memodifikasi nilai jika key sudah ada.
- **`.or_insert(default)`**: Memasukkan nilai default jika key belum ada, lalu mengembalikan `&mut V`.

```rust
// ✅ IDIOMATIK: Single hash lookup
map.entry(category)
    .and_modify(|total_stock| *total_stock += stock)
    .or_insert(stock);
```

---

#### 3. Kode Implementasi Terintegrasi (`src/fase6_task_4.rs`)

Selaras dengan checklist target Fase 6 Task 4:

```rust
// Fase 6 - Task 4: Collections — Vec<T> & HashMap Entry API
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub stock: u32,
    pub price: f64,
}

// 1. Vec::with_capacity & push
pub fn init_inventory_with_capacity(capacity: usize) -> Vec<Product> {
    Vec::with_capacity(capacity)
}

pub fn add_product(inventory: &mut Vec<Product>, product: Product) {
    inventory.push(product);
}

// 2. Safe get access
pub fn get_product_safely(inventory: &[Product], index: usize) -> Option<&Product> {
    inventory.get(index)
}

// 3. In-place retain
pub fn filter_in_stock_only(inventory: &mut Vec<Product>) {
    inventory.retain(|p| p.stock > 0);
}

// 4 & 5. HashMap Entry API: entry, and_modify, or_insert
pub fn calculate_stock_by_category(inventory: &[Product]) -> HashMap<String, u32> {
    let mut category_map: HashMap<String, u32> = HashMap::new();

    for product in inventory {
        category_map
            .entry(product.category.clone())
            .and_modify(|total| *total += product.stock)
            .or_insert(product.stock);
    }

    category_map
}

pub fn count_log_events(events: &[&str]) -> HashMap<String, usize> {
    let mut stats: HashMap<String, usize> = HashMap::new();

    for &event in events {
        stats
            .entry(event.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    stats
}
```

##### Hasil Eksekusi Output Terminal:
```text
=== Fase 6 - Task 4: Collections — Vec<T> & HashMap Entry API ===
Konsep Inti: Alokasi efisien with_capacity, safe access get, retain in-place, & Entry API

1. Demonstrasi Vec::with_capacity & push:
   - Initial State: len = 0, capacity = 5
   - After 5 Push : len = 5, capacity = 5

2. Akses Aman Index via .get():
   - Index 0  : Some("Mechanical Keyboard")
   - Index 99 : None

3. In-Place Filtering via .retain() (Membuang stock == 0):
   - Jumlah produk sebelum retain: 5
   - Jumlah produk setelah retain : 3
     * [ID: 1] Mechanical Keyboard (Stock: 15)
     * [ID: 3] Ergonomic Chair (Stock: 5)
     * [ID: 5] USB-C Hub (Stock: 25)

4. Agregasi Data via HashMap Entry API (.entry().and_modify().or_insert()):
   - Kategori 'Furniture': Total Stok = 5 unit
   - Kategori 'Electronics': Total Stok = 40 unit

5. Frekuensi Event Log via Entry API:
   - Event 'CLICK': 1 kali
   - Event 'LOGOUT': 1 kali
   - Event 'VIEW': 2 kali
   - Event 'LOGIN': 3 kali
```

---

### 6.5 Mini Project: CLI Task Manager v1 & Evaluasi Kelulusan Fase 6

Mini project ini menyatukan seluruh pilar Fase 6 ke dalam sebuah arsitektur aplikasi manajemen tugas (*Task Manager*) yang aman, efisien, dan bebas fatal crash (*zero unwrap*).

---

#### 1. Evaluasi Kriteria Kelulusan Fase 6

##### A. Kapan Menggunakan `Option<T>` vs `Result<T, E>`?
- **Gunakan `Option<T>`**: Ketika ketiadaan nilai adalah hal yang wajar (*normal absence of data*) dan caller tidak memerlukan alasan mengapa data itu kosong.
  - *Contoh*: Field `description` pada task (bisa `Some("detail")` atau `None`), lookup user di cache (jika miss, tinggal fetch ke DB).
- **Gunakan `Result<T, E>`**: Ketika sebuah operasi berpotensi gagal (*recoverable failure*) dan caller **wajib mengetahui alasan kegagalan** untuk mengambil tindakan korektif.
  - *Contoh*: Parsing ID dari input user (`InvalidId`), mencari task ID yang tidak terdaftar (`TaskNotFound`), atau operasi file (`Io`).

##### B. Disiplin *Zero `unwrap()`* pada Jalur Input
Pemanggilan `.unwrap()` pada data mentah dari pengguna adalah bom waktu (*runtime panic*). Di level produksi, semua parsing input dipetakan menggunakan `Result`, `map_err`, dan operator `?`:

```rust
// ✅ AMAN: Mengubah ParseIntError menjadi Domain Error tanpa crash
let parsed_id = raw_id
    .trim()
    .parse::<u32>()
    .map_err(|_| TaskManagerError::InvalidId(raw_id.to_string()))?;
```

##### C. HashMap Entry API untuk Analisis Domain
Menghitung agregasi data kategori task tanpa double lookup menggunakan pola idiomatik `.entry().and_modify().or_insert()`.

---

#### 2. Kode Implementasi Terintegrasi (`src/mini_project_6.rs`)

```rust
// Mini Project Fase 6: CLI Task Manager v1 & Error Handling Integration
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskManagerError {
    TaskNotFound(u32),
    EmptyTitle,
    InvalidId(String),
}

impl fmt::Display for TaskManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskManagerError::TaskNotFound(id) => write!(f, "Task dengan ID #{id} tidak ditemukan"),
            TaskManagerError::EmptyTitle => write!(f, "Judul task tidak boleh kosong"),
            TaskManagerError::InvalidId(raw) => {
                write!(f, "ID task '{raw}' tidak valid (harus berupa bilangan bulat positif)")
            }
        }
    }
}

impl std::error::Error for TaskManagerError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskItem {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub description: Option<String>,
}

pub struct TaskManager {
    tasks: Vec<TaskItem>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(16),
            next_id: 1,
        }
    }

    // 1. Add Task
    pub fn add_task(&mut self, title: &str, category: &str, description: Option<&str>) -> Result<u32, TaskManagerError> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err(TaskManagerError::EmptyTitle);
        }

        let id = self.next_id;
        self.next_id += 1;

        self.tasks.push(TaskItem {
            id,
            title: trimmed_title.to_string(),
            category: if category.trim().is_empty() { "General".to_string() } else { category.trim().to_string() },
            description: description.map(|d| d.trim().to_string()),
        });

        Ok(id)
    }

    // 2. List Tasks
    pub fn list_tasks(&self) -> &[TaskItem] {
        &self.tasks
    }

    // 3. Find Task
    pub fn find_task(&self, id: u32) -> Result<&TaskItem, TaskManagerError> {
        self.tasks.iter().find(|t| t.id == id).ok_or(TaskManagerError::TaskNotFound(id))
    }

    // 4. Delete Task
    pub fn delete_task(&mut self, id: u32) -> Result<TaskItem, TaskManagerError> {
        let index = self.tasks.iter().position(|t| t.id == id).ok_or(TaskManagerError::TaskNotFound(id))?;
        Ok(self.tasks.remove(index))
    }

    // 5. Entry API Kategori
    pub fn get_category_stats(&self) -> HashMap<String, usize> {
        let mut stats: HashMap<String, usize> = HashMap::new();
        for task in &self.tasks {
            stats.entry(task.category.clone()).and_modify(|count| *count += 1).or_insert(1);
        }
        stats
    }

    // 6. Safe Parsing Input Utama (Bebas unwrap)
    pub fn parse_and_find(&self, raw_id: &str) -> Result<&TaskItem, TaskManagerError> {
        let parsed_id = raw_id
            .trim()
            .parse::<u32>()
            .map_err(|_| TaskManagerError::InvalidId(raw_id.trim().to_string()))?;
        self.find_task(parsed_id)
    }
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Mini Project Fase 6: CLI Task Manager v1 & Error Safe ===
============================================================

1. Menambahkan Task (dengan Option description):
   [+] Task #1 dibuat
   [+] Task #2 dibuat
   [+] Task #3 dibuat
   [+] Task #4 dibuat

2. Daftar Task Aktif Saat Ini:
   - [#1 ] [Backend   ] Implementasi JWT Authentication  -> Gunakan RS256 algorithm dan refresh token rotation
   - [#2 ] [Database  ] Desain Database Schema           -> (Tanpa deskripsi)
   - [#3 ] [Testing   ] Buat Unit Test Axum Handlers     -> (Tanpa deskripsi)
   - [#4 ] [DevOps    ] Setup CI/CD Pipeline             -> GitHub Actions workflow

3. Pencarian Task (Find Task):
   [✓] Ditemukan: [#1] Implementasi JWT Authentication
   [✓ Diharapkan] Error ketika task tidak ditemukan: 'Task dengan ID #999 tidak ditemukan'

4. Proteksi Jalur Input Utama (Bebas unwrap!):
   - Input '2' -> Ditemukan: 'Desain Database Schema'
   - Input 'bukan_angka' -> Ditangani Aman: 'ID task 'bukan_angka' tidak valid (harus berupa bilangan bulat positif)'
   - Input '888' -> Ditangani Aman: 'Task dengan ID #888 tidak ditemukan'

5. Menghapus Task (Delete Task):
   [✓] Berhasil menghapus task #2: 'Desain Database Schema'
   [✓ Diharapkan] Menghapus task yang sudah terhapus: 'Task dengan ID #2 tidak ditemukan'

6. Statistik Kategori via HashMap Entry API:
   - Kategori 'DevOps': 1 task aktif
   - Kategori 'Backend': 1 task aktif
   - Kategori 'Testing': 1 task aktif

7. Eksekusi Perintah Parser CLI (ADD / FIND / DEL):
   - Command ADD  -> Ok("Task #5 ('RefactorCode') berhasil ditambahkan")
   - Command FIND -> Ok("Ditemukan: [#5] RefactorCode (Backend)")
   - Command DEL  -> Ok("Task #5 ('RefactorCode') berhasil dihapus")

8. Evaluasi Kriteria Lulus Fase 6:
   [x] Kapan Option vs Result: Option untuk ketiadaan nilai wajar; Result untuk operasi yang bisa gagal.
   [x] Pemakaian Operator ?: Propagasi error otomatis di parse_and_find dan execute_command.
   [x] Custom Error: TaskManagerError dengan trait Display, Debug, dan std::error::Error.
   [x] HashMap Entry API: Digunakan pada get_category_stats() dengan entry(), and_modify(), or_insert().
   [x] Zero unwrap() pada input: Semua input parsing dipetakan aman via Result & map_err.
```

---

## FASE 7: Generics, Traits, & Advanced Trait System

Trait dan Generics adalah pondasi abstraksi dan polimorfisme di Rust tanpa mengandalkan konsep pewarisan (*inheritance*).

### 7.1 Generic Functions & Trait Bounds (Task 1)

Generics memungkinkan penulisan algoritma yang fleksibel dan reusable untuk berbagai tipe data tanpa mengorbankan performa ataupun *type safety*.

#### 1. Masalah: Duplikasi Kode vs Abstraksi Generic
Tanpa generics, fungsi pencarian nilai terbesar membutuhkan definisi terpisah untuk setiap tipe data (`largest_i32`, `largest_f64`, `largest_char`). Dengan generic type parameter `<T>`, satu fungsi berlaku untuk semua tipe yang valid.

#### 2. Konsep Monomorphization (Zero-Cost Abstractions)
Rust tidak menggunakan runtime reflection atau dynamic boxing untuk generics standar. Saat proses kompilasi (`cargo build`), compiler melakukan proses **Monomorphization**, yaitu menduplikasi kode mesin spesifik untuk setiap tipe konkret yang digunakan (`find_largest` untuk `i32`, `find_largest` untuk `f64`, dst.). 
Dampaknya: **Performa eksekusi setara dengan kode yang ditulis khusus secara manual tanpa biaya abstraksi saat runtime (*Zero-Cost Abstraction*)!**

#### 3. Trait Bounds & Klausa `where`
Secara default, tipe generic `<T>` tidak memiliki kapabilitas bawaan apapun. Untuk melakukan operasi tertentu pada `T`, kita wajib membatasinya dengan **Trait Bounds**:
- `T: PartialOrd`: Mengizinkan operator perbandingan relasional (`>`, `<`, `>=`, `<=`).
- `T: std::fmt::Display`: Mengizinkan pemformatan teks ramah pengguna via `{}`.
- `T: std::fmt::Debug`: Mengizinkan inspeksi debugging via `{:?}`.
- **Klausa `where`**: Memindahkan deklarasi batasan kompleks ke baris bawah signature fungsi untuk keterbacaan kode yang bersih saat terdapat banyak parameter tipe atau batasan closure bertingkat.

---

#### 4. Bedah Implementasi Tiga Target Task 1

##### A. Memilih Nilai Terbesar: `find_largest<T: PartialOrd>(list: &[T]) -> Option<&T>`
- Menerima slice referensi `&[T]` agar tidak mengambil kepemilikan (*zero copy*).
- Menghindari fatal crash runtime: jika slice kosong (`list.is_empty()`), fungsi mengembalikan `None`.
- Menggunakan trait bound `PartialOrd` sehingga kompatibel dengan integer, float, string (`&str`), maupun struct kustom:

```rust
pub fn find_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}
```

##### B. Mencetak Nilai: `print_item`, `print_collection`, & `print_debug_item`
- `T: Display` digunakan untuk pencetakan langsung `{}`.
- `T: Debug` digunakan untuk inspeksi struktural `{:?}`.
- `print_collection` menerima slice koleksi `&[T]` dan mencetak setiap elemen secara rapi berurutan.

```rust
pub fn print_item<T: Display>(label: &str, item: &T) {
    println!("{label}: {item}");
}

pub fn print_collection<T: Display>(header: &str, items: &[T]) {
    println!("{header}:");
    for (idx, item) in items.iter().enumerate() {
        println!("  [{idx}] {item}");
    }
}
```

##### C. Mengubah Collection: Pemetaan & Mutasi In-Place
1. **Transformasi Pemetaan (`Vec<T>` -> `Vec<U>`)**:
   Menerima fungsi/closure `F: FnMut(T) -> U` untuk memetakan elemen tipe `T` ke tipe baru `U` (misal angka ke teks berformat). Alokasi memori dioptimalkan dengan `Vec::with_capacity(items.len())`.
2. **Transformasi Mutasi In-Place (`&mut [T]`)**:
   Memodifikasi elemen slice referensi secara langsung di tempat (*in-place*) tanpa alokasi heap baru, menggunakan closure `F: FnMut(&mut T)`.

```rust
pub fn transform_collection<T, U, F>(items: Vec<T>, mut transform_fn: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        result.push(transform_fn(item));
    }
    result
}

pub fn transform_collection_mut<T, F>(items: &mut [T], mut transform_fn: F)
where
    F: FnMut(&mut T),
{
    for item in items.iter_mut() {
        transform_fn(item);
    }
}
```

---

#### 5. Kode Lengkap Terintegrasi (`src/fase7_task_1.rs`)

```rust
use std::fmt::{Debug, Display};

pub fn find_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

pub fn print_item<T: Display>(label: &str, item: &T) {
    println!("{label}: {item}");
}

pub fn print_debug_item<T: Debug>(label: &str, item: &T) {
    println!("{label} (Debug): {item:?}");
}

pub fn print_collection<T: Display>(header: &str, items: &[T]) {
    println!("{header}:");
    if items.is_empty() {
        println!("  (koleksi kosong)");
        return;
    }
    for (idx, item) in items.iter().enumerate() {
        println!("  [{idx}] {item}");
    }
}

pub fn transform_collection<T, U, F>(items: Vec<T>, mut transform_fn: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        result.push(transform_fn(item));
    }
    result
}

pub fn transform_collection_mut<T, F>(items: &mut [T], mut transform_fn: F)
where
    F: FnMut(&mut T),
{
    for item in items.iter_mut() {
        transform_fn(item);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerScore {
    pub name: String,
    pub score: u32,
}

impl PartialOrd for PlayerScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player '{}' [Score: {}]", self.name, self.score)
    }
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Fase 7 - Task 1: Generic Functions & Trait Bounds    ===
============================================================

1. Memilih Nilai Terbesar (find_largest):
   - Integer terbesar dari [42, 108, 17, 99, 5]: 108
   - Float terbesar dari [3.14, 2.71, 9.81, 1.41]: 9.81
   - Word terbesar secara alfabetis dari ["rust", "generics", "monomorphization", "traits"]: "traits"
   - Skor tertinggi (Custom Struct): Player 'Bob' [Score: 550]
   - Slice kosong: None (Aman, zero panic)

2. Mencetak Nilai (print_item & print_collection):
   - Single Item (i32): 1337
   - Single Item (String): Rust Zero-Cost Abstractions
   - Debug Item (Debug): ["A", "B", "C"]
   - Daftar Kota:
  [0] Jakarta
  [1] Bandung
  [2] Surabaya
  [3] Yogyakarta

3. Mengubah Collection (transform_collection & transform_collection_mut):
   - Transformasi Pemetaan (Vec<i32> -> Vec<String>):
     * Item #1 (Kuadrat: 1)
     * Item #2 (Kuadrat: 4)
     * Item #3 (Kuadrat: 9)
     * Item #4 (Kuadrat: 16)
     * Item #5 (Kuadrat: 25)
   - In-place mutation sebelum: [10, 20, 30, 40]
   - In-place mutation setelah (x2): [20, 40, 60, 80]
```

---

### 7.2 Traits, Default Implementation & Trait Bounds (Task 2)

#### 1. Mental Model: Apa itu Trait?

Di bahasa pemrograman seperti Java, PHP, atau TypeScript, kita terbiasa dengan konsep **OOP Inheritance** (Class `Hewan`, lalu di-*extends* jadi `Kucing`).

Rust **tidak memiliki Class maupun pewarisan (inheritance)**. Sebagai gantinya, Rust menggunakan **Trait**.

> **Analogi Trait = "Sertifikat Kemampuan" atau "Kontrak Kerja".**

Bayangkan kita punya sertifikat: **`BisaDiringkas` (Trait `Summary`)**.
Sertifikat ini mensyaratkan satu hal:
> *"Siapa pun tipe data yang memegang sertifikat ini, wajib punya kemampuan untuk menghasilkan teks ringkasan lewat fungsi `summarize()`."*

Bentuk datanya boleh apa saja dan sangat berbeda di memori:
- `NewsArticle` punya `headline`, `location`, `author`, `content`.
- `Tweet` punya `username`, `content`, `reply`, `retweet`.

Keduanya adalah entitas data yang berbeda total, tetapi **keduanya sama-sama memegang sertifikat `Summary`**.

```text
┌───────────────────────┐         ┌───────────────────────┐
│  Struct: NewsArticle  │         │     Struct: Tweet     │
│  - headline           │         │  - username           │
│  - author             │         │  - content            │
└───────────┬───────────┘         └───────────┬───────────┘
            │                                 │
            ▼                                 ▼
   Keduanya sama-sama mengimplementasikan Trait:
   ┌─────────────────────────────────────────────────────┐
   │                    Trait: Summary                   │
   │  fn summarize(&self) -> String                      │
   └─────────────────────────────────────────────────────┘
```

---

#### 2. Anatomi Trait & Default Implementation
Sebuah trait dapat menyediakan implementasi default untuk salah satu atau semua method-nya. Ini seperti **template bawaan pabrik**:
- **Kasus 1: Meng-override (Menimpa) Default Implementation**:
  Pada `NewsArticle`, artikel berita ingin formatnya formal dan detail (headline + author + location), sehingga method default `summarize()` ditimpa dengan logika kustom.
- **Kasus 2: Memanfaatkan Default Implementation (Tanpa Tulis Ulang)**:
  Pada `Tweet`, cuitan Twitter hanya perlu memberitahu siapa nama usernya via `summarize_author()`, sedangkan pemanggilan `summarize()` otomatis mewarisi template bawaan: `"(Baca selengkapnya dari @username...)"`.
- **Kasus 3: Full Default Implementation**:
  Pada `CommunityNotice`, struct langsung mengosongkan blok `impl Summary for CommunityNotice {}` untuk mewarisi seluruh method bawaan tanpa ubahan sedikit pun.

```rust
pub trait Summary {
    // 1. Method pembantu (punya nilai bawaan "Kontributor Anonim")
    fn summarize_author(&self) -> String {
        String::from("Kontributor Anonim")
    }

    // 2. Default Implementation: Template kalimat standar
    fn summarize(&self) -> String {
        format!("(Baca selengkapnya dari {}...)", self.summarize_author())
    }
}
```

---

#### 3. Mengapa Perlu "Trait Bounds"?

Misalkan kita ingin membuat fungsi generic `notify`:

##### Mengapa kode berikut DITOLAK oleh compiler?
```rust
// ❌ ERROR COMPILER:
fn notify<T>(item: &T) {
    println!("Pemberitahuan: {}", item.summarize());
}
```
**Alasan Compiler:**
`T` bisa berupa apa saja—bisa integer `42`, boolean `true`, atau struct kosong. Angka `42` tidak memiliki method `.summarize()`. Jika dibiarkan lolos, program akan *crash runtime*!

##### Solusinya: Pasang Satpam Pembatas ("Trait Bound")
Kita wajib memberi tahu compiler:
> *"Fungsi ini generic untuk tipe `T`, TAPI hanya boleh menerima tipe `T` yang sudah mengimplementasikan trait `Summary`!"*

```rust
// ✅ VALID (Ada Trait Bound <T: Summary>):
pub fn notify<T: Summary>(item: &T) -> String {
    format!("Pemberitahuan Terkini: {}", item.summarize())
}
```

Compiler menjamin keamanan mutlak:
- `notify(&article)` -> ✅ **Diterima**, karena `NewsArticle` punya sertifikasi `Summary`.
- `notify(&tweet)` -> ✅ **Diterima**, karena `Tweet` punya sertifikasi `Summary`.
- `notify(&42)` -> ❌ **Ditolak saat kompilasi** (Zero runtime bugs).

---

#### 4. Ragam Gaya Menulis Trait Bounds

| Gaya Sintaks | Contoh Kode | Kapan Dipakai? |
| :--- | :--- | :--- |
| **1. Standard Trait Bound** | `fn notify<T: Summary>(item: &T)` | Paling umum, cocok jika parameter generic harus bertipe sama persis. |
| **2. Multiple Bounds (`+`)** | `fn notify_verbose<T: Summary + Display>(item: &T)` | Saat tipe membutuhkan lebih dari 1 trait sekaligus (bisa diringkas `Summary` DAN bisa dicetak via `Display`). |
| **3. Klausa `where`** | `fn notify_where<T>(item: &T) where T: Summary` | Dipakai saat batasannya panjang/kompleks agar signature fungsi tetap rapi dan mudah dibaca. |
| **4. `impl Trait` (Syntactic Sugar)** | `fn notify_impl(item: &impl Summary)` | Paling ringkas & ergonomis untuk fungsi sederhana. |


---

#### 5. Kode Lengkap Terintegrasi (`src/fase7_task_2.rs`)

```rust
use std::fmt::{self, Display, Formatter};

pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Kontributor Anonim")
    }

    fn summarize(&self) -> String {
        format!("(Baca selengkapnya dari {}...)", self.summarize_author())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, oleh {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[BERITA] \"{}\" - {}", self.headline, self.author)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[TWEET] @{}: \"{}\"", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) -> String {
    format!("Pemberitahuan Terkini: {}", item.summarize())
}

pub fn notify_verbose<T: Summary + Display>(item: &T) -> String {
    format!("Notifikasi Lengkap: {}\n   -> Display: {}", item.summarize(), item)
}

pub fn notify_where<T>(item: &T) -> String
where
    T: Summary,
{
    format!("Pemberitahuan (via where clause): {}", item.summarize())
}

pub fn notify_impl(item: &impl Summary) -> String {
    format!("Pemberitahuan (via impl Trait): {}", item.summarize())
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Fase 7 - Task 2: Traits & Trait Bounds               ===
============================================================

1. Eksekusi Trait Method:
   - NewsArticle (Overridden summarize) : Rust 2024 Edition Resmi Dirilis, oleh Tech Wire (San Francisco)
   - Tweet (Default summarize)           : (Baca selengkapnya dari @rustacean_id...)
   - CommunityNotice (Full Default)     : (Baca selengkapnya dari Kontributor Anonim...)

2. Pemanggilan Fungsi dengan Trait Bounds:
   a. notify<T: Summary>(&article):
      Pemberitahuan Terkini: Rust 2024 Edition Resmi Dirilis, oleh Tech Wire (San Francisco)
   b. notify<T: Summary>(&tweet):
      Pemberitahuan Terkini: (Baca selengkapnya dari @rustacean_id...)

3. Multiple Trait Bounds (<T: Summary + Display>):
   Notifikasi Lengkap: Rust 2024 Edition Resmi Dirilis, oleh Tech Wire (San Francisco)
   -> Display: [BERITA] "Rust 2024 Edition Resmi Dirilis" - Tech Wire
   Notifikasi Lengkap: (Baca selengkapnya dari @rustacean_id...)
   -> Display: [TWEET] @rustacean_id: "Belajar trait di Rust sangat menyenangkan dan type-safe!"

4. Sintaks Alternatif Trait Bounds:
   - where clause : Pemberitahuan (via where clause): Rust 2024 Edition Resmi Dirilis, oleh Tech Wire (San Francisco)
   - impl Trait   : Pemberitahuan (via impl Trait): (Baca selengkapnya dari @rustacean_id...)
```

---

### 7.3 Static Dispatch vs Dynamic Dispatch (Task 3)

Polimorfisme di Rust dapat diselesaikan melalui dua cara yang berbeda secara fundamental: **Static Dispatch** pada waktu kompilasi (*compile-time*) atau **Dynamic Dispatch** pada saat runtime (*run-time*).

```rust
// Dua Versi Pemrosesan Dispatch:
fn process_static<T: Summary>(item: &T);    // Static Dispatch (Generics & Monomorphization)
fn process_dynamic(item: &dyn Summary);     // Dynamic Dispatch (Trait Object & vtable)
```

---

#### 1. Perbandingan Karakteristik Head-to-Head

| Karakteristik | Static Dispatch (`T: Trait`) | Dynamic Dispatch (`&dyn Trait` / `Box<dyn Trait>`) |
| :--- | :--- | :--- |
| **Waktu Resolusi** | Saat kompilasi (*Compile-time*) | Saat program berjalan (*Runtime*) |
| **Mekanisme** | **Monomorphization**: Compiler membuat salinan fungsi biner khusus per tipe konkret | **Trait Object**: Menggunakan pointer ke tabel fungsi virtual (*vtable*) |
| **Overhead Performa** | **Nol (Zero-Cost Abstraction)**; instruksi langsung ke alamat fungsi | Ada sedikit overhead dereferensi pointer vtable (*pointer indirection*) |
| **Optimasi Compiler** | **Bisa di-inline** oleh LLVM untuk kecepatan maksimal | **Tidak bisa di-inline** karena target fungsi ditentukan saat runtime |
| **Koleksi Heterogen** | ❌ **Tidak bisa**: `Vec<T>` hanya dapat menampung satu tipe seragam | ✅ **Bisa**: `Vec<Box<dyn Trait>>` bisa menampung campuran berbagai struct |
| **Ukuran Pointer** | **Thin Pointer** (1 kata mesin: 8 bytes pada arsitektur 64-bit) | **Fat Pointer** (2 kata mesin: 16 bytes = 8B data ptr + 8B vtable ptr) |
| **Ukuran Binary** | Berpotensi membesar jika tipe sangat banyak (*Code Bloat*) | Ukuran biner lebih ramping karena hanya ada satu fungsi generik |

---

#### 2. Bedah Arsitektur Memori: Thin Pointer vs Fat Pointer

Mengapa pointer trait object (`&dyn Summary` atau `Box<dyn Summary>`) berukuran **16 bytes** sedangkan referensi biasa (`&NewsArticle`) hanya berukuran **8 bytes**?

```text
1. Thin Pointer (&NewsArticle - 8 Bytes):
   ┌────────────────────────┐
   │ Pointer Data (8 Bytes) ├────────► [ Data Struct NewsArticle di Memory ]
   └────────────────────────┘

2. Fat Pointer (&dyn Summary - 16 Bytes):
   ┌────────────────────────┐
   │ Pointer Data (8 Bytes) ├────────► [ Data Struct di Memory (NewsArticle / Tweet) ]
   ├────────────────────────┤
   │ Pointer Vtable (8 Bytes)├───────► [ Virtual Method Table (vtable) ]
   └────────────────────────┘          ├── Pointer ke method summarize()
                                       ├── Pointer ke method drop()
                                       ├── Ukuran tipe (size)
                                       └── Alignment memori (align)
```

Karena compiler tidak tahu struct apa yang berada di balik `&dyn Summary` saat runtime, Rust menyertakan **Fat Pointer** yang membawa alamat tabel metode (*vtable*) agar program tahu persis fungsi mana yang harus dipanggil.

---

#### 3. Kapan Menggunakan `Box<dyn Trait>`?

Secara default, tipe trait object `dyn Summary` adalah tipe **DST (Dynamically Sized Type)** yang ukurannya tidak diketahui secara pasti saat kompilasi. Oleh karena itu, kita tidak bisa menyimpan `dyn Summary` secara langsung di stack.

Kita wajib membungkusnya di balik pointer berukuran pasti:
- Sebagai referensi borrow: `&dyn Summary` (16 bytes di stack).
- Sebagai kepemilikan heap: `Box<dyn Summary>` (16 bytes pointer di stack, alokasi data di heap).

##### Contoh Kasus Nyata: Koleksi Heterogen
Jika kita ingin menyimpan daftar feed campuran yang berisi `NewsArticle`, `Tweet`, dan `PodcastEpisode` dalam satu `Vec`:

```rust
// ✅ Koleksi Heterogen: Seluruh tipe berbeda dibungkus Box<dyn Summary>
let feed: Vec<Box<dyn Summary>> = vec![
    Box::new(article),
    Box::new(tweet),
    Box::new(podcast),
];

for item in &feed {
    println!("{}", item.summarize()); // Dynamic dispatch memanggil method yang sesuai
}
```

---

#### 4. Kode Lengkap Terintegrasi (`src/fase7_task_3.rs`)

```rust
use crate::fase7_task_2::{NewsArticle, Summary, Tweet};
use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PodcastEpisode {
    pub show_name: String,
    pub host: String,
    pub episode_number: u32,
}

impl Summary for PodcastEpisode {
    fn summarize_author(&self) -> String {
        format!("Host: {}", self.host)
    }

    fn summarize(&self) -> String {
        format!("Podcast '{}' Ep. #{} ({})", self.show_name, self.episode_number, self.summarize_author())
    }
}

// 1. Static Dispatch: Monomorphization via Generics T: Summary
pub fn process_static<T: Summary>(item: &T) -> String {
    format!("[Static Dispatch] {}", item.summarize())
}

// 2. Dynamic Dispatch: Trait Object via Fat Pointer &dyn Summary
pub fn process_dynamic(item: &dyn Summary) -> String {
    format!("[Dynamic Dispatch] {}", item.summarize())
}

// 3. Dynamic Dispatch dengan Heap Allocation: Box<dyn Summary>
pub fn process_boxed(item: &Box<dyn Summary>) -> String {
    format!("[Boxed dyn Summary] {}", item.summarize())
}

// 4. Koleksi Heterogen
pub fn process_heterogeneous_collection(items: &[Box<dyn Summary>]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Fase 7 - Task 3: Static vs Dynamic Dispatch          ===
============================================================

1. Perbandingan Eksekusi:
   a. [Static Dispatch] Deep Dive Rust Dispatch, oleh Senior Rustacean (Jakarta)
   b. [Static Dispatch] (Baca selengkapnya dari @ferris_the_crab...)
   c. [Static Dispatch] Podcast 'Rust Nation Podcast' Ep. #42 (Host: Budi Santoso)

2. Dynamic Dispatch via &dyn Summary:
   a. [Dynamic Dispatch] Deep Dive Rust Dispatch, oleh Senior Rustacean (Jakarta)
   b. [Dynamic Dispatch] (Baca selengkapnya dari @ferris_the_crab...)
   c. [Dynamic Dispatch] Podcast 'Rust Nation Podcast' Ep. #42 (Host: Budi Santoso)

3. Koleksi Heterogen via Vec<Box<dyn Summary>>:
   [0] Deep Dive Rust Dispatch, oleh Senior Rustacean (Jakarta)
   [1] (Baca selengkapnya dari @ferris_the_crab...)
   [2] Podcast 'Rust Nation Podcast' Ep. #42 (Host: Budi Santoso)

4. Analisis Memori Fat Pointer vs Thin Pointer:
   - Ukuran referensi konkret &NewsArticle (Thin Pointer) : 8 bytes
   - Ukuran referensi trait object &dyn Summary (Fat Pointer): 16 bytes
   - Ukuran Box<NewsArticle> (Heap Thin Pointer)          : 8 bytes
   - Ukuran Box<dyn Summary> (Heap Fat Pointer)           : 16 bytes
   -> Fat Pointer berukuran 2x pointer biasa: [Pointer Data (8B)] + [Pointer Vtable (8B)]
```


---

### 7.4 Associated Types in Traits (Task 4)

Associated Types menghubungkan sebuah tipe *placeholder* di dalam definisi trait, di mana tipe konkretnya baru ditetapkan oleh struct yang mengimplementasikannya.

```rust
pub trait Repository {
    type Item;  // Associated type untuk tipe entitas
    type Error; // Associated type untuk tipe galat

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error>;
}
```

---

#### 1. Mengapa Associated Types? (Associated Types vs Generic Parameters)

Pertanyaan umum pemula: *Mengapa tidak memakai generic parameter biasa seperti `trait Repository<Item, Error>`?*

##### Perbandingan Arsitektural:

| Aspek | Generic Trait: `Repository<Item, Error>` | Associated Types: `type Item; type Error;` |
| :--- | :--- | :--- |
| **Hubungan Tipe** | **1-ke-Banyak (1-to-Many)**: Satu struct bisa mengimplementasikan `Repository<User>` DAN `Repository<Product>`. | **1-ke-1 (1-to-1)**: Satu struct secara tegas hanya mengelola 1 tipe entitas dan 1 tipe error. |
| **Kebersihan Signature** | ❌ **Berisik (Polluted Signature)**: Setiap fungsi harus menulis ulang parameter generic: `fn use_repo<R, I, E>(r: &R) where R: Repository<I, E>`. | ✅ **Bersih & Ergonomis**: Cukup tulis `fn use_repo<R: Repository>(r: &R)`. |
| **Ambiguitas Pemanggilan** | Rentan ambigu; compiler sering butuh anotasi turbofish `r.get::<User, DbError>(id)`. | Tidak ambigu; tipe kembalian otomatis terkunci pada `R::Item`. |

> **Kaidah Praktis (Best Practice Rust):**
> - Gunakan **Generic Trait (`trait Trait<T>`)** jika sebuah struct memang masuk akal memiliki banyak implementasi berbeda (contoh: trait `From<T>` di mana `String` bisa dibuat `From<&str>`, `From<u32>`, `From<char>`).
> - Gunakan **Associated Types (`type Item;`)** jika hanya ada tepat satu tipe logis yang terikat pada struct pengimplementasi (contoh: `Iterator::Item` atau `Repository::Item`).

---

#### 2. Proyeksi Tipe (`R::Item` & `R::Error`) pada Trait Bounds

Ketika membuat fungsi generic yang menerima suatu `Repository`, kita bisa merujuk langsung ke associated types milik implementor menggunakan sintaks `R::Item` atau `R::Error`:

```rust
pub fn fetch_and_display<R>(repo: &R, id: u64) -> Result<String, R::Error>
where
    R: Repository,
    R::Item: Display, // Batasan: Tipe entitas harus bisa dicetak via {}
{
    match repo.get(id)? {
        Some(item) => Ok(format!("Ditemukan: {item}")),
        None => Ok(format!("Entity ID #{id} tidak ditemukan")),
    }
}
```

---

#### 3. Kode Lengkap Terintegrasi (`src/fase7_task_4.rs`)

```rust
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    NotFound(u64),
    ConnectionFailed(String),
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::NotFound(id) => write!(f, "Entity dengan ID #{id} tidak ditemukan"),
            RepoError::ConnectionFailed(msg) => write!(f, "Koneksi database gagal: {msg}"),
        }
    }
}

impl std::error::Error for RepoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User [ID: {}, Username: '{}', Email: '{}']", self.id, self.username, self.email)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Product [ID: {}, Name: '{}', Price: Rp{:.2}]", self.id, self.name, self.price)
    }
}

pub trait Repository {
    type Item;
    type Error;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error>;
}

// Implementasi 1: InMemoryUserRepository
pub struct InMemoryUserRepository {
    storage: HashMap<u64, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn insert(&mut self, user: User) {
        self.storage.insert(user.id, user);
    }
}

impl Repository for InMemoryUserRepository {
    type Item = User;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.storage.get(&id).cloned())
    }
}

// Implementasi 2: InMemoryProductRepository
pub struct InMemoryProductRepository {
    storage: HashMap<u64, Product>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn insert(&mut self, product: Product) {
        self.storage.insert(product.id, product);
    }
}

impl Repository for InMemoryProductRepository {
    type Item = Product;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.storage.get(&id).cloned())
    }
}

// Implementasi 3: MockFaultyRepository (Simulasi Galat)
pub struct MockFaultyRepository;

impl Repository for MockFaultyRepository {
    type Item = User;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        if id == 0 {
            Err(RepoError::NotFound(0))
        } else {
            Err(RepoError::ConnectionFailed("Timeout server database mock".to_string()))
        }
    }
}

pub fn fetch_and_display<R>(repo: &R, id: u64) -> Result<String, R::Error>
where
    R: Repository,
    R::Item: Display,
{
    match repo.get(id)? {
        Some(item) => Ok(format!("Ditemukan: {item}")),
        None => Ok(format!("Entity ID #{id} tidak ditemukan di repository")),
    }
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Fase 7 - Task 4: Associated Types in Traits          ===
============================================================

1. InMemoryUserRepository (type Item = User):
   - Direct get(1) : User [ID: 1, Username: 'alice_crypto', Email: 'alice@rust.dev']
   - Direct get(99): None (Aman, data tidak ada)

2. InMemoryProductRepository (type Item = Product):
   - Direct get(101): Product [ID: 101, Name: 'Mechanical Keyboard 75%', Price: Rp1250000.00]

3. Generic Function Menggunakan Proyeksi R::Item & R::Error:
   - User Repo Query   -> Ditemukan: User [ID: 2, Username: 'bob_engineer', Email: 'bob@systems.id']
   - Product Repo Query-> Ditemukan: Product [ID: 102, Name: 'Wireless Ergonomic Mouse', Price: Rp650000.00]
   - Missing Item Query-> Entity ID #888 tidak ditemukan di repository

4. Simulasi Error via MockFaultyRepository:
   - ID 0 -> Error Ditangkap: Entity dengan ID #0 tidak ditemukan
   - ID 100 -> Error Ditangkap: Koneksi database gagal: Timeout server database mock
```

---

### 7.5 Newtype Pattern & Type Safety (Task 5)

Newtype Pattern adalah pola desain idiomatis di Rust di mana tipe primitif atau tipe eksternal dibungkus ke dalam **Tuple Struct dengan 1 elemen**.

```rust
pub struct UserId(pub u64);
pub struct OrderId(pub u64);
```

---

#### 1. Mengapa Type Safety Jauh Lebih Baik daripada Memakai `u64` untuk Semuanya?

Banyak developer pemula terjebak dalam anti-pattern **Primitive Obsession** (menggunakan tipe primitif mentah seperti `u64` atau `String` untuk segala jenis identitas data).

##### A. Bahaya Nyata Primitive Obsession
Bayangkan sebuah fungsi pemrosesan pesanan di sistem e-commerce:

```rust
// ❌ RAW PRIMITIVE (Rentan Human Error):
fn process_order_unsafe(user_id: u64, order_id: u64) {
    // ...
}

// Suatu hari, developer tidak sengaja menukar urutan argumen:
let user_id = 42;
let order_id = 100889;

// Keduanya bertipe u64! Compiler Rust TIDAK AKAN mendeteksi error ini:
process_order_unsafe(order_id, user_id); // ⚠️ BUG FATAL LOLOS KE PRODUCTION!
```
Dampaknya: User ID 100889 yang mungkin tidak bersalah didebet untuk pesanan ID 42!

##### B. Solusi Newtype: Pencegahan Bug Mutlak di Waktu Kompilasi
Dengan Newtype, `UserId` dan `OrderId` menjadi dua tipe yang **berbeda secara total** di mata compiler:

```rust
// ✅ TYPE-SAFE NEWTYPE:
fn process_order_safe(user_id: UserId, order_id: OrderId) {
    // ...
}

// Jika developer tidak sengaja menukar argumen:
process_order_safe(order_id, user_id);
// ❌ COMPILER ERROR:
// expected `UserId`, found `OrderId`
// expected `OrderId`, found `UserId`
```
> **Keuntungan Utama:** Bug tertangkap 100% oleh compiler saat Anda mengetik kode, bukan di runtime server production!

---

#### 2. Analisis Performa: Zero-Cost Abstraction

Apakah pembungkusan struct ini memperlambat eksekusi aplikasi atau memboroskan memori? **Sama sekali tidak!**

```rust
assert_eq!(std::mem::size_of::<UserId>(), 8); // Sama persis dengan u64 (8 bytes)
assert_eq!(std::mem::size_of::<OrderId>(), 8); // Sama persis dengan u64 (8 bytes)
```

Saat kompilasi optimasi (`cargo build --release`), compiler Rust menghapus lapisan struct Newtype dan menghasilkan kode mesin yang identik dengan manipulasi integer mentah. **Keamanan maksimal tanpa biaya runtime (*Zero-Cost Abstraction*)!**

---

#### 3. Manfaat Sekunder: Menembus Batasan *Orphan Rule*

Rust menerapkan **Orphan Rule**: kita dilarang mengimplementasikan Trait pada suatu Tipe jika KEDUA-DUANYA berasal dari luar crate lokal kita (eksternal/std).

Misalnya:
- Trait `std::fmt::Display` (berasal dari `std`)
- Tipe `Vec<String>` (berasal dari `std`)
- Menulis `impl Display for Vec<String>` ❌ **Ditolak oleh compiler!**

**Solusi dengan Newtype:** Bungkus `Vec<String>` ke dalam tuple struct lokal, lalu implementasikan `Display`:

```rust
pub struct TagList(pub Vec<String>);

impl Display for TagList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[Tags: {}]", self.0.join(", "))
    }
}
```

---

#### 4. Kode Lengkap Terintegrasi (`src/fase7_task_5.rs`)

```rust
use std::fmt::{self, Display, Formatter};
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub u64);

impl UserId {
    pub fn new(id: u64) -> Self { Self(id) }
    pub fn raw(&self) -> u64 { self.0 }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "USR-{:05}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(pub u64);

impl OrderId {
    pub fn new(id: u64) -> Self { Self(id) }
    pub fn raw(&self) -> u64 { self.0 }
}

impl Display for OrderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ORD-{:06}", self.0)
    }
}

// 1. Primitive Obsession vs Type Safety
pub fn process_order_unsafe(user_id: u64, order_id: u64) -> String {
    format!("Memproses pesanan ID #{order_id} untuk User #{user_id}")
}

pub fn process_order_safe(user_id: UserId, order_id: OrderId) -> String {
    format!("Memproses pesanan {order_id} untuk User {user_id}")
}

// 2. Bypass Orphan Rule via Newtype
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagList(pub Vec<String>);

impl Display for TagList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let joined = self.0.join(", ");
        write!(f, "[Tags: {joined}]")
    }
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Fase 7 - Task 5: Newtype Pattern & Type Safety       ===
============================================================

1. Mengapa Type Safety Lebih Baik daripada Raw u64?
   a. Versi Rentan (u64):
      * Argumen benar : Memproses pesanan ID #100889 untuk User #42
      * Argumen tertukar: Memproses pesanan ID #42 untuk User #100889 ❌ (Bug semantik lolos tanpa error!)

   b. Versi Type-Safe (Newtype):
      * Eksekusi aman : Memproses pesanan ORD-100889 untuk User USR-00042 ✓
      * Jika dibalik `process_order_safe(order_id, user_id)`: Ditolak total oleh compiler!

2. Analisis Ukuran Memori (Zero-Cost Abstraction):
   - Ukuran u64 mentah      : 8 bytes
   - Ukuran UserId (Newtype): 8 bytes
   - Ukuran OrderId (Newtype): 8 bytes
   -> Kesimpulan: Newtype memiliki runtime cost = 0! Pembungkus lenyap saat kompilasi.

3. Bypass Orphan Rule via Newtype (Display untuk Vec<String>):
   - Cetak TagList terformat via Display: [Tags: rust, type-safety, newtype, zero-cost]
```


---

### 7.6 Mini Project Fase 7 — Repository Abstraction Pattern

Mini Project ini mengintegrasikan seluruh materi utama Fase 7: **Trait Abstraction**, **Associated Types**, **Static vs Dynamic Dispatch**, serta **Newtype Pattern**.

```text
       ┌────────────────────────┐
       │    Repository Trait    │  (Kontrak Abstraksi dengan Associated Types)
       └───────────┬────────────┘
                   │
         ┌─────────┴─────────┐
         ▼                   ▼
┌──────────────────┐ ┌──────────────────┐
│ InMemoryAccount- │ │   MockAccount-   │
│    Repository    │ │    Repository    │  (Digunakan untuk unit testing & isolasi error)
└──────────────────┘ └──────────────────┘
```

---

#### 1. Arsitektur Repository Pattern dengan Associated Types

Trait `Repository` mendefinisikan operasi standar persistence data:

```rust
pub trait Repository {
    type Item;
    type Id;
    type Error: std::error::Error;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}
```

1. **`InMemoryAccountRepository`**: Menggunakan `HashMap<AccountId, Account>` untuk penyimpanan data sementara di memori lokal secara aman dan cepat.
2. **`MockAccountRepository`**: Digunakan untuk pengujian (*unit testing*), mampu disimulasikan skenario kegagalan I/O (`should_fail_on_save`) tanpa menyentuh storage sungguhan.
3. **Consumers**:
   - `audit_repository_static<R: Repository>(repo: &R)`: Static Dispatch (Monomorphized, Zero-Cost).
   - `audit_repository_dynamic(repo: &dyn Repository)`: Dynamic Dispatch (vtable fat pointer).

---

#### 2. Evaluasi 4 Kriteria Lulus Fase 7

1. **Generic vs Trait**:
   - *Generic* (`<T>`) adalah parameter tipe abstrak yang memungkinkan fungsi/struct bekerja untuk tipe apa pun.
   - *Trait* adalah kontrak perilaku (*behavior interface*) yang membatasi kapabilitas apa yang harus dimiliki oleh tipe tersebut.
2. **Static vs Dynamic Dispatch**:
   - *Static Dispatch* (`T: Trait`): Diputuskan saat kompilasi via monomorphization. Kecepatan maksimal, bisa di-inline compiler.
   - *Dynamic Dispatch* (`dyn Trait`): Diputuskan saat runtime via vtable fat pointer (16 bytes). Mendukung polimorfisme heterogen (`Vec<Box<dyn Trait>>`).
3. **Associated Types**:
   - Menghubungkan tipe entitas dan error secara 1-ke-1 langsung pada struct pengimplementasi (`type Item = Account; type Id = AccountId; type Error = RepositoryError;`), menghasilkan signature fungsi yang bersih tanpa polusi parameter generic.
4. **Orphan Rule & Newtype**:
   - Membungkus tipe primitif `u64` ke dalam tuple struct Newtype `AccountId(pub u64)` untuk mencegah bug tertukarnya parameter ID (*Primitive Obsession*) sekaligus membuka jalan implementasi trait eksternal secara legal.

---

#### 3. Kode Lengkap Terintegrasi (`src/mini_project_7.rs`)

```rust
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(pub u64);

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ACC-{:05}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: AccountId,
    pub holder: String,
    pub balance: f64,
    pub is_active: bool,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = if self.is_active { "AKTIF" } else { "NONAKTIF" };
        write!(
            f,
            "[{}] {} | Saldo: Rp{:.2} | Status: {}",
            self.id, self.holder, self.balance, status
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound(String),
    Duplicate(String),
    StorageFailure(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "[Error 404] Entitas tidak ditemukan: {msg}"),
            RepositoryError::Duplicate(msg) => write!(f, "[Error 409] Duplikasi entitas: {msg}"),
            RepositoryError::StorageFailure(msg) => write!(f, "[Error 500] Kegagalan storage: {msg}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

pub trait Repository {
    type Item;
    type Id;
    type Error: std::error::Error;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

// 1. InMemory Implementation
pub struct InMemoryAccountRepository {
    data: HashMap<AccountId, Account>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self { Self { data: HashMap::new() } }
    pub fn find_or_err(&self, id: &AccountId) -> Result<Account, RepositoryError> {
        self.find_by_id(id)?
            .ok_or_else(|| RepositoryError::NotFound(format!("Akun {id} tidak ditemukan")))
    }
}

impl Repository for InMemoryAccountRepository {
    type Item = Account;
    type Id = AccountId;
    type Error = RepositoryError;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        if self.data.contains_key(&item.id) {
            return Err(RepositoryError::Duplicate(format!("Akun dengan ID {} sudah ada", item.id)));
        }
        self.data.insert(item.id, item);
        Ok(())
    }

    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.data.get(id).cloned())
    }

    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }

    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let mut list: Vec<Account> = self.data.values().cloned().collect();
        list.sort_by_key(|a| a.id);
        Ok(list)
    }
}

// 2. Mock Implementation (Simulasi Failure)
pub struct MockAccountRepository {
    pub accounts: HashMap<AccountId, Account>,
    pub should_fail_on_save: bool,
    pub failure_message: String,
}

impl MockAccountRepository {
    pub fn with_simulated_failure(failure_message: &str) -> Self {
        Self {
            accounts: HashMap::new(),
            should_fail_on_save: true,
            failure_message: failure_message.to_string(),
        }
    }
}

impl Repository for MockAccountRepository {
    type Item = Account;
    type Id = AccountId;
    type Error = RepositoryError;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        if self.should_fail_on_save {
            return Err(RepositoryError::StorageFailure(self.failure_message.clone()));
        }
        self.accounts.insert(item.id, item);
        Ok(())
    }

    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.accounts.get(id).cloned())
    }

    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.accounts.remove(id).is_some())
    }

    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        Ok(self.accounts.values().cloned().collect())
    }
}

// 3. Consumers (Static & Dynamic Dispatch)
pub fn audit_repository_static<R>(repo: &R) -> Result<String, R::Error>
where
    R: Repository<Item = Account, Id = AccountId>,
{
    let accounts = repo.list_all()?;
    let total_balance: f64 = accounts.iter().map(|a| a.balance).sum();
    Ok(format!("[Audit Static] Total Akun: {} | Total Aset: Rp{:.2}", accounts.len(), total_balance))
}

pub fn audit_repository_dynamic(
    repo: &dyn Repository<Item = Account, Id = AccountId, Error = RepositoryError>,
) -> Result<String, RepositoryError> {
    let accounts = repo.list_all()?;
    let active_count = accounts.iter().filter(|a| a.is_active).count();
    Ok(format!("[Audit Dynamic] Akun Aktif: {} dari total {}", active_count, accounts.len()))
}
```

##### Hasil Eksekusi Output Terminal:
```text
============================================================
=== Mini Project Fase 7: Repository Abstraction Pattern  ===
============================================================

1. Mengoperasikan InMemoryAccountRepository:
   [+] Berhasil menyimpan 3 akun ke in-memory storage.
   [✓] Query Akun #102 (find_by_id): [ACC-00102] Siti Rahma | Saldo: Rp27500000.00 | Status: AKTIF
   [✓] Query Akun #999 (find_or_err): [Error 404] Entitas tidak ditemukan: Akun ACC-00999 tidak ditemukan
   [✓] Deteksi Duplikasi ID: [Error 409] Duplikasi entitas: Akun dengan ID ACC-00101 sudah ada

2. Evaluasi Dispatch pada Service Audit:
   - [Audit Static] Total Akun: 3 | Total Aset: Rp43000000.00
   - [Audit Dynamic] Akun Aktif: 2 dari total 3

3. Demonstrasi MockAccountRepository (Simulasi Failure):
   [✓] Sukses Menangkap Simulasi Error: [Error 500] Kegagalan storage: Koneksi Database Timeout

4. Operasi Modifikasi (Delete & List All):
   - Hapus Akun #103: Status = true
   - Sisa Akun di InMemory Storage:
     * [ACC-00101] Ahmad Dahlan | Saldo: Rp15000000.00 | Status: AKTIF
     * [ACC-00102] Siti Rahma | Saldo: Rp27500000.00 | Status: AKTIF

5. Evaluasi Kriteria Lulus Fase 7:
   [x] Generic vs Trait: Generics adalah parameter tipe abstrak; Trait adalah kontrak antarmuka.
   [x] Static vs Dynamic: Static = Monomorphized (Zero-Cost); Dynamic = vtable fat pointer.
   [x] Associated Types: Repository::Item, Id, & Error terikat 1-to-1 pada struct.
   [x] Orphan Rule & Newtype: AccountId tuple struct memberikan type safety dan enkapsulasi.
```



---

## FASE 8: Lifetimes Mendalam ('a, Structs, Impls, Elision Rules, 'static)

### 8.1 Filosofi Lifetime & Peran Simbol `'a`
> **Prinsip Utama**: Anotasi lifetime (`'a`) **TIDAK MENGUBAH atau MEMPERPANJANG masa hidup objek**. 
> Anotasi lifetime hanyalah **parameter deskriptif bagi borrow checker compiler** untuk memvalidasi hubungan ketergantungan masa hidup antara referensi input dan referensi output. Tujuannya adalah membuktikan secara matematis saat kompilasi bahwa tidak akan pernah terjadi pointer menggantung (*dangling reference*) saat runtime.

Dalam Rust, setiap referensi memiliki *lifetime* (rentang kode di mana referensi tersebut valid). Sering kali compiler dapat menginferensikannya secara otomatis (*lifetime elision*), namun ketika ada ambiguitas (misalnya fungsi menerima 2 referensi dan mengembalikan 1 referensi), compiler menuntut kita untuk menyatakan hubungan lifetime secara eksplisit.

#### 1. Masalah Nyata yang Ingin Dicegah: "Kertas Alamat & Rumah Kosong"
Di bahasa pemrograman seperti C/C++, ada bahaya besar bernama **Dangling Pointer** (penunjuk menggantung):
1. Bayangkan Anda punya selembar kertas bertuliskan alamat rumah teman: `Jl. Mawar No. 10`. Kertas ini adalah **Referensi (`&`)**.
2. Suatu hari, rumah teman Anda digusur dan rata dengan tanah (datanya di-drop/dihapus dari RAM).
3. Anda tidak tahu kalau rumahnya sudah tiada. Anda nekat mendatangi alamat itu lalu masuk.
4. **Akibatnya**: *Crash*, memori rusak (*segmentation fault*), atau celah keamanan dieksploitasi hacker.

Rust membuat aturan mutlak: **Referensi (`&`) tidak boleh menunjuk ke data yang sudah mati.**

---

#### 2. Kenapa Butuh Tanda Kurung Lancip `<>` Seperti Generic?
Alasannya sederhana: **Karena Lifetime memang SEBENARNYA ADALAH GENERIC.**  
Dalam dokumentasi resmi Rust, istilah resminya adalah **"Generic Lifetime Parameter"**.

Mari bandingkan langsung antara **Generic Tipe (`<T>`)** dengan **Generic Lifetime (`<'a>`)**:

| Aspek Perbandingan | Generic Tipe (`<T>`) | Generic Lifetime (`<'a>`) |
| :--- | :--- | :--- |
| **Apa yang belum diketahui saat nulis fungsi?** | **Tipe datanya** (apakah `i32`, `String`, atau `bool`). | **Lama masa hidup datanya** (apakah hidup 5 baris, 1 blok fungsi, atau sepanjang program). |
| **Fungsi deklarasi `<...>`** | Mendaftarkan *placeholder* (variabel pengganti) untuk **tipe**. | Mendaftarkan *placeholder* (variabel pengganti) untuk **scope / masa hidup**. |
| **Kapan nilainya ditentukan?** | Saat pemanggil memasukkan tipe konkret: `cetak::<i32>(5)`. | Saat pemanggil memasukkan referensi dengan scope nyata: `longest(&s1, &s2)`. |

##### Mengapa Harus Didaftarkan Dulu di Dalam `<>`?
Persis seperti variabel biasa, compiler tidak tahu siapa `'a` jika tidak dideklarasikan terlebih dahulu:
- Jika kita tulis `fn cetak(nilai: T)` tanpa `<T>`, compiler akan protes:  
  `error: cannot find type 'T' in this scope`.
- Hal serupa terjadi pada lifetime: jika kita tulis `fn longest(x: &'a str) -> &'a str` tanpa `<'a>`, compiler akan protes:  
  `error: use of undeclared lifetime name ''a'`.

Maka tanda `<'a>` berfungsi sebagai deklarasi: *"Halo compiler, siapkan placeholder nama `'a` untuk menampung rentang waktu referensi yang dipinjam nanti!"*

##### Kenapa Pakai Tanda Petik Tunggal (`'`)?
Tanda petik tunggal (`'`) dipakai agar compiler dan programmer bisa **membedakan mana Tipe dan mana Lifetime** ketika keduanya digabung di dalam kurung lancip yang sama:
```rust
// Menerima placeholder lifetime 'a DAN placeholder tipe T:
fn proses_item<'a, T>(item: &'a T) {
    // ...
}
```
Tanpa petik tunggal (`<a, T>`), compiler akan menganggap `a` sebagai tipe struct, bukan rentang waktu.

##### Nama `'a` Itu Bebas, Seperti Nama Variabel!
Simbol `'a` bukan kata kunci rahasia bawaan sistem. Sama halnya seperti Anda bebas menamai tipe generic dengan `<T>`, `<Item>`, atau `<Data>`, Anda juga bebas menamai lifetime:
```rust
// Ini 100% valid dan sah di Rust:
fn longest<'waktu_hidup>(x: &'waktu_hidup str, y: &'waktu_hidup str) -> &'waktu_hidup str {
    if x.len() >= y.len() { x } else { y }
}
```
Programmer Rust menggunakan nama pendek seperti `'a`, `'b` semata-mata karena ringkas dan cepat diketik (mirip seperti menamai variabel loop `for i in 0..10`).

---

#### 3. Dua Analogi Sederhana untuk Memahami Lifetime Tanpa Pusing

##### Analogi 1: Tiket Wahana Bermain (Fungsi dengan Lifetime)
- Anda (`string1`) punya tiket wahana bermain berlaku sampai jam **17:00**.
- Teman Anda (`string2`) punya tiket anak-anak setengah hari berlaku sampai jam **12:00**.
- Kalian memesan paket foto bersama berdua (`longest(&string1, &string2)`).
- **Pertanyaan**: Sampai jam berapa paket foto bersama itu sah dianggap turis aktif di dalam wahana?
- **Jawabannya**: **Jam 12:00!** Karena setelah jam 12:00, teman Anda sudah pulang (datanya musnah).
- **Peran `'a`**: Tulisan `'a` memberi tahu penjaga gerbang (compiler): *"Paket ini gugur saat orang pertama pulang."* Menulis `'a` **tidak bisa** memperpanjang tiket teman Anda sampai jam 17:00.

##### Analogi 2: Pembatas Buku di Perpustakaan (Struct dengan Referensi)
```rust
struct Parser<'a> {
    source: &'a str,
}
```
- `source` adalah buku fisik yang Anda pinjam dari perpustakaan.
- `Parser` adalah pembatas buku yang Anda selipkan di dalam buku tersebut.
- Jika buku perpustakaan itu sudah Anda kembalikan ke rak perpustakaan, pembatas buku Anda tidak ada gunanya lagi di meja kosong.
- Tanda `'a` pada struct memastikan bahwa: **Instance struct `Parser` TIDAK BOLEH hidup lebih lama daripada buku teks (`source`) yang sedang dipinjamnya.**

---

### 8.2 Fungsi dengan Lifetime & Multiple Lifetime Parameters

#### 1. Dasar Anotasi Fungsi: `longest<'a>`
```rust
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}
```
- **Mengapa butuh `'a`?**: Compiler mengevaluasi fungsi secara terisolasi tanpa melihat konteks pemanggilnya. Compiler tidak tahu apakah cabang `if` atau `else` yang akan dieksekusi saat runtime.
- **Arti `'a` pada return**: Referensi yang dikembalikan dijamin valid selama **irisan masa hidup terpendek (minimum overlap)** antara `x` dan `y`.

#### 2. Eksperimen: `first<'a>`
```rust
pub fn first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}
```
Meskipun `_b` menerima anotasi `'a'`, nilai kembalian murni berasal dari `a`. Namun karena `_b` beranotasi `'a'`, masa hidup kembalian tetap terikat pada durasi `_b`. Untuk membebaskannya, gunakan multiple lifetime parameter!

#### 3. Multiple Lifetime Parameters (`'a` dan `'b`)
```rust
pub fn choose_first_with_context<'a, 'b>(primary: &'a str, context: &'b str) -> &'a str {
    let _log = format!("[Context: {context}]");
    primary
}
```
- Jika kita memaksakan satu lifetime `'a` untuk `primary` dan `context`, maka masa hidup kembalian akan dibatasi oleh argumen yang paling cepat di-drop.
- Dengan memisahkan `'a` dan `'b`, argumen `context` boleh berasal dari inner scope yang sangat pendek tanpa membatasi masa berlaku referensi kembalian `primary`.

---

### 8.3 Struct Menyimpan Reference & Blok Implementasi (`impl<'a>`)

#### 1. Reference Sebagai Field Struct
Jika struct menyimpan referensi (bukan owned type seperti `String`), struct **wajib** mencantumkan parameter lifetime:
```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub cursor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}
```
**Aturan Emas**: Instance struct `Parser<'a>` **TIDAK BOLEH hidup lebih lama** dari string asli yang direferensikan oleh field `source`.

#### 2. Lifetime pada Blok `impl<'a>`
```rust
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        let remaining = &self.source[self.cursor..];
        let trimmed = remaining.trim_start();
        let leading_spaces = remaining.len() - trimmed.len();
        self.cursor += leading_spaces;

        if self.cursor >= self.source.len() {
            return None;
        }

        let slice = &self.source[self.cursor..];
        let token_len = slice.find(char::is_whitespace).unwrap_or(slice.len());
        let token = &slice[..token_len];
        self.cursor += token_len;
        Some(token)
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    pub fn announce_and_get_excerpt(&self, announcement: &str) -> &'a str {
        let _msg = format!("[INFO]: {announcement}");
        self.source
    }
}
```
- Deklarasi `impl<'a>` mendefinisikan generic lifetime parameter `'a`.
- `Parser<'a>` menyatakan bahwa method diimplementasikan untuk struct yang memiliki lifetime `'a`.

---

### 8.4 Tiga Aturan Lifetime Elision (Otomatisasi Kompiler)

Compiler Rust menerapkan **3 Aturan Elision** secara berurutan. Jika setelah menerapkan ketiga aturan ini compiler masih belum bisa menentukan lifetime dari referensi return type, kompilasi akan gagal dan menuntut anotasi manual:

1. **Aturan 1 (Setiap Input Mendapat Lifetime Unik)**:  
   Setiap parameter referensi input diberi parameter lifetime tersendiri:
   `fn foo(x: &str, y: &str)` ekuivalen dengan `fn foo<'a, 'b>(x: &'a str, y: &'b str)`.
2. **Aturan 2 (Satu Input Reference Menjadi Output Reference)**:  
   Jika hanya ada tepat satu input reference, lifetime tersebut otomatis diberikan ke semua referensi output:  
   `fn first_word(s: &str) -> &str` ekuivalen dengan `fn first_word<'a>(s: &'a str) -> &'a str`.
3. **Aturan 3 (Metode dengan `&self` Mengikat Output ke `self`)**:  
   Jika ada beberapa input reference dan salah satunya adalah `&self` atau `&mut self`, maka lifetime dari `self` otomatis diberikan ke semua referensi output.

---

### 8.5 Lifetime Khusus: `'static` (Literal vs Trait Bound)

Ada dua penggunaan utama `'static`:
1. **Referensi `'static` (`&'static T`)**:  
   Data hidup sepanjang seluruh eksekusi program. Contohnya adalah string literal (`"Hello"`), yang tersimpan permanen di segmen data binary executable.
2. **Trait Bound `'static` (`T: 'static`)**:  
   Artinya tipe `T` **bebas dari referensi non-static**. Tipe yang memiliki data sendiri (seperti `String`, `i32`, `Vec<u8>`) memenuhi syarat `T: 'static'` karena tidak ada kemungkinan referensinya menjadi invalid sewaktu-waktu.

```rust
pub const GLOBAL_SYSTEM_NAME: &'static str = "RUST_LEARNING_SYSTEM_V2";

pub fn verify_static_bound<T: Display + 'static>(val: T) -> String {
    format!("[Static Validated]: {val}")
}
```

---

### 8.6 Kombinasi Tingkat Lanjut: Generics + Lifetime & Trait + Lifetime

#### 1. Generic Tipe + Lifetime
```rust
#[derive(Debug)]
pub struct AnnotatedItem<'a, T> {
    pub item: &'a T,
    pub note: &'a str,
}

pub fn find_first_match<'a, T: PartialEq>(items: &'a [T], target: &T) -> Option<&'a T> {
    for item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

pub fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    let _log = format!("Pengumuman: {ann}");
    if x.len() >= y.len() { x } else { y }
}
```

#### 2. Trait + Lifetime
```rust
pub trait TextTokenizer<'a> {
    fn tokenize(&'a mut self) -> Vec<&'a str>;
}

pub trait Highlightable<'a> {
    fn get_highlight(&self) -> &'a str;
}
```

---

### 8.7 Eksperimen Dangling Reference & Analisis Borrow Checker

#### Kasus Error 1: Mengembalikan Referensi Data Lokal
```rust
// ❌ DITOLAK COMPILER:
fn create_dangling() -> &str {
    let s = String::from("halo lokal");
    &s // ERROR: returns a value referencing data owned by the current function
}
```
- **Error**: `s` di-drop dari Stack begitu fungsi selesai dieksekusi. Mengembalikan `&s` akan menciptakan dangling pointer.
- **Solusi Tanpa `.clone()`**: Kembalikan owned `String` secara langsung, menyerahkan kepemilikan utuh ke pemanggil.

#### Kasus Error 2: Struct Menyimpan Referensi Variabel Scope Sempit
```rust
// ❌ DITOLAK COMPILER:
let parser: Parser;
{
    let temporary_string = String::from("data sementara");
    parser = Parser { source: &temporary_string, cursor: 0 };
} // temporary_string di-drop di sini!
println!("{:?}", parser.source); // ERROR: `temporary_string` does not live long enough
```
- **Error**: Variabel `temporary_string` musnah sebelum `parser` selesai digunakan.
- **Solusi Tanpa `.clone()`**: Deklarasikan data sumber di scope yang setara atau lebih luas dari struct `parser`.

---

### 8.8 Kode Lengkap Terintegrasi (`src/fase8_task_1.rs`)

```rust
// Fase 8 - Task 1: Lifetimes Mendalam ('a, Structs, Impls, Elision Rules, 'static)
// Rujukan: rust_learning_guide.md (Sub-bab 8.1 - 8.9) & rust_execution_tasks.md (L728-L773)

use std::fmt::{self, Debug, Display, Formatter};

// 1. Dasar Lifetime: fn longest<'a>(...) & Eksperimen first<'a>(...)
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

pub fn first<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}

// 2. Multiple Lifetime Parameters ('a dan 'b)
pub fn choose_first_with_context<'a, 'b>(primary: &'a str, context: &'b str) -> &'a str {
    let _log = format!("[Context: {context}]");
    primary
}

// 3. Lifetime pada Struct & Reference sebagai Field
#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub cursor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

// 4. Lifetime pada Blok Implementasi (impl<'a>)
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        let remaining = &self.source[self.cursor..];
        let trimmed = remaining.trim_start();
        let leading_spaces = remaining.len() - trimmed.len();
        self.cursor += leading_spaces;

        if self.cursor >= self.source.len() {
            return None;
        }

        let slice = &self.source[self.cursor..];
        let token_len = slice.find(char::is_whitespace).unwrap_or(slice.len());
        let token = &slice[..token_len];
        self.cursor += token_len;
        Some(token)
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    pub fn announce_and_get_excerpt(&self, announcement: &str) -> &'a str {
        let _msg = format!("[INFO]: {announcement}");
        self.source
    }
}

// 5. Pembuktian 3 Aturan Lifetime Elision
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

pub fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    s
}

// 6. Lifetime Khusus: 'static
pub const GLOBAL_SYSTEM_NAME: &'static str = "RUST_LEARNING_SYSTEM_V2";

pub fn verify_static_bound<T: Display + 'static>(val: T) -> String {
    format!("[Static Validated]: {val}")
}

// 7. Generic Tipe + Lifetime
#[derive(Debug)]
pub struct AnnotatedItem<'a, T> {
    pub item: &'a T,
    pub note: &'a str,
}

impl<'a, T: Display> Display for AnnotatedItem<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Catatan: {})", self.item, self.note)
    }
}

pub fn find_first_match<'a, T: PartialEq>(items: &'a [T], target: &T) -> Option<&'a T> {
    for item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

pub fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    let _log = format!("Pengumuman: {ann}");
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// 8. Trait + Lifetime
pub trait TextTokenizer<'a> {
    fn tokenize(&'a mut self) -> Vec<&'a str>;
}

impl<'a> TextTokenizer<'a> for Parser<'a> {
    fn tokenize(&'a mut self) -> Vec<&'a str> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }
}

pub trait Highlightable<'a> {
    fn get_highlight(&self) -> &'a str;
}

impl<'a> Highlightable<'a> for Excerpt<'a> {
    fn get_highlight(&self) -> &'a str {
        self.part
    }
}

// 9. Eksperimen Dangling Reference & Solusi
pub fn safe_lifetime_retention(source: &str, start: usize, len: usize) -> Option<&str> {
    if start + len <= source.len() {
        Some(&source[start..start + len])
    } else {
        None
    }
}

// Demonstrator Runner
pub fn run() {
    println!("============================================================");
    println!("=== Fase 8: Lifetimes Mendalam ('a, Struct, Impl, Static) ===");
    println!("============================================================");

    // 1. fn longest<'a> & first<'a>
    println!("\n1. Demonstrasi fn longest<'a> dan first<'a>:");
    let string1 = String::from("Bahasa Pemrograman Rust");
    let string2 = "Modern 2024";
    let result_longest = longest(&string1, string2);
    let result_first = first(&string1, string2);
    println!("   - String 1        : \"{string1}\" (len: {})", string1.len());
    println!("   - String 2        : \"{string2}\" (len: {})", string2.len());
    println!("   - Hasil longest() : \"{result_longest}\"");
    println!("   - Hasil first()   : \"{result_first}\"");

    // 2. Multiple Lifetime Parameters ('a, 'b)
    println!("\n2. Multiple Lifetime Parameters ('a, 'b):");
    let primary_doc = String::from("Dokumen Rahasia Negara");
    let selected_doc;
    {
        let short_lived_context = String::from("Audit Sesi #8812");
        selected_doc = choose_first_with_context(&primary_doc, &short_lived_context);
        println!("   - Konteks aktif di inner scope: \"{short_lived_context}\"");
    }
    println!("   - Hasil di outer scope: \"{selected_doc}\" ✓ (Bebas dari batasan masa hidup konteks)");

    // 3. Lifetime pada Struct dan Impl (Parser<'a>)
    println!("\n3. Lifetime pada Struct dan Impl (Parser<'a>):");
    let raw_payload = String::from("POST /api/v1/auth/login HTTP/1.1");
    let mut parser = Parser::new(&raw_payload);
    println!("   - Sumber Teks      : \"{}\"", parser.source);
    println!("   - Token 1 (Method) : {:?}", parser.next_token().unwrap());
    println!("   - Token 2 (Path)   : {:?}", parser.next_token().unwrap());
    println!("   - Token 3 (Proto)  : {:?}", parser.next_token().unwrap());
    println!("   - Token 4 (Habis)  : {:?}", parser.next_token());
    println!("   - Peek sisa string : {:?}", parser.peek());
    println!("   - Method announce  : \"{}\"", parser.announce_and_get_excerpt("Parsing Header OK"));

    // 4. Pembuktian Tiga Aturan Lifetime Elision
    println!("\n4. Pembuktian Tiga Aturan Lifetime Elision:");
    let phrase = "Zero-Cost Abstraction in Rust";
    let elided_res = first_word(phrase);
    let explicit_res = first_word_explicit(phrase);
    println!("   - Frasa Asli       : \"{phrase}\"");
    println!("   - first_word (Elision Rule 1 & 2): \"{elided_res}\"");
    println!("   - first_word_explicit          : \"{explicit_res}\"");

    // 5. Analisis Lifetime 'static
    println!("\n5. Analisis Lifetime 'static:");
    println!("   - Konstanta Binary Global       : \"{GLOBAL_SYSTEM_NAME}\" (&'static str)");
    let owned_string = String::from("Data Dinamis di Heap");
    let static_validated_1 = verify_static_bound(GLOBAL_SYSTEM_NAME);
    let static_validated_2 = verify_static_bound(owned_string);
    println!("   - {static_validated_1}");
    println!("   - {static_validated_2} (String owned memenuhi trait bound T: 'static)");

    // 6. Generic Tipe + Lifetime & Trait + Lifetime
    println!("\n6. Generic Tipe + Lifetime & Trait + Lifetime:");
    let score = 98.75;
    let note = "Nilai evaluasi borrow checker sempurna";
    let annotated = AnnotatedItem { item: &score, note };
    println!("   - AnnotatedItem<f64>: {annotated}");

    let int_array = [10, 20, 30, 40, 50];
    let query_val = 30;
    let found = find_first_match(&int_array, &query_val);
    println!("   - find_first_match pada array: {:?}", found);

    let ann_res = longest_with_announcement("Alpha", "BetaGamma", 2026);
    println!("   - longest_with_announcement: \"{ann_res}\"");

    let excerpt = Excerpt { part: "Rust guarantees memory safety without garbage collection" };
    println!("   - Trait Highlightable: \"{}\"", excerpt.get_highlight());

    let mut query_parser = Parser::new("SELECT name FROM users");
    let tokens = query_parser.tokenize();
    println!("   - Trait TextTokenizer: {:?}", tokens);

    // 7. Evaluasi Dangling Reference & Validasi Kriteria Lulus Fase 8
    println!("\n7. Evaluasi Pemahaman Lifetime & Kriteria Lulus Fase 8:");
    let retained_slice = safe_lifetime_retention("Penyimpanan Memori Aman", 0, 11);
    println!("   - safe_lifetime_retention: {:?}", retained_slice);
    println!("   [x] Fungsi 'a: Menandai hubungan validitas antar referensi bagi borrow checker.");
    println!("   [x] Non-extending: Lifetime tidak memperpanjang umur memori objek yang dipinjam.");
    println!("   [x] Lifetime Elision: 3 aturan deterministik yang mengotomatisasi anotasi.");
    println!("   [x] Struct Reference: Struct Parser<'a> dan Excerpt<'a> valid selama sumber referensi hidup.");
}
```

---

### 8.9 Hasil Eksekusi Output Terminal & Evaluasi Kelulusan Fase 8

```text
============================================================
=== Fase 8: Lifetimes Mendalam ('a, Struct, Impl, Static) ===
============================================================

1. Demonstrasi fn longest<'a> dan first<'a>:
   - String 1        : "Bahasa Pemrograman Rust" (len: 23)
   - String 2        : "Modern 2024" (len: 11)
   - Hasil longest() : "Bahasa Pemrograman Rust"
   - Hasil first()   : "Bahasa Pemrograman Rust"

2. Multiple Lifetime Parameters ('a, 'b):
   - Konteks aktif di inner scope: "Audit Sesi #8812"
   - Hasil di outer scope: "Dokumen Rahasia Negara" ✓ (Bebas dari batasan masa hidup konteks)

3. Lifetime pada Struct dan Impl (Parser<'a>):
   - Sumber Teks      : "POST /api/v1/auth/login HTTP/1.1"
   - Token 1 (Method) : "POST"
   - Token 2 (Path)   : "/api/v1/auth/login"
   - Token 3 (Proto)  : "HTTP/1.1"
   - Token 4 (Habis)  : None
   - Peek sisa string : None
   - Method announce  : "POST /api/v1/auth/login HTTP/1.1"

4. Pembuktian Tiga Aturan Lifetime Elision:
   - Frasa Asli       : "Zero-Cost Abstraction in Rust"
   - first_word (Elision Rule 1 & 2): "Zero-Cost"
   - first_word_explicit          : "Zero-Cost"

5. Analisis Lifetime 'static:
   - Konstanta Binary Global       : "RUST_LEARNING_SYSTEM_V2" (&'static str)
   - [Static Validated]: RUST_LEARNING_SYSTEM_V2
   - [Static Validated]: Data Dinamis di Heap (String owned memenuhi trait bound T: 'static)

6. Generic Tipe + Lifetime & Trait + Lifetime:
   - AnnotatedItem<f64>: 98.75 (Catatan: Nilai evaluasi borrow checker sempurna)
   - find_first_match pada array: Some(30)
   - longest_with_announcement: "BetaGamma"
   - Trait Highlightable: "Rust guarantees memory safety without garbage collection"
   - Trait TextTokenizer: ["SELECT", "name", "FROM", "users"]

7. Evaluasi Pemahaman Lifetime & Kriteria Lulus Fase 8:
   - safe_lifetime_retention: Some("Penyimpanan")
   [x] Fungsi 'a: Menandai hubungan validitas antar referensi bagi borrow checker.
   [x] Non-extending: Lifetime tidak memperpanjang umur memori objek yang dipinjam.
   [x] Lifetime Elision: 3 aturan deterministik yang mengotomatisasi anotasi.
   [x] Struct Reference: Struct Parser<'a> dan Excerpt<'a> valid selama sumber referensi hidup.
```

---
---

## FASE 9: Functional Rust (Closures & Iterators)

### 9.1 Closures Mendalam (Captures, Move Keyword, Fn, FnMut, FnOnce)

Closure di Rust adalah fungsi anonim (*anonymous function*) yang dapat menangkap (*capture*) variabel dari lingkungannya (*enclosing environment*). Berbeda dengan fungsi biasa (`fn`), closure memiliki kemampuan menyimpan *state* dari scope luar. 

---

#### 9.1.1 Mental Model: Mengapa Sintaks `|...|` & Apa Hubungannya dengan Bahasa Lain?

Bagi developer yang datang dari bahasa lain (seperti JavaScript atau Python), closure terasa sangat familiar namun memiliki beberapa perbedaan fundamental:

1. **Persamaan dengan Bahasa Lain (Lambda / Arrow Function)**:
   Closure di Rust adalah konsep yang sama persis dengan fungsi anonim di bahasa lain:
   - **JavaScript / TypeScript**: `const add = (x) => x + bonus;`
   - **Python**: `add = lambda x: x + bonus`
   - **Go**: `add := func(x int) int { return x + bonus }`
   - **Rust**: `let add = |x| x + bonus;`

2. **Mengapa Sintaks Menggunakan Pipa `|x|` Bukan Kurung `(x)`?**
   - **Warisan Ruby & Smalltalk**: Rust mengadopsi sintaks `|params| body` dari bahasa Ruby (`items.each { |x| puts x }`) karena ringkas dan ekspresif.
   - **Mencegah Ambiguitas Parsing Compiler**: Di Rust, tanda kurung `(a, b)` sudah digunakan untuk banyak konstruksi penting: tuple `(1, 2)`, grouping aritmatika `(a + b) * c`, dan signature fungsi `fn(a, b)`. Dengan pembatas pipa `|...|`, parser compiler Rust dapat langsung mengenali deklarasi closure tanpa perlu kata kunci tambahan seperti `lambda` atau `function`.
     - Tanpa parameter: `|| cetak_log()`
     - Satu parameter: `|x| x * 2`
     - Banyak parameter: `|a, b| a + b`

3. **Kenapa di Bahasa Lain Terasa Santai, tetapi di Rust Ada `Fn`, `FnMut`, `FnOnce`?**
   - **Garbage Collector (GC) vs Zero-Cost Abstraction**: Di JavaScript, Python, atau Java, terdapat Garbage Collector (GC). Saat fungsi menangkap variabel luar, GC otomatis mengamankan variabel tersebut di Heap memory selamanya. Programmer tidak perlu memikirkan apakah variabel itu dipinjam atau dimutasi.
   - **Rust Tanpa GC**: Rust tidak memiliki runtime GC demi performa maksimal. Rust harus mengetahui secara pasti saat waktu kompilasi (*compile-time*): *Apakah variabel luar hanya dipinjam baca (`&`), dipinjam ubah (`&mut`), atau diambil kepemilikannya (`move`)?*
   - Oleh karena itu, Rust mendefinisikan 3 trait (`Fn`, `FnMut`, `FnOnce`) agar aturan Borrow Checker tetap berlaku 100% tanpa runtime overhead!

4. **Rahasia di Balik Layar: Compiler Menghasilkan Struct Anonim Unik**:
   Saat Anda menulis:
   ```rust
   let factor = 3;
   let multiply = |x: i32| x * factor;
   ```
   Secara internal, compiler Rust membuat sebuah `struct` anonim unik tak kasat mata:
   ```rust
   struct ClosureEnv<'a> {
       factor: &'a i32, // memegang referensi ke variabel factor
   }

   impl<'a> ClosureEnv<'a> {
       fn call(&self, x: i32) -> i32 {
           x * (*self.factor)
       }
   }
   ```

---

#### 9.1.2 Analogi Dunia Nyata: Tiga Kategori Trait Closure

Untuk mengingat ketiga trait ini dengan mudah, gunakan analogi kepemilikan sehari-hari:

| Trait Closure | Parameter Method Internal | Hak Akses Memori | Berapa Kali Bisa Dipanggil? | Analogi Dunia Nyata |
| :--- | :--- | :--- | :--- | :--- |
| **`Fn`** | `&self` | Meminjam secara immutable (`&`). Hanya membaca data lingkungan. | **Berkali-kali** (bahkan paralel lintas thread). | **Membaca Buku di Perpustakaan**: Buku tidak dicoret/rusak. Banyak orang boleh membaca berkali-kali bersamaan. |
| **`FnMut`** | `&mut self` | Meminjam secara mutable (`&mut`). Mengubah nilai data lingkungan. | **Berkali-kali** (harus berurutan/eksklusif). | **Buku Catatan Harian**: Anda mencoret dan menambah catatan baru. Bisa dipakai berulang kali, tapi harus antre saat menulis. |
| **`FnOnce`** | `self` | Mengambil ownership (*moves*). Mengonsumsi/menghancurkan data. | **Hanya 1 Kali**. | **Memakan Sepotong Kue / Membakar Surat**: Begitu kue dimakan atau surat dibakar, bendanya lenyap. Anda tidak bisa memakannya untuk kedua kali! |

---

#### 9.1.3 Hirarki dan Definisi Trait Closure di Rust
Rust Standard Library membagi closure ke dalam 3 trait fundamental:

```text
       FnOnce (paling umum: mengambil kepemilikan / moves, dipanggil 1 kali)
         ^
         |  (super-trait: pub trait FnMut<Args>: FnOnce<Args>)
       FnMut  (meminjam mutable &mut, bisa memutasi state, dipanggil berkali-kali)
         ^
         |  (super-trait: pub trait Fn<Args>: FnMut<Args>)
       Fn     (hanya meminjam immutable &, aman dipanggil berkali-kali secara concurrent)
```

**Aturan Hirarki Trait:**
1. **`FnOnce`**: Menerima parameter `self` by value (`fn call_once(self, args: Args) -> Self::Output`). Karena mengambil ownership `self`, closure ini mengonsumsi dirinya sendiri saat dipanggil dan hanya bisa dieksekusi **satu kali**.
2. **`FnMut`**: Menerima `&mut self` (`fn call_mut(&mut self, args: Args) -> Self::Output`). Mengizinkan pemutasi variabel internal/eksternal dan dapat dipanggil **berulang kali**.
3. **`Fn`**: Menerima `&self` (`fn call(&self, args: Args) -> Self::Output`). Hanya membaca variabel lingkungan secara immutable (`&`). Aman dipanggil **berulang kali**, bahkan secara concurrent dari multiple thread.

> **Hubungan Subtyping/Super-trait:**
> - Semua closure yang mengimplementasikan `Fn` otomatis mengimplementasikan `FnMut` dan `FnOnce`.
> - Semua closure yang mengimplementasikan `FnMut` otomatis mengimplementasikan `FnOnce`.
> - Closure yang mengonsumsi atau mendestruksi kepemilikan data **hanya** mengimplementasikan `FnOnce`.

**Dari Mana Sumber Trait Ini? (`std::ops` & Rust Standard Prelude)**  
Ketiga trait closure didefinisikan secara resmi di Standard Library Rust pada modul `std::ops` (atau `core::ops::function`):
- `std::ops::Fn`
- `std::ops::FnMut`
- `std::ops::FnOnce`

> **Mengapa Kita Tidak Perlu Menulis `use std::ops::Fn;`?**  
> Karena ketiga trait ini terdaftar di dalam **Rust Standard Prelude**. Rust otomatis meng-import trait-trait fundamental ini ke dalam setiap file `.rs` sejak awal kompilasi (bersama tipe dasar seperti `Option`, `Result`, `Vec`, `String`, dan trait seperti `Clone`, `Copy`, `Iterator`).

Definisi formal di dalam Standard Library Rust:
```rust
pub trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

---

#### 9.1.4 Empat Mode Capture pada Closure

1. **Closure Tanpa Capture (Zero-Sized Environment)**:
   - Closure tidak menangkap variabel apapun dari scope luar.
   - Ukuran struct di memori adalah 0 byte (`ZST`).
   - Memenuhi `Fn`, `FnMut`, dan `FnOnce`.
   - **Coercion**: Dapat di-*coerce* langsung menjadi function pointer murni (`fn(A, B) -> R`).
   ```rust
   let add = |a: i32, b: i32| a + b;
   let fn_ptr: fn(i32, i32) -> i32 = add; // Coercion sukses
   ```

2. **Closure Capture Immutable (`&T`)**:
   - Closure menangkap variabel lingkungan dengan meminjam secara baca-saja (*read-only reference*).
   - Variabel luar tetap valid dan dapat dibaca secara paralel karena hanya shared borrow `&` yang aktif.
   - Mengimplementasikan `Fn`, `FnMut`, dan `FnOnce`.
   ```rust
   let prefix = String::from("Rust");
   let greet = |name: &str| format!("{prefix}: {name}"); // capture &prefix
   ```

3. **Closure Capture Mutable (`&mut T`)**:
   - Closure meminjam variabel lingkungan secara eksklusif (*mutable reference*).
   - Pemanggilan closure memutasi state variabel tersebut.
   - Variabel closure **wajib** dideklarasikan dengan `let mut`.
   - Mengimplementasikan `FnMut` dan `FnOnce` (TIDAK mengimplementasikan `Fn`).
   ```rust
   let mut count = 0;
   let mut increment = || { count += 1; count }; // capture &mut count
   ```

4. **Keyword `move` (Ownership Transfer)**:
   - Menambahkan kata kunci `move` memaksa closure mengambil kepemilikan penuh (*takes ownership*) dari variabel yang ditangkap, bukan meminjamnya.
   - Sangat penting saat closure dikembalikan dari fungsi (`impl Fn`) atau dikirim ke thread lain (`std::thread::spawn`).
   - **Miskonsepsi Umum**: `move` tidak selalu membuat closure menjadi `FnOnce`!
     - Jika closure bertanda `move` hanya membaca data yang dipindahkan, closure tersebut **tetap mengimplementasikan `Fn`** dan dapat dipanggil berulang kali.
     - Jika closure bertanda `move` mendestruksi/mengonsumsi data tersebut (misal memanggil `drop(data)` atau memindahkan nilai keluar), barulah closure tersebut menjadi **`FnOnce`** murni.
   ```rust
   // Tetap Fn (bisa dipanggil berkali-kali)
   let text = String::from("Halo");
   let reader = move |name: &str| format!("{text} {name}");

   // Menjadi FnOnce murni (hanya bisa dipanggil 1x)
   let data = vec![1, 2, 3];
   let consumer = move || {
       let _owned = data; // data dipindahkan / dikonsumsi di sini
   };
   ```

---

#### 9.1.5 Higher-Order Functions (HOF) dengan Generic Trait Bounds

Closure umum digunakan sebagai argumen fungsi tingkat tinggi (Higher-Order Functions):

```rust
// 1. Menerima Fn: Operasi murni/read-only yang aman dipanggil berkali-kali
pub fn transform_elements<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(transform).collect()
}

// 2. Menerima FnMut: Operasi stateful atau agregasi/akumulator
pub fn aggregate_stateful<T, S, F>(items: &[T], mut initial: S, mut accumulator: F) -> S
where
    F: FnMut(&mut S, &T),
{
    for item in items {
        accumulator(&mut initial, item);
    }
    initial
}

// 3. Menerima FnOnce: Operasi transfer resource atau callback sekali panggil
pub fn execute_and_consume<T, R, F>(resource: T, consumer: F) -> R
where
    F: FnOnce(T) -> R,
{
    consumer(resource)
}
```

##### Deep Dive Generics pada HOF: `<T, U, F>`, Klausa `where`, dan `impl Trait`

1. **Apakah Tanda `<...>` di Rust Selalu Menandakan Generics?**  
   **Ya.** Tanda kurung siku `<...>` adalah penampung parameter generik yang dapat diisi oleh 3 jenis entitas:
   - **Type Generics**: `<T>`, `<U>` (placeholder untuk sembarang tipe data konkret).
   - **Lifetime Generics**: `<'a>`, `<'b>` (placeholder untuk durasi masa hidup referensi).
   - **Const Generics**: `<const N: usize>` (placeholder untuk nilai konstan saat compile-time, misal ukuran array `[i32; N]`).

2. **Mengapa Ada Banyak Parameter Tipe Sekaligus Seperti `<T, U, F>`?**  
   Huruf kapital tunggal adalah konvensi penamaan standar Rust:
   - `T` (*Type*): Tipe data input utama (misal `i32`).
   - `U` (*Unique / Unit*): Tipe data output atau hasil konversi (misal `String`).
   - `F` (*Function*): Tipe closure atau fungsi yang dioperasikan.
   - `K, V` (*Key, Value*): Pasangan kunci dan nilai (misal pada `HashMap<K, V>`).
   - `E` (*Error*): Tipe kegagalan/kesalahan (misal pada `Result<T, E>`).
   - `R` (*Return*): Tipe kembalian umum.

   > **Mengapa butuh `<T, U, F>` terpisah?**  
   > Jika hanya memakai satu huruf `T`, fungsi `transform` akan memaksa input dan output bertipe sama (`Vec<T> -> Vec<T>`). Dengan adanya `U`, fungsi menjadi 100% fleksibel: input bisa `i32` dan output bisa `String`! Dan karena setiap closure di Rust memiliki tipe struct unik yang digenerate compiler, kita butuh `F` agar compiler tahu ada tipe callable unik yang dimasukkan.

3. **Mengapa Menggunakan Klausa `where`?**  
   Klausa `where` digunakan untuk mendefinisikan **Trait Bounds** (persyaratan kontrak trait) bagi parameter generic.  
   Pernyataan `where F: Fn(&T) -> U` memberitahu compiler: *"Tipe `F` boleh bertipe apa saja, ASALKAN ia mengimplementasikan trait `Fn` yang menerima referensi `&T` dan mengembalikan nilai bertipe `U`."*

4. **Tiga Gaya Penulisan Ekuivalen (`impl Trait` vs Generic Inline vs `where`):**  
   Ketiga bentuk berikut menghasilkan kode biner mesin yang sama persis (*monomorphization / zero-cost static dispatch*):
   ```rust
   // Gaya 1: impl Trait (paling ringkas jika hanya 1 parameter closure)
   fn apply(f: impl Fn(i32) -> i32) { ... }

   // Gaya 2: Generic Inline di dalam <>
   fn apply<F: Fn(i32) -> i32>(f: F) { ... }

   // Gaya 3: Klausa where (paling rapi saat signature memiliki banyak generic)
   fn apply<F>(f: F) where F: Fn(i32) -> i32 { ... }
   ```

5. **Kapan Wajib Menggunakan `impl Trait`?**  
   Saat **mengembalikan closure dari sebuah fungsi** (*return type*):
   ```rust
   fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
       move |x| x * factor
   }
   ```
   Karena compiler Rust membuat nama struct anonim tersembunyi untuk setiap closure yang tidak bisa kita ketik namanya di kode, kita **wajib** menggunakan `-> impl Fn(...)` untuk memberitahu compiler bahwa fungsi mengembalikan sebuah objek yang mengimplementasikan trait `Fn`.

---

#### 9.1.6 Implementasi Lengkap Modul `fase9_task_1.rs`

Berikut adalah kode sumber lengkap yang telah diuji dan diintegrasikan pada `rust-learning-lab/src/fase9_task_1.rs`:

```rust
// Fase 9 - Task 1: Closures Mendalam (Captures, Move Keyword, Fn, FnMut, FnOnce)
// Rujukan: rust_learning_guide.md (Sub-bab 9.1) & rust_execution_tasks.md (L794-L800)

// ----------------------------------------------------------------------------
// 1. Helper Verifikasi Trait Kategori (Fn, FnMut, FnOnce)
// ----------------------------------------------------------------------------

pub fn verify_fn<F, R>(f: F) -> R
where
    F: Fn() -> R,
{
    f()
}

pub fn verify_fn_mut<F, R>(mut f: F) -> R
where
    F: FnMut() -> R,
{
    f()
}

pub fn verify_fn_once<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

// ----------------------------------------------------------------------------
// 2. Closure Tanpa Capture (Zero-sized Environment & Fn Pointer Coercion)
// ----------------------------------------------------------------------------

pub fn create_no_capture_closure() -> impl Fn(i32, i32) -> i32 {
    |a: i32, b: i32| a + b
}

pub fn call_as_fn_pointer(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

// ----------------------------------------------------------------------------
// 3. Closure Capture Immutable (&T)
// ----------------------------------------------------------------------------

pub fn demonstrate_immutable_capture(prefix: &str, items: &[&str]) -> Vec<String> {
    let formatter = |item: &str| format!("{prefix}: {item}");

    let mut results = Vec::new();
    for &item in items {
        results.push(formatter(item));
    }
    results
}

// ----------------------------------------------------------------------------
// 4. Closure Capture Mutable (&mut T)
// ----------------------------------------------------------------------------

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

pub fn demonstrate_mutable_capture(counter: &mut SimpleCounter, steps: &[i32]) {
    let mut step_recorder = |amount: i32| {
        counter.total += amount;
        counter.operations += 1;
    };

    for &step in steps {
        step_recorder(step);
    }
}

// ----------------------------------------------------------------------------
// 5. Keyword `move`: Pemindahan Kepemilikan (Ownership Transfer)
// ----------------------------------------------------------------------------

pub fn create_move_reader(prefix: String) -> impl Fn(&str) -> String {
    move |target: &str| format!("{prefix} -> {target}")
}

pub fn create_move_consumer(data: Vec<String>) -> impl FnOnce() -> (usize, String) {
    move || {
        let count = data.len();
        let combined = data.into_iter().collect::<Vec<_>>().join(", ");
        (count, combined)
    }
}

// ----------------------------------------------------------------------------
// 6. Higher-Order Functions (HOF) dengan Generic Trait Bounds
// ----------------------------------------------------------------------------

pub fn transform_elements<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(transform).collect()
}

pub fn aggregate_stateful<T, S, F>(items: &[T], mut initial: S, mut accumulator: F) -> S
where
    F: FnMut(&mut S, &T),
{
    for item in items {
        accumulator(&mut initial, item);
    }
    initial
}

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
    println!("   - pure_fn lewat verify_fn_mut: {}", verify_fn_mut(pure_fn));
    println!("   - pure_fn lewat verify_fn_once: {}", verify_fn_once(pure_fn));

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
    println!("   - once_fn lewat verify_fn_once: {}", verify_fn_once(once_fn));

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
```

---

#### 9.1.7 Hasil Eksekusi Output Terminal Task 1

```text
=== FASE 9: Functional Rust - Task 1 (Closures) ===

1. Closure Tanpa Capture:
   - add(15, 27) = 42
   - Coercion ke fn pointer: 6 * 7 = 42
   - Verifikasi trait: Fn tanpa capture valid

2. Closure Capture Immutable (&T):
   - Hasil capture immutable: ["TAG: alpha", "TAG: beta", "TAG: gamma"]
   - Tag di luar closure tetap utuh: "TAG"

3. Closure Capture Mutable (&mut T):
   - Counter setelah mutasi: total = 60, operations = 3

4. Keyword `move`:
   - Move-reader call 1: SISTEM -> INIT
   - Move-reader call 2: SISTEM -> READY
   - Move-consumer FnOnce result: count = 2, data = "data1, data2"

5. Hirarki & Penentuan Trait Closure:
   - pure_fn lewat verify_fn: 200
   - pure_fn lewat verify_fn_mut: 200
   - pure_fn lewat verify_fn_once: 200
   - mut_fn lewat verify_fn_mut: 15
   - once_fn lewat verify_fn_once: 20

6. Demonstrasi Higher-Order Functions (HOF):
   - HOF transform (Fn): [1, 4, 9, 16, 25]
   - HOF aggregate (FnMut): sum = 15
   - HOF execute_and_consume (FnOnce): sum = 60

[OK] Task 1 Closures selesai & terverifikasi.
```

---

### 9.2 Iterators Mendalam (Lazy Pipeline, Adaptors, dan Terminal Operations)

Iterator di Rust adalah pola desain yang memungkinkan Anda melintasi urutan elemen (*sequence of items*) satu per satu. Di Rust, iterator bersifat **zero-cost abstraction**: kompilasi pipeline iterator sering kali menghasilkan kode mesin assembly yang secepat atau bahkan lebih cepat daripada loop `for` manual karena optimasi unrolling dan inlining LLVM tanpa pemeriksaan batas berulang (*bounds checking*).

---

#### 9.2.1 Konsep Inti Trait `Iterator` & Evaluasi Malas (*Lazy Evaluation*)

Semua iterator di Rust dibangun di atas satu trait fundamental dari Standard Library:

```rust
pub trait Iterator {
    type Item; // Associated Type: tipe elemen yang dihasilkan

    // Satu-satunya method yang wajib diimplementasikan
    fn next(&mut self) -> Option<Self::Item>;

    // Puluhan method default lainnya (map, filter, take, fold, dll.)
    // dibangun otomatis di atas next()!
}
```

> **Hukum Evaluasi Malas (Lazy Evaluation):**  
> Di Rust, iterator bersifat **malas (*lazy*)**. Menulis `v.iter().map(...)` atau `.filter(...)` **tidak melakukan komputasi apapun** pada elemen! Komputasi baru benar-benar berjalan ketika ada method pemanggil (*consuming adaptor*) seperti `.next()`, `.collect()`, atau `.fold()` yang memaksa iterator menarik elemen keluar.

---

#### 9.2.2 Tiga Pintu Masuk Iterator dari Koleksi

Cara kita membuat iterator menentukan hak kepemilikan memori terhadap elemen koleksi:

| Method Koleksi | Tipe Elemen (`Item`) | Hak Akses Memori | Status Koleksi Asli Setelah Iterasi |
| :--- | :--- | :--- | :--- |
| **`.iter()`** | `&T` | **Immutable Borrow**: Hanya membaca elemen tanpa modifikasi. | **Tetap Utuh**: Masih bisa dipakai dan dibaca kembali. |
| **`.iter_mut()`** | `&mut T` | **Mutable Borrow**: Meminjam referensi eksklusif untuk mengubah elemen. | **Tetap Utuh Terbarui**: Nilai elemen berubah secara *in-place*. |
| **`.into_iter()`** | `T` | **Ownership Transfer**: Mengambil kepemilikan elemen langsung (*moves*). | **Dikonsumsi/Lenyap**: Koleksi asli hancur dan tidak bisa diakses lagi. |

```rust
let mut numbers = vec![1, 2, 3];

// 1. .iter() -> &i32
for &n in numbers.iter() { /* cuma baca */ }

// 2. .iter_mut() -> &mut i32
for n in numbers.iter_mut() { *n += 10; } // in-place update

// 3. .into_iter() -> i32 (numbers dipindahkan kepemilikannya)
let doubled: Vec<i32> = numbers.into_iter().map(|n| n * 2).collect();
// numbers sudah tidak bisa dipanggil lagi di bawah sini!
```

---

#### 9.2.3 Iterator Adaptors (Lazy Transformers)

*Iterator Adaptors* adalah method yang menerima sebuah iterator dan mengembalikan iterator baru yang ditransformasi. Method ini **tidak mengeksekusi iterasi**, melainkan hanya menyusun *blueprint* pipa pengolahan:

1. **`.map(closure)`**: Mengubah setiap elemen dari tipe `T` menjadi `U`.
2. **`.filter(predicate)`**: Menyaring elemen, hanya meneruskan elemen yang menghasilkan `true`.
3. **`.take(n)`**: Menghentikan iterasi setelah menghasilkan `n` elemen pertama.
4. **`.copied()`**: Mengubah iterator referensi `&T` menjadi iterator nilai murni `T` dengan menyalin nilai elemen (*bitwise copy*). Khusus untuk tipe yang mengimplementasikan trait `Copy` (seperti `i32`, `f64`, `bool`, `char`).
5. **`.cloned()`**: Versi umum dari `.copied()` untuk tipe data yang mengimplementasikan trait `Clone` (misal `String`, `Vec<T>`). Menghasilkan nilai kepemilikan baru dengan menduplikasi data (bisa memicu alokasi heap).

##### Mengapa `.copied()` Sangat Sering Digunakan di Pipeline?

Saat Anda memanggil `numbers.iter()`, tipe elemen yang mengalir di dalam iterator adalah referensi: `&i32`.

Tanpa `.copied()`:
```rust
let numbers = vec![10, 20, 30, 40];

// 1. filter menerima referensi ke item (&Item), sehingga jadi referensi ganda: &&i32!
// Kita terpaksa menulis pattern canggung |&&x| atau |&x| *x > 25
let filtered: Vec<&i32> = numbers.iter().filter(|&&x| x > 25).collect();

// 2. max() mengembalikan referensi Option<&i32>, bukan angka mandiri Option<i32>
let max_val: Option<&i32> = numbers.iter().max();
```

Dengan `.copied()`:
```rust
let numbers = vec![10, 20, 30, 40];

// .copied() langsung melepas lapisan referensi (&i32 -> i32) secara zero-cost
let filtered: Vec<i32> = numbers
    .iter()
    .copied()                    // Mengalirkan i32 murni
    .filter(|&x| x > 25)         // Cukup &x sederhana (bukan &&x)
    .collect();                  // Menghasilkan Vec<i32> murni

let max_val: Option<i32> = numbers.iter().copied().max(); // Langsung Option<i32>
```

> **Aturan Praktis:**
> - Jika koleksi berisi angka/tipe primitif (`Copy`) dan Anda ingin mengumpulkan nilai murni atau mencari `max()`/`min()`, selalu sisipkan `.copied()` tepat setelah `.iter()`.
> - Jika koleksi berisi `String` atau struct dan Anda butuh kepemilikan baru tanpa menghancurkan koleksi lama, gunakan `.cloned()`.

---

#### 9.2.4 Consuming Adaptors (Terminal Eager Operations)

*Consuming Adaptors* adalah operasi terminal yang mengonsumsi iterator dan memicu proses eksekusi loop di balik layar:

1. **`.collect()`**: Mengumpulkan hasil iterasi ke dalam koleksi target (misal `Vec<T>`, `HashSet<T>`, atau `HashMap<K, V>`). Sering membutuhkan anotasi tipe target atau sintaks *turbofish* `collect::<Vec<_>>()`.
2. **`.find(predicate)`**: Mencari elemen pertama yang memenuhi syarat. Bersifat *short-circuiting* (berhenti seketika saat elemen ditemukan), mengembalikan `Option<Item>`.
3. **`.any(predicate)`**: Mengembalikan `true` jika ada **minimal 1 elemen** yang cocok (*short-circuiting*).
4. **`.all(predicate)`**: Mengembalikan `true` jika **seluruh elemen** cocok (*short-circuiting*).
5. **`.fold(init, closure)`**: Mereduksi seluruh elemen menjadi nilai tunggal menggunakan akumulator awal: `fold(initial_state, |acc, item| acc + item)`.

---

#### 9.2.5 Custom Iterator: Implementasi Mandiri

Untuk membuat iterator sendiri, kita cukup membuat struct dan mengimplementasikan `next(&mut self) -> Option<Item>`:

```rust
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        let result = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}
```

---

#### 9.2.6 Implementasi Lengkap Modul `fase9_task_2.rs`

Berikut kode modul lengkap yang selaras 100% pada `rust-learning-lab/src/fase9_task_2.rs`:

```rust
// Fase 9 - Task 2: Lazy Iterators, Iterator Adaptors, dan Consuming Adaptors
// Rujukan: rust_learning_guide.md (Sub-bab 9.2) & rust_execution_tasks.md (L802-L825)

// ----------------------------------------------------------------------------
// 1. Tiga Metode Penghasil Iterator: iter(), iter_mut(), dan into_iter()
// ----------------------------------------------------------------------------

pub fn demonstrate_iter(items: &[i32]) -> Vec<i32> {
    items.iter().map(|&x| x * 2).collect()
}

pub fn demonstrate_iter_mut(items: &mut [i32], addition: i32) {
    for item in items.iter_mut() {
        *item += addition;
    }
}

pub fn demonstrate_into_iter(items: Vec<String>) -> Vec<String> {
    items
        .into_iter()
        .map(|s| format!("[Processed: {s}]"))
        .collect()
}

// ----------------------------------------------------------------------------
// 2. Iterator Pipeline: iter() -> filter() -> map() -> take() -> collect()
// ----------------------------------------------------------------------------

pub fn build_lazy_pipeline(
    numbers: &[i32],
    is_even: bool,
    multiplier: i32,
    limit: usize,
) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&x| if is_even { x % 2 == 0 } else { x % 2 != 0 })
        .map(|&x| x * multiplier)
        .take(limit)
        .collect()
}

// ----------------------------------------------------------------------------
// 3. Consuming Adaptors: find, any, all, fold, collect
// ----------------------------------------------------------------------------

pub fn find_first_gt<'a>(numbers: &'a [i32], threshold: i32) -> Option<&'a i32> {
    numbers.iter().find(|&&x| x > threshold)
}

pub fn verify_predicates(numbers: &[i32], min_val: i32) -> (bool, bool) {
    let has_negative = numbers.iter().any(|&x| x < 0);
    let all_above_min = numbers.iter().all(|&x| x >= min_val);
    (has_negative, all_above_min)
}

pub fn calculate_stats_with_fold(numbers: &[i32]) -> (i32, i64) {
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    let product = numbers.iter().fold(1i64, |acc, &x| acc * (x as i64));
    (sum, product)
}

// ----------------------------------------------------------------------------
// 4. Custom Iterator (Implementasi Trait Iterator Mandiri)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Fibonacci {
    pub curr: u64,
    pub next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        let result = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

// ----------------------------------------------------------------------------
// 5. Runner Interaktif (Demonstrasi Lengkap)
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== FASE 9: Functional Rust - Task 2 (Iterators) ===");

    let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 1. iter()
    println!("\n1. Menggunakan .iter() [Immutable Borrow]:");
    let doubled = demonstrate_iter(&source);
    println!("   - Source tetap utuh: {:?}", source);
    println!("   - Hasil doubled: {:?}", doubled);

    // 2. iter_mut()
    println!("\n2. Menggunakan .iter_mut() [Mutable Borrow In-Place]:");
    let mut mutable_data = vec![10, 20, 30];
    println!("   - Data sebelum: {:?}", mutable_data);
    demonstrate_iter_mut(&mut mutable_data, 5);
    println!("   - Data setelah (+5 in-place): {:?}", mutable_data);

    // 3. into_iter()
    println!("\n3. Menggunakan .into_iter() [Ownership Transfer]:");
    let words = vec![
        String::from("rust"),
        String::from("iterator"),
        String::from("zero-cost"),
    ];
    let processed_words = demonstrate_into_iter(words);
    println!("   - Hasil into_iter: {:?}", processed_words);

    // 4. Pipeline: iter() -> filter() -> map() -> take() -> collect()
    println!("\n4. Pipeline Lazy: filter -> map -> take -> collect:");
    let pipeline_res = build_lazy_pipeline(&source, true, 3, 3);
    println!("   - Input numbers: {:?}", source);
    println!("   - Filter Genap, Kali 3, Ambil 3: {:?}", pipeline_res);

    // 5. find()
    println!("\n5. Consuming Adaptor .find():");
    let found = find_first_gt(&source, 7);
    println!("   - Angka pertama > 7: {:?}", found);

    // 6. any() dan all()
    println!("\n6. Predikat .any() dan .all():");
    let (has_neg, all_ge_zero) = verify_predicates(&source, 0);
    println!("   - Mengandung angka negatif? {has_neg}");
    println!("   - Semua angka >= 0? {all_ge_zero}");

    // 7. fold()
    println!("\n7. Reduksi .fold():");
    let sample = [1, 2, 3, 4, 5];
    let (sum, prod) = calculate_stats_with_fold(&sample);
    println!("   - Data: {:?}", sample);
    println!("   - Sum via fold: {sum}");
    println!("   - Product via fold: {prod}");

    // 8. Custom Iterator Fibonacci
    println!("\n8. Custom Iterator (Fibonacci):");
    let fib_first_8: Vec<u64> = Fibonacci::new().take(8).collect();
    println!("   - 8 angka pertama Fibonacci: {:?}", fib_first_8);

    println!("\n[OK] Task 2 Iterators selesai & terverifikasi.");
}
```

---

#### 9.2.7 Hasil Eksekusi Output Terminal Task 2

```text
=== FASE 9: Functional Rust - Task 2 (Iterators) ===

1. Menggunakan .iter() [Immutable Borrow]:
   - Source tetap utuh: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   - Hasil doubled: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]

2. Menggunakan .iter_mut() [Mutable Borrow In-Place]:
   - Data sebelum: [10, 20, 30]
   - Data setelah (+5 in-place): [15, 25, 35]

3. Menggunakan .into_iter() [Ownership Transfer]:
   - Hasil into_iter: ["[Processed: rust]", "[Processed: iterator]", "[Processed: zero-cost]"]

4. Pipeline Lazy: filter -> map -> take -> collect:
   - Input numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   - Filter Genap, Kali 3, Ambil 3: [6, 12, 18]

5. Consuming Adaptor .find():
   - Angka pertama > 7: Some(8)

6. Predikat .any() dan .all():
   - Mengandung angka negatif? false
   - Semua angka >= 0? true

7. Reduksi .fold():
   - Data: [1, 2, 3, 4, 5]
   - Sum via fold: 15
   - Product via fold: 120

8. Custom Iterator (Fibonacci):
   - 8 angka pertama Fibonacci: [0, 1, 1, 2, 3, 5, 8, 13]

[OK] Task 2 Iterators selesai & terverifikasi.
```

---

### 9.3 Mini Project Fase 9: Statistics Processor (Arsitektur Pipeline Fungsional)

Mini Project Fase 9 menggabungkan seluruh konsep closure dan iterator ke dalam sistem pengolah statistik numerik yang idiomatic, aman, dan deklaratif. Tanpa loop `for` manual, seluruh kalkulasi dilakukan menggunakan pipeline iterator adaptors yang bersih dan efisien (*zero-cost abstraction*).

---

#### 9.3.1 Analisis Desain Pipeline & 7 Metrik Statistik

Sistem memproses input data `[10, 20, 11, 30, 40, 21, 50]` untuk menghasilkan 7 indikator statistik utama:

1. **Angka Genap (`filter`)**:
   - `data.iter().copied().filter(|&x| x % 2 == 0).collect()`
   - Hasil: `[10, 20, 30, 40, 50]`
2. **Angka di Atas Ambang Batas / Threshold (`filter`)**:
   - Menggunakan closure yang menangkap variabel `threshold`: `data.iter().copied().filter(|&x| x > threshold).collect()`
   - Untuk threshold = 25, hasil: `[30, 40, 50]`
3. **Square / Kuadrat (`map`)**:
   - `data.iter().map(|&x| (x as i64) * (x as i64)).collect()`
   - Dikonversi ke `i64` untuk mencegah overflow integer.
   - Hasil: `[100, 400, 121, 900, 1600, 441, 2500]`
4. **Sum / Total Penjumlahan (`sum`)**:
   - `data.iter().map(|&x| x as i64).sum()`
   - Total: `182`
5. **Average / Rata-rata (`f64` division & `Option`)**:
   - Menghindari *panic zero-division* dengan membungkus hasil dalam `Option<f64>`. Jika slice kosong, mengembalikan `None`.
   - Hasil: `Some(26.00)`
6. **Maximum (`max`)**:
   - Menggunakan iterator consuming adaptor `.max()`, mengembalikan `Option<i32>`.
   - Hasil: `Some(50)`
7. **Minimum (`min`)**:
   - Menggunakan iterator consuming adaptor `.min()`, mengembalikan `Option<i32>`.
   - Hasil: `Some(10)`

---

#### 9.3.2 Pembuktian & Evaluasi Kriteria Lulus Fase 9

Untuk memastikan kelulusan Fase 9, tiga pengujian fundamental diuji secara komputasional:

1. **Membedakan 3 Mode Iterasi**:
   - `.iter()`: Meminjam immutable (`&T`). Koleksi asal tetap utuh dan valid.
   - `.iter_mut()`: Meminjam mutable (`&mut T`). Mengubah elemen langsung secara in-place.
   - `.into_iter()`: Mengambil ownership (`T`). Koleksi asal dikonsumsi / dipindahkan.

2. **Membuktikan Karakteristik Laziness Iterator**:
   - Disusun pipeline `numbers.iter().map(...)` dengan counter pemantau langkah.
   - Sebelum terminal method dipanggil, counter langkah bernilai `0` (tidak ada komputasi yang jalan!).
   - Saat dipanggil `.take(2).collect()`, hanya **2 langkah** yang dieksekusi, meskipun array berisi 5 elemen. Komputasi hanya dilakukan sesuai kebutuhan (*pull-based*).

3. **Membuktikan 3 Kategori Trait Closure (`Fn`, `FnMut`, `FnOnce`)**:
   - `Fn`: Closure hanya membaca variabel lingkungan (`greeting.len()`). Aman dipanggil berkali-kali.
   - `FnMut`: Closure memutasi state luar (`counter += 5`). Variabel closure wajib `let mut`.
   - `FnOnce`: Closure mengonsumsi data via ownership move (`data.into_iter().sum()`). Hanya bisa dieksekusi satu kali.

---

#### 9.3.3 Implementasi Kode Lengkap `mini_project_9.rs`

Berikut adalah kode sumber lengkap yang telah diuji dan diintegrasikan pada `rust-learning-lab/src/mini_project_9.rs`:

```rust
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
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[i32] {
        &self.data
    }

    pub fn evens(&self) -> Vec<i32> {
        self.data.iter().copied().filter(|&x| x % 2 == 0).collect()
    }

    pub fn above_threshold(&self, threshold: i32) -> Vec<i32> {
        self.data.iter().copied().filter(|&x| x > threshold).collect()
    }

    pub fn squares(&self) -> Vec<i64> {
        self.data
            .iter()
            .map(|&x| (x as i64) * (x as i64))
            .collect()
    }

    pub fn sum(&self) -> i64 {
        self.data.iter().map(|&x| x as i64).sum()
    }

    pub fn average(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.sum() as f64 / self.data.len() as f64)
        }
    }

    pub fn maximum(&self) -> Option<i32> {
        self.data.iter().copied().max()
    }

    pub fn minimum(&self) -> Option<i32> {
        self.data.iter().copied().min()
    }

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

    pub fn custom_filter<F>(&self, predicate: F) -> Vec<i32>
    where
        F: Fn(&i32) -> bool,
    {
        self.data.iter().copied().filter(predicate).collect()
    }

    pub fn custom_transform<U, F>(&self, mapper: F) -> Vec<U>
    where
        F: Fn(i32) -> U,
    {
        self.data.iter().copied().map(mapper).collect()
    }
}

pub fn verify_iteration_modes() -> (&'static str, &'static str, &'static str) {
    let mode_iter = "iter(): meminjam immutable (&T), koleksi asal tetap utuh.";
    let mode_iter_mut = "iter_mut(): meminjam mutable (&mut T), mutasi elemen in-place.";
    let mode_into_iter = "into_iter(): mengonsumsi koleksi (T), ownership dipindahkan.";
    (mode_iter, mode_iter_mut, mode_into_iter)
}

pub fn verify_iterator_laziness() -> usize {
    use std::cell::Cell;
    let step_count = Cell::new(0);
    let numbers = [1, 2, 3, 4, 5];

    let pipeline = numbers.iter().map(|&x| {
        step_count.set(step_count.get() + 1);
        x * 2
    });

    assert_eq!(step_count.get(), 0);

    let _: Vec<_> = pipeline.take(2).collect();
    step_count.get()
}

pub fn verify_closure_traits() -> (&'static str, i32, usize) {
    let greeting = String::from("Halo");
    let fn_read = || greeting.len();
    let res_fn = fn_read();

    let mut counter = 10;
    let mut fn_mut = || {
        counter += 5;
        counter
    };
    let res_fn_mut = fn_mut();

    let data = vec![100, 200, 300];
    let fn_once = move || data.into_iter().sum::<i32>();
    let res_fn_once = fn_once();

    ("Fn: baca saja", res_fn_mut, res_fn + (res_fn_once as usize))
}

pub fn run() {
    println!("=== Mini Project Fase 9: Statistics Processor ===");

    let sample_input = vec![10, 20, 11, 30, 40, 21, 50];
    let processor = StatisticsProcessor::new(sample_input);
    let threshold = 25;

    let report = processor.generate_report(threshold);
    println!("{report}");

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
```

---

#### 9.3.4 Hasil Eksekusi Output Terminal Mini Project 9

```text
=== Mini Project Fase 9: Statistics Processor ===
--- Statistics Processor Report ---
Input Data         : [10, 20, 11, 30, 40, 21, 50]
1. Angka Genap     : [10, 20, 30, 40, 50]
2. Angka > 25 (th) : [30, 40, 50]
3. Square (Kuadrat): [100, 400, 121, 900, 1600, 441, 2500]
4. Sum (Jumlah)    : 182
5. Average (Rata)  : 26.00
6. Maximum         : 50
7. Minimum         : 10

Custom Pipeline Demo (slice length: 7):
   - Kelipatan 10: [10, 20, 30, 40, 50]
   - Format Label: ["N:10", "N:20", "N:11", "N:30", "N:40", "N:21", "N:50"]

--- Evaluasi & Bukti Kriteria Lulus Fase 9 ---
1. Pemahaman Mode Iterasi:
   - iter(): meminjam immutable (&T), koleksi asal tetap utuh.
   - iter_mut(): meminjam mutable (&mut T), mutasi elemen in-place.
   - into_iter(): mengonsumsi koleksi (T), ownership dipindahkan.

2. Bukti Laziness Iterator:
   - Diprogram untuk ambil 2 elemen (take(2)) dari 5.
   - Jumlah langkah eksekusi aktual: 2 langkah (bukan 5!).

3. Bukti Kategori Fn / FnMut / FnOnce:
   - Fn: baca saja
   - FnMut mutasi counter: 15
   - FnOnce mengonsumsi Vec: checksum = 604

[OK] Mini Project Fase 9 selesai & terverifikasi.
```

---

# BAGIAN III: MEMORY, CONCURRENCY, & ASYNCHRONOUS SYSTEMS

## FASE 10: Smart Pointers & Interior Mutability

### 10.1 Konsep Fundamental Smart Pointer & Alur Pedagogis Memori

Di Rust, perbedaan antara *reference* biasa dan *smart pointer* terletak pada kemampuan pengelolaan kepemilikan (*ownership*), metadata tambahan, dan implementasi dua trait kunci: `Deref` dan `Drop`.

| Jenis Pointer | Kepemilikan (*Ownership*) | Lokasi Target | Overhead Runtime | Trait Kunci | Kasus Penggunaan Utama |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Reference (`&T`, `&mut T`)** | Tidak (hanya meminjam) | Stack atau Heap | Zero-cost (pointer mentah) | Copy | Akses memori sementara tanpa alokasi baru. |
| **`Box<T>`** | Ya (eksklusif / single owner) | Heap | Sangat kecil (alokasi heap) | `Deref`, `Drop` | Ukuran dinamis/rekursif, hindari stack copy data besar. |
| **`Rc<T>`** | Ya (shared / reference counted) | Heap | Increment/decrement counter | `Deref`, `Drop` | Multi-ownership pada thread tunggal (*single-thread*). |
| **`Arc<T>`** | Ya (atomic shared ownership) | Heap | Atomic memory ordering | `Deref`, `Drop` | Multi-ownership aman lintas thread OS (*concurrency*). |
| **`RefCell<T>`** | Mengelola mutabilitas interior | Stack / Heap | Runtime borrow checking | `Deref`, `Drop` | Mutasi data di balik `&T` (aturan borrow dicek runtime). |
| **`Mutex<T>` / `RwLock<T>`** | Mengunci akses data | Stack / Heap | Lock syscall / futex | `Deref`, `Drop` | Akses mutasi aman lintas thread secara sinkron. |

---

### 10.2 `Box<T>`: Heap Allocation & Analisis Memori Stack vs Heap

`Box<T>` adalah smart pointer paling sederhana di Rust. Box mengalokasikan data ke memori **Heap**, sementara pointer alamatnya disimpan di **Stack**.

#### 1. Visualisasi Memori Stack vs Heap

Ketika Anda menulis `let b = Box::new(42);`:

```text
┌───────────────────────────┐                ┌───────────────────────────┐
│        STACK MEMORY       │                │        HEAP MEMORY        │
│                           │                │                           │
│  Variabel `b`             │                │  Payload Nilai Sebenarnya │
│  ┌─────────────────────┐  │                │  ┌─────────────────────┐  │
│  │ Pointer: 0x7ffd9a00 ├──┼───────────────┼─>│ Nilai i32: 42       │  │
│  │ Ukuran: 8 bytes     │  │ (Menunjuk ke)  │  │ Ukuran: 4 bytes     │  │
│  └─────────────────────┘  │                │  └─────────────────────┘  │
└───────────────────────────┘                └───────────────────────────┘
```

- **Ukuran di Stack**: Pada arsitektur 64-bit, ukuran variabel `Box<T>` di stack selalu tepat **8 bytes** (`usize`), apapun tipe `T` di dalamnya.
- **Ukuran di Heap**: Memori di heap dialokasikan persis sebesar `size_of::<T>()`.
- **Otomatis Bebas (Zero Memory Leak)**: Ketika variabel `b` keluar dari scope (*out of scope*), trait `Drop` otomatis dipanggil untuk membebaskan (*deallocate*) memori di heap tanpa perlu memanggil `free()` manual seperti di C.

#### 2. Kapan Menggunakan `Box<T>`?
1. **Mencegah Stack Overflow untuk Data Berukuran Besar**:
   Stack thread biasanya dibatasi (misal 2 MB di Linux). Jika membuat array `[u8; 1024 * 1024]` (1 MB) langsung di stack, beberapa frame fungsi dapat menyebabkan stack overflow. Dengan `Box`, data 1 MB tersebut hidup aman di Heap.
2. **Transfer Kepemilikan Cepat Tanpa Copy Memori Besar**:
   Memindahkan (*move*) `Box<LargeBuffer>` antar-fungsi hanya menyalin pointer 8 byte di stack, bukan menyalin seluruh payload megabyte di heap.
3. **Recursive Types**: Menyediakan ukuran tipe yang pasti pada waktu kompilasi (*compile-time sized*).
4. **Trait Objects (`Box<dyn Trait>`)**: Menyimpan berbagai tipe berbeda yang mengimplementasikan trait yang sama dalam satu koleksi (polimorfisme dinamis).

---

### 10.3 Recursive Types: Masalah Ukuran Kompilasi (`Sized`) & Solusi Indirection

#### 1. Apa Itu Tipe Data Rekursif? (Mental Model Ramah Pemula)
**Tipe data rekursif** adalah tipe data yang **di dalam dirinya memuat nilai dari tipe dirinya sendiri**.

* **Analogi Boneka Rusia (Matryoshka)**:
  Buka satu boneka besar $\to$ di dalamnya ada boneka yang sama $\to$ buka lagi ada boneka lagi $\to$ sampai ke boneka terkecil (ujung dasar/basis).
* **Analogi Folder di Komputer**:
  Sebuah `Folder` bisa berisi file, namun juga bisa berisi `Folder` lain di dalamnya.
* **Analogi Gerbong Kereta (Linked List)**:
  Setiap gerbong membawa muatan dan **menggandeng gerbong berikutnya** (yang tipenya sama-sama gerbong), hingga gerbong terakhir yang tidak menggandeng apapun (`Nil`).
* **Pohon Ekspresi Matematika (AST)**:
  Ekspresi `(10 + 5) * 4` adalah operasi perkalian (`Expr::Mul`) yang memiliki dua anak: anak kiri adalah penjumlahan (`Expr::Add`), anak kanan adalah angka (`Expr::Number`). Keduanya sama-sama bertipe `Expr`.

#### 2. Mengapa Compiler Rust Menolak Tipe Rekursif Tanpa Pointer?
Rust harus mengetahui ukuran setiap tipe data pada waktu kompilasi (*compile-time*) agar dapat mengalokasikan memori stack frame dengan tepat.

Bayangkan definisi linked list rekursif naif berikut:
```rust
// GAGAL COMPILE! Error E0072: recursive type has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
```
Berapa ukuran `List` di Stack?
- `List` butuh ruang untuk `i32` + `List`.
- Tetapi `List` di dalamnya butuh ruang untuk `i32` + `List` lagi...
- Hal ini berulang tanpa batas $\to$ **ukuran tak terhingga (*infinite size*)!** Compiler bingung harus memesan ruang berapa byte di Stack.

#### 3. Solusi: Indirection Menggunakan `Box<T>`
Dengan membungkus anak rekursif dalam `Box<List<T>>`:
```rust
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
```
Sekarang compiler tahu pasti:
- Varian `Cons` berukuran: `size_of::<T>()` + `size_of::<usize>()` (8 byte pointer Box) + padding alignment.
- Ukurannya kini **terbatas dan pasti**.

#### Implementasi Nyata 1: Functional Cons List
Pada file `fase10_task_1.rs`, kita membangun struktur linked list fungsional lengkap:
- `prepend(elem)`: Menambahkan node baru di depan dalam waktu $O(1)$.
- `len()`: Menghitung total simpul secara iteratif melalui traversing pointer heap.
- `to_vec()`: Mengonversi rantai linked list menjadi `Vec<T>`.

#### Implementasi Nyata 2: Abstract Syntax Tree (AST) untuk Evaluasi Ekspresi
Struktur rekursif sangat krusial dalam pembuatan parser, compiler, dan kalkulator ekspresi:
```rust
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
```
Setiap operasi biner (`Add`, `Sub`, `Mul`, `Div`) menyimpan dua sub-pohon `Box<Expr>`. Method `eval()` mengevaluasi ekspresi secara rekursif dan memproteksi pembagian dengan angka nol (*zero-division error*).

---

### 10.4 Dereferencing `Box<T>`, Trait `Deref` & Deref Coercion

#### 1. Operator Dereference (`*`)
Untuk mengakses atau memutasi nilai asli yang berada di dalam Heap:
```rust
let mut b = Box::new(100);
*b += 50; // Mengubah nilai 100 menjadi 150 di heap
assert_eq!(*b, 150);
```

#### 2. Unboxing (Memindahkan Nilai Keluar dari Box)
Jika tipe data `T` memiliki ukuran tetap (`Sized`), kita dapat memindahkan kepemilikannya keluar dari Box ke stack frame lokal:
```rust
let b = Box::new(String::from("Rust"));
let s: String = *b; // Nilai String dipindahkan (moved) keluar dari Box heap
```

#### 3. Trait `std::ops::Deref` dan `DerefMut`
Rust mengizinkan tipe kustom bertindak seperti pointer melalui trait `Deref`:
```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```
Ketika menulis `*b`, di balik layar compiler Rust mengeksekusi:
```rust
*(b.deref())
```

#### 4. Deref Coercion: Kemudahan Tanpa Boilerplate
*Deref Coercion* adalah fitur ergonomis compiler Rust yang mengonversi referensi suatu tipe yang mengimplementasikan `Deref` ke referensi tipe targetnya secara otomatis:
- `&Box<String>` $\to$ `&String` $\to$ `&str`
- `&Box<Vec<T>>` $\to$ `&Vec<T>` $\to$ `&[T]`

Contoh:
```rust
fn greet(name: &str) { ... }

let boxed_name = Box::new(String::from("Rustacean"));
// Deref coercion otomatis mengubah &Box<String> menjadi &str:
greet(&boxed_name);
```

---

### 10.5 Box untuk Trait Objects (`Box<dyn Trait>`) & Dynamic Dispatch

Ketika kita ingin menyimpan kumpulan objek berbeda dalam satu koleksi seragam (*heterogeneous collection*):
```rust
pub trait Renderable {
    fn render(&self) -> String;
}

// Koleksi heterogen yang menampung Header, Paragraph, CodeBlock:
let doc: Vec<Box<dyn Renderable>> = vec![
    Box::new(Header { text: "Judul".into() }),
    Box::new(Paragraph { content: "Isi paragraf".into() }),
];
```
- **Fat Pointer**: `Box<dyn Trait>` terdiri dari 2 pointer (total 16 bytes):
  1. Pointer ke alamat data objek aktual di heap.
  2. Pointer ke tabel fungsi virtual (**vtable**) untuk memanggil method yang sesuai saat runtime (*dynamic dispatch*).

---

### 10.6 Kode Implementasi Modul: `fase10_task_1.rs`

Seluruh konsep di atas diuji dan diverifikasi secara komprehensif pada file:
`rust-learning-lab/src/fase10_task_1.rs`

```rust
// Jalankan modul melalui binary:
// cargo run
// Atau jalankan unit test khusus modul ini:
// cargo test --bin rust-learning-lab -- fase10_task_1
```

Hasil eksekusi program:
```text
=== FASE 10 TASK 1: SMART POINTER BOX<T> ===

1. Heap Allocation & Ukuran Memori:
   - Nilai dalam Box: 42
   - Ukuran variabel Box di Stack: 8 bytes (pointer 64-bit)
   - Ukuran payload nilai di Heap: 4 bytes
   - LargeBuffer (1 MB) sukses dialokasikan di Heap. Byte pertama: 7

2. Recursive Type: Cons List (Linked List):
   - List berhasil dibentuk: Cons(10, Cons(20, Cons(30, Nil)))
   - Panjang list: 3 (is_empty: false)
   - Representasi Vec: [10, 20, 30]

3. Recursive Type: Abstract Syntax Tree (AST):
   - Kedalaman pohon ekspresi AST: 4
   - Hasil evaluasi AST ((10+5)*4) - (50/2) = 35
   - Uji pembagian dengan nol: Err("Peringatan: Pembagian dengan angka nol!")

4. Dereferencing & Deref Coercion:
   - Nilai awal: 100
   - Nilai setelah mutasi via deref: 150
   - Nilai setelah unboxing (move out): 150
   - Deref coercion string: Halo, Rustacean!
   - Deref coercion slice: sum = 15
   - Custom MyBox DerefMut hasil: Hello World

5. Box<dyn Trait> Heterogeneous Collection:
   Rendered document:
# Dokumentasi Smart Pointer

<p>Box<T> adalah smart pointer paling sederhana di Rust.</p>

```rust
let b = Box::new(42);
```

[OK] Task Fase 10 (Box) selesai & terverifikasi.
```

---

### 10.7 Smart Pointer `Rc<T>` (Reference Counting & Shared Ownership)

Secara default, aturan ownership Rust menegaskan: **Satu nilai hanya memiliki TEPAT SATU pemilik tunggal (*single owner*)**. Ketika pemilik keluar dari scope, nilai tersebut dihancurkan.

Namun, dalam struktur data tertentu (seperti Grafik, Node Pohon dengan banyak orang tua / *multi-parent DAG*, atau state konfigurasi bersama), satu data di Heap perlu dimiliki secara sah oleh **banyak pemilik sekaligus** tanpa menduplikasi isi memori (*zero deep-copy*). Inilah peran dari `Rc<T>` (*Reference Counted*).

---

#### 10.7.0 Mental Model & Analogi Ramah Pemula: "TV Ruang Tengah Kosan"

Bagi pemula yang baru belajar Rust, konsep `Rc<T>` sering membingungkan jika hanya melihat istilah teknisnya. Mari kita pahami dengan analogi kehidupan nyata:

##### 1. Masalah Kepemilikan Biasa di Rust: "Hanya Boleh Ada 1 Pemilik"
Secara default di Rust, satu data hanya boleh dimiliki oleh **tepat satu variabel**. Begitu pemiliknya keluar dari scope `{ }`, datanya **langsung dibuang dari RAM**.

Contoh masalah dengan `Box<T>`:
```rust
let budi = Box::new(String::from("TV Kosan"));

// Ani mau ikut punya TV yang sama:
let ani = budi; // KEPEMILIKAN PINDAH (MOVE)!
// Sekarang Budi TIDAK BISA nonton TV lagi! (Compiler error: value borrowed after move)
```

Jika kita ingin Budi dan Ani sama-sama bisa memiliki TV tanpa `Rc`, kita terpaksa menduplikasi datanya via `.clone()`:
```rust
let ani = budi.clone(); // Membeli TV baru di RAM (Alokasi Heap baru!)
```
Jika data tersebut berukuran 500 MB, RAM langsung bengkak jadi 1000 MB!

##### 2. Solusi `Rc<T>`: TV Bersama + Papan Catatan Penonton
`Rc` singkatan dari **Reference Counting** (Penghitung Referensi).

Bayangkan TV ditaruh di ruang tengah kosan. Di samping TV ada **papan catatan kecil** (`strong_count`):

```text
┌──────────────────────────────────────────────┐
│                  MEMORI HEAP                 │
│                                              │
│   Papan Catatan: [ strong_count = 3 ]        │
│   Barang Asli  : "TV Bersama"                │
└──────────────────────────────────────────────┘
         ▲               ▲              ▲
         │               │              │
    Remote Budi     Remote Ani     Remote Caca
```

1. **Budi beli TV kosan**:
   ```rust
   let budi = Rc::new(String::from("TV Bersama"));
   // Di papan catatan tertulis: strong_count = 1
   ```
2. **Ani ikut bergabung**:
   ```rust
   let ani = Rc::clone(&budi);
   // Di papan catatan tertulis: strong_count = 2
   ```
   *Catatan penting*: `Rc::clone` **BUKAN** meng-copy TV-nya! Komputer hanya menambah angka di papan catatan ($+1$) dan memberikan "remote control" (pointer stack 8 byte) baru ke Ani. Sangat cepat dan hemat memori!
3. **Caca ikut bergabung**:
   ```rust
   let caca = Rc::clone(&budi);
   // Di papan catatan tertulis: strong_count = 3
   ```

##### 3. Kapan TV-nya Dihapus dari RAM?
Kuncinya: **Data di Heap TIDAK AKAN DIHAPUS selama masih ada yang memakai (`strong_count > 0`)**.
- **Budi lulus & pindah kos**: Variabel `budi` selesai $\to$ Angka di papan turun: `strong_count = 2`. TV **belum dihapus** karena Ani dan Caca masih ada di kosan.
- **Ani pindah kos**: Variabel `ani` selesai $\to$ Angka di papan turun: `strong_count = 1`. TV **belum dihapus** karena Caca masih nonton.
- **Caca pindah kos (orang terakhir)**: Variabel `caca` selesai $\to$ Angka di papan jadi: `strong_count = 0`. Sekarang sudah tidak ada penonton sama sekali $\to$ Rust otomatis mematikan dan membebaskan TV dari RAM (`Drop`). **Zero Memory Leak!**

##### 4. Rangkuman Pertanyaan Kunci
| Pertanyaan | Jawaban Ramah Pemula |
| :--- | :--- |
| **Apa itu `Rc<T>`?** | Smart pointer untuk **berbagi 1 data ke banyak pemilik** secara hemat di 1 thread. |
| **Kenapa tidak pakai `Box<T>`?** | `Box` cuma membolehkan 1 pemilik tunggal. Kalau di-share harus menduplikasi data di heap. |
| **Apa fungsi `Rc::clone`?** | Bukan meng-copy isi data, cuma **menambah angka penghitung ($+1$)** dan membagikan pointer. |
| **Kapan data di-drop?** | Otomatis saat **semua pemilik sudah selesai** (`strong_count == 0`). |
| **Kenapa single-thread?** | Papan hitungannya memakai operasi non-atomic agar super cepat, sehingga tidak aman untuk multi-thread (gunakan `Arc` untuk multi-thread). |

---

#### 10.7.1 Analisis Memori: `Rc<T>` di Balik Layar

Ketika Anda membungkus nilai dalam `Rc::new(data)`, Rust mengalokasikan memori Heap yang memuat:
1. **Nilai Asli (`Value T`)**: Payload data sebenarnya.
2. **`strong_count`**: Jumlah pointer aktif yang memegang kepemilikan nilai ini.
3. **`weak_count`**: Jumlah referensi lemah (*weak references*) yang mengamati nilai ini.

```text
┌───────────────────────────┐
│        STACK MEMORY       │
│                           │
│  Variabel `data_a`        │
│  ┌─────────────────────┐  │
│  │ Pointer: 0x5f24bb63 ├──┼───────────────┐
│  └─────────────────────┘  │               │
│                           │               │
│  Variabel `data_b`        │               │         ┌──────────────────────────────────────┐
│  ┌─────────────────────┐  │               │         │             HEAP MEMORY              │
│  │ Pointer: 0x5f24bb63 ├──┼───────────────┼────────>│  ┌──────────────┬──────────────────┐  │
│  └─────────────────────┘  │ (Menunjuk ke  │         │  │ strong_count: 2                 │  │
│                           │  alamat sama) │         │  ├──────────────┼──────────────────┤  │
│  Variabel `data_c`        │               │         │  │ weak_count:   0                 │  │
│  ┌─────────────────────┐  │               │         │  ├──────────────┴──────────────────┤  │
│  │ Pointer: 0x5f24bb63 ├──┼───────────────┘         │  │ Payload: TrackedData ("Config") │  │
│  └─────────────────────┘  │                         │  └─────────────────────────────────┘  │
└───────────────────────────┘                         └──────────────────────────────────────┘
```

- **Operasi `Rc::clone(&rc)`**:
  - **TIDAK melakukan deep-copy data heap!**
  - Hanya menyalin pointer stack 8 byte dan menaikkan angka `strong_count` sebesar $+1$.
  - Biaya performanya sangat murah ($O(1)$).
  - *Konvensi Idiomatik Rust*: Gunakan `Rc::clone(&data)` alih-alih `data.clone()` agar terlihat jelas bagi pembaca kode bahwa kita hanya menggandakan pointer smart pointer, bukan menyalin seluruh payload data.

- **Operasi `Drop` Otomatis**:
  - Setiap kali salah satu variabel `Rc` keluar dari scope, method `drop` otomatis menurunkan `strong_count` sebesar $-1$.
  - Hanya ketika `strong_count == 0`, memori payload di heap beserta metadata ref-count benar-benar dibebaskan (*deallocated*).

---

#### 10.7.2 Mengapa `Rc<T>` Hanya untuk Single-Thread?

- `Rc<T>` memodifikasi `strong_count` menggunakan instruksi aritmatika CPU biasa (non-atomic).
- **Keuntungan**: Eksekusi super cepat tanpa overhead instruksi *atomic bus locking* prosesor.
- **Keterbatasan**: Jika dua thread berbeda mencoba menaikkan atau menurunkan `strong_count` secara bersamaan, akan terjadi **Data Race** pada counter.
- **Proteksi Compiler**: Rust secara sadar **TIDAK mengimplementasikan marker trait `Send` dan `Sync` pada `Rc<T>`**. Jika Anda mencoba mengirim `Rc<T>` ke thread lain lewat `thread::spawn`, compiler akan langsung menolak saat waktu kompilasi (*compile-time error*). Untuk konkurensi multi-thread, Rust menyediakan padanannya yaitu `Arc<T>` (*Atomic Reference Counting*).

---

#### 10.7.3 Kasus Penggunaan Riil: Directed Acyclic Graph (DAG) Berbentuk Y

Bayangkan dua cabang terpisah (*Branch A* dan *Branch B*) yang sama-sama berujung pada simpul ekor bersama (*Shared Node C*):

```text
Branch A [Node A] ──┐
                    ▼
               [Node C] (Shared Tail)
                    ▲
Branch B [Node B] ──┘
```

Jika menggunakan pointer tunggal `Box<TreeNode>`:
- `Branch A` mengambil ownership `Node C`.
- `Branch B` **tidak bisa** memegang `Node C` yang sama tanpa menduplikasi seluruh node dengan alokasi heap baru!

Dengan `Rc<TreeNode>`:
Kedua cabang cukup memanggil `Rc::clone(&shared_c)`. Keduanya memegang alamat pointer heap yang identik:
```rust
let shared_c = Rc::new(TreeNode::new("Node-C", None));
let branch_a = Rc::new(TreeNode::new("Node-A", Some(Rc::clone(&shared_c))));
let branch_b = Rc::new(TreeNode::new("Node-B", Some(Rc::clone(&shared_c))));

// Alamat memori heap Node C yang dirujuk Branch A dan Branch B terbukti persis sama!
```

---

#### 10.7.4 Mencegah Memory Leak Siklus Referensi: `Weak<T>` (`Rc::downgrade`)

Jika dua `Rc` saling memegang referensi satu sama lain (Node A menunjuk Node B, dan Node B menunjuk Node A), maka:
- `strong_count` keduanya tidak akan pernah menyentuh 0.
- Memori heap keduanya tidak akan pernah dibebaskan seumur hidup program $\to$ **Memory Leak (Siklus Referensi)**!

**Solusi Rust: `Weak<T>`**
- Dibuat menggunakan `Rc::downgrade(&rc_ref)`.
- Menaikkan `weak_count`, tetapi **TIDAK menaikkan `strong_count`**.
- Menandakan relasi non-owning (misal: anak memegang referensi lemah `Weak<Parent>` ke orang tuanya).
- Untuk membaca data dari `Weak<T>`, panggil `.upgrade()`, yang menghasilkan `Option<Rc<T>>`. Jika data induk sudah di-drop, method ini mengembalikan `None` secara aman tanpa undefined behavior.

---

#### 10.7.5 Kode Implementasi & Bukti Eksekusi: `fase10_task_2.rs`

Implementasi komprehensif seluruh konsep di atas dapat dilihat di:
[fase10_task_2.rs](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase10_task_2.rs)

Hasil eksekusi program:
```text
=== FASE 10 TASK 2: SMART POINTER RC<T> ===

1. Shared Ownership & strong_count Lifecycle:
   [DROP EVENT] TrackedData 'SharedConfig' dibebaskan dari heap!
   - strong_count awal (data_a)                  : 1
   - strong_count setelah Rc::clone (data_b)     : 2
   - strong_count di dalam inner scope (data_c)  : 3

2. Verifikasi Trait Drop saat strong_count == 0:
   - Instansiasi root: strong_count = 1
   - Dibagikan ke 2 client: strong_count = 3
   - drop(client_a): strong_count = 2
   - drop(client_b): strong_count = 1
   - Menghancurkan root...
   [DROP EVENT] TrackedData 'SessionToken' dibebaskan dari heap!
   - Status Drop setelah semua owner selesai: true

3. Multi-Parent Graph (Y-Shaped DAG):
   - Cabang A: value = Node-A
   - Cabang B: value = Node-B
   - Node C yang dishare memiliki strong_count = 3
   - Alamat heap Node C via Branch A: 0x5f24bb634f80
   - Alamat heap Node C via Branch B: 0x5f24bb634f80
   - Terbukti: Kedua cabang merujuk ke blok memori heap YANG SAMA!

4. Weak References (Rc::downgrade):
   - strong_count = 1, weak_count = 1
   - Weak pointer berhasil diupgrade: true

[OK] Task Fase 10 (Rc) selesai & terverifikasi.
```

---

### 10.8 Smart Pointer `RefCell<T>` & Interior Mutability

Secara default di Rust, jika Anda memiliki variabel immutable (`let x = ...;`) atau referensi immutable (`&self`, `&T`), Anda **dilarang keras** memutasi data di dalamnya. Aturan ini ditegakkan oleh compiler pada waktu kompilasi (*compile-time borrow checking*).

Namun, ada situasi di mana data perlu dimutasi meskipun dibungkus dalam referensi immutable—misalnya ketika mengimplementasikan trait pihak ketiga yang metodenya hanya menerima `&self`, atau saat membuat Mock Object untuk unit testing. Pola desain ini disebut **Interior Mutability**, dan `RefCell<T>` adalah alat utamanya.

---

#### 10.8.0 Mental Model & Analogi Ramah Pemula: "Buku Tamu yang Dijaga Satpam"

Mari kita gunakan analogi sederhana untuk memahami perbedaan antara borrowing biasa dan `RefCell`:

##### 1. Borrowing Biasa (`&T` / `&mut T`): "Pemeriksaan di Pintu Gerbang (Compile-Time)"
- Compiler adalah satpam di pintu gerbang kampus.
- Satpam memeriksa seluruh jadwal kunjungan Anda sebelum Anda diizinkan masuk.
- Jika ada kemungkinan jadwal Anda bentrok (dua orang mau mencoret buku di saat yang sama), satpam langsung **melarang program dikompilasi**.
- **Kelebihan**: Zero overhead runtime! Program yang lolos dijamin 100% aman dari data race dan dangling pointer.
- **Kekurangan**: Terlalu kaku. Terkadang kita tahu jadwal kita aman di runtime, tetapi satpam gerbang menolaknya karena tidak bisa membuktikannya secara statis.

##### 2. `RefCell<T>`: "Pemeriksaan di Meja Buku Tamu (Runtime)"
- Satpam tidak berjaga di pintu gerbang, melainkan **berdiri tepat di samping meja buku tamu saat program berjalan (Runtime)**.
- Ketika pengunjung datang:
  - Mau baca saja? Pengunjung memanggil `.borrow()`. Satpam mengizinkan banyak orang membaca bersamaan.
  - Mau mencatat/menulis? Pengunjung memanggil `.borrow_mut()`. Satpam memastikan **hanya 1 orang** yang boleh memegang pena eksklusif.
- **Apa yang terjadi jika ada orang kedua nekat memanggil `.borrow_mut()` saat ada yang sedang menulis?**
  Satpam langsung **membunyikan sirine alarm dan menghentikan seluruh gedung (PANIC saat runtime)!**

```text
Aturan Emas Borrow Checker Rust Tetap Berlaku Sama Persis:
┌────────────────────────────────────────────────────────────────────────┐
│  Boleh BANYAK Pembaca (.borrow()) ATAU SATU Penulis (.borrow_mut()),   │
│                      TIDAK BOLEH KEDUANYA!                             │
└────────────────────────────────────────────────────────────────────────┘
Perbedaannya:
- Referensi biasa: Aturan diperiksa saat COMPILE-TIME (program gagal dicompile).
- RefCell<T>    : Aturan diperiksa saat RUNTIME (program panic jika melanggar).
```

---

#### 10.8.1 Perbandingan: Referensi Biasa vs `RefCell<T>`

| Karakteristik | Referensi Biasa (`&T` / `&mut T`) | `RefCell<T>` |
| :--- | :--- | :--- |
| **Kapan Borrow Check?** | Compile-Time (waktu kompilasi). | Runtime (saat program berjalan). |
| **Overhead Performa** | Nol mutlak (*Zero-Cost Abstraction*). | Sangat kecil (menyimpan counter peminjam internal di memori). |
| **Akibat Pelanggaran** | Compiler Error (`E0502`, `E0499`). | **Runtime PANIC** (`RefCell already borrowed`). |
| **Mutasi di Balik `&T`?** | Tidak bisa (ditolak compiler). | **Bisa!** Melalui `.borrow_mut()`. |
| **Lingkungan Thread** | Single-thread & Multi-thread (jika `Sync`). | **Hanya Single-Thread** (`!Sync`). |

---

#### 10.8.2 Method Kunci `borrow()` dan `borrow_mut()`

1. **`.borrow() -> Ref<T>`**:
   - Meminjam nilai secara immutable (baca saja).
   - Menambah counter pembaca internal.
   - Boleh dipanggil berkali-kali secara bersamaan selama tidak ada `borrow_mut()` aktif.
2. **`.borrow_mut() -> RefMut<T>`**:
   - Meminjam nilai secara mutable (tulis/ubah).
   - Memastikan counter pembaca bernilai 0 dan belum ada mutable borrow lain yang aktif.
   - Mengembalikan guard `RefMut<T>`. Saat guard ini keluar dari scope (`Drop`), status peminjaman otomatis dilepaskan.
3. **Alternatif Aman Tanpa Panic**:
   - `.try_borrow() -> Result<Ref<T>, BorrowError>`
   - `.try_borrow_mut() -> Result<RefMut<T>, BorrowMutError>`
   - Mengembalikan `Err` daripada memicu panic jika aturan peminjaman sedang dilanggar.

---

#### 10.8.3 Duet Maut di Rust: `Rc<RefCell<T>>` (Shared Mutable State)

Mengapa Rustacean sering menggabungkan `Rc` dan `RefCell` menjadi `Rc<RefCell<T>>`?

- **`Rc<T>`**: Memungkinkan **banyak pemilik**, tetapi hanya mengizinkan akses baca immutable (`&T`). Anda tidak bisa mengubah data di dalamnya.
- **`RefCell<T>`**: Mengizinkan **mutasi data di balik referensi immutable**, tetapi hanya memiliki **1 pemilik tunggal**.
- **`Rc<RefCell<T>>`**: Menggabungkan kekuatan keduanya $\to$ **Banyak pemilik yang sama-sama bisa memutasi satu data bersama di Heap!**

```text
          ┌──────────────────────────────────────────────┐
          │                  MEMORI HEAP                 │
          │                                              │
          │  Rc:      [ strong_count = 2 ]               │
          │  RefCell: [ borrow_count = 0 ]               │
          │  Payload: Vec<String> ["Log 1", "Log 2"]     │
          └──────────────────────────────────────────────┘
                     ▲                          ▲
                     │                          │
        node_a.shared_data.borrow_mut()   node_b.shared_data.borrow_mut()
            (Service A menulis log)           (Service B menulis log)
```

Contoh di kode nyata:
```rust
let shared_log = Rc::new(RefCell::new(Vec::new()));

let service_a = SharedNode::new("Service-A", Rc::clone(&shared_log));
let service_b = SharedNode::new("Service-B", Rc::clone(&shared_log));

// Kedua service dapat menulis ke Vec yang sama persis:
service_a.append("Koneksi dibuka");
service_b.append("Data diproses");

println!("Total log: {}", shared_log.borrow().len()); // Output: 2
```

---

#### 10.8.4 Kode Implementasi & Bukti Eksekusi: `fase10_task_3.rs`

Implementasi komprehensif seluruh konsep di atas dapat dilihat di:
[fase10_task_3.rs](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase10_task_3.rs)

Hasil eksekusi program:
```text
=== FASE 10 TASK 3: SMART POINTER REFCELL<T> ===

1. Demonstrasi borrow() dan borrow_mut():
   - Nilai awal cell          : 10
   - Nilai setelah borrow_mut : 35
   - MockCounter dipanggil via &counter (&self): count = 2

2. Eksperimen Sengaja Double Mutable Borrow:
   - Terbukti: Panic tertangkap di Runtime!
   - Pesan Panic: "already borrowed: BorrowMutError"
   - Detail error raw: RefCell already borrowed

3. Pemeriksaan Tanpa Panic (try_borrow & try_borrow_mut):
   - try_borrow() saat borrow_mut aktif    : false (Aman! Mengembalikan Err)
   - try_borrow_mut() saat borrow_mut aktif: false (Aman! Mengembalikan Err)

4. Pola Kolaborasi Rc<RefCell<T>>:
   - Isi log bersama (diakses & dimutasi oleh Node A dan Node B):
     1. [Service-A] Inisialisasi koneksi
     2. [Service-B] Menerima request #101
     3. [Service-A] Menulis transaksi database
     4. [Service-B] Mengirim HTTP Response 200

5. Mock Messenger Pattern (Interior Mutability):
   - Total pesan terkirim yang dicatat: 2

[OK] Task Fase 10 (RefCell) selesai & terverifikasi.
```

---

### 10.9 Smart Pointer `Arc<T>` (Atomic Reference Counting & Multi-Threading)

Pada sub-bab sebelumnya (10.7), kita telah mempelajari `Rc<T>` untuk berbagi kepemilikan (*shared ownership*) di lingkungan single-thread. Namun, ketika kita mencoba memindahkan `Rc<T>` ke thread lain melalui `thread::spawn`, compiler Rust langsung menolaknya:
`the trait Send is not implemented for Rc<T>`.

Untuk kebutuhan multi-threading, Rust menyediakan **`Arc<T>`** (*Atomic Reference Counting*).

---

#### 10.9.0 Mental Model & Analogi Ramah Pemula: "Papan Skor Digital Terkunci (Atomik)"

##### 1. Masalah `Rc` di Multi-Thread: "Papan Catatan Manual yang Rawan Rebutan"
- Ingat analogi papan catatan `strong_count` di `Rc`?
- Pada `Rc`, angka dinaikkan dengan operasi CPU biasa:
  1. Baca angka saat ini (misal: 2).
  2. Tambahkan 1 (2 + 1 = 3).
  3. Simpan kembali angka 3.
- Jika ada **Thread 1** dan **Thread 2** yang sama-sama melakukan `Rc::clone` di mikrodetik yang persis sama:
  - Thread 1 baca: 2
  - Thread 2 baca: 2 (sebelum Thread 1 sempat menulis 3!)
  - Keduanya menulis 3 $\to$ **Harusnya 4, tapi cuma tercatat 3!**
  - Akibatnya: Memori bisa di-drop padahal masih ada thread yang memakainya (**Data Race / Use-After-Free**).

##### 2. Solusi `Arc`: "Papan Skor Digital dengan Kunci Otomatis (Atomic)"
- Huruf **'A'** pada `Arc` adalah singkatan dari **Atomic**.
- Di level hardware CPU, operasi penambahan/pengurangan ref-count dilakukan menggunakan instruksi atomik khusus (`fetch_add`, `fetch_sub`).
- Instruksi ini mengunci bus memori di prosesor sehingga **tidak mungkin ada dua core CPU yang berebut mencatat di saat bersamaan**.
- Jika Thread 1 sedang menambah counter, Thread 2 dipaksa menunggu sekejap sampai operasi Thread 1 selesai 100%.

##### 3. Kapan Memilih `Rc<T>` vs `Arc<T>`?

| Smart Pointer | Keamanan Multi-Thread | Biaya Performa CPU | Kapan Digunakan? |
| :--- | :--- | :--- | :--- |
| **`Rc<T>`** | ❌ Tidak aman (`!Send`, `!Sync`) | Super Cepat (operasi integer biasa) | Single-threaded apps, logika UI lokal, state internal komponen. |
| **`Arc<T>`** | ✅ Aman lintas thread OS (`Send + Sync`) | Sedikit overhead (instruksi atomik hardware) | Multi-threading, web server concurrent, worker pool, data sharing antar-thread. |

> [!TIP]
> **Kenapa Rust tidak membuat semua pointer menjadi `Arc` saja?**
> Filosofi Rust adalah **Zero-Cost Abstraction**. Jika aplikasi Anda hanya berjalan di 1 thread, Anda tidak perlu membayar biaya penalti performa dari operasi atomik hardware CPU.

---

#### 10.9.1 Mekanisme Berbagi Nilai Antar-Thread via `Arc::clone`

Ketika Anda membuat `let shared = Arc::new(data);` lalu memanggil `Arc::clone(&shared)` untuk diberikan ke thread pekerja:
1. Data asli di Heap **TIDAK PERNAH DI-COPY**.
2. Yang disalin ke tiap thread hanyalah pointer stack (8 bytes) + atomic counter dinaikkan $+1$.
3. Tiap thread membaca blok memori Heap yang **sama persis** (terbukti alamat pointernya identik).

```text
┌───────────────────────────┐
│        MAIN THREAD        │
│  Variabel `config`        │
│  ┌─────────────────────┐  │
│  │ Pointer: 0x5ec887e9 ├──┼───────────────┐
│  └─────────────────────┘  │               │
└───────────────────────────┘               │
                                            │         ┌──────────────────────────────────────┐
┌───────────────────────────┐               │         │             HEAP MEMORY              │
│       WORKER THREAD 1     │               │         │                                      │
│  Variabel `config_clone`  │               ├────────>│  ┌────────────────┬───────────────┐  │
│  ┌─────────────────────┐  │ (Semua thread │         │  │ Atomic Counter:│ 3             │  │
│  │ Pointer: 0x5ec887e9 ├──┼── menunjuk ke ┼────────>│  ├────────────────┴───────────────┤  │
│  └─────────────────────┘  │  alamat sama) │         │  │ Payload: AppConfig ("Server")   │  │
└───────────────────────────┘               │         │  └─────────────────────────────────┘  │
                                            │         └──────────────────────────────────────┘
┌───────────────────────────┐               │
│       WORKER THREAD 2     │               │
│  Variabel `config_clone`  │               │
│  ┌─────────────────────┐  │               │
│  │ Pointer: 0x5ec887e9 ├──┼───────────────┘
│  └─────────────────────┘  │
└───────────────────────────┘
```

---

#### 10.9.2 Pola Komputasi Paralel (Parallel Read / Map-Reduce)

`Arc<T>` sangat ideal untuk membagi dataset read-only berukuran besar ke beberapa thread pekerja tanpa overhead memori sama sekali:
```rust
let dataset = Arc::new(vec![1, 2, 3, ..., 1_000_000]);

// Tiap thread membaca irisan data secara simultan dan paralel
let worker_1 = {
    let d = Arc::clone(&dataset);
    thread::spawn(move || d[0..250_000].iter().sum::<i64>())
};
```
Semua thread membaca secara aman dan paralel karena referensi `&T` di balik `Arc` bersifat immutable dan thread-safe (`Sync`).

---

#### 10.9.3 Kode Implementasi & Bukti Eksekusi: `fase10_task_4.rs`

Implementasi komprehensif seluruh konsep di atas dapat dilihat di:
[fase10_task_4.rs](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase10_task_4.rs)

Hasil eksekusi program:
```text
=== FASE 10 TASK 4: SMART POINTER ARC<T> ===

1. Berbagi Nilai Immutable ke Multi-Thread (Arc::clone):
   - Worker #1 terhubung ke 'RustLearningServer' [Endpoint: /api/v1/tasks]
   - Worker #2 terhubung ke 'RustLearningServer' [Endpoint: /api/v1/health]
   - Worker #3 terhubung ke 'RustLearningServer' [Endpoint: /api/v1/auth]

2. Siklus Hidup Arc strong_count:
   - strong_count awal (main thread)          : 1
   - strong_count setelah clone untuk thread  : 2
   - strong_count setelah worker thread join  : 1

3. Pemrosesan Paralel Tanpa Copy Heap Memori:
   - Alamat Heap dataset: Pointer { addr: 0x5ec887e9e8f0, metadata: 1000 }
   - Total komputasi 4 worker thread: 500500
   - Nilai ekspektasi (1..=1000)     : 500500
   - Verifikasi integritas: COCOK 100%!

[OK] Task Fase 10 (Arc) selesai & terverifikasi.
```

---

### 10.10 Smart Pointer `Mutex<T>` & `RwLock<T>` (Multi-Thread Shared Mutable State)

Pada sub-bab 10.9, kita menggunakan `Arc<T>` untuk membagikan data **hanya baca (*read-only*)** ke banyak thread. Namun, bagaimana jika banyak thread perlu **mengubah (*mutate*)** data bersama tersebut secara serentak?

Rust melarang mutasi bersama tanpa sinkronisasi untuk mencegah **Data Race**. Untuk melakukan mutasi bersama lintas thread, Rust menyediakan dua mekanisme penguncian (*locking primitives*):
1. **`Mutex<T>`** (*Mutual Exclusion*)
2. **`RwLock<T>`** (*Reader-Writer Lock*)

Keduanya hampir selalu dibungkus dalam `Arc`: **`Arc<Mutex<T>>`** atau **`Arc<RwLock<T>>`**.

---

#### 10.10.0 Mental Model & Analogi Ramah Pemula

##### 1. `Mutex<T>`: "Kamar Mandi Umum dengan Kunci Pintu Otomatis"
- Bayangkan sebuah kamar mandi umum di stasiun.
- Hanya **1 orang** yang boleh masuk ke dalam kamar mandi dalam satu waktu (tidak peduli orang itu mau mandi atau sekadar mencuci tangan).
- Jika pintu sedang terkunci dari dalam, orang berikutnya yang memanggil `.lock()` harus **berdiri mengantre** sampai orang di dalam keluar.
- **Kunci Otomatis (RAII Guard)**:
  Begitu orang di dalam melangkah keluar pintu (variabel guard keluar dari scope `{ }`), pintu **otomatis terbuka sendiri** tanpa perlu tombol manual. Tidak ada risiko lupa membuka kunci!

##### 2. `RwLock<T>`: "Papan Pengumuman Kantor (Reader-Writer Lock)"
- Bayangkan sebuah papan pengumuman besar di dinding kantor:
  - **Membaca (`read()` lock)**: 100 karyawan boleh berdiri bersamaan membaca isi pengumuman tanpa saling menghalangi (*concurrent readers*).
  - **Menulis/Mengubah (`write()` lock)**: Jika manajer ingin mengganti kertas pengumuman, manajer harus menunggu sampai seluruh pembaca selesai. Begitu manajer menulis, tirai ditutup rapat: **tidak ada orang lain yang boleh membaca ataupun menulis** sampai manajer selesai.

##### 3. Contoh Kode Lengkap: `Arc<Mutex<T>>`
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn contoh_mutex_multithread() {
    // 1. Bungkus data dalam Mutex, lalu bungkus dalam Arc untuk multi-owner
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 2. Spawn 5 worker thread yang sama-sama menambah counter
    for i in 1..=5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mengambil kunci kamar mandi: thread menunggu giliran jika sedang ada yang masuk
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Worker #{} selesai menambah nilai. Nilai saat ini = {}", i, *num);
            // Kunci otomatis dilepas di sini saat variabel `num` keluar scope!
        });
        handles.push(handle);
    }

    // 3. Tunggu semua thread selesai
    for handle in handles {
        handle.join().unwrap();
    }

    // 4. Baca nilai akhir
    println!("Hasil Akhir Counter: {}", *counter.lock().unwrap()); // Pasti 5 (Determinik!)
}
```

##### 4. Contoh Kode Lengkap: `Arc<RwLock<T>>`
```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn contoh_rwlock_multithread() {
    let cache = Arc::new(RwLock::new(vec![String::from("Config A"), String::from("Config B")]));

    // A. Spawn 3 Thread Pembaca (Bisa membaca BERSAMAAN tanpa saling tunggu!)
    let mut reader_handles = vec![];
    for i in 1..=3 {
        let cache_reader = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let r = cache_reader.read().unwrap(); // Read Lock
            println!("Reader #{} melihat isi: {:?}", i, *r);
        });
        reader_handles.push(handle);
    }

    // B. Spawn 1 Thread Penulis (Membutuhkan akses EKSKLUSIF)
    let cache_writer = Arc::clone(&cache);
    let writer_handle = thread::spawn(move || {
        let mut w = cache_writer.write().unwrap(); // Write Lock
        w.push(String::from("Config C Baru"));
        println!("Writer berhasil menambahkan konfigurasi baru!");
    });

    for h in reader_handles {
        h.join().unwrap();
    }
    writer_handle.join().unwrap();
}
```

##### 5. Kapan Memilih `Mutex<T>` vs `RwLock<T>`?

| Kondisi Aplikasi | Pilihan Terbaik | Alasan |
| :--- | :--- | :--- |
| **Operasi Tulis Sering Terjadi** (Write-Heavy) | **`Mutex<T>`** | Overhead penguncian `Mutex` jauh lebih ringan dan cepat daripada `RwLock`. |
| **Sangat Sering Dibaca, Jarang Diubah** (Read-Heavy, misal: 95% baca, 5% tulis) | **`RwLock<T>`** | Pembaca tidak saling memblokir satu sama lain, melipatgandakan throughput konkurensi. |
| **Data Berukuran Kecil & Sederhana** (misal: Integer counter / Flag boolean) | **Atomic (`AtomicUsize`, `AtomicBool`)** | Jauh lebih cepat tanpa syscall locking OS sama sekali. |

---

#### 10.10.1 Anatomi `MutexGuard` & Pola Kritis: "Scope Lock"

Ketika Anda memanggil `let mut guard = mutex.lock().unwrap();`:
1. Thread akan diblokir (*sleep/wait*) jika mutex sedang dipegang thread lain.
2. Saat giliran tiba, pemanggilan mengembalikan **`MutexGuard<T>`**.
3. `MutexGuard<T>` mengimplementasikan `Deref` dan `DerefMut`, sehingga Anda dapat mengakses dan memutasi data `T` seolah-olah referensi biasa (`*guard` / `guard.method()`).
4. **Pola Scope Lock**: Batasi masa hidup guard menggunakan kurung kurawal `{ }` agar thread lain tidak kelaparan (*lock contention*):

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

let shared_data = Arc::new(Mutex::new(0));

// CONTOH YANG BAIK (Scope Lock Singkat):
{
    let mut guard = shared_data.lock().unwrap();
    *guard += 10;
} // <- `guard` DI-DROP DI SINI! Pintu langsung terbuka seketika untuk thread lain.

// Pekerjaan komputasi berat / I/O dilakukan DI LUAR scope lock:
thread::sleep(Duration::from_millis(50)); // Thread lain TIDAK terhalang!
```

---

#### 10.10.2 Analisis Non-Blocking: `try_lock()`, `try_read()`, dan `try_write()`

Selain metode pemblokir (`lock()` dan `read()`), Rust menyediakan metode non-blocking agar thread tidak tertahan macet (*freeze*):

- `mutex.try_lock()`: Jika sedang terkunci, langsung mengembalikan `Err(TryLockError::WouldBlock)` tanpa membuat thread menunggu.
- `rwlock.try_read()` / `rwlock.try_write()`: Memeriksa ketersediaan akses baca/tulis secara instan.

```rust
use std::sync::{Mutex, RwLock};

fn contoh_non_blocking_check() {
    // 1. Mutex try_lock
    let m = Mutex::new(100);
    let guard1 = m.lock().unwrap();

    // Coba minta lock saat guard1 masih aktif di tangan kita:
    match m.try_lock() {
        Ok(_) => println!("Lock berhasil didapat"),
        Err(_) => println!("Lock sedang sibuk! Thread tidak diblokir, bisa lakukan task lain."),
    }
    drop(guard1); // Buka kunci manual

    // 2. RwLock try_read & try_write
    let rw = RwLock::new(String::from("Data"));
    let write_guard = rw.write().unwrap();

    // Selagi write_guard aktif, pembaca dan penulis lain pasti ditolak:
    assert!(rw.try_read().is_err());
    assert!(rw.try_write().is_err());
    drop(write_guard); // Buka kunci eksklusif

    // Sekarang bebas dibaca
    assert!(rw.try_read().is_ok());
}
```

---

#### 10.10.3 Kode Implementasi & Bukti Eksekusi: `fase10_task_5.rs`

Implementasi komprehensif seluruh konsep di atas dapat dilihat di:
[fase10_task_5.rs](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase10_task_5.rs)

Hasil eksekusi program:
```text
=== FASE 10 TASK 5: MUTEX<T> & RWLOCK<T> ===

1. Mutex<T> Shared Mutable State & Scope Lock:
   - Saldo awal: Rp1000.0, 5 worker masing-masing setor Rp200.0
   - Saldo akhir (determinik berkat Mutex): Rp2000.00

2. Non-blocking try_lock():
   - Apakah try_lock() mendeteksi lock sedang sibuk? true (Aman tanpa deadlock!)

3. RwLock<T> Konkurensi Banyak Pembaca (Read Lock):
   - Hasil baca simultan dari 4 thread: [1, 1, 1, 1]
   - Terbukti: 4 thread pembaca membaca secara paralel tanpa saling blokir!

4. RwLock<T> Eksklusivitas Penulis (Write Lock):
   - Versi cache setelah write lock: v2
   - Apakah pembaca diizinkan masuk selagi write lock aktif? false
   - Terbukti: Write lock mengisolasi data 100% dari semua pembaca & penulis lain!

[OK] Task Fase 10 (Mutex / RwLock) selesai & terverifikasi.
```

---

### 10.11 Mini Project Fase 10: Multi-Threaded Shared Counter & Evaluasi Kelulusan Fase 10

Mini Project Fase 10 menggabungkan smart pointer `Arc<T>` (Atomic Reference Counting) dan `Mutex<T>` (Mutual Exclusion) untuk membangun sistem penghitung bersama (*shared counter*) yang dimutasi secara konkuren oleh 10 worker thread secara simultan dan menghasilkan nilai akhir yang **100% deterministik tanpa data race**.

---

#### 10.11.1 Arsitektur `Arc<Mutex<i32>>` & Pola Eksekusi Konkuren

Dalam arsitektur konkuren Rust, pola `Arc<Mutex<T>>` adalah standar industri untuk *shared mutable state* lintas thread:

```text
               ┌────────────────────────────────────────────────────────┐
               │                      HEAP MEMORY                       │
               │                                                        │
               │   Arc:   [ Atomic Strong Count = 10 ]                  │
               │   Mutex: [ Lock Status: UNLOCKED / LOCKED ]            │
               │   Data:  i32 Counter = 1000                            │
               └────────────────────────────────────────────────────────┘
                       ▲               ▲                ▲
                       │               │                │
                Arc::clone(&cnt) Arc::clone(&cnt) Arc::clone(&cnt)
                       │               │                │
                 Worker Thread 1  Worker Thread 2  Worker Thread 10
                 (Scope Lock 1)   (Scope Lock 2)   (Scope Lock 10)
```

1. **`Arc<T>`**: Menyediakan kepemilikan bersama yang aman lintas thread OS (*thread-safe multi-ownership*). Setiap thread memegang klon pointer 8 byte ke alokasi heap yang sama.
2. **`Mutex<T>`**: Memastikan hanya ada **tepat satu thread** yang dapat mengakses dan memutasi nilai counter pada satu waktu (*mutual exclusion*). Thread lain yang memanggil `.lock()` secara otomatis ditidurkan (*sleep*) oleh OS hingga kunci dilepas.
3. **Pola Scope Lock Singkat**: Kunci Mutex segera dilepas begitu operasi mutasi integer selesai, mencegah perebutan kunci yang berlebihan (*lock contention*).
4. **Sinkronisasi Thread via `join()`**: Main thread menunggu seluruh 10 worker thread menyelesaikan tugasnya menggunakan `handle.join()`, menjamin nilai akhir dapat dibaca secara deterministik.

---

#### 10.11.2 Evaluasi Tiga Kriteria Lulus Fase 10

Tiga konsep fundamental penguasaan memori dan konkurensi diuji dan divalidasi:

1. **Perbedaan `Rc<T>` vs `Arc<T>`**:
   - **`Rc<T>` (*Reference Counting*)**:
     - Menggunakan penghitung non-atomic (operasi integer CPU biasa).
     - Super cepat dengan nol overhead atomik.
     - **Hanya untuk single-thread**: Compiler Rust secara sadar tidak mengimplementasikan `Send` dan `Sync` pada `Rc<T>`.
   - **`Arc<T>` (*Atomic Reference Counting*)**:
     - Menggunakan instruksi CPU atomik hardware (`fetch_add` / `fetch_sub`).
     - **Aman untuk multi-thread (`Send + Sync`)**: Dapat dipindahkan dan diakses lintas thread OS secara bebas.
     - Memiliki sedikit overhead latensi CPU karena penyelarasan memori atomik (*memory ordering*).

2. **Perbedaan `RefCell<T>` vs `Mutex<T>`**:
   - **`RefCell<T>` (*Single-Thread Interior Mutability*)**:
     - Aturan peminjaman (borrow checker) diperiksa saat **runtime**.
     - Hanya boleh digunakan dalam 1 thread tunggal.
     - Jika aturan dilanggar (misal 2 mutable borrow bersamaan), seketika terjadi **PANIC**.
   - **`Mutex<T>` (*Multi-Thread Synchronization*)**:
     - Memproteksi mutabilitas bersama lintas banyak thread OS.
     - Jika terjadi perebutan akses, thread yang menunggu **TIDAK PANIC**, melainkan **DIBLOKIR / ANTRE (*sleep*)** hingga kunci dilepaskan melalui RAII guard.

3. **Panduan Memilih `Mutex<T>` vs `RwLock<T>`**:
   - **Pilih `Mutex<T>`**:
     - Saat beban kerja dominan operasi **TULIS / UBAH (*write-heavy*)**.
     - Saat ukuran data kecil atau durasi penguncian sangat singkat.
     - Overhead locking `Mutex` jauh lebih ringan dan sederhana dibandingkan `RwLock`.
   - **Pilih `RwLock<T>`**:
     - Saat beban kerja dominan operasi **BACA (*read-heavy*)**, misalnya 90% pembaca dan 10% penulis (contoh: cache konfigurasi sistem).
     - Banyak thread pembaca dapat mengakses data secara paralel (*concurrent read*) tanpa saling memblokir satu sama lain.

---

#### 10.11.3 Kode Implementasi Lengkap `mini_project_10.rs`

Berikut adalah kode sumber lengkap yang telah diuji dan diintegrasikan pada `rust-learning-lab/src/mini_project_10.rs`:

```rust
// Mini Project Fase 10: Multi-Threaded Shared Counter (Smart Pointers & Concurrency)
// Rujukan: rust_learning_guide.md (Sub-bab 10.11) & rust_execution_tasks.md (L903-L917)

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Abstraksi Counter Bersama Terproteksi Mutex lintas Thread.
#[derive(Debug, Clone)]
pub struct SharedCounter {
    counter: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial: i32) -> Self {
        Self {
            counter: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn get_value(&self) -> i32 {
        *self.counter.lock().expect("Gagal mengunci Mutex saat membaca nilai")
    }

    pub fn increment_by(&self, amount: i32) {
        let mut guard = self.counter.lock().expect("Gagal mengunci Mutex saat increment");
        *guard += amount;
    }

    pub fn get_arc(&self) -> Arc<Mutex<i32>> {
        Arc::clone(&self.counter)
    }

    pub fn execute_workers(
        &self,
        num_workers: usize,
        increments_per_worker: usize,
        step: i32,
    ) -> Vec<JoinHandle<usize>> {
        let mut handles = Vec::with_capacity(num_workers);

        for worker_id in 0..num_workers {
            let counter_clone = Arc::clone(&self.counter);

            let handle = thread::spawn(move || {
                for _ in 0..increments_per_worker {
                    let mut guard = counter_clone.lock().expect("Mutex poison error");
                    *guard += step;
                }
                worker_id
            });

            handles.push(handle);
        }

        handles
    }
}

pub fn explain_rc_vs_arc() -> (&'static str, &'static str) {
    let rc_desc = "Rc<T> (Single-Thread): Menggunakan penghitung non-atomic (integer biasa). \
                   Sangat cepat tanpa overhead atomik hardware, tetapi TIDAK aman untuk multi-thread (!Send, !Sync).";
    let arc_desc = "Arc<T> (Multi-Thread): Menggunakan Atomic Reference Counting via instruksi CPU atomik \
                    (fetch_add/fetch_sub). Aman dibagikan lintas thread OS (Send + Sync) dengan sedikit overhead CPU.";
    (rc_desc, arc_desc)
}

pub fn explain_refcell_vs_mutex() -> (&'static str, &'static str) {
    let refcell_desc = "RefCell<T> (Single-Thread Interior Mutability): Borrow check diperiksa saat RUNTIME. \
                        Hanya untuk 1 thread. Jika aturan dilanggar, seketika terjadi PANIC.";
    let mutex_desc = "Mutex<T> (Multi-Thread Synchronization): Memproteksi mutabilitas bersama lintas thread. \
                      Thread yang berebut tidak panic, melainkan DIBLOKIR / ANTRE (sleep) hingga lock dilepas via RAII guard.";
    (refcell_desc, mutex_desc)
}

pub fn explain_mutex_vs_rwlock() -> (&'static str, &'static str) {
    let mutex_choice = "Pilih Mutex<T>: Saat beban kerja dominan TULIS (write-heavy) atau mutasi berlangsung sering. \
                        Overhead locking Mutex jauh lebih ringan dan sederhana dibandingkan RwLock.";
    let rwlock_choice = "Pilih RwLock<T>: Saat beban kerja dominan BACA (read-heavy, misal 90% baca, 10% tulis). \
                         Banyak thread pembaca (read lock) dapat mengakses data bersamaan secara simultan tanpa saling tunggu.";
    (mutex_choice, rwlock_choice)
}

pub fn run() {
    println!("=== Mini Project Fase 10: Multi-Threaded Shared Counter ===");

    let initial_value = 0;
    let shared_counter = SharedCounter::new(initial_value);
    println!("1. Inisialisasi: Arc<Mutex<i32>> dengan nilai awal = {initial_value}");

    let num_workers = 10;
    let increments_per_worker = 100;
    let step = 1;
    let expected_final = initial_value + (num_workers as i32 * increments_per_worker as i32 * step);

    println!("2. Memulai {num_workers} Worker Threads...");
    println!("   - Tiap worker melakukan {increments_per_worker}x increment (+{step})");
    println!("   - Target nilai akhir deterministik: {expected_final}");

    let handles = shared_counter.execute_workers(num_workers, increments_per_worker, step);

    for handle in handles {
        let _ = handle.join().expect("Worker thread panic");
    }
    println!("3. Seluruh {num_workers} thread sukses di-join (selesai).");

    let final_value = shared_counter.get_value();
    println!("4. Hasil Akhir Counter Terbaca: {final_value}");
    if final_value == expected_final {
        println!("   [✓] Sukses: Hasil akhir 100% DETERMINISTIK tanpa data race!");
    } else {
        println!("   [✗] Gagal: Data race terdeteksi!");
    }

    let direct_arc = shared_counter.get_arc();
    let arc_val = *direct_arc.lock().unwrap();
    shared_counter.increment_by(5);
    println!("   - get_arc() membaca: {arc_val}, setelah increment_by(+5): {}", shared_counter.get_value());

    println!("\n--- Evaluasi & Bukti Kriteria Lulus Fase 10 ---");
    let (rc_info, arc_info) = explain_rc_vs_arc();
    println!("A. Perbedaan Rc vs Arc:");
    println!("   - {rc_info}");
    println!("   - {arc_info}");

    let (refcell_info, mutex_info) = explain_refcell_vs_mutex();
    println!("\nB. Perbedaan RefCell vs Mutex:");
    println!("   - {refcell_info}");
    println!("   - {mutex_info}");

    let (mutex_ch, rwlock_ch) = explain_mutex_vs_rwlock();
    println!("\nC. Panduan Memilih Mutex vs RwLock:");
    println!("   - {mutex_ch}");
    println!("   - {rwlock_ch}");

    println!("\n[OK] Mini Project Fase 10 selesai & terverifikasi.");
}
```

---

#### 10.11.4 Hasil Eksekusi Output Terminal Mini Project 10

```text
=== Mini Project Fase 10: Multi-Threaded Shared Counter ===
1. Inisialisasi: Arc<Mutex<i32>> dengan nilai awal = 0
2. Memulai 10 Worker Threads...
   - Tiap worker melakukan 100x increment (+1)
   - Target nilai akhir deterministik: 1000
3. Seluruh 10 thread sukses di-join (selesai).
4. Hasil Akhir Counter Terbaca: 1000
   [✓] Sukses: Hasil akhir 100% DETERMINISTIK tanpa data race!
   - get_arc() membaca: 1000, setelah increment_by(+5): 1005

--- Evaluasi & Bukti Kriteria Lulus Fase 10 ---
A. Perbedaan Rc vs Arc:
   - Rc<T> (Single-Thread): Menggunakan penghitung non-atomic (integer biasa). Sangat cepat tanpa overhead atomik hardware, tetapi TIDAK aman untuk multi-thread (!Send, !Sync).
   - Arc<T> (Multi-Thread): Menggunakan Atomic Reference Counting via instruksi CPU atomik (fetch_add/fetch_sub). Aman dibagikan lintas thread OS (Send + Sync) dengan sedikit overhead CPU.

B. Perbedaan RefCell vs Mutex:
   - RefCell<T> (Single-Thread Interior Mutability): Borrow check diperiksa saat RUNTIME. Hanya untuk 1 thread. Jika aturan dilanggar, seketika terjadi PANIC.
   - Mutex<T> (Multi-Thread Synchronization): Memproteksi mutabilitas bersama lintas thread. Thread yang berebut tidak panic, melainkan DIBLOKIR / ANTRE (sleep) hingga lock dilepas via RAII guard.

C. Panduan Memilih Mutex vs RwLock:
   - Pilih Mutex<T>: Saat beban kerja dominan TULIS (write-heavy) atau mutasi berlangsung sering. Overhead locking Mutex jauh lebih ringan dan sederhana dibandingkan RwLock.
   - Pilih RwLock<T>: Saat beban kerja dominan BACA (read-heavy, misal 90% baca, 10% tulis). Banyak thread pembaca (read lock) dapat mengakses data bersamaan secara simultan tanpa saling tunggu.

[OK] Mini Project Fase 10 selesai & terverifikasi.
```

---


## FASE 11: Concurrency (Multi-Threading & Shared State)

### 11.1 Native OS Threads, `thread::spawn`, & `JoinHandle`

Rust mengadopsi model **1:1 OS Threading** (satu thread Rust dipetakan langsung ke satu thread kernel sistem operasi). Ini berbeda dengan model M:N (green threads/goroutines) yang membutuhkan runtime berat di latar belakang.

#### Konsep Kunci:
1. **`thread::spawn(|| { ... })`**:
   - Meminta sistem operasi membuat native thread baru untuk mengeksekusi closure yang diberikan.
   - Mengembalikan struct `JoinHandle<T>`, di mana `T` adalah tipe return value dari closure tersebut.
2. **`JoinHandle<T>` & `.join()`**:
   - Jika fungsi `main()` selesai, seluruh thread turunan yang masih berjalan akan langsung dimatikan paksa (*premature exit*).
   - Memanggil `handle.join()` akan **memblokir (*block*)** thread pemanggil sampai thread target selesai bekerja.
   - Mengembalikan tipe `Result<T, Box<dyn Any + Send + 'static>>`. Jika worker thread mengalami `panic!`, varian `Err` akan dikembalikan alih-alih merusak (*crash*) thread utama.
3. **Keyword `move` pada Closure Thread**:
   - Karena compiler Rust tidak dapat memprediksi berapa lama sebuah OS thread akan berjalan, referensi lokal (`&data`) dilarang dipinjam oleh thread lain (bisa menyebabkan *dangling reference* atau *use-after-free*).
   - Keyword `move` memaksa closure untuk mengambil alih kepemilikan (*ownership*) dari semua variabel lingkungan yang digunakan.
4. **Mengembalikan Nilai (*Return Values*) dari Thread**:
   - Thread di Rust dapat mengembalikan nilai secara langsung seperti fungsi biasa. Nilai ini diekstraksi saat memanggil `.join().unwrap()`.
5. **`thread::Builder`**:
   - Digunakan untuk kustomisasi thread, seperti memberikan nama thread (sangat berguna untuk log profiling/debugging) dan ukuran stack spesifik.

#### Contoh Implementasi (Selaras dengan `fase11_task_1.rs`):
```rust
use std::thread::{self, JoinHandle};
use std::time::Duration;

// 1. Spawning 2 Threads & Sinkronisasi
pub fn spawn_two_threads_demo() -> (String, String) {
    let handle_alpha = thread::spawn(|| {
        thread::sleep(Duration::from_millis(20));
        String::from("Hasil dari Alpha: OK")
    });

    let handle_beta = thread::spawn(|| {
        thread::sleep(Duration::from_millis(15));
        String::from("Hasil dari Beta: OK")
    });

    // Menunggu kedua thread selesai dan mengekstrak return value
    let res_a = handle_alpha.join().expect("Alpha panic");
    let res_b = handle_beta.join().expect("Beta panic");

    (res_a, res_b)
}

// 2. Spawning Banyak Thread & Parallel Chunk Reduction
pub fn parallel_sum(data: &[u64], chunks_count: usize) -> u64 {
    let chunk_size = (data.len() + chunks_count - 1) / chunks_count;
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();

    for chunk in data.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec(); // Move data mandiri ke tiap thread
        let handle = thread::spawn(move || {
            let partial: u64 = chunk_vec.iter().sum();
            partial
        });
        handles.push(handle);
    }

    // Mengumpulkan dan menjumlahkan partial sum
    let mut total = 0;
    for handle in handles {
        total += handle.join().expect("Thread error");
    }
    total
}

// 3. Isolasi Panic: Main thread tetap aman jika worker panic
pub fn panic_isolation_demo() -> bool {
    let handle = thread::spawn(|| {
        panic!("Simulasi panic darurat di worker thread!");
    });

    // .join() menangkap panic dan mengembalikan Result::Err
    match handle.join() {
        Ok(_) => false,
        Err(_) => {
            println!("Panic berhasil diisolasi! Main thread tetap berjalan normal.");
            true
        }
    }
}
```

---

### 11.2 Message Passing: Channels (`mpsc`)

Rust menganut filosofi konkurensi legendaris Erlang & Go: *"Do not communicate by sharing memory; instead, share memory by communicating."*

Pustaka standar menyediakan channel berjenis **MPSC (Multi-Producer, Single-Consumer)** di modul `std::sync::mpsc`.

---

#### A. Analogi Pipa Paralon Antar-Kamar

Bayangkan channel seperti **pipa paralon** yang menghubungkan beberapa kamar pekerja ke meja seorang bos:
- **`tx` (Transmitter / Pengirim)**: **Corong Masuk Pipa**. Tempat memasukkan pesan/barang.
- **`rx` (Receiver / Penerima)**: **Ujung Keluar Pipa**. Tempat menangkap pesan/barang yang jatuh.

```text
 [ Worker 1 ] ──(tx1.send)──┐
                            │
 [ Worker 2 ] ──(tx2.send)──┼───> [ PIPA CHANNEL mpsc ] ───> (rx.recv) ──> [ Main Thread (Bos) ]
                            │
 [ Worker 3 ] ──(tx3.send)──┘
   (Multi-Producer / tx.clone)                                           (Single-Consumer / 1 rx saja)
```

---

#### B. Konsep Kunci:

1. **Multi-Producer (`Sender<T>` / Transmitter)**:
   - Struct `Sender<T>` mengimplementasikan trait `Clone`.
   - Artinya, corong masuk pipa bisa difotokopi/diduplikasi menggunakan **`tx.clone()`**.
   - Setiap worker thread memegang satu salinan corong untuk melempar data ke dalam pipa yang sama.
2. **Single-Consumer (`Receiver<T>` / Receiver)**:
   - Struct `Receiver<T>` **TIDAK** mengimplementasikan `Clone`.
   - Di ujung pipa hanya ada **satu penerima tunggal** yang bertugas mengambil pesan secara berurutan (*FIFO: First-In, First-Out*).
3. **Tiga Cara Mengonsumsi Pesan dari `rx`**:

| Perintah | Karakteristik | Cara Kerja (Analogi) | Kapan Digunakan? |
|---|---|---|---|
| **`rx.recv()`** | **Blocking** | Duduk di ujung pipa, menengadahkan tangan lalu **tidur**. Baru bangun saat ada barang jatuh atau channel ditutup. Mengembalikan `Result<T, RecvError>`. | Saat thread penerima tidak punya tugas lain selain menunggu data tiba. |
| **`rx.try_recv()`** | **Non-blocking** | Cuma **melirik sekilas** ke lubang pipa. Jika ada barang langsung ambil (`Ok(T)`), jika kosong langsung lanjut kerja tanpa menunggu (`Err(TryRecvError::Empty)`). | Pada aplikasi game loop, UI/GUI, atau background poller yang tidak boleh macet. |
| **`for msg in rx { }`** | **Iterator Blocking** | Berdiri di ujung pipa, mengambil pesan satu per satu secara kontinu **sampai seluruh pipa resmi ditutup**. | Pola paling standar untuk memproses batch tugas worker sampai tuntas. |

---

#### C. Mengapa `drop(tx)` pada Main Thread SANGAT KRUSIAL?

> **Aturan Mutlak Rust:** Channel hanya dinyatakan **resmi ditutup (*Disconnected*)** jika **SEMUA instance `Sender` (`tx`) yang masih hidup di memori telah di-drop / dihancurkan**.

Mari bedah skenario yang sering menyebabkan **DEADLOCK / PROGRAM MACET**:
1. Di Main Thread, kamu membuat: `let (tx, rx) = mpsc::channel();` *(Sekarang ada 1 buah `tx` di Main Thread)*.
2. Kamu kloning: `let tx1 = tx.clone();` lalu diberikan ke Worker Thread *(Sekarang ada 2 buah `tx`: 1 di Main, 1 di Worker)*.
3. Worker selesai bekerja, thread worker mati $\rightarrow$ `tx1` otomatis hancur (*drop*).
4. Kamu di Main Thread mulai membaca: `for msg in rx { ... }`.

**HASILNYA: Program HANG / DEADLOCK selamanya!**
**Penyebab:** Main Thread masih mengantongi `tx` asli di tangannya. Receiver berpikir: *"Saya tidak boleh berhenti, karena Main Thread masih memegang `tx` asli, siapa tahu nanti dia mau kirim data lagi!"*

**SOLUSI:**
Panggil **`drop(tx)`** pada transmitter asli di Main Thread tepat setelah semua worker selesai di-spawn. Karena Main Thread hanya bertugas MENDENGAR (`rx`), bukan MENGIRIM (`tx`):

```rust
let (tx, rx) = mpsc::channel();

for id in 1..=3 {
    let thread_tx = tx.clone();
    thread::spawn(move || {
        thread_tx.send(format!("Laporan dari worker #{}", id)).unwrap();
        // thread_tx hancur otomatis saat worker thread selesai
    });
}

// ⚠️ WAJIB: Buang corong asli di main thread agar channel bisa ditutup!
drop(tx);

// Begitu worker terakhir selesai, SEMUA tx sudah hancur -> loop rx langsung selesai mulus!
for laporan in rx {
    println!("Diterima: {}", laporan);
}
println!("Semua laporan selesai diproses.");
```

---

#### D. Pipa Biasa (`channel`) vs Pipa Kaku Berkapasitas (`sync_channel`)

1. **`mpsc::channel()` (Unbounded / Pipa Elastis Tak Terbatas)**:
   - Kapasitas buffer tidak terbatas di RAM.
   - Operasi `tx.send()` tidak pernah menahan pengirim (langsung sukses seketika).
   - *Risiko:* Jika worker memproduksi 10.000 pesan/detik tapi consumer lambat, memori RAM bisa bengkak (*memory leak/OOM*).
2. **`mpsc::sync_channel(size)` (Bounded / Pipa Kaku Berukuran `size`)**:
   - Pipa hanya muat sebanyak `size` pesan.
   - Memberikan mekanisme **backpressure**: jika pipa sudah penuh dan belum dibaca oleh receiver, maka pemanggilan `tx.send()` berikutnya akan **memblokir / menahan thread pengirim** sampai ada ruang yang dikosongkan oleh receiver.

---

#### E. Contoh Implementasi Lengkap (Selaras dengan `fase11_task_2.rs`):
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn multi_producer_demo() {
    let (tx, rx) = mpsc::channel();

    // 1. Multi-Producer: Spawn 3 worker dengan tx.clone()
    for id in 1..=3 {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let msg = format!("Data dari sensor #{}", id);
            thread::sleep(Duration::from_millis(10));
            thread_tx.send(msg).unwrap();
        });
    }

    // 2. KRUSIAL: Drop transmitter utama di main thread
    // Tanpa drop(tx), loop 'for msg in rx' akan deadlock selamanya!
    drop(tx);

    // 3. Single Consumer: Mengonsumsi pesan hingga channel tertutup
    for received in rx {
        println!("Diterima: {}", received);
    }
}
```

---

### 11.3 Marker Traits: `Send` & `Sync` (Thread-Safety Guarantees)

Di Rust, keamanan konkurensi (*fearless concurrency*) dijamin langsung oleh sistem tipe pada saat kompilasi melalui dua buah **Marker Traits**: `Send` dan `Sync`.

> **Apa itu Marker Trait?**
> Trait kosong yang tidak memiliki method (`pub unsafe trait Send {}`). Trait ini berfungsi sebagai "label sertifikasi keamanan" otomatis (*auto trait*) yang disematkan oleh compiler ke tipe data Anda.

---

#### A. Definisi: `Send` vs `Sync`

1. **`Send` (Transfer of Ownership)**:
   - Menandakan bahwa **kepemilikan (*ownership*)** tipe data `T` aman dipindahkan (*moved / transferred*) melintasi batas thread.
   - Jika suatu tipe adalah `Send`, kita bisa mengoper nilainya ke dalam closure `thread::spawn(move || { ... })` atau mengirimkannya lewat channel `tx.send(val)`.
2. **`Sync` (Concurrent Shared References)**:
   - Menandakan bahwa tipe data `T` aman diakses melalui **referensi bersama (`&T`)** oleh banyak thread secara simultan tanpa memicu *data race*.
   - Jika suatu tipe adalah `Sync`, banyak thread boleh membaca nilai yang sama secara paralel.

---

#### B. Hubungan Krusial: `T is Sync <=> &T is Send`

Aturan emas konkurensi Rust menyatakan:
$$\mathbf{T \text{ adalah } Sync \iff \&T \text{ adalah } Send}$$

**Mengapa demikian?**
Bayangkan Anda ingin membagikan data `T` ke thread lain tanpa memindahkan kepemilikannya. Anda akan meminjamkan referensi `&T` ke thread tersebut.
Ketika referensi `&T` diserahkan ke thread lain, objek yang sebenarnya *dikirim (*Send*)* adalah pointer referensi `&T` itu sendiri!
Oleh karena itu:
- Jika referensi `&T` **aman dipindahkan** ke thread lain (`&T: Send`), maka thread tersebut aman mengakses `T` bersamaan dengan thread pembuatnya.
- Ini berarti tipe `T` dijamin aman untuk *concurrency* alias berstatus **`Sync`**!

---

#### C. Tabel Klasifikasi Tipe Data Populer

| Tipe Data | `Send`? | `Sync`? | Alasan & Catatan |
|---|---|---|---|
| **Primitif (`i32`, `f64`, `bool`, `char`)** | **Ya** | **Ya** | Nilai independen, immutable secara default, aman dibaca dari banyak thread. |
| **`String` / `Vec<T>`** | **Ya** | **Ya** | Heap-allocated dengan kepemilikan tunggal yang bersih (selama `T: Send + Sync`). |
| **`Arc<T>`** | **Ya** | **Ya** | *Atomic Reference Counting*. Sinkronisasi thread-safe counter. |
| **`Mutex<T>` / `RwLock<T>`** | **Ya** | **Ya** | Menyediakan kunci eksklusif (*locking*), aman dimutasi bersama lintas thread. |
| **`AtomicBool` / `AtomicI32`** | **Ya** | **Ya** | Operasi atomik tingkat instruksi CPU (lock-free). |
| **`RefCell<T>` / `Cell<T>`** | **Ya** | **TIDAK** | **Boleh dipindah kepemilikan ke thread lain**, tetapi **DILARANG diakses bersama via `&RefCell`** karena borrow-counter internalnya tidak atomic (bisa race condition). |
| **`mpsc::Receiver<T>`** | **Ya** | **TIDAK** | Sifatnya *Single-Consumer*. Boleh dipindah ke thread lain, tapi tidak boleh dibaca bersamaan. |
| **`Rc<T>`** | **TIDAK** | **TIDAK** | *Reference counting* biasa (non-atomic). Clone/drop bersamaan dari 2 thread dapat merusak memori (*double free*). |
| **`*const T` / `*mut T`** | **TIDAK** | **TIDAK** | Raw pointer C tidak memiliki garansi keamanan memori dari compiler. |

---

#### D. Compile-Time Trait Bounds Assertion & Contoh Kode (Selaras dengan `fase11_task_3.rs`):

Kita dapat menguji sifat `Send` dan `Sync` suatu tipe data langsung saat kompilasi menggunakan helper function statis:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// 1. Compile-Time Helper: Jika tipe tidak sesuai, kompilator langsung menolak!
const fn assert_send<T: Send>() {}
const fn assert_sync<T: Sync>() {}

fn verify_traits() {
    assert_send::<String>();
    assert_sync::<String>();

    // RefCell adalah Send, tapi BUKAN Sync:
    assert_send::<RefCell<i32>>();
    // assert_sync::<RefCell<i32>>(); // ❌ ERROR KOMPILASI jika di-uncomment!

    // Rc bukan Send dan bukan Sync:
    // assert_send::<Rc<i32>>();      // ❌ ERROR KOMPILASI!
    // assert_sync::<Rc<i32>>();      // ❌ ERROR KOMPILASI!
}

// 2. Pembuktian Hubungan: T is Sync <=> &T is Send
pub fn demonstrate_sync_ref_send() -> i32 {
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let counter_ref = Arc::clone(&shared_counter);
        // Mutex<i32> adalah Sync -> maka &Mutex<i32> aman di-Send ke closure thread!
        let handle = thread::spawn(move || {
            let mut guard = counter_ref.lock().unwrap();
            *guard += 50;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_val = *shared_counter.lock().unwrap();
    final_val // 100
}
```

---

### 11.4 Mini Project: Worker Pool CLI Architecture

Worker Pool adalah pola arsitektur konkurensi standar industri untuk memproses volume tugas (*jobs*) dalam jumlah besar secara simultan tanpa overhead boros dari membuat dan menghancurkan OS thread berulang kali.

```text
       ┌─────────────┐
       │ Main Thread │
       └──────┬──────┘
              │ 1. Kirim Job (Job Channel)
              ▼
   ┌───────────────────────┐
   │ Arc<Mutex<Receiver>>  │ <── Antrean Bersama (Job Queue)
   └──┬───────┬───────┬────┘
      │       │       │       │ 2. Worker berebut job secara aman
      ▼       ▼       ▼       ▼
   ┌──────┐┌──────┐┌──────┐┌──────┐
   │ W-1  ││ W-2  ││ W-3  ││ W-4  │ (4 Worker Threads Standby)
   └──┬───┘└──┬───┘└──┬───┘└──┬───┘
      │       │       │       │ 3. Kirim hasil (Result Channel)
      └───────┼───────┼───────┘
              ▼
     ┌─────────────────┐
     │ Result Receiver │ <── 4. Main Thread Kumpulkan Hasil
     └─────────────────┘
```

#### Alur Kerja & Fitur Utama:
1. **Kirim Job (`job_tx.send(job)`)**:
   Main thread memasukkan paket pekerjaan (`Job`) yang memuat data dan beban komputasi ke dalam *Job Channel*.
2. **Worker Memproses Job (`Arc<Mutex<Receiver<Job>>>`)**:
   Karena `mpsc::Receiver` di standard library bersifat *Single-Consumer*, kita membungkusnya dalam `Arc<Mutex<...>>`. Setiap worker thread mengunci Mutex secara bergiliran untuk mengambil satu job, melepaskan kunci, lalu memproses tugas tersebut secara paralel tanpa menahan antrean worker lain.
3. **Worker Mengirim Hasil (`result_tx.send(result)`)**:
   Setelah tugas selesai diproses, worker membungkus luaran ke dalam `JobResult` dan mengirimkannya kembali ke *Result Channel*.
4. **Main Mengumpulkan Hasil (`result_rx.recv()`)**:
   Main thread mengonsumsi seluruh laporan `JobResult` dari *Result Channel* dan mencetak rekap status eksekusi.
5. **Graceful Completion & Shutdown**:
   Main thread mendrop `job_tx`. Ketika buffer job kosong dan `job_tx` sudah musnah, pemanggilan `lock.recv()` pada worker akan mengembalikan `None`. Seluruh worker thread keluar dari loop kerja secara anggun (*clean exit*), dan dipanggil `.join()` hingga tidak ada thread liar (*zombie thread*).

#### Contoh Implementasi (Selaras dengan `mini_project_11.rs`):
```rust
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct Job {
    pub id: usize,
    pub payload: String,
}

pub struct JobResult {
    pub job_id: usize,
    pub worker_id: usize,
    pub output: String,
}

pub struct WorkerPool {
    workers: Vec<Option<JoinHandle<()>>>,
    job_tx: Option<Sender<Job>>,
    result_rx: Receiver<JobResult>,
}

impl WorkerPool {
    pub fn new(num_workers: usize) -> Self {
        let (job_tx, job_rx) = mpsc::channel::<Job>();
        let (result_tx, result_rx) = mpsc::channel::<JobResult>();
        let shared_job_rx = Arc::new(Mutex::new(job_rx));

        let mut workers = Vec::with_capacity(num_workers);
        for id in 1..=num_workers {
            let rx = Arc::clone(&shared_job_rx);
            let tx = result_tx.clone();

            let handle = thread::spawn(move || {
                while let Ok(job) = rx.lock().unwrap().recv() {
                    let res = JobResult {
                        job_id: job.id,
                        worker_id: id,
                        output: format!("Diproses oleh W-{}", id),
                    };
                    let _ = tx.send(res);
                }
            });
            workers.push(Some(handle));
        }

        Self {
            workers,
            job_tx: Some(job_tx),
            result_rx,
        }
    }

    pub fn shutdown(&mut self) {
        drop(self.job_tx.take()); // Tutup antrean job
        for handle in &mut self.workers {
            if let Some(h) = handle.take() {
                let _ = h.join();
            }
        }
    }
}
```

---

#### Evaluasi Kelulusan FASE 11:
1. **Concurrency vs Parallelism**:
   - **Concurrency (Struktur Program)**: Desain menangani banyak hal secara bergantian atau tumpang tindih (*dealing with lots of things at once*). Bisa berjalan di 1 CPU core via time-slicing.
   - **Parallelism (Eksekusi Fisik)**: Eksekusi komputasi nyata secara bersamaan pada waktu yang sama di beberapa CPU core fisik (*doing lots of things at once*).
2. **`Send` vs `Sync`**:
   - `Send`: Garansi kepemilikan (*ownership*) tipe aman dipindahkan melintasi batas thread.
   - `Sync`: Garansi referensi bersama (`&T`) aman dibaca paralel oleh beberapa thread sekaligus ($\mathbf{T: Sync \iff \&T: Send}$).
3. **Pola Worker Pool**:
   - Mengalokasikan sejumlah OS thread tetap di awal, mendistribusikan antrean kerja lewat channel sinkron, dan mengumpulkan hasil secara terkontrol.

---

## FASE 12: Modern Asynchronous Rust & Tokio Runtime

### 12.1 Cara Kerja Asynchronous Rust di Balik Layar (Mental Model: Future, Poll, Executor, Waker, & Pin)

Tidak seperti runtime bahasa lain (seperti JavaScript atau Go), bahasa Rust **tidak menyertakan built-in asynchronous runtime** di dalam standard library-nya. Standard library Rust hanya mendefinisikan kontrak tipe dasar: trait `Future`, `Context`, `Poll`, `Pin`, dan `Waker`.

#### 1. Mengapa Future di Rust Bersifat Lazy (Pull-Based vs Push-Based)
- **Push-Based / Eager (JavaScript Promise, Go Goroutine)**:
  Begitu sebuah `Promise` dibuat di JavaScript (`new Promise(...)`), eksekusi fungsinya langsung berjalan di background event loop tanpa menunggu `then()` atau `await`.
- **Pull-Based / Lazy (Rust `Future`)**:
  Sebuah `Future` di Rust hanyalah sebuah *state machine pasif* (struct/enum). Jika Anda memanggil `async fn` atau membuat `Future` tetapi tidak pernah memanggil `.await` atau menyerahkannya ke executor untuk di-`poll()`, maka **nol baris kode yang dieksekusi** dan tidak ada alokasi thread atau memori heap tersembunyi.
- **Keuntungan Desain Lazy**:
  1. **Zero-Cost Abstraction**: Tidak ada overhead thread OS atau alokasi memori sebelum dibutuhkan.
  2. **Zero-Cost Cancellation**: Membatalkan future sangat sederhana dan aman: cukup berhenti mem-poll dan biarkan struct Future di-`drop` (`Drop` trait). Tidak memerlukan cancellation token yang kompleks.
  3. **Komposisi Kombinator Sangat Efisien**: Operasi seperti `tokio::select!` atau timeout dapat memilih future mana yang di-poll dan membatalkan cabang yang kalah tanpa membebani runtime.

#### 2. Trait `Future` dan Metode `poll()`
Definisi trait `std::future::Future` pada standard library:
```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```
- Saat sebuah Future di-poll oleh executor:
  - Mengembalikan `Poll::Ready(output)` jika komputasi/operasi I/O telah rampung.
  - Mengembalikan `Poll::Pending` jika masih menunggu event eksternal (misal timer hardware, paket data socket TCP, pembacaan file disk).
- Siklus Polling State Machine:
  ```text
  [Created: Lazy/Dormant] 
             ↓
        executor.poll()
             ↓
     +---------------+
     | Belum siap?   | ---> return Poll::Pending (Simpan cx.waker())
     +---------------+                           ↓ (Tidur/Parkir thread)
             | Selesai!                          ↓ (Event selesai -> waker.wake())
             ↓                                   ↓
    return Poll::Ready(val) <--- executor.poll() kembali
  ```

#### 3. Peran `Waker` dan `Context`: Menghindari 100% CPU Spinning
Jika sebuah Future mengembalikan `Poll::Pending`, bagaimana executor tahu kapan harus mem-poll lagi?
- **Tanpa Waker**: Executor harus melakukan *busy-waiting* (looping `while pending { poll() }`), yang akan memakan utilisasi CPU hingga 100% secara sia-sia.
- **Dengan Waker (`std::task::Waker`)**:
  - Saat `poll()` mengembalikan `Poll::Pending`, future mengkloning waker dari `cx.waker()`.
  - Future mendaftarkan waker tersebut ke event source (misal thread helper atau OS epoll/kqueue).
  - Executor memarkir (menidurkan) thread (`thread::park()`).
  - Ketika data/event siap, event source memanggil `waker.wake()`.
  - `waker.wake()` membangunkan thread executor (`thread::unpark()`) dan memasukkan task kembali ke antrean siap jalan (*run queue*) untuk di-poll kembali.

#### 4. Peran `Executor` & Runtime (`block_on`)
Karena standard library Rust tidak menyediakan loop eksekutor bawaan, aplikasi membutuhkan *executor* (seperti `tokio`, `async-std`, atau mini-executor buatan sendiri).
Executor bertanggung jawab untuk:
1. Mengemas Future ke dalam `Pin`.
2. Menyediakan `Context` yang berisi `Waker`.
3. Memanggil `poll()` berulang kali hingga menghasilkan `Poll::Ready`.
4. Menidurkan thread ketika task berstatus `Pending`.

#### 5. Mengapa `Pin` dan `Unpin` Ada (Self-Referential Structs)
- **Masalah**: Ketika kita menulis `async fn`, compiler Rust mengubahnya menjadi enum state machine. Jika di dalam `async fn` terdapat variabel lokal yang direferensikan melintasi titik `.await`, struct state machine tersebut akan menyimpan pointer ke field di dalam dirinya sendiri (**Self-Referential Struct**).
  ```rust
  async {
      let mut text = String::from("Hello");
      let ptr = &text; // pointer menunjuk ke field di dalam frame yang sama
      timer_sleep().await; // State disimpan di dalam struct Future!
      println!("{}", ptr);
  }
  ```
- **Bahaya Memory Safety**: Jika struct yang bersifat self-referential ini dipindahkan (*move*) ke alamat memori lain (misalnya dioper ke fungsi lain, dimasukkan ke Vec, atau di-swap):
  - Alamat `text` berpindah ke alamat fisik baru.
  - Namun `ptr` masih menunjuk ke alamat fisik lama!
  - Menghasilkan **dangling pointer** dan **Undefined Behavior** fatal.
- **Solusi dengan `Pin`**:
  - `Pin<P>` adalah wrapper pointer (seperti `Pin<&mut T>` atau `Pin<Box<T>>`) yang menjamin bahwa data bertipe `T` **tidak akan pernah dipindahkan (moved) dari alamat memorinya** selama belum di-drop.
  - Trait marker `Unpin`: Tipe data reguler (`i32`, `String`, struct biasa) secara otomatis mengimplementasikan `Unpin` (bebas dipindahkan meski di dalam `Pin`).
  - Tipe `!Unpin` (seperti compiler-generated async futures yang self-referential) terkunci secara permanen di alamat memorinya.
  - Method `poll(self: Pin<&mut Self>, ...)` mewajibkan future di-pin sebelum dapat dieksekusi, sehingga keamanan memori terjamin 100% pada saat compile time!

#### 6. Implementasi Nyata Mini-Executor & Future Manual (`fase12_task_1.rs`)
Rujuk implementasi lengkap pada `rust-learning-lab/src/fase12_task_1.rs` yang mengilustrasikan:
1. `LazyCalculationFuture`: Pembuktian bahwa future tidak melakukan mutasi apapun sebelum di-poll.
2. `CountdownFuture`: Pelacakan multi-step `Poll::Pending` hingga `Poll::Ready`.
3. `ThreadWaker` & `mini_block_on`: Pembuatan executor `block_on` murni standard library dengan thread parking dan waking.
4. `AsyncTimerYieldFuture`: Simulasi async delay menggunakan worker thread terpisah yang memicu `waker.wake()`.
5. `UnsafeSelfRefSimulator`: Simulasi self-referential struct untuk membuktikan secara empiris bahwa pemindahan memori tanpa `Pin` menghasilkan pointer invalid (dangling).

---


### 12.2 Modern Tokio Asynchronous Runtime (Task 2)

**Tokio** adalah platform runtime asynchronous standar industri untuk Rust yang menyediakan event loop berbasis I/O non-blocking (epoll/kqueue/IOCP), work-stealing thread pool scheduler, pengatur waktu (timers), dan abstraksi task concurrency.

```bash
# Menambahkan Tokio dengan seluruh modul bawaan ke Cargo.toml:
cargo add tokio --features full
```

#### 1. Makro `#[tokio::main]` & Inisialisasi Runtime
Rust `fn main()` standar adalah fungsi sinkron (synchronous). Untuk menjalankan kode asynchronous di tingkat atas, runtime Tokio harus dibangun terlebih dahulu.

Makro `#[tokio::main]` adalah syntactic sugar compiler yang mengekspansi kode:
```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
}
```
Menjadi ekivalen manual:
```rust
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Gagal membuat runtime")
        .block_on(async {
            println!("Hello from Tokio!");
        });
}
```

#### 2. `async fn` dan Operator `.await`
- `async fn`: Mengubah fungsi biasa menjadi fungsi yang mengembalikan compiler-generated struct `impl Future<Output = T>`.
- `.await`: Menangguhkan eksekusi fungsi saat ini secara kooperatif jika future belum siap (`Poll::Pending`), mengembalikan kendali ke thread pool Tokio agar thread tersebut dapat mengeksekusi task lain. Saat event selesai, task dilanjutkan persis di titik `.await`.

#### 3. `tokio::spawn` & `JoinHandle` (Work-Stealing Task Concurrency)
- `tokio::spawn(async { ... })`: Menjadwalkan async closure/future ke thread pool Tokio secara independen (sering disebut *green thread* / *lightweight task*).
- Karakteristik:
  1. **Alokasi Rendah**: Ukuran task Tokio hanya beberapa ratus byte (jauh lebih ringan dibanding thread OS ~2MB).
  2. **Syarat Trait**: Task yang di-spawn harus memenuhi bound `'static + Send` karena dapat dipindahkan antar-thread worker oleh work-stealing scheduler.
  3. **Mengembalikan `JoinHandle<T>`**: Dapat di-await untuk memperoleh output `Result<T, JoinError>`.
  4. **Isolasi Panic**: Jika task yang di-spawn mengalami panic, thread worker Tokio TIDAK crash! Panic tersebut diisolasi dan dilaporkan melalui `join_err.is_panic()`.

#### 4. `tokio::join!` vs `tokio::spawn`
| Fitur | `tokio::join!(fut1, fut2)` | `tokio::spawn(fut)` |
|---|---|---|
| **Lokasi Eksekusi** | Berjalan di task & thread yang sama | Didaftarkan ke global worker thread pool |
| **Bound Lifetime** | Mendukung referensi lokal (`'a`) | Wajib kepemilikan penuh (`'static + Send`) |
| **Overhead** | Nol alokasi (zero allocation) | Alokasi heap untuk task state machine |
| **Tujuan** | Menjalankan beberapa I/O konkuren bersamaan | Memproses background job independen jangka panjang |

#### 5. `tokio::select!` (Multiplexing, Racing, & Cancellation)
`tokio::select!` memantau beberapa future secara simultan pada task yang sama:
- **Cabang Pertama Menang**: Begitu salah satu cabang menyelesaikan operasinya (`Poll::Ready`), blok kode cabang tersebut langsung dieksekusi.
- **Auto-Cancellation**: Seluruh cabang future lain yang belum selesai akan langsung di-`drop` (dibatalkan tanpa kebocoran resource berkat model *lazy future*).
- Sangat ideal untuk pola perlombaan koneksi (*hedged requests*), graceful shutdown, dan timeout.

#### 6. `tokio::time::sleep` vs `std::thread::sleep`
> [!CAUTION]
> **Dilarang Keras** memanggil `std::thread::sleep()` di dalam fungsi asynchronous Tokio!
> `std::thread::sleep` memblokir OS thread fisik secara total, sehingga puluhan task async lain yang mengantre di thread tersebut akan mengalami mogok (*thread starvation*).
> **Selalu gunakan `tokio::time::sleep(duration).await`** yang mendaftarkan timer ke driver Tokio tanpa memblokir thread.

#### 7. `tokio::time::timeout` (Deadlines Protection)
Mencegah operasi I/O (seperti HTTP request atau query database) menggantung tanpa batas waktu:
```rust
use tokio::time::{timeout, Duration};

let res = timeout(Duration::from_millis(50), fetch_sensor(10)).await;
match res {
    Ok(data) => println!("Sukses sebelum deadline: {:?}", data),
    Err(_elapsed) => eprintln!("Operasi dibatalkan karena melebihi batas waktu!"),
}
```

#### 8. `tokio::task::spawn_blocking` (Offloading Heavy CPU Work)
Jika program harus menjalankan komputasi intensif CPU (misal enkripsi, parsing JSON gigabyte, kompresi gambar, atau library FFI sinkron C):
- Gunakan `tokio::task::spawn_blocking(|| { ... })`.
- Tokio mengalihkan closure tersebut ke **blocking thread pool terpisah** (dapat berkembang hingga 512 thread OS bawaan), menjaga core async worker thread pool tetap responsif melayani ribuan request I/O.

#### 9. Rujukan Kode Implementasi Lengkap
- Proyek demonstrasi modul lab: [`rust-learning-lab/src/fase12_task_2.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase12_task_2.rs)
- Proyek standalone hands-on: [`tokio-lab/src/main.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/tokio-lab/src/main.rs)

---

### 12.3 Concurrent Tasks & Aggregation (Task 3)

Dalam arsitektur backend asynchronous, seringkali kita perlu meluncurkan sejumlah task secara simultan (batch spawning)—misalnya memanggil 10 API vendor eksternal, membaca 10 file partisi, atau memproses 10 item antrean sekaligus.

#### 1. Batch Spawning dengan Tokio Task
- **Efisiensi Memori**: Membuat 10 atau 10.000 Tokio task jauh lebih murah daripada membuat 10 OS Thread. Sebuah OS thread memerlukan alokasi stack 2MB–8MB, sedangkan sebuah Tokio task hanyalah struct state machine berukuran beberapa ratus byte di heap.
- **Eksekusi Asinkron**: Setiap task berjalan independen di thread pool work-stealing. Task yang memiliki durasi sleep/tunggu lebih singkat akan selesai lebih awal tanpa terhalang oleh task yang lambat (*non-blocking*).

#### 2. Pola 1: Pengumpulan Hasil via `Vec<JoinHandle<T>>`
Pola klasik untuk spawning batch dan menanti seluruh hasilnya:
```rust
let mut handles = Vec::with_capacity(10);

for id in 1..=10 {
    let sleep_ms = ((11 - id) * 5) as u64; // Delay berbeda
    let handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        format!("Task {} selesai", id)
    });
    handles.push(handle);
}

// Join seluruh task dari parent
let mut results = Vec::new();
for handle in handles {
    let res = handle.await.expect("Task panic!");
    results.push(res);
}
```
> [!NOTE]
> Pola `Vec<JoinHandle>` memproses hasil secara berurutan sesuai urutan indeks di vector. Jika Task 1 selesai dalam 50ms dan Task 10 selesai dalam 5ms, parent loop tetap menunggu `handle[0]` selesai terlebih dahulu.

#### 3. Pola 2: Pengumpulan Hasil via `tokio::task::JoinSet<T>` (Idiomatic Modern Tokio)
`JoinSet` adalah koleksi task bawaan Tokio (sejak v1.21) yang dirancang khusus untuk mengelola batch task dinamis:
```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();

for id in 1..=10 {
    let sleep_ms = ((11 - id) * 5) as u64;
    set.spawn(async move {
        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
        format!("Task {} selesai", id)
    });
}

// Hasil keluar seketika begitu task manapun selesai (Out-of-Order / Stream-like)
while let Some(res) = set.join_next().await {
    let output = res.expect("Worker panic");
    println!("Diterima langsung: {}", output);
}
```
**Keunggulan `JoinSet`**:
1. **Out-of-Order Completion**: Task tercepat (sleep 5ms) langsung keluar dan diproses tanpa harus menunggu task terlama (sleep 50ms).
2. **Auto-Cancellation on Drop**: Jika struct `JoinSet` di-drop sebelum seluruh task selesai, seluruh sisa task yang masih berjalan akan otomatis dibatalkan (*aborted*), mencegah kebocoran background task (*task leakage*).

#### 4. Bukti Kinerja Konkurensi: Durasi Riil vs Sekuensial
Pada pengujian 10 task dengan delay 50ms, 45ms, ..., 5ms:
- **Akumulasi Sekuensial**: $50 + 45 + 40 + \dots + 5 = \mathbf{275\text{ ms}}$.
- **Durasi Riil Konkuren Tokio**: $\approx \max(\text{durasi}) = \mathbf{52\text{ ms}}$.
- Terbukti terjadi penghematan waktu lebih dari **80%** karena seluruh operasi tunggu I/O berjalan tumpang tindih secara konkuren!

#### 5. Rujukan Kode Implementasi Lengkap
- Modul lab: [`rust-learning-lab/src/fase12_task_3.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase12_task_3.rs)

---

### 12.4 Asynchronous Timeout Pattern & Deadline Management (Task 4)

Dalam aplikasi backend atau sistem jaringan produksi, **setiap operasi asynchronous WAJIB memiliki batas waktu (deadline)**. Tanpa timeout, satu request jaringan yang hang atau database yang lambat dapat menumpuk ribuan task yang menggantung hingga menghabiskan memori dan koneksi socket (*resource exhaustion*).

#### 1. Mekanisme `tokio::time::timeout`
Fungsi `tokio::time::timeout` membungkus `Future` apapun dengan batas durasi:
```rust
use tokio::time::{timeout, Duration};

let res = timeout(Duration::from_millis(50), simulate_api_request(100)).await;
```
Signature konseptual:
```rust
pub async fn timeout<F: Future>(duration: Duration, future: F) -> Result<F::Output, Elapsed>;
```
- **`Ok(T)`**: Future target berhasil selesai sebelum batas waktu berlalu.
- **`Err(Elapsed)`**: Waktu habis sebelum future selesai. Future target langsung di-drop dan dibatalkan secara bersih (*zero-cost cancellation*).

#### 2. Mekanisme Internal: Perlombaan (Race) via `tokio::select!`
Fungsi `timeout` pada dasarnya adalah abstraksi dari `tokio::select!` yang membalapkan operasi target melawan timer sleep:
```rust
tokio::select! {
    res = target_operation() => Ok(res),
    _ = tokio::time::sleep(limit) => Err(ApiError::Timeout),
}
```
Ketika cabang timer menang, cabang `target_operation` langsung dihentikan dan dilepaskan dari memori runtime.

#### 3. Penanganan Error Idiomatik (`Result<T, ApiError>`)
Praktik terbaik di Rust adalah memetakan `Elapsed` ke domain error aplikasi:
```rust
#[derive(Debug)]
pub enum ApiError {
    Timeout { endpoint: String, limit: Duration, actual_delay: Duration },
    ServiceUnavailable(String),
}

let result = match timeout(timeout_limit, simulate_api_request(endpoint, delay)).await {
    Ok(data) => Ok(data),
    Err(_elapsed) => Err(ApiError::Timeout {
        endpoint: endpoint.to_string(),
        limit: timeout_limit,
        actual_delay: delay,
    }),
};
```

#### 4. Studi Kasus Perbandingan: 100ms Request
| Durasi Request | Batas Timeout | Durasi Eksekusi Riil | Hasil (`Result`) | Status |
|---|---|---|---|---|
| **100 ms** | **50 ms** | **~50 ms** | `Err(ApiError::Timeout)` | Terhenti tepat saat 50ms, hemat 50ms sisa waktu tunggu |
| **100 ms** | **500 ms** | **~100 ms** | `Ok("200 OK")` | Sukses selesai dalam 100ms jauh sebelum batas 500ms |

#### 5. Rujukan Kode Implementasi Lengkap
- Modul lab: [`rust-learning-lab/src/fase12_task_4.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase12_task_4.rs)

---

### 12.5 Asynchronous Cancellation & Graceful Shutdown Pattern (Task 5)

Dalam aplikasi backend atau sistem worker terdistribusi, **mematikan proses secara paksa (hard kill / abort) dapat menyebabkan korupsi data**, transaksi database yang menggantung, atau file log yang terpotong di tengah jalan. **Graceful Shutdown** adalah pola wajib di mana worker diberi kesempatan menyelesaikan tugas aktif (*in-flight tasks*) dan membersihkan resource sebelum program benar-benar berhenti.

#### 1. Mekanisme Sinyal Shutdown di Tokio
Ada dua pola utama untuk menyalurkan sinyal shutdown di Tokio:
- **`tokio::sync::oneshot`**: Pola 1-ke-1 untuk mematikan worker tunggal secara presisi.
- **`tokio::sync::watch`**: Pola broadcast 1-ke-banyak (single-producer, multi-consumer) yang paling direkomendasikan untuk mematikan sekumpulan worker pool secara serentak.

#### 2. Pola Event Loop Worker Menggunakan `tokio::select!`
Worker menjalankan loop utama yang membalapkan pekerjaan rutin melawan sinyal shutdown:
```rust
use tokio::sync::watch;

async fn worker_loop(mut shutdown_rx: watch::Receiver<bool>) {
    loop {
        // Cek cepat apakah shutdown sudah aktif
        if *shutdown_rx.borrow() {
            break;
        }

        tokio::select! {
            // Cabang 1: Sinyal shutdown diterima dari main task
            changed = shutdown_rx.changed() => {
                if changed.is_ok() && *shutdown_rx.borrow() {
                    println!("Sinyal shutdown diterima! Memulai fase cleanup...");
                    // Selesaikan pekerjaan in-flight & flush buffer
                    break;
                }
            }
            // Cabang 2: Melakukan pekerjaan rutin terjadwal
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                proses_pekerjaan();
            }
        }
    }
}
```

#### 3. Siklus Hidup (Lifecycle) Graceful Shutdown
```text
[1. Worker Berjalan Normal] 
             ↓
[2. Sinyal Shutdown Dikirim (SIGINT/CTRL+C atau channel.send)]
             ↓
[3. Fase Drain: Worker menolak job baru & menyelesaikan sisa tugas aktif]
             ↓
[4. Fase Cleanup: Melepaskan koneksi DB, flush buffer memori ke disk]
             ↓
[5. Worker Loop Berhenti Bersih -> Parent JoinHandle selesai]
```

#### 4. Rujukan Kode Implementasi Lengkap
- Modul lab: [`rust-learning-lab/src/fase12_task_5.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase12_task_5.rs)

---

### 12.6 Mini Project: Async Job Processor & Evaluasi Kelulusan Fase 12

Mini Project Fase 12 menggabungkan seluruh pilar arsitektur Tokio runtime menjadi sebuah sistem pemrosesan antrean kerja asynchronous bertingkat produksi:

```text
  +-----------------------------------------------------------+
  |                        API / Main                         |
  +-----------------------------------------------------------+
                                ↓  (submit async job)
  +-----------------------------------------------------------+
  |              Job Channel (tokio::sync::mpsc)              |
  +-----------------------------------------------------------+
                                ↓  (shared lock/pull)
  +-----------------------------------------------------------+
  |      Async Worker Pool (tokio::spawn + tokio::select!)    |
  +-----------------------------------------------------------+
           ↓ (I/O sleep)       ↓ (spawn_blocking)    ↓ (timeout)
  +-----------------------------------------------------------+
  |                       Processing                          |
  +-----------------------------------------------------------+
                                ↓  (send result)
  +-----------------------------------------------------------+
  |            Result Channel (tokio::sync::mpsc)             |
  +-----------------------------------------------------------+
                                ↓
  +-----------------------------------------------------------+
  |                 Result Collector & Display                |
  +-----------------------------------------------------------+
```

#### 1. Arsitektur Komponen Utama
1. **Job Channel (`tokio::sync::mpsc`)**: Menampung antrean pekerjaan berkapasitas terbatas (*bounded*) agar produser tidak membanjiri memori jika konsumen lambat (*backpressure*).
2. **Worker Pool (`tokio::spawn` & `tokio::select!`)**: Worker berjalan sebagai lightweight task independen, mendengarkan pekerjaan baru atau sinyal broadcast shutdown (`tokio::sync::watch`).
3. **Penanganan Beban Kerja Hybrid (I/O vs CPU)**:
   - Beban I/O dijalankan langsung secara non-blocking via `.await`.
   - Beban CPU intensif dialihkan ke **`tokio::task::spawn_blocking`** untuk menjaga kelancaran async event loop.
   - Pekerjaan berisiko dibatasi menggunakan **`tokio::time::timeout`**.
4. **Graceful Shutdown**: Main task menyiarkan sinyal `shutdown_tx.send(true)`, memicu seluruh worker menyelesaikan sisa in-flight tasks dan menutup koneksi secara bersih.

#### 2. Rangkuman Kriteria Kelulusan FASE 12
- **`Future`**: State machine pasif berbasis pull (lazy). Tidak menjalankan komputasi apapun sebelum di-poll oleh executor. Menghasilkan abstraksi tanpa biaya tambahan (zero-cost) dan pembatalan instan yang aman.
- **OS Thread vs Tokio Task**:
  - *OS Thread*: Dikelola kernel OS, alokasi memori stack besar (2MB-8MB), mahal saat context switch. Terbatas ribuan thread.
  - *Tokio Task*: Green thread ringan (~ratusan byte) di user-space, dijadwalkan oleh work-stealing scheduler Tokio. Berpindah secara kooperatif di titik `.await`. Mampu menangani ratusan ribu task simultan.
- **`tokio::spawn`**: Meluncurkan task asinkron independen ke runtime thread pool. Membutuhkan bound `'static + Send`. Mengembalikan `JoinHandle` yang mengisolasi panic jika worker crash.
- **`tokio::select!`**: Memultipleks beberapa future secara simultan. Cabang pertama yang siap (`Poll::Ready`) akan dieksekusi, cabang lain langsung di-drop (dibatalkan tanpa kebocoran resource).
- **`tokio::task::spawn_blocking`**: Wajib digunakan untuk komputasi CPU intensif atau I/O sinkron blocking. Mengalihkan eksekusi ke dedicated blocking OS thread pool (hingga 512 thread) agar worker thread async tidak mengalami *thread starvation*.

#### 3. Rujukan Kode Implementasi Lengkap
- Proyek Mini Project: [`rust-learning-lab/src/mini_project_12.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/mini_project_12.rs)

---

## FASE 13: Testing & Quality Assurance (Unit, Integration, Documentation, Async Tests)

Pengujian di Rust bukan sekadar sarana pengecekan fungsionalitas di akhir pengembangan, melainkan pilar utama dalam desain arsitektur perangkat lunak. Sistem tipe dan model kepemilikan Rust mencegah *undefined behavior* dan *data race* pada waktu kompilasi, sedangkan pengujian otomatis menjamin kebenaran logika bisnis (*business logic correctness*).

---

### 13.1 Piramida dan Arsitektur Pengujian di Ekosistem Rust

Rust mengkategorikan pengujian ke dalam 4 tingkatan utama:

```text
               +-----------------------------+
               |     Documentation Tests     |  (Verifikasi contoh kode di dokumentasi)
               +-----------------------------+
               |      Integration Tests      |  (Black-box testing di direktori tests/)
               +-----------------------------+
               |         Unit Tests          |  (White-box testing di modul internal)
               +-----------------------------+
               |     Async Tests (Tokio)     |  (Event-loop & non-blocking execution)
               +-----------------------------+
```

#### 1. Unit Tests (White-Box Testing)
- **Lokasi**: Ditulis di file modul yang sama dengan kode yang diuji, di dalam submodul `mod tests`.
- **Atribut Kompilasi Kondisional `#[cfg(test)]`**:
  Anotasi ini memerintahkan kompilator rustc untuk **hanya mengompilasi dan menyertakan kode pengujian ketika menjalankan perintah `cargo test`**. Saat Anda mengompilasi binary rilis (`cargo build --release`), seluruh kode test diabaikan sepenuhnya sehingga tidak menambah ukuran binary ataupun overhead runtime (zero-cost abstraction).
- **Aksesibilitas**: Unit test memiliki hak istimewa untuk menguji fungsi dan field bertingkat privat (`fn internal_helper()`), sehingga sangat ideal untuk menguji detail logika internal.

#### 2. Integration Tests (Black-Box Testing)
- **Lokasi**: Berada di luar direktori `src/`, yakni di folder khusus `tests/` pada root crate (contoh: `tests/fase13_integration_test.rs`).
- **Kompilasi Mandiri**: Setiap file `.rs` di dalam direktori `tests/` dikompilasi oleh Cargo sebagai *crate terpisah* yang mengimpor library Anda melalui `use rust_learning_lab::...;`.
- **Tujuan**: Memperlakukan library Anda persis seperti konsumen eksternal yang mengonsumsi crate publik Anda. File integration test **tidak bisa mengakses item privat**, sehingga memastikan bahwa API publik library telah lengkap, ergonomis, dan tidak bocor implementasi internal.

#### 3. Documentation Tests (Doc-Tests)
- **Lokasi**: Ditulis langsung di dalam komentar dokumentasi tiga garis miring (`///`) di atas deklarasi struct, enum, atau fungsi publik di library (`lib.rs`).
- **Mencegah Documentation Rot**: Pada banyak bahasa lain, contoh kode pada dokumentasi sering kali usang dan usang seiring refactoring. Di Rust, perintah `cargo test` mengekstrak setiap blok kode markdown ` ```rust ... ``` ` pada komentar dokumentasi, lalu mengompilasi dan menjalankannya sebagai test case resmi.

#### 4. Asynchronous Testing (`#[tokio::test]`)
- **Tantangan**: Runner test bawaan Rust (`#[test]`) hanya mendukung eksekusi fungsi sinkron reguler (`fn`). Fungsi asinkron membutuhkan thread executor yang aktif untuk mem-poll future hingga selesai via `.await`.
- **Solusi**: Atribut `#[tokio::test]` dari crate Tokio secara otomatis menginisialisasi runtime Tokio sementara (biasanya *current-thread runtime*), membungkus fungsi `async fn`, dan menjalankan future tersebut hingga selesai dengan memanggil executor `block_on`.

---

### 13.2 Anatomi Macro Assertions Rust

Rust menyediakan macro bawaan yang kaya untuk memvalidasi kondisi pengujian:

| Macro | Fungsi Utama | Contoh Pemakaian |
| :--- | :--- | :--- |
| `assert!(kondisi)` | Memastikan ekspresi bernilai `true` (truthy). Gagal jika `false`. | `assert!(acc.is_active());` |
| `assert_eq!(kiri, kanan)` | Memastikan dua nilai bernilai sama (memerlukan trait `PartialEq` dan `Debug`). | `assert_eq!(acc.balance(), 1000.0);` |
| `assert_ne!(kiri, kanan)` | Memastikan dua nilai **tidak** sama. | `assert_ne!(acc1.id, acc2.id);` |
| `assert!(kondisi, "format", args...)` | Menambahkan pesan diagnostik kustom jika assertion gagal. | `assert_eq!(a, b, "Saldo akun {} tidak cocok!", id);` |

#### Penanganan Khusus untuk Nilai Desimal (Floating Point)
Tipe data floating point (`f64` / `f32`) tunduk pada standar presisi desimal IEEE 754, sehingga operasi aritmatika desimal sering kali menghasilkan angka pembulatan mikro (misal `0.1 + 0.2 != 0.3`).
- **Jangan gunakan**: `assert_eq!(balance, 0.0)` pada hasil operasi berulang.
- **Gunakan Epsilon Margin**:
  ```rust
  pub const FLOAT_EPSILON: f64 = 1e-7;
  assert!(acc.balance().abs() < FLOAT_EPSILON, "Saldo harus mendekati 0");
  ```

---

### 13.3 Strategi Pengujian: Success, Error, dan Edge Cases

Sebuah test suite yang tangguh wajib mencakup tiga skenario utama:

```text
+-------------------+---------------------------------------------------------+
| Skenario          | Deskripsi & Fokus Pengujian                             |
+-------------------+---------------------------------------------------------+
| 1. Success Case   | Happy path: input valid, alur bisnis normal, saldo      |
|    (Jalur Sukses) | bertambah/berkurang dengan benar.                       |
+-------------------+---------------------------------------------------------+
| 2. Error Case     | Sad path: input salah, batas terlampaui (overdraft),     |
|    (Jalur Gagal)  | akun nonaktif. Memastikan Result::Err dikembalikan rapi |
|                   | tanpa menyebabkan program mengalami panic.              |
+-------------------+---------------------------------------------------------+
| 3. Edge Case      | Nilai batas ekstrem: string kosong/whitespace, saldo    |
|    (Kasus Batas)  | ditarik tepat habis (0.0), pembagian dengan nol,        |
|                   | toleransi presisi floating-point.                       |
+-------------------+---------------------------------------------------------+
```

---

### 13.4 Implementasi Kode Acuan FASE 13

Berikut arsitektur kode lengkap yang telah diimplementasikan pada proyek laboratorium kita:

#### 1. Entitas & Logika Bisnis (`Account`)
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: u64,
    owner: String,
    balance: f64,
    is_active: bool,
}

impl Account {
    pub fn new(id: u64, owner: &str, initial_balance: f64) -> Result<Self, AccountError> {
        let trimmed = owner.trim();
        if trimmed.is_empty() {
            return Err(AccountError::InvalidOwnerName("Nama tidak boleh kosong".into()));
        }
        if initial_balance < 0.0 {
            return Err(AccountError::NegativeOrZeroAmount(initial_balance));
        }
        Ok(Self { id, owner: trimmed.to_string(), balance: initial_balance, is_active: true })
    }

    pub fn deposit(&mut self, amount: f64) -> Result<f64, AccountError> {
        if !self.is_active { return Err(AccountError::AccountInactive(self.id)); }
        if amount <= 0.0 { return Err(AccountError::NegativeOrZeroAmount(amount)); }
        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, AccountError> {
        if !self.is_active { return Err(AccountError::AccountInactive(self.id)); }
        if amount <= 0.0 { return Err(AccountError::NegativeOrZeroAmount(amount)); }
        if self.balance < amount {
            return Err(AccountError::InsufficientFunds {
                available: self.balance,
                required: amount,
            });
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    pub async fn settle_transaction_async(
        &mut self,
        amount: f64,
        fee: f64,
        latency_ms: u64,
    ) -> Result<f64, AccountError> {
        if !self.is_active { return Err(AccountError::AccountInactive(self.id)); }
        let total = amount + fee;
        if self.balance < total {
            return Err(AccountError::InsufficientFunds { available: self.balance, required: total });
        }
        tokio::time::sleep(std::time::Duration::from_millis(latency_ms)).await;
        self.balance -= total;
        Ok(self.balance)
    }
}
```

#### 2. Unit Testing Lengkap (`#[cfg(test)] mod tests`)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Assertions check
    #[test]
    fn test_assertions_equality_and_inequality() {
        let acc1 = Account::new(10, "User Alpha", 100.0).unwrap();
        let acc2 = Account::new(20, "User Beta", 200.0).unwrap();
        assert_eq!(acc1.id, 10);
        assert_ne!(acc1.id, acc2.id);
        assert!(acc1.is_active());
    }

    // Success case
    #[test]
    fn test_deposit_and_withdraw_success_pipeline() {
        let mut acc = Account::new(1, "Test User", 500.0).unwrap();
        assert_eq!(acc.deposit(250.0).unwrap(), 750.0);
        assert_eq!(acc.withdraw(300.0).unwrap(), 450.0);
    }

    // Error case
    #[test]
    fn test_insufficient_funds_error() {
        let mut acc = Account::new(1, "Alice", 100.0).unwrap();
        let err = acc.withdraw(150.0).unwrap_err();
        assert_eq!(err, AccountError::InsufficientFunds { available: 100.0, required: 150.0 });
    }

    // Edge case
    #[test]
    fn test_edge_case_whitespace_owner_and_zero_balance() {
        assert!(Account::new(1, "   \t  ", 100.0).is_err());

        let mut acc = Account::new(2, "Exact Target", 150.0).unwrap();
        assert!(acc.withdraw(150.0).is_ok());
        assert!(acc.balance().abs() < FLOAT_EPSILON);
    }

    // Async test
    #[tokio::test]
    async fn test_async_settlement_success_with_fee() {
        let mut acc = Account::new(55, "Merchant", 1000.0).unwrap();
        let remaining = acc.settle_transaction_async(200.0, 5.0, 10).await.unwrap();
        assert_eq!(remaining, 795.0);
    }
}
```

---

### 13.5 Standar Perintah Quality Assurance di Rust

Untuk memastikan kode siap produksi (*production-ready*), seluruh pipeline QA dijalankan secara bertahap:

1. **`cargo fmt --check`**: Memastikan konsistensi gaya kode mengikuti standar resmi rustfmt tanpa memodifikasi file.
2. **`cargo clippy --all-targets --all-features -- -D warnings`**: Linter komprehensif yang menganalisis antipattern, inefisiensi alokasi, dan idiomatisasi kode.
3. **`cargo test`**: Menjalankan seluruh test suite secara otomatis (Unit Tests, Integration Tests di `tests/`, dan Documentation Tests).
4. **`cargo build --release`**: Menghasilkan biner teroptimasi penuh dengan optimasi LTO dan stripping simbol debug.

---

---

### 13.6 Mini Project: Test Suite Task Manager (CRUD, Error, Concurrency, Async, & Integration Tests)

Mini Project Fase 13 menyatukan seluruh pilar pengujian perangkat lunak modern di Rust ke dalam sebuah arsitektur **Test Suite Task Manager**. Proyek ini mendemonstrasikan bagaimana sebuah sistem yang konkuren dan asinkron dapat diverifikasi secara deterministik tanpa celah *race condition*, *deadlock*, ataupun *unhandled errors*.

```text
                                  +-----------------------+
                                  |      TaskManager      |
                                  +-----------------------+
                                              |
                     +------------------------+------------------------+
                     |                        |                        |
           [State Management]        [Concurrency Guard]       [Async Execution]
           Arc<AtomicU64> (ID)       Arc<RwLock<HashMap>>      Tokio JoinSet & Timeout
                     |                        |                        |
          +----------+----------+             |             +----------+----------+
          | CRUD & State Flow   |             |             | Async Batch & Limit |
          | Pending -> InProg   |             |             | 10 Concurrent Tasks |
          | InProg -> Completed |             |             | 10ms Timeout Guard  |
          +---------------------+             |             +---------------------+
                                              |
                                    [Multi-Thread Stress]
                                    10 OS Threads × 20 Tasks
                                    Simultaneous Read/Write
```

#### 1. Lima Pilar Pengujian yang Diterapkan:
1. **CRUD Tests**:
   - *Create*: Validasi pembuatan task baru dengan atomic ID auto-increment dan verifikasi inisialisasi status `Pending`.
   - *Read*: Pengambilan task by ID, listing terurut, dan filtering multidimensi berdasarkan `TaskStatus` dan `TaskPriority`.
   - *Update*: Modifikasi judul, pergantian prioritas, dan pembaruan status transisi yang valid (`Pending` -> `InProgress` -> `Completed`).
   - *Delete*: Penghapusan record dari map internal dan verifikasi bahwa query selanjutnya mengembalikan `TaskManagerError::TaskNotFound`.
2. **Error Tests**:
   - Penolakan judul kosong atau hanya berisi karakter whitespace (`TaskManagerError::EmptyTitle`).
   - Penolakan manipulasi task dengan ID fiktif pada operasi `get_task`, `delete_task`, dan `transition_status`.
   - Proteksi State Machine: Mencegah transisi status ilegal (contoh: lompat langsung dari `Pending` ke `Completed` tanpa melewati `InProgress`, atau memodifikasi task yang telah berstatus `Completed`).
3. **Concurrency Tests**:
   - Menguji keandalan primitif sinkronisasi `Arc<RwLock<HashMap<u64, TaskRecord>>>` terhadap serangan akses data serentak.
   - Meluncurkan 10 OS Thread (`std::thread::spawn`) yang serentak menulis 20 task per thread (total 200 tasks), diselingi pembacaan paralel.
   - Memastikan tidak ada *poisoned lock*, tidak ada data race, dan seluruh 200 ID task bersifat unik (determinisme terjamin via `AtomicU64::fetch_add`).
4. **Asynchronous Tests (`#[tokio::test]`)**:
   - Pengujian siklus hidup pekerjaan asinkron dengan simulasi non-blocking I/O (`sleep(..).await`).
   - Penanganan *Deadline & Timeout*: Membungkus future dengan `tokio::time::timeout`. Task yang melebihi batas waktu (misal butuh 50ms namun diberi limit 10ms) otomatis dibatalkan (*cancelled*) dan mengembalikan `TaskManagerError::ExecutionTimeout`.
   - Pemrosesan Batch Konkuren: Memanfaatkan `tokio::task::JoinSet` untuk memproses sekumpulan task secara simultan di atas thread pool Tokio runtime.
5. **Integration Tests (Black-Box Testing)**:
   - Ditempatkan di direktori `tests/mini_project_13_integration_test.rs`.
   - Menguji library crate `rust_learning_lab` dari perspektif konsumen eksternal independen tanpa akses ke detail privat modul.

#### 2. Rangkuman Kode Implementasi Inti
```rust
#[derive(Debug, Clone)]
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<u64, TaskRecord>>>,
    next_id: Arc<AtomicU64>,
}

impl TaskManager {
    pub fn create_task(&self, title: &str, desc: &str, priority: TaskPriority) -> Result<u64, TaskManagerError> {
        let trimmed = title.trim();
        if trimmed.is_empty() { return Err(TaskManagerError::EmptyTitle); }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let record = TaskRecord { id, title: trimmed.into(), description: desc.into(), priority, status: TaskStatus::Pending, created_at_ms: Self::now_ms(), completed_at_ms: None };
        self.tasks.write().unwrap().insert(id, record);
        Ok(id)
    }

    pub async fn execute_task_async(&self, id: u64, duration_ms: u64) -> Result<TaskRecord, TaskManagerError> {
        self.transition_status(id, TaskStatus::InProgress)?;
        tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
        self.transition_status(id, TaskStatus::Completed)?;
        self.get_task(id)
    }
}
```

---

### 13.7 Rujukan File Proyek & Evaluasi Kelulusan FASE 13

- **Modul Task Fase 13**: [`rust-learning-lab/src/fase13_task_1.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/fase13_task_1.rs)
- **Modul Mini Project Fase 13**: [`rust-learning-lab/src/mini_project_13.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/mini_project_13.rs)
- **Integration Test Suite**: [`rust-learning-lab/tests/mini_project_13_integration_test.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/tests/mini_project_13_integration_test.rs)
- **Library Root & Re-exports**: [`rust-learning-lab/src/lib.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/lib.rs)
- **Binary Entrypoint**: [`rust-learning-lab/src/main.rs`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust-learning-lab/src/main.rs)
- **Daftar Checklist Task**: [`rust_execution_tasks.md`](file:///mnt/windows/Users/boyblanco/Documents/code/web/rust_belajar/rust_execution_tasks.md#L1138-L1157)

---

# BAGIAN IV: ADVANCED RUST & PRODUCTION WEB BACKEND

## FASE 14: Advanced Rust (Unsafe & Macros)

### 14.1 Unsafe Rust: Batasan, Raw Pointers, & Invariant Keamanan

#### 1. Filosofi Safe vs Unsafe Rust
Secara bawaan (*default*), compiler Rust adalah garda depan yang sangat ketat melalui **Borrow Checker** dan sistem tipe statis untuk mencegah seluruh bug memori (*memory bugs*) seperti *null pointer dereference*, *use-after-free*, *double free*, dan *data race*. Semua jaminan ini disebut **Safe Rust**.

Namun, pada level terdalam (sistem operasi, manipulasi perangkat keras langsung, interaksi C via FFI, maupun implementasi struktur data dasar seperti `Vec`, `Box`, atau `Arc`), compiler tidak dapat membuktikan keamanan memori secara statis tanpa asumsi tambahan. Di sinilah **Unsafe Rust** dibutuhkan:
- Keyword `unsafe` **bukan berarti kode tersebut salah atau buruk**.
- Keyword `unsafe` adalah cara programmer memberi tahu compiler: *"Saya tahu apa yang saya lakukan. Compiler tidak perlu membuktikan bagian ini karena saya pribadi yang menjamin keamanannya."*
- **Penting:** Blok `unsafe` **tidak** menonaktifkan borrow checker atau type checking biasa; ia hanya membuka izin untuk 5 "kemampuan khusus" (*superpowers*).

#### 2. Lima Kemampuan Khusus (The 5 Superpowers of Unsafe)
Di dalam blok `unsafe { ... }` atau fungsi `unsafe fn`, programmer diizinkan melakukan 5 hal berikut:
1. **Melakukan dereferensi *raw pointers*** (`*const T`, `*mut T`).
2. **Memanggil fungsi atau method `unsafe`** (contoh: fungsi C via FFI atau method internal standar seperti `slice::from_raw_parts_mut`).
3. **Mengimplementasikan trait `unsafe`** (seperti implementasi manual trait `Send` atau `Sync`).
4. **Mengakses atau memodifikasi variabel `static mut`**.
5. **Mengakses field dari tipe `union`** (sering dipakai saat interoperabilitas dengan struktur union bahasa C).

---

#### 3. Anatomi Raw Pointers (`*const T` & `*mut T`)
Raw pointer adalah pointer primitif tanpa perlindungan borrow checker Rust:
- **`*const T`**: Raw pointer *immutable* (hanya untuk membaca data).
- **`*mut T`**: Raw pointer *mutable* (dapat membaca dan mengubah data).

**Karakteristik Kritis Raw Pointers:**
- Boleh mengabaikan aturan borrowing (dapat memiliki banyak `*mut T` dan `*const T` yang menunjuk ke memori yang sama secara bersamaan).
- Tidak dijamin menunjuk ke memori yang valid (bisa bernilai `null` atau *dangling*).
- Tidak mengimplementasikan pembersihan otomatis (*Drop*).
- Tidak memiliki *lifetime* yang dilacak compiler.

##### A. Pembuatan Raw Pointer (Operasi Safe)
Perhatikan bahwa **membuat** raw pointer adalah operasi yang 100% **SAFE**. Blok `unsafe` hanya diwajibkan saat **mendereferensi** (mengakses isi memori).

```rust
use std::ptr;

let mut angka: i32 = 42;

// 1. Metode Klasik: Casting referensi aman
let p_const: *const i32 = &angka as *const i32;
let p_mut: *mut i32 = &mut angka as *mut i32;

// 2. Metode Modern (Rust 2024 / 1.51+): addr_of! dan addr_of_mut!
// Rekomendasi utama: Menghindari pembuatan referensi perantara yang berpotensi memicu UB jika memori unaligned.
let modern_const = ptr::addr_of!(angka);
let modern_mut = ptr::addr_of_mut!(angka);

// 3. Null Pointer
let null_ptr: *const i32 = ptr::null();
let null_mut: *mut i32 = ptr::null_mut();
assert!(null_ptr.is_null());
```

##### B. Dereferensi dan Modifikasi (Wajib Blok `unsafe`)
Untuk membaca nilai (`*p_const`) atau menulis nilai (`*p_mut = ...`), kita wajib menggunakan blok `unsafe`:

```rust
let mut angka: i32 = 42;
let p_const = std::ptr::addr_of!(angka);
let p_mut = std::ptr::addr_of_mut!(angka);

unsafe {
    // Membaca memori
    println!("Nilai dibaca: {}", *p_const); // Output: 42

    // Menulis memori
    *p_mut = 100;
    println!("Nilai setelah mutasi: {}", *p_const); // Output: 100
}

println!("Nilai pada variabel safe: {}", angka); // Output: 100
```

---

#### 4. Enam Invariant Keselamatan Memori (Programmer Invariants)
Saat menulis kode `unsafe`, kegagalan programmer memenuhi kontrak keamanan memori akan memicu **Undefined Behavior (UB)**. UB dapat berakibat pada korupsi data senyap, celah eksploitasi keamanan, atau crash (*segmentation fault*). 

Enam invariant pokok yang **wajib dijaga oleh programmer**:

| Invariant | Nama Kontrak | Konsekuensi Pelanggaran | Cara Mitigasi |
| :--- | :--- | :--- | :--- |
| **1. Non-Null** | Pointer tidak boleh `0x0` saat didereferensi. | Segfault seketika pada OS modern / panic fatal hardware. | Periksa `!ptr.is_null()` sebelum dereferensi. |
| **2. Proper Alignment** | Alamat wajib kelipatan `align_of::<T>()`. | Crash arsitektur (ARM/SPARC) atau penurunan performa ekstrem (x86). | Gunakan `(ptr as usize).is_multiple_of(align_of::<T>())`, atau gunakan `ptr::read_unaligned`. |
| **3. Valid & Non-Dangling** | Menunjuk ke blok memori aktif yang belum di-deallocate. | *Use-after-free*, eksploitasi pembacaan memori acak. | Pastikan alokasi pemilik (*owner*) belum keluar dari scope (*drop*). |
| **4. Proper Initialization** | Nilai bit pada memori valid untuk tipe data yang dibaca. | UB compiler optimasi (contoh: membaca bool dengan bit selain 0 atau 1). | Jangan baca memori mentah sebelum diinisialisasi; gunakan `std::mem::MaybeUninit`. |
| **5. No Aliasing Violation** | Menghormati aturan Stacked Borrows / Tree Borrows. | Optimasi compiler menghasilkan kode salah karena menduga pointer eksklusif. | Jangan buat dua `&mut` bersamaan yang merujuk ke data yang sama. |
| **6. Bounds Integrity** | Pointer arithmetic tidak boleh melompati alokasi buffer. | Membaca / menulis area memori program lain (*buffer overflow*). | Selalu lakukan boundary check eksplisit (`mid <= len`, dsb). |

---

#### 5. Pola Desain Produksi: Safe Abstraction (Pemisahan Aman di atas Unsafe)
Aturan emas di ekosistem Rust profesional: **Jangan biarkan `unsafe` bocor ke API publik.** 
Bungkus blok `unsafe` di dalam fungsi aman (*Safe Abstraction*) yang memvalidasi seluruh invariant di awal sebelum mengeksekusi operasi tak aman.

##### Studi Kasus: Implementasi `custom_split_at_mut`
Dalam safe Rust, kita tidak bisa memotong slice menjadi dua mutable slice sekaligus karena compiler menganggap kita meminjam `&mut` ganda dari array yang sama:
```rust
// Kode safe ini TIDAK BISA dikompilasi oleh borrow checker:
// let left = &mut slice[..mid];
// let right = &mut slice[mid..]; // ERROR: cannot borrow `slice` as mutable more than once at a time
```

Solusi dengan Safe Abstraction berbasis raw pointer:
```rust
use std::slice;

/// Safe wrapper di atas operasi unsafe raw pointer.
pub fn custom_split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    // 1. Verifikasi Invariant secara ketat sebelum unsafe:
    assert!(mid <= len, "Index mid ({}) melebihi panjang slice ({})", mid, len);

    let ptr: *mut T = slice.as_mut_ptr();

    // 2. Blok unsafe yang aman karena invariant telah diverifikasi:
    // SAFETY:
    // - ptr valid dan aligned dari slice aktif.
    // - mid <= len menjamin pointer arithmetic ptr.add(mid) berada dalam buffer.
    // - left dan right bersifat disjoint (tidak tumpang tindih), invariant eksklusivitas &mut terjaga.
    unsafe {
        let left = slice::from_raw_parts_mut(ptr, mid);
        let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        (left, right)
    }
}
```

##### Demonstrasi Penggunaan:
```rust
let mut dataset = [10, 20, 30, 40, 50, 60];
let (left, right) = custom_split_at_mut(&mut dataset, 3);

// Kita dapat memutasi kedua potongan slice secara paralel tanpa konflik!
for item in left.iter_mut() { *item *= 2; }
for item in right.iter_mut() { *item += 5; }

assert_eq!(dataset, [20, 40, 60, 45, 55, 65]);
```

Kode lengkap implementasi dan rangkaian pengujian untuk materi ini dapat dilihat langsung pada modul [fase14_task_1.rs](rust-learning-lab/src/fase14_task_1.rs).

---

### 14.2 Foreign Function Interface (FFI) & ABI Boundaries

#### 1. Konsep Application Binary Interface (ABI)
Ketika dua program yang dikompilasi (misalnya pustaka C dan aplikasi Rust) ingin saling memanggil fungsi dan bertukar struktur data di memori, mereka membutuhkan **ABI (Application Binary Interface)** yang disepakati:
- **API (Application Programming Interface)** adalah kontrak pada level kode sumber (*source code*, nama fungsi, tipe parameter).
- **ABI (Application Binary Interface)** adalah kontrak pada level biner dan register mesin (*machine level*):
  1. **Calling Convention**: Register CPU mana yang dipakai untuk mengirim argumen ke fungsi (contoh: `rdi`, `rsi`, `rdx` pada AMD64 Linux), bagaimana nilai balik dikembalikan, dan siapa yang bertanggung jawab membersihkan *stack frame* (caller vs callee).
  2. **Data Layout & Padding**: Ukuran dan urutan byte field struct. Secara default, Rust **tidak memiliki stable ABI**—compiler bebas mereorder posisi field demi optimasi memori terkecil.
  3. **Name Mangling**: Compiler Rust mengubah nama fungsi menjadi hash unik (contoh: `_ZN17rust_learning_lab14fase14_task_214c_add_integers17h...`) untuk mendukung namespace modul dan generic. Linker bahasa C tidak memahami format ini.
  4. **Unwinding**: Mekanisme penanganan exception/panic melintasi batasan runtime bahasa yang berbeda.

Bahasa C memiliki ABI standar yang sangat stabil di hampir semua sistem operasi (*lingua franca* komputasi). Rust menggunakan ABI C untuk berkomunikasi dengan dunia luar.

```rust
/// Struktur data yang aman dilewatkan melintasi batas FFI C:
/// Atribut `#[repr(C)]` mematikan reordering field Rust dan menyelaraskan padding persis seperti struct C.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CPoint2D {
    pub x: f64,
    pub y: f64,
    pub id: i32,
}
```

---

#### 2. Sintaks `extern "C"`: Exporting & Importing

##### A. Mengekspor Fungsi Rust ke C ABI (`#[unsafe(no_mangle)]`)
Agar fungsi Rust dapat dipanggil oleh bahasa C, Python, Go, atau runtime lainnya:
1. Gunakan konvensi pemanggilan `extern "C"`.
2. Gunakan tipe data C dari modul `std::ffi` (seperti `c_int`, `c_char`, `c_double`).
3. Matikan name mangling. **Catatan Rust 2024:** Karena mematikan name mangling dapat memicu tabrakan simbol linker global, atribut ini sekarang wajib ditulis sebagai `#[unsafe(no_mangle)]`.

```rust
use std::ffi::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn c_add_integers(a: c_int, b: c_int) -> c_int {
    a.saturating_add(b)
}
```

##### B. Mengimpor Fungsi C Eksternal (`unsafe extern "C"`)
Rust dapat memanggil fungsi pustaka C (seperti pustaka standar `libc`):
- Pada **Rust 2024 Edition**, blok `extern` wajib diawali dengan keyword `unsafe` (`unsafe extern "C"`).
- Pemanggilan fungsi eksternal C **selalu dianggap `unsafe`** oleh Rust karena compiler tidak dapat memeriksa kode C tersebut.

```rust
use std::ffi::{c_char, c_int};

// Mengimpor fungsi pustaka standar C (libc)
unsafe extern "C" {
    pub fn abs(x: c_int) -> c_int;
    pub fn strlen(s: *const c_char) -> usize;
}

// Pemanggilan wajib di dalam blok unsafe:
let absolute = unsafe { abs(-42) };
assert_eq!(absolute, 42);
```

---

#### 3. Penanganan String Lintas Batas FFI (`std::ffi`)
Tipe string Rust (`&str`, `String`) berbeda secara fundamental dengan C string:

| Fitur | Rust String (`&str` / `String`) | C String (`*const c_char`) |
| :--- | :--- | :--- |
| **Penyimpanan Panjang** | Pointer fat (`ptr` + `len` eksplisit) | Diakhiri byte null (`\0` *null-terminated*) |
| **Karakter Null** | Boleh memiliki byte `\0` di tengah teks | Karakter `\0` adalah penanda akhir string |
| **Encoding** | Wajib UTF-8 valid (dijamin compiler) | Tidak ada jaminan encoding (urutan byte mentah) |

Untuk menjembatani perbedaan ini, Rust menyediakan tipe khusus:
- **`CString`**: String teralokasi di heap (*owned*) yang dijamin memiliki akhiran byte `\0` dan tidak memiliki interior null.
- **`CStr`**: Referensi pinjaman (*borrowed*) ke string null-terminated yang valid di memori.

---

#### 4. Identifikasi Boundary Safe / Unsafe (The Adapter Pattern)
Aturan terpenting dalam integrasi sistem modern: **Jangan mengekspos pemanggilan FFI mentah ke aplikasi utama.** Buatlah *Safe Adapter / Wrapper* yang memvalidasi kontrak keamanan:

1. **Validasi Null Pointer**: Periksa apakah pointer `*const c_char` bernilai null sebelum diakses.
2. **Validasi UTF-8**: Gunakan `CStr::to_str()` yang mengembalikan `Result<&str, Utf8Error>`.
3. **Isolasi Panic (Anti-Crash Boundary)**: **Panic Rust dilarang keras unwind menembus batas ABI `extern "C"`!** Unwinding ke C runtime berujung pada *Undefined Behavior* atau abort seketika. Selalu isolasi panic dengan `std::panic::catch_unwind`.

##### Contoh Implementasi Safe Adapter Lengkap:
```rust
use std::ffi::{CStr, CString, c_char, c_int};
use std::panic::catch_unwind;

/// Safe wrapper di atas C strlen:
pub fn safe_c_strlen(text: &str) -> Result<usize, String> {
    // 1. Validasi: tolak jika ada byte null di tengah string Rust
    let c_str = CString::new(text).map_err(|e| format!("Interior null: {}", e))?;
    
    // 2. Transisi Unsafe: panggil libc strlen
    let len = unsafe { strlen(c_str.as_ptr()) };
    Ok(len)
}

/// Fungsi C ABI dengan isolasi panic Rust:
#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_safe_divide(a: c_int, b: c_int, out: *mut c_int) -> c_int {
    if out.is_null() { return -1; }

    // Menjaga agar panic tidak membocorkan stack unwinding ke pemanggil C
    let outcome = catch_unwind(|| {
        if b == 0 { panic!("Divide by zero"); }
        a / b
    });

    match outcome {
        Ok(val) => {
            unsafe { *out = val; }
            0 // Exit code sukses standard C
        }
        Err(_) => -2, // Error ditangkap secara tertib
    }
}
```

Kode lengkap demonstrasi dan 9 unit test pengujian untuk materi FFI ini tersedia pada modul [fase14_task_2.rs](rust-learning-lab/src/fase14_task_2.rs).

---

### 14.3 Declarative Macro System (`macro_rules!`)

#### 1. Konsep Metaprogramming & Higienitas Macro
Metaprogramming adalah teknik di mana kode program dapat menulis atau menghasilkan kode program lainnya. Di Rust, macro bekerja pada level sintaks pohon token (*TokenTree* / *Abstract Syntax Tree*) saat kompilasi, bukan sekadar penggantian teks mentah (*string replacement*) seperti `#define` pada C/C++:
- **Macro Bersifat Higienis (*Hygienic*)**: Variabel internal yang dideklarasikan di dalam macro tidak akan bertabrakan atau menimpa variabel lokal di tempat pemanggilan (*call site*).
- **Zero-Runtime Overhead**: Seluruh ekspansi macro diselesaikan 100% pada fase kompilasi. Binary yang dihasilkan sama efisiennya dengan kode yang ditulis manual.
- **Variadic Arguments**: Berbeda dengan fungsi Rust biasa yang memiliki jumlah parameter tetap, macro mendukung jumlah argumen dinamis (variadic) seperti `println!`, `vec!`, dan `format!`.

---

#### 2. Tabel Fragmen Penentu (*Designators*)
Saat mendefinisikan matcher pada macro, kita menentukan tipe fragmen token menggunakan sintaks `$nama:designator`:

| Designator | Kegunaan | Contoh Sintaks Valid |
| :--- | :--- | :--- |
| **`expr`** | Ekspresi apa pun yang mengevaluasi nilai | `10 + 20`, `"halo"`, `hitung_total()` |
| **`item`** | Deklarasi item bahasa tingkat atas | `struct User { id: u64 }`, `fn test() {}` |
| **`ident`** | Nama pengidentifikasi (identifier) mentah | Nama fungsi, variabel, nama struct, atau field |
| **`ty`** | Tipe data Rust yang valid | `i32`, `Vec<String>`, `Option<T>` |
| **`pat`** | Pola pencocokan (*pattern matching arm*) | `Some(x)`, `1..=10`, `(a, b)` |
| **`block`** | Blok kode yang diapit kurung kurawal `{}` | `{ let x = 1; x + 2 }` |
| **`stmt`** | Sebuah statement tunggal | `let x = 5;` |
| **`literal`** | Nilai literal konstan | `42`, `"string"`, `'🦀'`, `true` |
| **`vis`** | Penentu visibilitas | `pub`, `pub(crate)` |

---

#### 3. Sintaks Repetisi (`$(...)*`, `$(...)+`, `$(...)?`)
Macro declarative menggunakan tanda dollar dan kurung `$( ... )` untuk menangkap pengulangan token:
- **`*`**: Nol kali atau lebih (*zero or more*).
- **`+`**: Satu kali atau lebih (*one or more*).
- **`?`**: Nol atau satu kali (*optional*).
- **Separator**: Diletakkan sebelum simbol repetisi, contoh: `$( $x:expr ),*` (dipisahkan koma) atau `$( $k:expr => $v:expr ),+` (pemisah pasangan key-value).
- **Trailing Comma**: Pola `$(,)?` di akhir repetisi memungkinkan pemanggilan macro dengan koma di akhir baris (*trailing comma friendly*).

---

#### 4. Pola Implementasi Macro Produksi

##### A. Repetition & Expression Fragment (`calculate_sum!`)
```rust
#[macro_export]
macro_rules! calculate_sum {
    // Pola 1: Tanpa argumen
    () => { 0 };
    // Pola 2: Satu atau banyak ekspresi berulang
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

let hasil = calculate_sum!(10, 20, 30, 40); // 100
```

##### B. Custom Delimiter & Pattern Matching (`make_map!`)
```rust
#[macro_export]
macro_rules! make_map {
    () => { std::collections::HashMap::new() };
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

let roles = make_map!(
    "admin" => 1,
    "editor" => 2,
);
```

##### C. Multi-Branch Pattern Matching (`log_event!`)
```rust
#[macro_export]
macro_rules! log_event {
    (INFO: $msg:expr) => { format!("[INFO] {}", $msg) };
    (WARN: $code:expr, $msg:expr) => { format!("[WARN][Code: {}] {}", $code, $msg) };
    (METRIC: $name:expr => $val:expr) => { format!("[METRIC] {}={:.2}", $name, $val as f64) };
}

let msg = log_event!(WARN: 404, "Page not found");
```

##### D. Item Fragment (`$it:item`) untuk Boilerplate Derivation
Item fragment mencakup seluruh deklarasi item beserta visibilitas dan doc-comment:
```rust
#[macro_export]
macro_rules! wrap_with_debug_item {
    ( $it:item ) => {
        #[derive(Debug, Clone, PartialEq)]
        $it
    };
}

wrap_with_debug_item!(
    pub struct SensorReading {
        pub sensor_id: u32,
        pub value: f64,
    }
);
```

##### E. Identifier & Type Fragment (`$name:ident`, `$t:ty`)
Digunakan untuk membuat generator kode tingkat lanjut:
```rust
#[macro_export]
macro_rules! define_metric_pair {
    ($struct_name:ident { $field1:ident: $type1:ty, $field2:ident: $type2:ty }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $struct_name {
            pub $field1: $type1,
            pub $field2: $type2,
        }
    };
}
```

Kode lengkap demonstrasi dan 7 unit test pengujian untuk materi Macro System ini tersedia pada modul [fase14_task_3.rs](rust-learning-lab/src/fase14_task_3.rs).

---

### 14.4 Mini Project Fase 14: Utility Macro Suite & Evaluasi Kelulusan

#### 1. Deskripsi Mini Project
Mini Project Fase 14 mengintegrasikan seluruh materi Advanced Rust (Unsafe, FFI, dan Macro System) dengan merancang **Utility Macro Suite** siap pakai untuk kebutuhan produksi:
- **`log_value!(name, value)`**: Macro pencatat log terstruktur multi-cabang dengan dukungan tag konteks.
- **`create_vec!(...)`**: Macro inisialisasi Vector cerdas dengan optimasi pra-alokasi kapasitas (`with_capacity`) pada waktu kompilasi (*compile-time length calculation*).
- **`retry_operation!(max, op)`**: Macro kendali alur untuk melakukan retry otomatis pada operasi fallible (`Result`).
- **`timed_exec!(label, expr)`**: Macro benchmarking instan untuk mengukur durasi eksekusi ekspresi.

---

#### 2. Implementasi Macro Inti

##### A. Structured Logging Macro (`log_value!`)
```rust
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
```

##### B. Optimized Vector Macro (`create_vec!`)
Menghindari re-alokasi memori berulang dengan menghitung jumlah token di waktu kompilasi:
```rust
#[macro_export]
macro_rules! create_vec {
    () => { Vec::new() };
    (repeat: $elem:expr; $count:expr) => { vec![$elem; $count] };
    ( $( $elem:expr ),+ $(,)? ) => {{
        let count = <[()]>::len(&[ $( $crate::create_vec!(@replace_unit $elem) ),+ ]);
        let mut v = Vec::with_capacity(count);
        $( v.push($elem); )+
        v
    }};
    (@replace_unit $e:expr) => { () };
}
```

##### C. Resilience & Benchmark Macros (`retry_operation!` & `timed_exec!`)
```rust
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
```

---

#### 3. Jawaban Evaluasi Kelulusan FASE 14

| Kriteria Lulus | Pertanyaan Pokok | Jawaban Evaluasi Mendalam |
| :--- | :--- | :--- |
| **1. Keberadaan Unsafe** | Mengapa keyword `unsafe` harus ada di Rust? | Perangkat keras fisik (CPU, RAM, register MMIO) tidak memiliki konsep borrow checker. Rust membutuhkan pintu darurat (*escape hatch*) untuk mengakses memori mentah, mengimplementasikan tipe data fundamental (`Vec`, `Box`, `Arc`, `String`), mengoptimalkan algoritma kritis, dan menghubungkan ekosistem pustaka C (FFI). Keyword `unsafe` tidak mematikan borrow checker biasa, melainkan mengizinkan 5 aksi khusus di bawah tanggung jawab langsung programmer. |
| **2. Safe Abstraction** | Apa perbedaan Safe Abstraction dan Unsafe Implementation? | *Unsafe implementation* adalah operasi berisiko tinggi yang mendereferensi pointer mentah, mengabaikan boundary check, atau memanipulasi memori uninitialized. *Safe abstraction* adalah fungsi atau struct aman yang membungkus blok unsafe tersebut, di mana seluruh invariant (non-null, alignment, bounds check, non-aliasing) diverifikasi secara ketat terlebih dahulu. Contoh: `slice::split_at_mut` memverifikasi `mid <= len` secara aman sebelum menjalankan pointer arithmetic di dalam unsafe. |
| **3. Macro System** | Bagaimana cara kerja dan manfaat `macro_rules!`? | `macro_rules!` adalah sistem metaprogramming deklaratif berbasis pencocokan pola pohon token (*AST matching*) saat kompilasi. Manfaat utamanya adalah eliminasi duplikasi kode (*boilerplate*), penyediaan antarmuka variadic dinamis, sifat higienis (*hygienic* tidak mencemari variabel lokal), dan zero-runtime overhead. |
| **4. Konsep FFI & ABI** | Apa itu FFI dan Application Binary Interface (ABI)? | FFI (*Foreign Function Interface*) adalah mekanisme jembatan untuk memanggil fungsi dari/ke bahasa lain. ABI (*Application Binary Interface*) adalah kontrak biner tingkat mesin yang mengatur register pemanggilan fungsi (*calling convention*), tata letak dan padding memori (`#[repr(C)]`), pencegahan name mangling (`#[unsafe(no_mangle)]`), serta isolasi panic (`catch_unwind`) agar tidak merusak runtime bahasa lain. |

Kode lengkap demonstrasi dan rangkaian pengujian untuk Mini Project ini tersedia pada modul [mini_project_14.rs](rust-learning-lab/src/mini_project_14.rs).

---

## FASE 15: Production Backend Ecosystem & Capstone Web API Project

### 15.1 Arsitektur Microservice REST API Modern
Capstone Project ini mengintegrasikan seluruh kurikulum 15 fase:
- **Rust Edition 2024**.
- **Web Framework**: **Axum** (Routing, State Extractors, Json Responses).
- **Asynchronous Engine**: **Tokio Runtime**.
- **Serialization**: **Serde & Serde JSON**.
- **Storage Layer**: Clean Architecture Repository Pattern (`trait TaskRepository: Send + Sync`) di balik thread-safe `Arc<RwLock<HashMap>>`.
- **Domain Error Handling**: Custom Error mapping ke status code HTTP standard (`IntoResponse`).
- **Production Practices**: Graceful Shutdown listener pada sinyal `SIGINT` (Ctrl+C).

---

### 15.2 File Konfigurasi: `Cargo.toml`
```toml
[package]
name = "task_service_api"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = { version = "0.8", features = ["tokio"] }
tokio = { version = "1.49", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

---

### 15.3 File Implementasi Lengkap: `src/main.rs`
```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

// ==========================================
// 1. DOMAIN MODELS & ENUMS
// ==========================================
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
    pub priority: Priority,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskDto {
    pub title: String,
    pub priority: Priority,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskStatusDto {
    pub status: TaskStatus,
}

// ==========================================
// 2. ERROR ARCHITECTURE (Domain -> HTTP)
// ==========================================
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": err_msg
        }));

        (status, body).into_response()
    }
}

// ==========================================
// 3. REPOSITORY TRAIT (Clean Architecture)
// ==========================================
pub trait TaskRepository: Send + Sync {
    fn create(&mut self, title: String, priority: Priority) -> Result<Task, ApiError>;
    fn find_all(&self) -> Vec<Task>;
    fn find_by_id(&self, id: u64) -> Result<Task, ApiError>;
    fn update_status(&mut self, id: u64, status: TaskStatus) -> Result<Task, ApiError>;
    fn delete(&mut self, id: u64) -> Result<(), ApiError>;
}

// In-Memory Thread-Safe Implementation
pub struct InMemoryTaskRepo {
    store: HashMap<u64, Task>,
    next_id: u64,
}

impl InMemoryTaskRepo {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            next_id: 1,
        }
    }
}

impl TaskRepository for InMemoryTaskRepo {
    fn create(&mut self, title: String, priority: Priority) -> Result<Task, ApiError> {
        if title.trim().is_empty() {
            return Err(ApiError::BadRequest("Judul tugas tidak boleh kosong".into()));
        }

        let id = self.next_id;
        let task = Task {
            id,
            title: title.trim().to_string(),
            status: TaskStatus::Todo,
            priority,
        };

        self.store.insert(id, task.clone());
        self.next_id += 1;
        Ok(task)
    }

    fn find_all(&self) -> Vec<Task> {
        let mut list: Vec<Task> = self.store.values().cloned().collect();
        list.sort_by_key(|t| t.id);
        list
    }

    fn find_by_id(&self, id: u64) -> Result<Task, ApiError> {
        self.store
            .get(&id)
            .cloned()
            .ok_or_else(|| ApiError::NotFound(format!("Tugas #{} tidak ditemukan", id)))
    }

    fn update_status(&mut self, id: u64, status: TaskStatus) -> Result<Task, ApiError> {
        let task = self
            .store
            .get_mut(&id)
            .ok_or_else(|| ApiError::NotFound(format!("Tugas #{} tidak ditemukan", id)))?;

        task.status = status;
        Ok(task.clone())
    }

    fn delete(&mut self, id: u64) -> Result<(), ApiError> {
        self.store
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| ApiError::NotFound(format!("Tugas #{} tidak ditemukan", id)))
    }
}

// App State Container
pub type AppState = Arc<RwLock<Box<dyn TaskRepository>>>;

// ==========================================
// 4. CONTROLLER / HANDLERS
// ==========================================
async fn list_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let repo = state.read().await;
    Json(repo.find_all())
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, ApiError> {
    let repo = state.read().await;
    let task = repo.find_by_id(id)?;
    Ok(Json(task))
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskDto>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    let mut repo = state.write().await;
    let created = repo.create(payload.title, payload.priority)?;
    Ok((StatusCode::CREATED, Json(created)))
}

async fn update_task_status(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTaskStatusDto>,
) -> Result<Json<Task>, ApiError> {
    let mut repo = state.write().await;
    let updated = repo.update_status(id, payload.status)?;
    Ok(Json(updated))
}

async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    let mut repo = state.write().await;
    repo.delete(id)?;
    Ok(StatusCode::NO_CONTENT)
}

// ==========================================
// 5. SERVER ENTRYPOINT & GRACEFUL SHUTDOWN
// ==========================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inisialisasi logging
    tracing_subscriber::fmt::init();

    // Inisialisasi thread-safe app state
    let repo_store: Box<dyn TaskRepository> = Box::new(InMemoryTaskRepo::new());
    let state: AppState = Arc::new(RwLock::new(repo_store));

    // Router Axum
    let app = Router::new()
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route(
            "/api/tasks/{id}",
            get(get_task).delete(delete_task),
        )
        .route("/api/tasks/{id}/status", patch(update_task_status))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Server microservice aktif di http://{}", addr);

    // Graceful Shutdown listener
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("Server ditutup secara aman (Graceful shutdown complete).");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Gagal mengikat sinyal Ctrl+C");
    println!("Sinyal shutdown (Ctrl+C) diterima. Menyelesaikan sisa request...");
}
```

---

### 15.4 Production Docker Multi-Stage Build
Untuk menghasilkan binary Rust produksi yang aman, minimalis (~15MB), dan bebas dependensi runtime sistem:

```dockerfile
# Stage 1: Build binary teroptimasi
FROM rust:1.85-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Stage 2: Minimalist Scratch / Alpine Image
FROM alpine:3.21
WORKDIR /app
COPY --from=builder /app/target/release/task_service_api /app/task_service_api

EXPOSE 3000
ENTRYPOINT ["/app/task_service_api"]
```

---

## Ringkasan Akhir
Dengan menyelesaikan ke-15 fase ini, pemahaman Anda telah mencakup spektrum penuh ekosistem Rust modern:
1. **Fondasi Kompilasi**: Dari alokasi memori Stack/Heap dan model Borrow Checker.
2. **Abstraksi Berskala Besar**: Module system, generics, associated traits, dan lifetimes.
3. **Eksekusi Asynchronous**: Runtime Tokio, task spawning, dan concurrency thread-safe.
4. **Production Readiness**: Arsitektur REST API Axum, deserialisasi aman Serde, hingga image container minimalis siap deploy ke Kubernetes/Cloud.
