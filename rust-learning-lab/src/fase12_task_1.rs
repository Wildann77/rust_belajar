// Fase 12 - Task 1: Future Mental Model (Lazy Futures, poll, Executor, Waker, & Pin)
// Rujukan: rust_learning_guide.md (Sub-bab 12.1) & rust_execution_tasks.md (L1016-L1022)

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::Duration;

// ============================================================================
// 1. Konsep 1: Mengapa Future Bersifat Lazy (Pull-Based vs Push-Based)
// ============================================================================

/// Struct penjelasan teoritis tentang karakteristik Future di Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureLazinessExplanation {
    pub execution_model: &'static str,
    pub comparison_with_js_promise: &'static str,
    pub cancellation_benefit: &'static str,
    pub allocation_overhead: &'static str,
}

/// Menghasilkan rincian mengapa Future di Rust didesain bersifat lazy (pull-based).
pub fn explain_future_laziness() -> FutureLazinessExplanation {
    FutureLazinessExplanation {
        execution_model: "Pull-based: Future adalah state machine pasif. Komputasi TIDAK berjalan sampai ada pemanggilan poll() oleh executor.",
        comparison_with_js_promise: "JavaScript Promise bersifat push-based/eager (langsung jalan saat dibuat). Rust Future bersifat lazy (nol biaya CPU/memori jika tidak di-await/di-poll).",
        cancellation_benefit: "Pembatalan (cancellation) bersifat gratis dan aman: cukup berhenti mem-poll dan drop struct Future. Tidak memerlukan token pembatalan rumit.",
        allocation_overhead: "Zero-cost abstraction: Future tidak memerlukan alokasi heap atau thread background tersembunyi kecuali diminta secara eksplisit.",
    }
}

/// Future demonstrasi untuk membuktikan bahwa tanpa pemanggilan `poll()`,
/// tidak ada kode atau mutasi state yang berjalan sama sekali.
pub struct LazyCalculationFuture {
    initial_value: usize,
    poll_counter: Arc<AtomicUsize>,
}

impl LazyCalculationFuture {
    pub fn new(initial_value: usize, poll_counter: Arc<AtomicUsize>) -> Self {
        Self {
            initial_value,
            poll_counter,
        }
    }
}

impl Future for LazyCalculationFuture {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Efek samping (peningkatan counter) HANYA terjadi saat method poll dieksekusi oleh executor
        self.poll_counter.fetch_add(1, Ordering::SeqCst);
        Poll::Ready(self.initial_value * 2)
    }
}

// ============================================================================
// 2. Konsep 2: Pahami `poll` dan Siklus State Machine
// ============================================================================

/// Future kustom bertahap yang membutuhkan sejumlah `target_polls` sebelum selesai.
/// Ini mendemonstrasikan bagaimana sebuah Future mengembalikan `Poll::Pending`
/// ketika pekerjaannya belum rampung, dan `Poll::Ready(T)` ketika selesai.
pub struct CountdownFuture {
    remaining_polls: usize,
    total_polled: usize,
}

impl CountdownFuture {
    pub fn new(steps: usize) -> Self {
        Self {
            remaining_polls: steps,
            total_polled: 0,
        }
    }

    pub fn total_polled(&self) -> usize {
        self.total_polled
    }
}

impl Future for CountdownFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.total_polled += 1;

        if self.remaining_polls > 1 {
            self.remaining_polls -= 1;
            // Minta bangunkan kembali agar langsung dijadwalkan ulang dalam tes polling berulang
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            self.remaining_polls = 0;
            Poll::Ready(format!(
                "Selesai setelah {} kali polling!",
                self.total_polled
            ))
        }
    }
}

// ============================================================================
// 3. Konsep 3 & 4: Waker dan Mini Executor (`block_on` buatan sendiri)
// ============================================================================

/// Implementasi `std::task::Wake` berbasis thread parking bawaan Rust.
/// Ketika event I/O atau komputasi selesai, pemanggil memanggil `waker.wake()`,
/// yang kemudian memanggil `thread.unpark()` untuk membangunkan thread executor yang sedang tidur.
pub struct ThreadWaker {
    pub thread: Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.thread.unpark();
    }
}

