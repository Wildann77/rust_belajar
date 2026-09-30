// Fase 12 - Task 2: Tokio Asynchronous Runtime
// Rujukan: rust_learning_guide.md (Sub-bab 12.2) & rust_execution_tasks.md (L1024-L1045)
//
// Cakupan Materi:
// 1. #[tokio::main] & Tokio Runtime Initialization
// 2. async fn & .await
// 3. tokio::spawn & JoinHandle
// 4. tokio::join! (Concurrent Multi-Future Join)
// 5. tokio::select! (Branch Multiplexing & Race)
// 6. tokio::time::sleep (Non-blocking Asynchronous Delay)
// 7. tokio::time::timeout (Deadline & Timeout Handling)
// 8. tokio::task::spawn_blocking (Offloading Heavy CPU Work)

use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};

// ============================================================================
// 1. async fn & .await Dasar
// ============================================================================

/// Fungsi asynchronous sederhana yang mensimulasikan panggilan API / fetch data I/O.
pub async fn fetch_user_data(user_id: u32, delay_ms: u64) -> String {
    // tokio::time::sleep TIDAK memblokir OS thread; ia menangguhkan task kooperatif
    sleep(Duration::from_millis(delay_ms)).await;
    format!("User_{{{}}}: Active", user_id)
}

/// Fungsi async chaining yang menggunakan operator .await untuk mengeksekusi operasi berurutan.
pub async fn sequential_pipeline(id: u32) -> (String, u64) {
    let start = std::time::Instant::now();
    let res = fetch_user_data(id, 15).await;
    let elapsed = start.elapsed().as_millis() as u64;
    (res, elapsed)
}

// ============================================================================
// 2. tokio::spawn & JoinHandle
// ============================================================================

/// Mendemonstrasikan spawning task baru secara independen ke thread pool Tokio work-stealing scheduler.
/// Mengembalikan `JoinHandle<T>` yang dapat di-await untuk memperoleh hasilnya.
pub async fn demonstrate_spawn_and_join_handle() -> Vec<String> {
    println!("   [Spawn] Meluncurkan 3 background task ke thread pool Tokio...");

    let handle_1: JoinHandle<String> = tokio::spawn(async {
        sleep(Duration::from_millis(20)).await;
        "Task 1 selesai dari worker thread pool".to_string()
    });

    let handle_2: JoinHandle<String> = tokio::spawn(async {
        sleep(Duration::from_millis(15)).await;
        "Task 2 selesai dari worker thread pool".to_string()
    });

    let handle_3: JoinHandle<String> = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "Task 3 selesai dari worker thread pool".to_string()
    });

    // Menunggu setiap task selesai menggunakan JoinHandle.await
    let res_1 = handle_1.await.expect("Task 1 panic!");
    let res_2 = handle_2.await.expect("Task 2 panic!");
    let res_3 = handle_3.await.expect("Task 3 panic!");

    vec![res_1, res_2, res_3]
}

/// Mendemonstrasikan penanganan panic secara aman pada task yang di-spawn melalui JoinHandle.
pub async fn demonstrate_spawn_panic_isolation() -> bool {
    let handle: JoinHandle<()> = tokio::spawn(async {
        // Simulasi panic di dalam Tokio task terisolasi
        panic!("Simulasi crash pada isolated task!");
    });

    match handle.await {
        Ok(_) => false,
        Err(join_err) => {
            // JoinError::is_panic() bernilai true jika task crash karena panic
            join_err.is_panic()
        }
    }
}

// ============================================================================
// 3. tokio::join! (Concurrent Multi-Future Join)
// ============================================================================

