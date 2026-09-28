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

## FASE 3: Rust Type System (Structs, Enums, & Advanced Pattern Matching)

### 3.1 Structs: Classic, Tuple, Unit, & Methods
```rust
// 1. Classic Struct
#[derive(Debug, Clone)]
pub struct Account {
    pub id: u64,
    pub username: String,
    pub active: bool,
    pub balance: f64,
}

// 2. Tuple Struct
pub struct ColorRgb(pub u8, pub u8, pub u8);

// 3. Unit-like Struct (tanpa field, berguna untuk marker trait)
pub struct AdminPrivilege;

impl Account {
    // Associated function (constructor)
    pub fn new(id: u64, username: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            active: true,
            balance: 0.0,
        }
    }

    // Method membaca (&self)
    pub fn is_solvent(&self) -> bool {
        self.balance >= 0.0
    }

    // Method mutasi (&mut self)
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}
```

---

### 3.2 Advanced Pattern Matching
Pattern matching di Rust bersifat *exhaustive* (wajib menangani semua kemungkinan cabang).

```rust
#[derive(Debug)]
pub enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn process_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Halaman dimuat"),
        
        // Pattern dengan Match Guard (if condition)
        WebEvent::KeyPress(c) if c.is_ascii_digit() => {
            println!("Tombol angka ditekan: {}", c);
        }
        WebEvent::KeyPress(c) => println!("Tombol karakter: {}", c),

        WebEvent::Paste(ref text) => println!("Paste: {}", text),

        // Destructuring Struct-like variant dengan binding @ dan range
        WebEvent::Click { x: x @ 0..=100, y } => {
            println!("Klik di area pojok kiri: x={}, y={}", x, y);
        }
        WebEvent::Click { x, y } => println!("Klik koordinat: {}, {}", x, y),
    }
}

fn pattern_idioms_demo() {
    // while let: loop selama pola masih cocok
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Pop: {}", top);
    }

    // if let: ringkas jika hanya menguji 1 kondisi
    let config_max = Some(100u32);
    if let Some(max) = config_max {
        println!("Max disetel ke: {}", max);
    }
}
```

---

## FASE 4: Module System & Code Organization

Rust menyediakan sistem pengorganisasian kode hierarkis: **Packages -> Crates -> Modules -> Paths**.

### 4.1 Struktur Hierarki Standar Proyek
```text
my_service/
├── Cargo.toml          # Manifes package
├── src/
│   ├── main.rs         # Root binary crate (fn main)
│   ├── lib.rs          # Root library crate
│   ├── models.rs       # Module models
│   └── auth/           # Sub-module auth (folder berbasis nama module)
│       ├── mod.rs      # Entry point auth module (atau auth.rs)
│       └── token.rs    # Sub-module token
└── tests/
    └── api_tests.rs    # Integration test crate
```

---

### 4.2 Visibility & Scope Modifiers
Secara default, semua item di Rust bersifat **private**.
- `pub`: Terbuka untuk umum (*public*).
- `pub(crate)`: Hanya dapat diakses di dalam crate yang sama (tersembunyi dari luar library).
- `pub(super)`: Hanya dapat diakses oleh module parent setingkat di atasnya.
- `pub use`: Menampilkan kembali (*re-export*) item untuk memudahkan konsumsi pengguna library (*Facade Pattern*).

```rust
// File: src/auth/token.rs
pub struct Claims {
    pub sub: String,
    pub(crate) internal_session_id: u64, // Terlihat di crate ini saja
    secret_hash: String,                 // Private di file ini saja
}

// File: src/lib.rs
pub mod auth {
    pub mod token;
}
pub mod models;

// Re-exporting: memudahkan import dari luar
pub use auth::token::Claims;
```

---

## FASE 5: Cargo Tingkat Lanjut & Workspace Management

### 5.1 Cargo.toml Modern (Rust Edition 2024)
```toml
[package]
name = "enterprise_core"
version = "0.1.0"
edition = "2024"
authors = ["Senior Engineer <dev@enterprise.internal>"]
license = "MIT OR Apache-2.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.49", features = ["full"], optional = true }

[dev-dependencies]
criterion = "0.5" # Untuk benchmarking

[features]
default = []
async_runtime = ["dep:tokio"] # Feature flag modular

[profile.release]
opt-level = 3        # Optimasi maksimum
lto = "fat"          # Link-Time Optimization antar crate
codegen-units = 1    # Maksimalkan optimasi inline binary
panic = "abort"      # Hapus stack unwinding untuk ukuran binary minimal
strip = true         # Buang symbol debug dari executable binary
```

---

### 5.2 Multi-Crate Workspace Architecture
Untuk aplikasi berskala enterprise, gunakan `[workspace]` untuk menyatukan beberapa crate independen dalam satu repositori:
```toml
# File: ./Cargo.toml (Workspace Root)
[workspace]
resolver = "3"
members = [
    "crates/core_domain",
    "crates/database_adapter",
    "crates/api_server",
]

[workspace.dependencies]
tokio = { version = "1.49", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
tracing = "0.1"
```
Setiap sub-crate dapat memanggil dependensi bersama:
```toml
# File: ./crates/api_server/Cargo.toml
[dependencies]
tokio = { workspace = true }
core_domain = { path = "../core_domain" }
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