/// Mini Executor sederhana (`block_on`) murni standard library Rust tanpa dependensi eksternal!
/// Menunjukkan bagaimana runtime seperti Tokio atau futures-rs mengeksekusi Future:
/// 1. Membungkus future dalam `Pin`.
/// 2. Membuat `Waker` dan `Context`.
/// 3. Loop memanggil `poll()`. Jika `Pending`, thread diparkir (`thread::park`).
/// 4. Saat `wake()` dipanggil dari thread lain, thread executor aktif kembali dan mem-poll ulang.
pub fn mini_block_on<F: Future>(future: F) -> F::Output {
    // 1. Pinning future ke dalam Box heap agar lokasinya stabil di memori
    let mut pinned_future = Box::pin(future);

    // 2. Buat waker yang terikat dengan thread saat ini
    let thread_waker = Arc::new(ThreadWaker {
        thread: thread::current(),
    });
    let waker = Waker::from(thread_waker);
    let mut context = Context::from_waker(&waker);

    // 3. Polling loop
    loop {
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // Jangan busy-spin (menghabiskan CPU 100%).
                // Parkir thread sampai di-unpark oleh waker.wake()!
                thread::park();
            }
        }
    }
}

/// Future simulasi timer async kustom yang mengandalkan `Waker` di thread latar belakang.
/// Pada poll ke-1: Mengembalikan `Poll::Pending` dan mengirimkan clone Waker ke background worker thread.
/// Setelah waktu simulasi berlalu: Background thread memanggil `waker.wake()`.
/// Pada poll ke-2: State sudah selesai, mengembalikan `Poll::Ready`.
pub struct AsyncTimerYieldFuture {
    duration: Duration,
    started: Arc<AtomicBool>,
    completed: Arc<AtomicBool>,
}

impl AsyncTimerYieldFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            started: Arc::new(AtomicBool::new(false)),
            completed: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Future for AsyncTimerYieldFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed.load(Ordering::SeqCst) {
            return Poll::Ready("Operasi async timer selesai!");
        }

        // Jika belum pernah dimulai, spawn thread helper yang meniru callback I/O kernel
        if !self.started.swap(true, Ordering::SeqCst) {
            let waker = cx.waker().clone();
            let completed_flag = Arc::clone(&self.completed);
            let delay = self.duration;

            thread::spawn(move || {
                thread::sleep(delay);
                completed_flag.store(true, Ordering::SeqCst);
                // Beri sinyal ke executor bahwa future sudah siap di-poll kembali!
                waker.wake();
            });
        }

        Poll::Pending
    }
}

// ============================================================================
// 4. Konsep 5: Mengapa `Pin` Ada? (Self-Referential Structs & Memory Safety)
// ============================================================================

/// Ringkasan konsep mengapa `Pin` ada di Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinExplanation {
    pub problem: &'static str,
    pub self_referential_cause: &'static str,
    pub pin_guarantee: &'static str,
    pub unpin_trait: &'static str,
}

pub fn explain_pin_concept() -> PinExplanation {
    PinExplanation {
        problem: "Ketika data dipindahkan (move) di memori, alamat fisiknya berubah. Pointer yang menunjuk ke memori lama akan menjadi dangling pointer (Undefined Behavior).",
        self_referential_cause: "Compiler Rust mengubah async fn menjadi state machine enum/struct. Jika ada variabel lokal yang meminjam variabel lain melintasi titik .await, struct tersebut menjadi self-referential (menyimpan pointer ke dirinya sendiri).",
        pin_guarantee: "Pin<P> menjamin bahwa data yang ditunjuk TIDAK akan pernah dipindahkan atau di-swap di memori selama belum di-drop (jika tipe tersebut !Unpin).",
        unpin_trait: "Sebagian besar tipe biasa (i32, String) otomatis mengimplementasikan auto-trait `Unpin` (aman dipindahkan). Sedangkan Future hasil kompilasi async bertipe `!Unpin` sehingga wajib di-Pin sebelum di-poll.",
    }
}

/// Struktur demonstrasi edukatif untuk mengilustrasikan bahaya self-referential struct
/// jika terjadi operasi pemindahan (move) di memori tanpa proteksi `Pin`.
pub struct UnsafeSelfRefSimulator {
    pub value: String,
    pub self_ptr: *const String,
}

