// Fase 7 - Task 3: Dispatch — Static Dispatch vs Dynamic Dispatch
// Rujukan: rust_learning_guide.md (Sub-bab 7.3) & rust_execution_tasks.md (L666-L677)

use crate::fase7_task_2::{NewsArticle, Summary, Tweet};
use std::mem;

// ----------------------------------------------------------------------------
// 1. Tipe Tambahan Pengimplementasi Trait Summary
// ----------------------------------------------------------------------------

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
        format!(
            "Podcast '{}' Ep. #{} ({})",
            self.show_name,
            self.episode_number,
            self.summarize_author()
        )
    }
}

// ----------------------------------------------------------------------------
// 2. Versi 1: Static Dispatch (Monomorphization via Generics `T: Summary`)
// ----------------------------------------------------------------------------

/// Memproses item yang mengimplementasikan trait `Summary` menggunakan Static Dispatch.
///
/// Karakteristik:
/// - Compiler membuat duplikasi fungsi biner spesifik untuk tiap tipe konkret saat kompilasi.
/// - Tidak ada overhead runtime (Zero-Cost Abstraction).
/// - Compiler dapat meng-inline fungsi untuk kecepatan maksimal.
/// - Tipe harus homogen / diketahui secara pasti saat compile time.
pub fn process_static<T: Summary>(item: &T) -> String {
    format!("[Static Dispatch] {}", item.summarize())
}

// ----------------------------------------------------------------------------
// 3. Versi 2: Dynamic Dispatch via Trait Object (`&dyn Summary`)
// ----------------------------------------------------------------------------

/// Memproses item menggunakan Dynamic Dispatch lewat referensi Trait Object (`&dyn Summary`).
///
/// Karakteristik:
/// - Menggunakan Fat Pointer (16 bytes di arsitektur 64-bit):
///   1. Pointer ke data konkret di memori (8 bytes).
///   2. Pointer ke Virtual Method Table / vtable (8 bytes).
/// - Resolusi method terjadi saat runtime melalui dereferensi pointer vtable.
/// - Memungkinkan polimorfisme sejati untuk data heterogen.
pub fn process_dynamic(item: &dyn Summary) -> String {
    format!("[Dynamic Dispatch] {}", item.summarize())
}

// ----------------------------------------------------------------------------
// 4. Dynamic Dispatch dengan Heap Allocation: `Box<dyn Summary>`
// ----------------------------------------------------------------------------

/// Memproses satu item boxed trait object.
pub fn process_boxed(item: &Box<dyn Summary>) -> String {
    format!("[Boxed dyn Summary] {}", item.summarize())
}

