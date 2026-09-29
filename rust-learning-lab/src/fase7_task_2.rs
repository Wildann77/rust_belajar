// Fase 7 - Task 2: Trait, Default Implementation & Trait Bounds
// Rujukan: rust_learning_guide.md (Sub-bab 7.2) & rust_execution_tasks.md (L652-L665)

use std::fmt::{self, Display, Formatter};

// ----------------------------------------------------------------------------
// 1. Definisi Trait Summary dengan Default Implementation
// ----------------------------------------------------------------------------

/// Trait `Summary` mendefinisikan kontrak abstraksi untuk menghasilkan ringkasan teks.
pub trait Summary {
    /// Method opsional dengan implementasi default untuk mengambil identitas penulis.
    fn summarize_author(&self) -> String {
        String::from("Kontributor Anonim")
    }

    /// Method dengan default implementation.
    /// Memanggil `summarize_author()` secara dinamis.
    /// Pengimplementasi bebas meng-override method ini jika menginginkan format kustom.
    fn summarize(&self) -> String {
        format!("(Baca selengkapnya dari {}...)", self.summarize_author())
    }
}

// ----------------------------------------------------------------------------
// 2. Struct 1: NewsArticle (Meng-override Default Implementation)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    /// Override kustom: Menyajikan headline, penulis, dan lokasi artikel.
    fn summarize(&self) -> String {
        format!("{}, oleh {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[BERITA] \"{}\" - {}", self.headline, self.author)
    }
}

// ----------------------------------------------------------------------------
// 3. Struct 2: Tweet (Memanfaatkan Default Implementation)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /// Hanya meng-override `summarize_author`.
    /// Method `summarize()` tetap menggunakan implementasi default bawaan trait!
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[TWEET] @{}: \"{}\"", self.username, self.content)
    }
}

// ----------------------------------------------------------------------------
// Struct Tambahan: CommunityNotice (Menggunakan Full Default Implementation)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunityNotice {
    pub title: String,
    pub notice_id: u32,
}

// Menggunakan seluruh implementasi bawaan tanpa override satu pun
impl Summary for CommunityNotice {}

// ----------------------------------------------------------------------------
// 4. Trait Bounds Functions
// ----------------------------------------------------------------------------

/// Fungsi generic dengan Trait Bound eksplisit `<T: Summary>`.
/// Membatasi tipe `T` hanya pada tipe yang memenuhi kontrak trait `Summary`.
pub fn notify<T: Summary>(item: &T) -> String {
    format!("Pemberitahuan Terkini: {}", item.summarize())
}

/// Fungsi generic dengan Multiple Trait Bounds: `<T: Summary + Display>`.
/// Menuntut tipe `T` harus mengimplementasikan trait `Summary` DAN `Display`.
pub fn notify_verbose<T: Summary + Display>(item: &T) -> String {
    format!("Notifikasi Lengkap: {}\n   -> Display: {}", item.summarize(), item)
}

/// Fungsi generic menggunakan `where` clause untuk kerapian deklarasi bounds.
pub fn notify_where<T>(item: &T) -> String
where
    T: Summary,
{
    format!("Pemberitahuan (via where clause): {}", item.summarize())
}

