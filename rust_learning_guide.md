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

Rust menolak konsep `null` pointer dan perkecualian runtime tak tertangani (*unhandled exceptions*). Rust mengadopsi penanganan eksplisit via enum `Option<T>` dan `Result<T, E>`.

### 6.1 Operator `?` & Error Combinators
Operator `?` mengekstrak nilai jika `Ok(T)`, atau langsung mengembalikan `Err(E)` keluar dari fungsi jika gagal.

```rust
use std::fs::File;
use std::io::{self, Read};

// Operator ? melakukan return dini otomatis jika terjadi error
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

// Combinators: map, and_then, unwrap_or_else
fn combinator_demo() {
    let raw_input: Option<&str> = Some("42");

    // Mengubah Option<&str> -> Option<i32> secara fungsional
    let parsed: Option<i32> = raw_input
        .and_then(|s| s.parse::<i32>().ok())
        .map(|n| n * 2);

    let final_val = parsed.unwrap_or(0);
    println!("Parsed: {}", final_val);
}
```

---

### 6.2 Custom Error Types dengan `thiserror` Pattern
Di backend profesional, buat custom enum error yang mendeskripsikan seluruh kemungkinan kegagalan domain:
```rust
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    DatabaseFailure(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Data Tidak Ditemukan: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Input Tidak Valid: {}", msg),
            AppError::DatabaseFailure(msg) => write!(f, "Kesalahan Database: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
```

---

### 6.3 Collections: Vector & HashMap Entry API
```rust
use std::collections::HashMap;

fn collections_deep_dive() {
    // 1. Vector: alokasi kapasitas awal mencegah re-alokasi berulang
    let mut vec = Vec::with_capacity(100);
    vec.extend([10, 20, 30]);
    vec.retain(|&x| x > 15); // Menyaring in-place: tersisa [20, 30]

    // 2. HashMap dengan SipHash DoS Protection
    let mut stats = HashMap::new();
    let text = "apel jeruk apel semangka jeruk apel";

    // Entry API: efisiensi update frekuensi kata
    for word in text.split_whitespace() {
        stats.entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    println!("Frekuensi: {:?}", stats);
}
```

---

## FASE 7: Generics, Traits, & Advanced Trait System

Trait adalah pondasi abstraksi dan polimorfisme di Rust.

### 7.1 Static Dispatch vs Dynamic Dispatch
- **Static Dispatch (Generics `T: Trait`)**: Compiler membuat duplikat kode mesin untuk setiap tipe konkret saat kompilasi (**Monomorphization**). Tidak ada overhead performa saat runtime (*Zero-Cost Abstraction*).
- **Dynamic Dispatch (`dyn Trait`)**: Menggunakan pointer tabel fungsi virtual (*vtable fat pointer*). Digunakan saat koleksi menyimpan objek-objek heterogen.

```rust
pub trait Storage: Send + Sync {
    fn save(&self, data: &[u8]) -> Result<(), String>;
}

// Static Dispatch: Monomorphized, performa maksimal
pub fn persist_static<T: Storage>(storage: &T, data: &[u8]) {
    let _ = storage.save(data);
}

// Dynamic Dispatch: Menggunakan vtable di balik Box/referensi
pub fn persist_dynamic(storage: &dyn Storage, data: &[u8]) {
    let _ = storage.save(data);
}
```

---

### 7.2 Associated Types, Supertraits, & Orphan Rule
```rust
// 1. Associated Types: mendefinisikan tipe terikat di dalam trait
pub trait Repository {
    type Item; // Associated type
    type Error;

    fn get_by_id(&self, id: u64) -> Result<Option<Self::Item>, Self::Error>;
}

// 2. Supertraits: trait yang membutuhkan implementasi trait lain
pub trait Auditable: std::fmt::Debug + Clone {
    fn audit_log(&self) -> String;
}

// 3. Orphan Rule & Newtype Pattern
// Aturan: Trait hanya boleh diimplementasikan jika Trait ATAU Tipe dibuat di crate lokal.
// Jika ingin mengimplementasikan Trait eksternal ke Tipe eksternal, bungkus ke Tuple Struct baru (Newtype):
pub struct CustomJson<T>(pub T);
```

---

## FASE 8: Lifetimes Mendalam ('a)

### 8.1 Filosofi Lifetime
> **Prinsip Utama**: Anotasi lifetime (`'a`) **tidak mengubah atau memperpanjang masa hidup objek**. Anotasi lifetime hanyalah parameter penjelas bagi compiler untuk membuktikan hubungan ketergantungan masa hidup antara input dan output referensi agar pointer tidak menggantung (*dangling pointer*).

