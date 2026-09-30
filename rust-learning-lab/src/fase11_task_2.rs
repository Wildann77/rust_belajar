// Fase 11 - Task 2: Channels (mpsc, Multi-Producer, Single-Consumer, Transmitter Cloning, & Graceful Shutdown via drop(tx))
// Rujukan: rust_learning_guide.md (Sub-bab 11.2) & rust_execution_tasks.md (L945-L952)

use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

// ============================================================================
// 1. Single Producer, Single Consumer & Receiver Methods (.recv vs .try_recv)
// ============================================================================

/// Data pesan terstruktur yang dikirim melalui channel.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub sender_id: String,
    pub content: String,
}

/// Mendemonstrasikan channel dasar dengan 1 pengirim (single producer) dan 1 penerima (single consumer).
/// Memperlihatkan perbedaan antara `.recv()` (blocking) dan `.try_recv()` (non-blocking polling).
pub fn demonstrate_single_producer() -> (Vec<Message>, bool) {
    // mpsc::channel() mengembalikan pasangan (Sender, Receiver)
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Uji .try_recv(): saat channel masih kosong, tidak memblokir melainkan mengembalikan Empty
    let empty_before = matches!(rx.try_recv(), Err(TryRecvError::Empty));

    // Spawn producer thread tunggal
    thread::spawn(move || {
        let msg1 = Message {
            sender_id: "Worker-Single".to_string(),
            content: "Laporan 1: Operasi dimulai".to_string(),
        };
        let msg2 = Message {
            sender_id: "Worker-Single".to_string(),
            content: "Laporan 2: Operasi selesai".to_string(),
        };

        // .send() mentransfer ownership pesan ke receiver via buffer channel
        tx.send(msg1).expect("Gagal mengirim msg1");
        thread::sleep(Duration::from_millis(10));
        tx.send(msg2).expect("Gagal mengirim msg2");
        // tx keluar scope di sini dan otomatis di-drop
    });

    let mut received = Vec::new();

    // .recv() akan memblokir thread saat ini sampai pesan berikutnya tiba
    // Ketika seluruh Sender di-drop, .recv() mengembalikan Err(RecvError) tanda channel tutup
    while let Ok(msg) = rx.recv() {
        received.push(msg);
    }

    (received, empty_before)
}

// ============================================================================
// 2. Multi-Producer (Cloning Transmitter) & Single Consumer Iteration
// ============================================================================

/// Mendemonstrasikan pola MPSC di mana beberapa worker (Multi-Producer)
/// mengirim pesan ke 1 receiver pusat (Single Consumer) menggunakan `tx.clone()`.
pub fn demonstrate_multi_producer(producers_count: usize) -> Vec<Message> {
    let (tx, rx) = mpsc::channel();

    for id in 1..=producers_count {
        // Transmitter (Sender) BISA dan HARUS di-clone untuk tiap producer baru
        let thread_tx = tx.clone();

        thread::spawn(move || {
            let msg = Message {
                sender_id: format!("Producer-{}", id),
                content: format!("Data metrik dari sensor #{}", id),
            };
            thread::sleep(Duration::from_millis(5 * (id as u64)));
            thread_tx
                .send(msg)
                .expect("Gagal mengirim pesan dari producer");
            // thread_tx di-drop di akhir thread ini
        });
    }

    // ========================================================================
    // KRUSIAL: drop(tx) pada Transmitter Asli di Main Thread!
    // ========================================================================
    // Transmitter asli `tx` yang dibuat saat `mpsc::channel()` masih ada di scope ini.
    // Jika `tx` ini TIDAK di-drop, receiver loop `for msg in rx` akan mengira masih
    // ada kemungkinan pesan dikirim oleh main thread, sehingga program akan DEADLOCK / HANG!
    drop(tx);

    let mut all_messages = Vec::new();

    // Receiver mengimplementasikan IntoIterator.
    // Loop ini otomatis berhenti ketika seluruh Sender (termasuk tx asli) sudah di-drop!
    for msg in rx {
        all_messages.push(msg);
    }

    all_messages
}

// ============================================================================
// 3. Graceful Shutdown & Deteksi Channel Tutup via drop(tx)
// ============================================================================