/// Sintaks alternatif `impl Trait` (Syntactic Sugar untuk Trait Bound sederhana).
pub fn notify_impl(item: &impl Summary) -> String {
    format!("Pemberitahuan (via impl Trait): {}", item.summarize())
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 7 - Task 2: Traits & Trait Bounds               ===");
    println!("============================================================");

    // 1. Instansiasi 2 Struct
    let article = NewsArticle {
        headline: "Rust 2024 Edition Resmi Dirilis".to_string(),
        location: "San Francisco".to_string(),
        author: "Tech Wire".to_string(),
        content: "Edisi terbaru menghadirkan peningkatan performa kompilasi dan sintaks modern.".to_string(),
    };

    let tweet = Tweet {
        username: "rustacean_id".to_string(),
        content: "Belajar trait di Rust sangat menyenangkan dan type-safe!".to_string(),
        reply: false,
        retweet: false,
    };

    let notice = CommunityNotice {
        title: "Pemeliharaan Server Lab".to_string(),
        notice_id: 101,
    };

    // 2. Evaluasi Default vs Override Implementation
    println!("\n1. Eksekusi Trait Method:");
    println!("   - NewsArticle (Overridden summarize) : {}", article.summarize());
    println!("   - Tweet (Default summarize)           : {}", tweet.summarize());
    println!("   - CommunityNotice (Full Default)     : {}", notice.summarize());

    // 3. Demonstrasi Trait Bound pada Fungsi Generic
    println!("\n2. Pemanggilan Fungsi dengan Trait Bounds:");
    println!("   a. notify<T: Summary>(&article):");
    println!("      {}", notify(&article));
    println!("   b. notify<T: Summary>(&tweet):");
    println!("      {}", notify(&tweet));

    // 4. Multiple Trait Bounds (T: Summary + Display)
    println!("\n3. Multiple Trait Bounds (<T: Summary + Display>):");
    println!("   {}", notify_verbose(&article));
    println!("   {}", notify_verbose(&tweet));

    // 5. Sintaks where clause & impl Trait
    println!("\n4. Sintaks Alternatif Trait Bounds:");
    println!("   - where clause : {}", notify_where(&article));
    println!("   - impl Trait   : {}", notify_impl(&tweet));
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_article_custom_summary() {
        let article = NewsArticle {
            headline: "Judul Berita".to_string(),
            location: "Jakarta".to_string(),
            author: "Budi".to_string(),
            content: "Konten berita singkat.".to_string(),
        };

        assert_eq!(article.summarize_author(), "Budi");
        assert_eq!(article.summarize(), "Judul Berita, oleh Budi (Jakarta)");
    }

    #[test]
    fn test_tweet_default_summary() {
        let tweet = Tweet {
            username: "johndoe".to_string(),
            content: "Halo dunia Rust!".to_string(),
            reply: false,
            retweet: true,
        };

        assert_eq!(tweet.summarize_author(), "@johndoe");
        assert_eq!(tweet.summarize(), "(Baca selengkapnya dari @johndoe...)");
    }

    #[test]
    fn test_full_default_implementation() {
        let notice = CommunityNotice {
            title: "Pengumuman".to_string(),
            notice_id: 1,
        };

        assert_eq!(notice.summarize_author(), "Kontributor Anonim");
        assert_eq!(notice.summarize(), "(Baca selengkapnya dari Kontributor Anonim...)");
    }

    #[test]
    fn test_trait_bound_notify() {
        let tweet = Tweet {
            username: "coder".to_string(),
            content: "Coding Rust".to_string(),
            reply: false,
            retweet: false,
        };

        let result = notify(&tweet);
        assert!(result.contains("Pemberitahuan Terkini"));
        assert!(result.contains("@coder"));
    }

    #[test]
    fn test_multiple_trait_bounds_notify_verbose() {
        let article = NewsArticle {
            headline: "Rilis".to_string(),
            location: "Bandung".to_string(),
            author: "Siti".to_string(),
            content: "Isi".to_string(),
        };

        let result = notify_verbose(&article);
        assert!(result.contains("Notifikasi Lengkap: Rilis, oleh Siti (Bandung)"));
        assert!(result.contains("[BERITA] \"Rilis\" - Siti"));
    }

    #[test]
    fn test_notify_where_and_impl() {
        let tweet = Tweet {
            username: "tester".to_string(),
            content: "Test".to_string(),
            reply: false,
            retweet: false,
        };

        assert!(notify_where(&tweet).contains("via where clause"));
        assert!(notify_impl(&tweet).contains("via impl Trait"));
    }
}