### 8.2 Anotasi Lifetime pada Fungsi & Struct
```rust
// Compiler tahu bahwa slice kembalian memiliki masa hidup yang valid
// selama 'a (irisan masa hidup terpendek antara s1 dan s2)
fn find_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct yang menyimpan referensi wajib dianotasi lifetime:
// Struct Parser tidak boleh hidup lebih lama dari teks sumber `&'a str` yang dipegangnya!
pub struct Parser<'a> {
    pub source: &'a str,
    pub cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }
}
```

### 8.3 Tiga Aturan Lifetime Elision (Otomatisasi Compiler)
Compiler menginferensikan lifetime secara otomatis tanpa perlu ditulis manual jika:
1. Setiap parameter referensi mendapatkan parameter lifetime unik tersendiri.
2. Jika hanya ada tepat 1 parameter referensi input, lifetime input tersebut otomatis dipasangkan ke semua referensi output.
3. Jika terdapat parameter `&self` atau `&mut self` pada method, lifetime `self` otomatis dipasangkan ke semua referensi output.

---

## FASE 9: Functional Rust (Closures & Iterators)

### 9.1 Tiga Kategori Trait Closure
1. **`FnOnce`**: Mengambil kepemilikan (*moves*) variabel dari luar scope. Hanya bisa dipanggil 1 kali.
2. **`FnMut`**: Meminjam secara mutable (`&mut`) variabel dari luar scope. Bisa dipanggil berkali-kali dan memutasi state.
3. **`Fn`**: Hanya meminjam secara immutable (`&`) variabel dari luar scope. Bisa dipanggil berkali-kali secara aman (bahkan concurrent).

```rust
fn closure_demo() {
    let factor = 3;
    // Closure Fn (read-only borrow)
    let calc = |x: i32| x * factor;
    println!("Hasil: {}", calc(10));

    // Closure FnOnce dengan keyword `move`
    let data = vec![1, 2, 3];
    let consumer = move || {
        println!("Mengonsumsi data: {:?}", data);
    };
    consumer(); // data sudah di-drop di sini
}
```

---

### 9.2 Lazy Iterator Pipeline
```rust
fn iterator_pipeline() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Evaluasi lazy: kalkulasi hanya dieksekusi saat `.collect()` dipanggil
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)      // Ambil yang genap: [2, 4, 6, 8, 10]
        .map(|&x| x * x)              // Kuadratkan: [4, 16, 36, 64, 100]
        .take(3)                      // Ambil 3 pertama: [4, 16, 36]
        .collect();

    println!("Pipeline Result: {:?}", result);
}
```

---

# BAGIAN III: MEMORY, CONCURRENCY, & ASYNCHRONOUS SYSTEMS

## FASE 10: Smart Pointers & Interior Mutability

### 10.1 Alur Pedagogis Struktur Memori
1. **References (`&T`, `&mut T`)**: Pointer dasar dengan borrow check saat compile-time.
2. **`Box<T>`**: Mengalokasikan nilai di Heap, memegang pointer di Stack. Berguna untuk recursive types berukuran dinamis.
3. **`Deref` & `Drop` Traits**: Memungkinkan pointer transparan saat diakses (`*ptr`) dan otomatis melepaskan alokasi saat keluar scope.
4. **`Rc<T>`**: Reference Counting untuk kepemilikan bersama (*shared ownership*) pada lingkungan **single-thread**.
5. **`RefCell<T>` & Interior Mutability**: Mengizinkan mutasi data di balik referensi immutable (`&self`), dengan aturan borrowing diperiksa saat **runtime** (panic jika ada 2 mutable borrow bersamaan).
6. **`Arc<T>` (Atomic Reference Counting)**: Versi thread-safe dari `Rc<T>` untuk berbagi kepemilikan lintas thread OS.
7. **`Mutex<T>` & `RwLock<T>`**: Sinkronisasi kunci akses data bersama lintas thread.

```rust
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use std::rc::Rc;

