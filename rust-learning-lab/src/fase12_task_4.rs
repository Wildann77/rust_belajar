// Fase 12 - Task 4: Asynchronous Timeout Pattern & Deadline Management
// Rujukan: rust_learning_guide.md (Sub-bab 12.4) & rust_execution_tasks.md (L1062-L1068)
//
// Cakupan Checklist:
// - Request simulasi 100ms.
// - Timeout 50ms (ekspektasi: Err/Timeout).
// - Timeout 500ms (ekspektasi: Ok/Sukses).
// - Handle keduanya menggunakan Result<T, E> secara idiomatik.

use std::fmt;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};

// ============================================================================
// 1. Domain Error untuk Timeout Handling
// ============================================================================

/// Tipe error khusus untuk menangani kegagalan operasi jaringan / timeout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    Timeout {
        endpoint: String,
        limit: Duration,
        actual_delay: Duration,
    },
    #[allow(dead_code)]
    ServiceUnavailable(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Timeout {
                endpoint,
                limit,
                actual_delay,
            } => {
                write!(
                    f,
                    "Request ke '{}' melebihi batas timeout {} ms (request butuh {} ms)",
                    endpoint,
                    limit.as_millis(),
                    actual_delay.as_millis()
                )
            }
            ApiError::ServiceUnavailable(msg) => write!(f, "Layanan tidak tersedia: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

// ============================================================================
// 2. Simulasi Request Asynchronous (100ms)
// ============================================================================

/// Mensimulasikan network/API request yang membutuhkan waktu tertentu (default: 100ms).
pub async fn simulate_api_request(endpoint: &str, delay: Duration) -> String {
    // Non-blocking asynchronous sleep
    sleep(delay).await;
    format!("200 OK: Data dari '{}' berhasil diambil", endpoint)
}

// ============================================================================
// 3. Handler Timeout Menggunakan `tokio::time::timeout`
// ============================================================================

/// Menjalankan request simulasi dengan perlindungan batas waktu (deadline)
/// menggunakan `tokio::time::timeout`, lalu memetakan hasilnya ke `Result<String, ApiError>`.
pub async fn request_with_timeout(
    endpoint: &str,
    request_delay: Duration,
    timeout_limit: Duration,
) -> Result<String, ApiError> {
    let endpoint_str = endpoint.to_string();

    // Membungkus future request dengan deadline timeout
    let timeout_future = timeout(
        timeout_limit,
        simulate_api_request(&endpoint_str, request_delay),
    );

    match timeout_future.await {
        Ok(response) => Ok(response),
        Err(_elapsed) => Err(ApiError::Timeout {
            endpoint: endpoint_str,
            limit: timeout_limit,
            actual_delay: request_delay,
        }),
    }
}

// ============================================================================
// 4. Implementasi Timeout Alternatif Menggunakan `tokio::select!`
// ============================================================================

/// Mengilustrasikan bagaimana timeout bekerja di tingkat fundamental
/// dengan cara membalapkan request future melawan timer sleep menggunakan `tokio::select!`.
pub async fn request_with_select_timeout(
    endpoint: &str,
    request_delay: Duration,
    timeout_limit: Duration,
) -> Result<String, ApiError> {
    let endpoint_str = endpoint.to_string();

    tokio::select! {
        res = simulate_api_request(&endpoint_str, request_delay) => {
            Ok(res)
        }
        _ = sleep(timeout_limit) => {
            Err(ApiError::Timeout {
                endpoint: endpoint_str,
                limit: timeout_limit,
                actual_delay: request_delay,
            })
        }
    }
}

// ============================================================================
// 5. Demonstrasi Utama (pub fn run())
// ============================================================================

pub fn run() {
    println!("=== FASE 12 - TASK 4: ASYNC TIMEOUT & DEADLINES ===");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Gagal membuat Tokio runtime untuk Task 4");

    runtime.block_on(async {
        let simulated_request_duration = Duration::from_millis(100);

        // 1. Skenario Timeout 50ms (Gagal karena 100ms > 50ms)
        println!("\n1. Skenario Timeout 50ms (Request butuh 100ms):");
        let timeout_50ms = Duration::from_millis(50);
        let start_a = Instant::now();

        let result_50ms = request_with_timeout(
            "/api/v1/fast-checkout",
            simulated_request_duration,
            timeout_50ms,
        )
        .await;

        let elapsed_a = start_a.elapsed().as_millis();
        println!("   Durasi eksekusi: {} ms", elapsed_a);
        match &result_50ms {
            Ok(data) => panic!("Seharusnya timeout, tapi berhasil: {}", data),
            Err(err) => {
                println!("   [Result Error]: {}", err);
                assert!(matches!(err, ApiError::Timeout { .. }));
            }
        }

        // 2. Skenario Timeout 500ms (Sukses karena 100ms < 500ms)
        println!("\n2. Skenario Timeout 500ms (Request butuh 100ms):");
        let timeout_500ms = Duration::from_millis(500);
        let start_b = Instant::now();

        let result_500ms = request_with_timeout(
            "/api/v1/analytics-report",
            simulated_request_duration,
            timeout_500ms,
        )
        .await;

        let elapsed_b = start_b.elapsed().as_millis();
        println!("   Durasi eksekusi: {} ms", elapsed_b);
        match &result_500ms {
            Ok(data) => {
                println!("   [Result Sukses]: {}", data);
                assert!(data.contains("200 OK"));
            }
            Err(err) => panic!("Seharusnya sukses, tapi timeout: {}", err),
        }

        // 3. Verifikasi dengan `tokio::select!` Timeout Pattern
        println!("\n3. Verifikasi Menggunakan tokio::select! Pattern:");
        let select_res_fail = request_with_select_timeout(
            "/api/v1/select-fail",
            simulated_request_duration,
            timeout_50ms,
        )
        .await;
        assert!(select_res_fail.is_err());
        println!("   Select timeout (50ms): {:?}", select_res_fail);

        let select_res_ok = request_with_select_timeout(
            "/api/v1/select-ok",
            simulated_request_duration,
            timeout_500ms,
        )
        .await;
        assert!(select_res_ok.is_ok());
        println!("   Select timeout (500ms): {:?}", select_res_ok);

        println!("\n[OK] FASE 12 Task 4 (Async Timeout) tuntas & terverifikasi!\n");
    });
}

// ============================================================================
// 6. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_50ms_fails_for_100ms_request() {
        let req_duration = Duration::from_millis(100);
        let limit = Duration::from_millis(50);

        let res = request_with_timeout("/api/test-short", req_duration, limit).await;
        assert!(res.is_err());

        if let Err(ApiError::Timeout {
            endpoint,
            limit: lim,
            actual_delay,
        }) = res
        {
            assert_eq!(endpoint, "/api/test-short");
            assert_eq!(lim, Duration::from_millis(50));
            assert_eq!(actual_delay, Duration::from_millis(100));
        } else {
            panic!("Ekspektasi ApiError::Timeout");
        }
    }

    #[tokio::test]
    async fn test_timeout_500ms_succeeds_for_100ms_request() {
        let req_duration = Duration::from_millis(100);
        let limit = Duration::from_millis(500);

        let res = request_with_timeout("/api/test-long", req_duration, limit).await;
        assert!(res.is_ok());
        let body = res.unwrap();
        assert!(body.contains("200 OK"));
        assert!(body.contains("/api/test-long"));
    }

    #[tokio::test]
    async fn test_select_based_timeout_equivalence() {
        let req_duration = Duration::from_millis(100);

        let fail_res =
            request_with_select_timeout("/api/select", req_duration, Duration::from_millis(30))
                .await;
        assert!(fail_res.is_err());

        let ok_res =
            request_with_select_timeout("/api/select", req_duration, Duration::from_millis(300))
                .await;
        assert!(ok_res.is_ok());
    }

    #[test]
    fn test_api_error_display() {
        let err = ApiError::ServiceUnavailable("Database timeout".to_string());
        assert!(format!("{}", err).contains("Database timeout"));
    }

    #[test]
    fn test_sync_runner_execution() {
        run();
    }
}