impl UnsafeSelfRefSimulator {
    pub fn new(txt: &str) -> Self {
        Self {
            value: txt.to_string(),
            self_ptr: std::ptr::null(),
        }
    }

    /// Menginisialisasi pointer internal agar menunjuk ke field `value` miliknya sendiri.
    pub fn init_internal_pointer(&mut self) {
        self.self_ptr = &self.value as *const String;
    }

    /// Membaca data melalui pointer internal.
    /// Ini aman HANYA jika struct belum berpindah alamat memori.
    pub fn read_via_pointer(&self) -> Option<&String> {
        if self.self_ptr.is_null() {
            None
        } else {
            // Unsafe dereferensi: bergantung pada validitas alamat self_ptr
            unsafe { Some(&*self.self_ptr) }
        }
    }

    /// Menguji apakah alamat fisik memori `self.value` saat ini masih sama dengan `self_ptr`.
    pub fn is_pointer_still_valid(&self) -> bool {
        let current_addr = &self.value as *const String;
        current_addr == self.self_ptr
    }
}

// ============================================================================
// 5. Demonstrasi Utama (pub fn run())
// ============================================================================

pub fn run() {
    println!("=== FASE 12 - TASK 1: FUTURE MENTAL MODEL ===");

    // 1. Jelaskan mengapa Future bersifat lazy
    println!("\n1. Mengapa Future Bersifat Lazy (Pull-Based Model):");
    let lazy_info = explain_future_laziness();
    println!("   - Model: {}", lazy_info.execution_model);
    println!(
        "   - Komparasi JS: {}",
        lazy_info.comparison_with_js_promise
    );
    println!("   - Pembatalan: {}", lazy_info.cancellation_benefit);
    println!("   - Alokasi: {}", lazy_info.allocation_overhead);

    let counter = Arc::new(AtomicUsize::new(0));
    let lazy_fut = LazyCalculationFuture::new(21, Arc::clone(&counter));
    println!(
        "   [Bukti] Future dibuat tapi belum di-poll: counter = {}",
        counter.load(Ordering::SeqCst)
    );
    assert_eq!(
        counter.load(Ordering::SeqCst),
        0,
        "Future tidak boleh berjalan sebelum di-poll!"
    );

    // Eksekusi dengan mini executor kita
    let computed_val = mini_block_on(lazy_fut);
    println!(
        "   [Bukti] Setelah di-poll oleh executor: hasil = {}, counter = {}",
        computed_val,
        counter.load(Ordering::SeqCst)
    );
    assert_eq!(computed_val, 42);
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    // 2. Pahami `poll` dan siklus Ready vs Pending
    println!("\n2. Pahami Trait `Future` & Method `poll()`:");
    let countdown = CountdownFuture::new(3);
    let thread_waker = Arc::new(ThreadWaker {
        thread: thread::current(),
    });
    let waker = Waker::from(thread_waker);
    let mut cx = Context::from_waker(&waker);

    // Pinning di heap secara aman menggunakan Box::pin
    let mut pinned_countdown = Box::pin(countdown);
    println!(
        "   Polling #1: {:?}",
        pinned_countdown.as_mut().poll(&mut cx)
    );
    println!(
        "   Polling #2: {:?}",
        pinned_countdown.as_mut().poll(&mut cx)
    );
    match pinned_countdown.as_mut().poll(&mut cx) {
        Poll::Ready(msg) => {
            println!("   Polling #3: Poll::Ready -> \"{}\"", msg);
            assert_eq!(pinned_countdown.total_polled(), 3);
        }
        Poll::Pending => panic!("Seharusnya sudah Ready pada polling ke-3!"),
    }

    // 3 & 4. Pahami Waker & Executor
    println!("\n3 & 4. Pahami Waker & Executor (Mini block_on):");
    println!("   Menjalankan AsyncTimerYieldFuture (delay simulasi 30ms)...");
    let timer_future = AsyncTimerYieldFuture::new(Duration::from_millis(30));
    let timer_result = mini_block_on(timer_future);
    println!("   Hasil eksekusi mini_block_on: \"{}\"", timer_result);
    assert_eq!(timer_result, "Operasi async timer selesai!");

    // 5. Pahami Mengapa Pin Ada
    println!("\n5. Pahami Secara Konseptual Mengapa `Pin` Ada:");
    let pin_info = explain_pin_concept();
    println!("   - Masalah: {}", pin_info.problem);
    println!(
        "   - Penyebab di Async: {}",
        pin_info.self_referential_cause
    );
    println!("   - Jaminan Pin: {}", pin_info.pin_guarantee);
    println!("   - Trait Unpin: {}", pin_info.unpin_trait);

    // Simulasi bahaya perpindahan memori
    let mut sim = UnsafeSelfRefSimulator::new("Rust Async Power");
    sim.init_internal_pointer();
    println!(
        "   [Sebelum Move] Pointer valid? {}",
        sim.is_pointer_still_valid()
    );
    assert!(sim.is_pointer_still_valid());
    assert_eq!(
        sim.read_via_pointer().map(|s| s.as_str()),
        Some("Rust Async Power")
    );

    // Lakukan pemindahan (move) struct ke alokasi box/variabel baru
    let moved_sim = Box::new(sim);
    println!(
        "   [Setelah Move] Pointer valid? {}",
        moved_sim.is_pointer_still_valid()
    );
    // Alamat `moved_sim.value` sekarang di heap, sedangkan `self_ptr` masih menunjuk ke stack frame lama!
    assert!(
        !moved_sim.is_pointer_still_valid(),
        "Setelah move, pointer internal menjadi invalid (dangling)!"
    );
    println!(
        "   -> Terbukti! Tanpa Pin, pemindahan struct self-referential menghasilkan pointer invalid."
    );
    println!("   -> Pin mencegah move ini secara mutlak pada compile-time!");

    println!("\n[OK] FASE 12 Task 1 (Future Mental Model) selesai dan terverifikasi!\n");
}

