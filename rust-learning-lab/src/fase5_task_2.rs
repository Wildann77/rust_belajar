// Fase 5 - Task 2: Cargo Features & Optional Dependencies
// Rujukan: rust_learning_guide.md (Sub-bab 5.2) & rust_execution_tasks.md (L486-L498)

/// Mengembalikan deskripsi mode runtime saat ini berdasarkan fitur yang aktif saat kompilasi.
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

/// Memeriksa apakah feature async_runtime diaktifkan saat kompilasi.
pub fn is_async_enabled() -> bool {
    cfg!(feature = "async_runtime")
}

#[cfg(feature = "async_runtime")]
pub async fn execute_async_task(id: u64, name: &str) -> String {
    // Simulasi pekerjaan asinkron menggunakan tokio
    format!("[ASYNC RUNTIME] Task #{id} '{name}' berhasil diproses secara non-blocking via tokio")
}

#[cfg(not(feature = "async_runtime"))]
pub fn execute_sync_task(id: u64, name: &str) -> String {
    // Pekerjaan sinkron / blocking standar
    format!("[SYNC RUNTIME] Task #{id} '{name}' berhasil diproses secara sekuensial (blocking)")
}

/// Fungsi dispatcher utama yang otomatis beradaptasi dengan fitur yang dikompilasi.
pub fn process_task(id: u64, name: &str) -> String {
    #[cfg(feature = "async_runtime")]
    {
        // Jalankan future asinkron dengan membuat mini runtime tokio
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Gagal menginisialisasi runtime tokio");

        rt.block_on(async { execute_async_task(id, name).await })
    }

    #[cfg(not(feature = "async_runtime"))]
    {
        execute_sync_task(id, name)
    }
}

pub fn run() {
    println!("=== Fase 5 - Task 2: Cargo Features & Optional Dependencies ===");

    println!("1. Status Runtime Saat Ini:");
    println!("   {}", runtime_status());
    println!("   Fitur 'async_runtime' aktif? {}", is_async_enabled());

    println!("\n2. Eksekusi Workload:");
    let result_1 = process_task(101, "Sync/Async Database Ping");
    let result_2 = process_task(102, "Process Payment Event");
    println!("   {}", result_1);
    println!("   {}", result_2);

    println!("\n3. Konsep Inti Cargo Features:");
    println!("   - Fitur dideklarasikan di Cargo.toml pada blok [features].");
    println!(
        "   - Dependensi opsional ('optional = true') tidak akan diunduh/dikompilasi jika fitur tidak dipanggil."
    );
    println!(
        "   - Menggunakan sintaks 'dep:nama_crate' (Rust 2021/2024 edition) untuk mengaitkan fitur ke dependency."
    );
    println!(
        "   - Conditional compilation: #[cfg(feature = \"...\")] atau macro cfg!(feature = \"...\")."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_detection_and_execution() {
        let status = runtime_status();
        assert!(!status.is_empty());

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