fn smart_pointers_summary() {
    // Heap Box
    let b = Box::new(1024);

    // Single-thread shared mutability: Rc<RefCell<T>>
    let local_state = Rc::new(RefCell::new(0));
    *local_state.borrow_mut() += 1;

    // Multi-thread shared mutability: Arc<Mutex<T>> atau Arc<RwLock<T>>
    let shared_state = Arc::new(RwLock::new(vec![1, 2, 3]));
    {
        let mut write_guard = shared_state.write().unwrap();
        write_guard.push(4);
    } // write lock dilepas otomatis di sini
}
```

---

## FASE 11: Concurrency (Multi-Threading & Shared State)

### 11.1 Marker Traits: `Send` & `Sync`
- **`Send`**: Menandakan bahwa kepemilikan tipe data aman dipindahkan (*transferred*) ke thread lain.
- **`Sync`**: Menandakan bahwa tipe data aman diakses melalui referensi bersama (`&T`) dari beberapa thread secara bersamaan (`T` adalah `Sync` jika dan hanya jika `&T` adalah `Send`).

---

### 11.2 Message Passing: Channels (`mpsc`)
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn mpsc_demo() {
    let (tx, rx) = mpsc::channel();

    // Spawn 3 worker threads mengirim pesan ke 1 receiver
    for i in 1..=3 {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let msg = format!("Task #{} selesai", i);
            thread::sleep(Duration::from_millis(50));
            thread_tx.send(msg).unwrap();
        });
    }
    drop(tx); // Drop transmitter utama agar receiver tahu kapan channel ditutup

    for received in rx {
        println!("Diterima di main thread: {}", received);
    }
}
```

---

## FASE 12: Modern Asynchronous Rust & Tokio Runtime

### 12.1 Cara Kerja Asynchronous Rust di Balik Layar
Async Rust bersifat **pull-based** (kooperatif). Sebuah `Future` tidak melakukan komputasi apapun sampai ia di-*poll* oleh executor runtime.

```rust
// Definisi konseptual trait Future di Standard Library:
pub trait Future {
    type Output;
    // fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
- **`Pin`**: Mengunci lokasi memori Future agar tidak dipindahkan saat referensi internalnya aktif (*self-referential structs*).
- **`Waker`**: Mekanisme sinyal bagi task untuk memberitahu runtime executor: *"Saya sudah siap di-poll kembali karena I/O sudah selesai"*.

---

### 12.2 Task Concurrency dengan Tokio
- `tokio::spawn`: Menjadwalkan async task baru secara independen ke thread pool work-stealing scheduler Tokio. Mengembalikan `JoinHandle`.
- `tokio::select!`: Memultipleks beberapa Future dalam satu task tunggal (mengambil cabang pertama yang selesai).
- `tokio::task::spawn_blocking`: Menjalankan komputasi CPU-berat agar thread pool I/O async tidak kelaparan (*starvation*).

```rust
use tokio::time::{sleep, Duration};

async fn async_worker(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Worker {} selesai", id)
}

#[tokio::main]
async fn tokio_orchestration() {
    // 1. Spawning Concurrent Background Tasks
    let handle = tokio::spawn(async {
        async_worker(1).await
    });

    // 2. Select: Menunggu operasi atau Timeout
    tokio::select! {
        res = handle => println!("Hasil: {}", res.unwrap()),
        _ = sleep(Duration::from_millis(500)) => println!("Timeout terjadi!"),
    }
}
```

---

## FASE 13: Testing & Quality Assurance

Rust memiliki test harness kelas satu yang terpasang langsung di dalam compiler dan Cargo.

```rust
// File: src/calculator.rs
pub fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Pembagian dengan nol dilarang")
    } else {
        Ok(a / b)
    }
}

// ==========================================
// UNIT TESTS (Di dalam modul yang sama)
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let res = safe_divide(10.0, 0.0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Pembagian dengan nol dilarang");
    }

    #[tokio::test]
    async fn test_async_operation() {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(true);
    }
}
```

---

# BAGIAN IV: ADVANCED RUST & PRODUCTION WEB BACKEND

## FASE 14: Advanced Rust (Unsafe & Macros)

### 14.1 Unsafe Rust: Batasan & Kegunaan
Keyword `unsafe` memberi izin khusus kepada programmer untuk melakukan 5 aksi yang tidak dapat dijamin keamanannya oleh compiler:
1. Melakukan dereferensi *raw pointers* (`*const T`, `*mut T`).
2. Memanggil fungsi atau method `unsafe` (misal fungsi C via FFI).
3. Mengimplementasikan trait `unsafe`.
4. Mengubah nilai variabel `static mut`.
5. Mengakses field dari tipe `union`.

```rust
fn unsafe_raw_pointers() {
    let mut num = 42;

    // Membuat raw pointer dari referensi adalah SAFE:
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferensi raw pointer WAJIB di dalam blok unsafe:
    unsafe {
        println!("r1 poin ke: {}", *r1);
        *r2 = 99;
        println!("r2 diubah jadi: {}", *r2);
    }
}
```

---

### 14.2 Macro System
- **Declarative Macros (`macro_rules!`)**: Pembangkit kode berbasis pattern matching sintaks (*Metaprogramming*).
```rust
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

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