// ============================================================================
// 6. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_is_lazy_until_polled() {
        let counter = Arc::new(AtomicUsize::new(0));
        let fut = LazyCalculationFuture::new(10, Arc::clone(&counter));

        // Sebelum di-poll, counter harus tetap 0
        assert_eq!(counter.load(Ordering::SeqCst), 0);

        let res = mini_block_on(fut);
        assert_eq!(res, 20);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_countdown_future_manual_polling_lifecycle() {
        let fut = CountdownFuture::new(2);
        let mut pinned = Box::pin(fut);

        let thread_waker = Arc::new(ThreadWaker {
            thread: thread::current(),
        });
        let waker = Waker::from(thread_waker);
        let mut cx = Context::from_waker(&waker);

        // Poll 1: harus Pending
        assert!(matches!(pinned.as_mut().poll(&mut cx), Poll::Pending));
        assert_eq!(pinned.total_polled(), 1);

        // Poll 2: harus Ready
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => {
                assert!(val.contains("Selesai setelah 2 kali polling"));
                assert_eq!(pinned.total_polled(), 2);
            }
            Poll::Pending => panic!("Ekspektasi Poll::Ready pada poll ke-2"),
        }
    }

    #[test]
    fn test_mini_block_on_with_waker_thread() {
        let timer_fut = AsyncTimerYieldFuture::new(Duration::from_millis(15));
        let res = mini_block_on(timer_fut);
        assert_eq!(res, "Operasi async timer selesai!");
    }

    #[test]
    fn test_self_referential_pointer_invalidated_on_move() {
        let mut item = UnsafeSelfRefSimulator::new("Pin Safety Test");
        item.init_internal_pointer();
        assert!(item.is_pointer_still_valid());

        // Pindahkan item ke lokasi memori baru
        let moved_item = Box::new(item);
        // Pointer internal sekarang tidak cocok lagi dengan alamat baru
        assert!(!moved_item.is_pointer_still_valid());
    }

    #[test]
    fn test_waker_wake_by_ref() {
        let thread_waker = Arc::new(ThreadWaker {
            thread: thread::current(),
        });
        // wake_by_ref harus meng-unpark thread tanpa error/panic
        thread_waker.wake_by_ref();
    }

    #[test]
    fn test_theory_explanations_completeness() {
        let laziness = explain_future_laziness();
        assert!(laziness.execution_model.contains("Pull-based"));
        assert!(laziness.comparison_with_js_promise.contains("push-based"));

        let pin = explain_pin_concept();
        assert!(pin.problem.contains("dangling pointer"));
        assert!(pin.pin_guarantee.contains("TIDAK akan pernah dipindahkan"));
    }
}