/// Memproses sekumpulan item heterogen di dalam koleksi `Vec<Box<dyn Summary>>`.
///
/// Karena tipe objek dienkapsulasi dalam pointer `Box<dyn Summary>`, sebuah `Vec`
/// dapat menampung objek-objek dengan ukuran memori berbeda (NewsArticle, Tweet, PodcastEpisode)
/// dalam satu koleksi seragam.
pub fn process_heterogeneous_collection(items: &[Box<dyn Summary>]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 7 - Task 3: Static vs Dynamic Dispatch          ===");
    println!("============================================================");

    let article = NewsArticle {
        headline: "Deep Dive Rust Dispatch".to_string(),
        location: "Jakarta".to_string(),
        author: "Senior Rustacean".to_string(),
        content: "Static dispatch monomorphizes; Dynamic dispatch uses vtables.".to_string(),
    };

    let tweet = Tweet {
        username: "ferris_the_crab".to_string(),
        content: "Static dispatch is fast, but dyn Trait gives flexibility!".to_string(),
        reply: false,
        retweet: true,
    };

    let podcast = PodcastEpisode {
        show_name: "Rust Nation Podcast".to_string(),
        host: "Budi Santoso".to_string(),
        episode_number: 42,
    };

    // 1. Static Dispatch
    println!("\n1. Perbandingan Eksekusi:");
    println!("   a. {}", process_static(&article));
    println!("   b. {}", process_static(&tweet));
    println!("   c. {}", process_static(&podcast));

    // 2. Dynamic Dispatch (&dyn Summary)
    println!("\n2. Dynamic Dispatch via &dyn Summary:");
    println!("   a. {}", process_dynamic(&article));
    println!("   b. {}", process_dynamic(&tweet));
    println!("   c. {}", process_dynamic(&podcast));

    // 3. Dynamic Dispatch Heterogeneous Collection (Box<dyn Summary>)
    println!("\n3. Koleksi Heterogen via Vec<Box<dyn Summary>>:");
    let feed: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet), Box::new(podcast)];

    let summaries = process_heterogeneous_collection(&feed);
    for (idx, summary) in summaries.iter().enumerate() {
        println!("   [{idx}] {summary}");
    }

    // 4. Bedah Memori: Thin Pointer vs Fat Pointer
    println!("\n4. Analisis Memori Fat Pointer vs Thin Pointer:");
    println!(
        "   - Ukuran referensi konkret &NewsArticle (Thin Pointer) : {} bytes",
        mem::size_of::<&NewsArticle>()
    );
    println!(
        "   - Ukuran referensi trait object &dyn Summary (Fat Pointer): {} bytes",
        mem::size_of::<&dyn Summary>()
    );
    println!(
        "   - Ukuran Box<NewsArticle> (Heap Thin Pointer)          : {} bytes",
        mem::size_of::<Box<NewsArticle>>()
    );
    println!(
        "   - Ukuran Box<dyn Summary> (Heap Fat Pointer)           : {} bytes",
        mem::size_of::<Box<dyn Summary>>()
    );
    println!(
        "   -> Fat Pointer berukuran 2x pointer biasa: [Pointer Data (8B)] + [Pointer Vtable (8B)]"
    );
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_static() {
        let tweet = Tweet {
            username: "tester".to_string(),
            content: "Unit test static dispatch".to_string(),
            reply: false,
            retweet: false,
        };
        let result = process_static(&tweet);
        assert!(result.contains("[Static Dispatch]"));
        assert!(result.contains("@tester"));
    }

    #[test]
    fn test_process_dynamic() {
        let article = NewsArticle {
            headline: "Kompilasi Sukses".to_string(),
            location: "Bandung".to_string(),
            author: "Admin".to_string(),
            content: "Isi".to_string(),
        };
        let result = process_dynamic(&article);
        assert!(result.contains("[Dynamic Dispatch]"));
        assert!(result.contains("Kompilasi Sukses"));
    }

    #[test]
    fn test_process_boxed_and_heterogeneous_collection() {
        let feed: Vec<Box<dyn Summary>> = vec![
            Box::new(NewsArticle {
                headline: "A".to_string(),
                location: "B".to_string(),
                author: "C".to_string(),
                content: "D".to_string(),
            }),
            Box::new(Tweet {
                username: "user1".to_string(),
                content: "Halo".to_string(),
                reply: false,
                retweet: false,
            }),
            Box::new(PodcastEpisode {
                show_name: "AudioLab".to_string(),
                host: "Host1".to_string(),
                episode_number: 1,
            }),
        ];

        let summaries = process_heterogeneous_collection(&feed);
        assert_eq!(summaries.len(), 3);
        assert!(summaries[0].contains("A, oleh C (B)"));
        assert!(summaries[1].contains("@user1"));
        assert!(summaries[2].contains("AudioLab"));

        let boxed_single = process_boxed(&feed[0]);
        assert!(boxed_single.contains("[Boxed dyn Summary]"));
    }

    #[test]
    fn test_pointer_size_characteristics() {
        // Pada arsitektur 64-bit, thin pointer = 8 bytes, fat pointer = 16 bytes
        let thin_size = mem::size_of::<&NewsArticle>();
        let fat_size = mem::size_of::<&dyn Summary>();
        assert_eq!(fat_size, thin_size * 2);

        let box_thin = mem::size_of::<Box<NewsArticle>>();
        let box_fat = mem::size_of::<Box<dyn Summary>>();
        assert_eq!(box_fat, box_thin * 2);
    }
}