/// Menggabungkan beberapa Future secara konkuren di dalam satu task tanpa perlu spawn thread baru.
/// Semua cabang dijalankan bersamaan dan fungsi menunggu hingga SELURUHNYA selesai.
pub async fn demonstrate_tokio_join() -> (String, String, String) {
    let start = std::time::Instant::now();

    // Tiga operasi masing-masing 25ms, jika sekuensial akan butuh ~75ms.
    // Dengan tokio::join!, ketiganya berjalan konkuren sehingga selesai dalam ~25ms.
    let (res_a, res_b, res_c) = tokio::join!(
        fetch_user_data(101, 25),
        fetch_user_data(102, 25),
        fetch_user_data(103, 25)
    );

    let duration_ms = start.elapsed().as_millis();
    println!(
        "   [tokio::join!] 3 request konkuren selesai dalam {} ms (jauh lebih cepat dari sekuensial ~75ms)",
        duration_ms
    );

    (res_a, res_b, res_c)
}

// ============================================================================
// 4. tokio::select! (Branch Multiplexing, Racing & Cancellation)
// ============================================================================

/// Mendemonstrasikan race condition terkendali menggunakan tokio::select!.
/// Cabang yang pertama kali selesai akan dieksekusi, sedangkan cabang lainnya dibatalkan (dropped).
pub async fn demonstrate_tokio_select_race() -> &'static str {
    let fast_op = async {
        sleep(Duration::from_millis(15)).await;
        "Koneksi Primer Cepat"
    };

    let slow_op = async {
        sleep(Duration::from_millis(80)).await;
        "Koneksi Sekunder Lambat"
    };

    tokio::select! {
        winner = fast_op => {
            println!("   [tokio::select!] Pemenang: {}", winner);
            winner
        }
        winner = slow_op => {
            println!("   [tokio::select!] Pemenang: {}", winner);
            winner
        }
    }
}

// ============================================================================
// 5. tokio::time::timeout
// ============================================================================

/// Menjalankan future dengan batas waktu (deadline) menggunakan `tokio::time::timeout`.
pub async fn demonstrate_timeout_handling()
-> (Result<String, &'static str>, Result<String, &'static str>) {
    // 1. Kasus Sukses: Operasi 20ms dengan batas timeout 100ms
    let success_case = timeout(Duration::from_millis(100), async {
        sleep(Duration::from_millis(20)).await;
        "Respons Berhasil Tiba".to_string()
    })
    .await;

    let success_result = match success_case {
        Ok(data) => Ok(data),
        Err(_) => Err("Timeout terlampaui!"),
    };

    // 2. Kasus Timeout: Operasi 150ms dengan batas timeout 30ms
    let timeout_case = timeout(Duration::from_millis(30), async {
        sleep(Duration::from_millis(150)).await;
        "Respons Telat".to_string()
    })
    .await;

    let timeout_result = match timeout_case {
        Ok(data) => Ok(data),
        Err(_) => Err("Request dibatalkan karena timeout > 30ms"),
    };

    (success_result, timeout_result)
}

// ============================================================================
// 6. tokio::task::spawn_blocking (Heavy CPU Offloading)
// ============================================================================

/// Fungsi simulasi komputasi CPU berat (sinkron dan intensif).
pub fn heavy_cpu_computation(iterations: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=iterations {
        sum = sum.wrapping_add(i.wrapping_mul(3));
    }
    sum
}

/// Mendemonstrasikan pemindahan tugas CPU berat dari async worker pool ke thread pool blocking khusus.
/// Ini mencegah terjadinya kelaparan task (thread starvation) pada async event loop.
pub async fn demonstrate_spawn_blocking() -> u64 {
    println!(
        "   [spawn_blocking] Mengirimkan komputasi intensif ke thread pool dedicated blocking..."
    );
    let result = tokio::task::spawn_blocking(|| heavy_cpu_computation(2_000_000))
        .await
        .expect("spawn_blocking thread panic!");

    result
}

// ============================================================================
// 7. Demonstrasi Keseluruhan (pub fn run())
// ============================================================================