/// Memverifikasi bahwa Receiver mendeteksi status terputus (Disconnected)
/// tepat setelah seluruh Sender di-drop.
pub fn demonstrate_channel_disconnect() -> bool {
    let (tx, rx) = mpsc::channel::<i32>();

    // Kirim data lalu drop eksplisit transmitter
    tx.send(100).unwrap();
    drop(tx); // Tutup channel secara sadar

    // Ambil data pertama
    let first = rx.recv();
    assert_eq!(first, Ok(100));

    // Panggilan .recv() berikutnya segera mengembalikan Err(RecvError) tanda channel ditutup
    let second = rx.recv();
    second.is_err()
}

// ============================================================================
// 4. Bounded Sync Channel (mpsc::sync_channel) & Backpressure
// ============================================================================

/// Menunjukkan perbedaan channel berukuran tetap (sync_channel) yang menyediakan backpressure.
/// Jika buffer penuh, sender akan terblokir sampai receiver membaca data.
pub fn demonstrate_sync_channel_backpressure() -> (usize, Vec<i32>) {
    // Buffer kapasitas hanya 2
    let (tx, rx) = mpsc::sync_channel::<i32>(2);

    let handle = thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        // tx.send(3) akan memblokir thread ini sampai receiver membaca salah satu item!
        tx.send(3).unwrap();
        drop(tx);
    });

    thread::sleep(Duration::from_millis(20));

    let mut collected = Vec::new();
    while let Ok(val) = rx.recv() {
        collected.push(val);
    }

    handle.join().unwrap();
    (collected.len(), collected)
}

// ============================================================================
// Runner Entry Point
// ============================================================================

pub fn run() {
    println!("=== FASE 11: Task 2 - Concurrency Channels (mpsc) ===");

    // 1. Single Producer
    println!("\n1. Single Producer, Single Consumer & Non-blocking try_recv():");
    let (single_res, empty_ok) = demonstrate_single_producer();
    println!(
        "   Status awal channel sebelum kirim (empty?): {}",
        empty_ok
    );
    for m in &single_res {
        println!("   [{}] -> Diterima: \"{}\"", m.sender_id, m.content);
    }
    assert_eq!(single_res.len(), 2);

    // 2. Multi-Producer (Cloning Transmitter)
    println!("\n2. Multi-Producer via tx.clone() (3 Producers -> 1 Consumer):");
    let multi_res = demonstrate_multi_producer(3);
    for m in &multi_res {
        println!("   [{}] -> {}", m.sender_id, m.content);
    }
    assert_eq!(multi_res.len(), 3);

    // 3. Graceful Shutdown & Disconnect via drop(tx)
    println!("\n3. Channel Closure via drop(tx) & Disconnect Detection:");
    let disconnected = demonstrate_channel_disconnect();
    println!(
        "   Apakah receiver mendeteksi penutupan channel dengan benar? {}",
        disconnected
    );
    assert!(disconnected);

    // 4. Bounded Sync Channel
    println!("\n4. Bounded Sync Channel (mpsc::sync_channel):");
    let (count, items) = demonstrate_sync_channel_backpressure();
    println!("   Total item tersinkronisasi: {} {:?}", count, items);
    assert_eq!(items, vec![1, 2, 3]);

    println!("\n[OK] Seluruh demonstrasi FASE 11 Task 2 (Channels) sukses & verified!\n");
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_producer_receives_all() {
        let (msgs, empty_initially) = demonstrate_single_producer();
        assert!(empty_initially);
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].content, "Laporan 1: Operasi dimulai");
        assert_eq!(msgs[1].content, "Laporan 2: Operasi selesai");
    }

    #[test]
    fn test_multi_producer_all_received() {
        let msgs = demonstrate_multi_producer(4);
        assert_eq!(msgs.len(), 4);
        let mut ids: Vec<String> = msgs.into_iter().map(|m| m.sender_id).collect();
        ids.sort();
        assert_eq!(
            ids,
            vec!["Producer-1", "Producer-2", "Producer-3", "Producer-4"]
        );
    }

    #[test]
    fn test_disconnect_on_tx_drop() {
        assert!(demonstrate_channel_disconnect());
    }

    #[test]
    fn test_sync_channel() {
        let (count, items) = demonstrate_sync_channel_backpressure();
        assert_eq!(count, 3);
        assert_eq!(items, vec![1, 2, 3]);
    }
}
