// tokio-lab: Standalone Hands-On Project for Tokio Async Runtime
// Checklist: #[tokio::main], async fn, .await, tokio::spawn, JoinHandle,
//            tokio::join!, tokio::select!, tokio::time::sleep, timeout, spawn_blocking

use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};

// 1. async fn & .await
async fn fetch_sensor(id: u32, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Sensor-{} [OK]", id)
}

// 2. CPU-heavy synchronous function
fn compute_hash_simulation(val: u64) -> u64 {
    let mut acc = val;
    for i in 1..=500_000 {
        acc = acc.wrapping_add(i);
    }
    acc
}

#[tokio::main]
async fn main() {
    println!("=== TOKIO LAB: HANDS-ON DEMONSTRATION ===");

    // 1. #[tokio::main], async fn, .await
    println!("\n1. async fn & .await:");
    let s1 = fetch_sensor(1, 20).await;
    println!("   Hasil .await langsung: {}", s1);

    // 2. tokio::spawn & JoinHandle
    println!("\n2. tokio::spawn & JoinHandle:");
    let handle: JoinHandle<String> = tokio::spawn(async {
        sleep(Duration::from_millis(15)).await;
        "Background task sukses!".to_string()
    });
    let spawned_res = handle.await.expect("Task failed");
    println!("   Hasil join handle: {}", spawned_res);

    // 3. tokio::join!
    println!("\n3. tokio::join! (Concurrent):");
    let (a, b) = tokio::join!(fetch_sensor(10, 20), fetch_sensor(20, 20));
    println!("   Join results: {} | {}", a, b);

    // 4. tokio::select! & tokio::time::sleep
    println!("\n4. tokio::select!:");
    tokio::select! {
        res = fetch_sensor(100, 10) => println!("   Select winner: {}", res),
        _ = sleep(Duration::from_millis(50)) => println!("   Select: sleep selesai"),
    }

    // 5. timeout
    println!("\n5. tokio::time::timeout:");
    match timeout(Duration::from_millis(50), fetch_sensor(200, 10)).await {
        Ok(data) => println!("   Timeout test: Sukses -> {}", data),
        Err(_) => println!("   Timeout terlampaui"),
    }

    // 6. spawn_blocking
    println!("\n6. tokio::task::spawn_blocking:");
    let blocking_res = tokio::task::spawn_blocking(|| compute_hash_simulation(42))
        .await
        .expect("Blocking task failed");
    println!("   Hasil compute blocking: {}", blocking_res);

    println!("\n[OK] tokio-lab standalone project berhasil dieksekusi!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sensor_fetch() {
        let res = fetch_sensor(1, 10).await;
        assert_eq!(res, "Sensor-1 [OK]");
    }

    #[tokio::test]
    async fn test_join() {
        let (a, b) = tokio::join!(fetch_sensor(1, 10), fetch_sensor(2, 10));
        assert_eq!(a, "Sensor-1 [OK]");
        assert_eq!(b, "Sensor-2 [OK]");
    }
}
