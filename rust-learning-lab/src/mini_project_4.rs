// Mini Project Fase 4: Modular Task App & Module System
// Mengorganisasikan kode ke dalam Package, Crate, Modules, dan Visibility Modifiers
// Rujukan: rust_learning_guide.md (FASE 4) & rust_execution_tasks.md (L413-L459)

use rust_learning_lab::{Claims, Priority, Task, TaskService, TaskStatus, authenticate};

pub fn run() {
    println!("============================================================");
    println!("=== Mini Project Fase 4: Modular Task App & Architecture ===");
    println!("============================================================");

    // 1. Penjelasan Struktur Modular
    println!("\n1. Struktur Modul & Crate yang Dibangun:");
    println!("   rust-learning-lab/ (Package)");
    println!("   ├── Cargo.toml");
    println!("   ├── src/");
    println!("   │   ├── lib.rs            <- Root Library Crate (rust_learning_lab)");
    println!("   │   ├── main.rs           <- Root Binary Crate");
    println!("   │   ├── models.rs         <- Modul Domain Model (Task, Priority, TaskStatus)");
    println!("   │   ├── auth/             <- Sub-modul Direktori Auth");
    println!("   │   │   ├── mod.rs        <- Entry Point Auth & pub(super) consumer");
    println!("   │   │   └── token.rs      <- Sub-modul Token (pub, pub(super), pub(crate))");
    println!("   │   └── services/         <- Modul Services");
    println!("   │       ├── mod.rs        <- Entry Point Services & re-export");
    println!("   │       └── task_service.rs <- Business Logic TaskService");
    println!("   └── tests/");
    println!("       └── task_app_integration_test.rs <- Integration Test Crate");

    // 2. Demonstrasi Re-export (Facade Pattern via pub use)
    println!("\n2. Re-export Ergonomics (`pub use` Facade Pattern):");
    println!("   Konsumen mengimpor langsung dari crate root `rust_learning_lab`:");
    println!("   `use rust_learning_lab::{{authenticate, Claims, Priority, Task, TaskService}};`");

    // 3. Demonstrasi Autentikasi (Module `auth`)
    println!("\n3. Demonstrasi Sub-modul `auth`:");
    let user_token = "johndoe:developer";
    let admin_token = "root:admin";

    let dev_claims: Claims = match authenticate(user_token) {
        Ok(c) => {
            println!("   [+] Berhasil login user: {}", c.summary());
            c
        }
        Err(e) => panic!("Gagal login: {e}"),
    };

    let admin_claims: Claims = match authenticate(admin_token) {
        Ok(c) => {
            println!("   [+] Berhasil login admin: {}", c.summary());
            c
        }
        Err(e) => panic!("Gagal login: {e}"),
    };

    // 4. Demonstrasi Service & Domain Model (Module `services` & `models`)
    println!("\n4. Demonstrasi Business Service & Enkapsulasi Visibility:");
    let mut task_service = TaskService::new("RustModularHub");

    // Developer membuat task normal
    let t1_id = task_service
        .create_task("Desain REST API Gateway", Priority::High, &dev_claims)
        .expect("Developer harus bisa buat task biasa");
    println!("   [✓] Task #{} dibuat oleh {}", t1_id, dev_claims.sub);

    // Admin membuat task critical
    let t2_id = task_service
        .create_task(
            "Mitigasi Zero-day Auth Exploit",
            Priority::Critical,
            &admin_claims,
        )
        .expect("Admin harus bisa buat critical task");
    println!("   [✓] Task #{} dibuat oleh {}", t2_id, admin_claims.sub);

    // Menampilkan daftar task melalui method display()
    println!("\n   Daftar Task Saat Ini:");
    for t in task_service.list_tasks() {
        println!("     {}", t.display());
    }

    // 5. Demonstrasi Validasi Hak Akses & Enkapsulasi
    println!("\n5. Uji Hak Akses & Role Boundary:");
    println!(
        "   Skenario A: Developer mencoba memajukan task Critical (#{}):",
        t2_id
    );
    match task_service.advance_task(t2_id, &dev_claims) {
        Ok(_) => println!("     [!] Error: Developer tidak boleh memajukan task Critical!"),
        Err(err) => println!("     [Ditolak Aman] {}", err),
    }

    println!("   Skenario B: Admin memajukan task Critical (#{}):", t2_id);
    match task_service.advance_task(t2_id, &admin_claims) {
        Ok(new_status) => println!(
            "     [Disetujui] Status berhasil dimajukan ke: {:?}",
            new_status
        ),
        Err(err) => println!("     [!] Gagal: {}", err),
    }

    println!(
        "   Skenario C: Developer memajukan task normal miliknya (#{}):",
        t1_id
    );
    match task_service.advance_task(t1_id, &dev_claims) {
        Ok(new_status) => println!(
            "     [Disetujui] Status berhasil dimajukan ke: {:?}",
            new_status
        ),
        Err(err) => println!("     [!] Gagal: {}", err),
    }

    println!("\n   Status Akhir Task:");
    for t in task_service.list_tasks() {
        println!("     {}", t.display());
    }

    // Filter menggunakan tipe Task dan TaskStatus
    let in_progress_tasks: Vec<&Task> = task_service
        .list_tasks()
        .iter()
        .filter(|t| t.status == TaskStatus::InProgress)
        .collect();
    println!("\n   Total task aktif: {}", task_service.total_tasks());
    println!(
        "   Task yang berstatus InProgress: {} buah",
        in_progress_tasks.len()
    );

    // 6. Ringkasan Visibility Modifiers
    println!("\n6. Ringkasan Tingkat Akses (Visibility):");
    println!("   - private      : Hanya terlihat di file/modul tempat dideklarasikan.");
    println!("   - pub(super)   : Hanya terlihat di modul parent setingkat di atasnya.");
    println!("   - pub(crate)   : Terlihat oleh seluruh modul dalam crate `rust_learning_lab`.");
    println!("   - pub          : Terbuka untuk umum (publik) ke luar crate/konsumen.");
    println!("   - pub use      : Re-export item agar API ergonomis dan modular.");
}