/// Entrypoint demonstrasi sinkron yang menjembatani ke Tokio Runtime multi-thread.
/// Meniru perilaku macro `#[tokio::main]`.
pub fn run() {
    println!("=== FASE 12 - TASK 2: MODERN TOKIO RUNTIME ===");

    // Inisialisasi Tokio Multi-threaded Runtime secara terprogram
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Gagal menginisialisasi Tokio Runtime");

    runtime.block_on(async {
        // 1. #[tokio::main], async fn, & .await
        println!("\n1. async fn & .await (Sequential & Cooperative):");
        let (user_msg, time_ms) = sequential_pipeline(42).await;
        println!(
            "   Data diterima: \"{}\" dalam durasi {} ms",
            user_msg, time_ms
        );
        assert_eq!(user_msg, "User_{42}: Active");

        // 2. tokio::spawn & JoinHandle
        println!("\n2. tokio::spawn & JoinHandle (Thread Pool Work-Stealing):");
        let spawned_results = demonstrate_spawn_and_join_handle().await;
        for msg in &spawned_results {
            println!("   -> {}", msg);
        }
        assert_eq!(spawned_results.len(), 3);

        // Isolasi panic
        let panic_caught = demonstrate_spawn_panic_isolation().await;
        println!(
            "   Apakah panic pada spawned task berhasil diisolasi aman? {}",
            panic_caught
        );
        assert!(panic_caught);

        // 3. tokio::join!
        println!("\n3. tokio::join! (Concurrent Execution):");
        let (u1, u2, u3) = demonstrate_tokio_join().await;
        println!("   Hasil join: \"{}\", \"{}\", \"{}\"", u1, u2, u3);
        assert_eq!(u1, "User_{101}: Active");
        assert_eq!(u2, "User_{102}: Active");
        assert_eq!(u3, "User_{103}: Active");

        // 4. tokio::select! & tokio::time::sleep
        println!("\n4. tokio::select! (Branch Multiplexing & Race):");
        let race_winner = demonstrate_tokio_select_race().await;
        assert_eq!(race_winner, "Koneksi Primer Cepat");

        // 5. timeout
        println!("\n5. tokio::time::timeout (Deadlines):");
        let (ok_res, err_res) = demonstrate_timeout_handling().await;
        println!("   Skenario Tepat Waktu : {:?}", ok_res);
        println!("   Skenario Melebihi Batas: {:?}", err_res);
        assert!(ok_res.is_ok());
        assert!(err_res.is_err());

        // 6. spawn_blocking
        println!("\n6. tokio::task::spawn_blocking (Heavy CPU Offloading):");
        let calc_result = demonstrate_spawn_blocking().await;
        println!("   Hasil kalkulasi spawn_blocking: {}", calc_result);
        assert!(calc_result > 0);

        println!("\n[OK] Seluruh demonstrasi FASE 12 Task 2 (Tokio) sukses & verified!\n");
    });
}

// ============================================================================
// 8. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_fn_and_await() {
        let res = fetch_user_data(99, 10).await;
        assert_eq!(res, "User_{99}: Active");
    }

    #[tokio::test]
    async fn test_spawn_and_join() {
        let results = demonstrate_spawn_and_join_handle().await;
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("Task 1 selesai"));
    }

    #[tokio::test]
    async fn test_spawn_panic_isolation() {
        let is_isolated = demonstrate_spawn_panic_isolation().await;
        assert!(is_isolated);
    }

    #[tokio::test]
    async fn test_tokio_join_concurrency() {
        let (a, b, c) = demonstrate_tokio_join().await;
        assert_eq!(a, "User_{101}: Active");
        assert_eq!(b, "User_{102}: Active");
        assert_eq!(c, "User_{103}: Active");
    }

    #[tokio::test]
    async fn test_tokio_select_fast_branch_wins() {
        let winner = demonstrate_tokio_select_race().await;
        assert_eq!(winner, "Koneksi Primer Cepat");
    }

    #[tokio::test]
    async fn test_timeout_success_and_failure() {
        let (ok_res, err_res) = demonstrate_timeout_handling().await;
        assert_eq!(ok_res, Ok("Respons Berhasil Tiba".to_string()));
        assert_eq!(err_res, Err("Request dibatalkan karena timeout > 30ms"));
    }

    #[tokio::test]
    async fn test_spawn_blocking_computes_correctly() {
        let sum = demonstrate_spawn_blocking().await;
        assert!(sum > 0);
    }

    #[test]
    fn test_sync_runner() {
        // Menguji bahwa synchronous bridge runner bekerja mulus
        run();
    }
}
