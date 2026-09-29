// Fase 6 - Task 1: Robust Error Handling — Option<T>
// Rujukan: rust_learning_guide.md (Sub-bab 6.1) & rust_execution_tasks.md (L558-L567)

/// Representasi alamat fisik user yang bersifat opsional
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub city: String,
    pub postal_code: Option<String>,
}

/// Representasi profil pengguna dengan berbagai field opsional
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
    pub address: Option<Address>,
}

/// Repositori user sederhana dalam memori
pub struct UserRepository {
    users: Vec<UserProfile>,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository {
    /// Inisialisasi data sample repository
    pub fn new() -> Self {
        Self {
            users: vec![
                UserProfile {
                    id: 1,
                    username: "alice".to_string(),
                    email: Some("alice@example.com".to_string()),
                    address: Some(Address {
                        city: "Jakarta".to_string(),
                        postal_code: Some("10110".to_string()),
                    }),
                },
                UserProfile {
                    id: 2,
                    username: "bob".to_string(),
                    email: Some("bob@rustacean.org".to_string()),
                    address: Some(Address {
                        city: "Bandung".to_string(),
                        postal_code: None, // Postal code belum diisi
                    }),
                },
                UserProfile {
                    id: 3,
                    username: "charlie".to_string(),
                    email: None,   // Tidak memiliki email
                    address: None, // Tidak memiliki alamat sama sekali
                },
            ],
        }
    }

    /// 1 & 2. Mendemonstrasikan penggunaan Some dan None
    /// Mengembalikan Some(&UserProfile) jika user ditemukan, atau None jika ID tidak ada.
    pub fn find_by_id(&self, id: u64) -> Option<&UserProfile> {
        self.users.iter().find(|u| u.id == id)
    }
}

/// 3. Mendemonstrasikan exhaustive pattern matching dengan `match`
/// Compiler memaksa penanganan kedua cabang: Some(value) dan None.
pub fn format_user_greeting(user_opt: Option<&UserProfile>) -> String {
    match user_opt {
        Some(user) => format!("Halo, {}! (ID: {})", user.username, user.id),
        None => "User tidak ditemukan dalam sistem!".to_string(),
    }
}

/// 4. Mendemonstrasikan `if let` untuk percabangan ringkas (idiomatic & concise)
/// Digunakan saat kita hanya peduli pada varian `Some` dan mengabaikan `None`.
pub fn check_has_email(user: &UserProfile) -> String {
    if let Some(ref email) = user.email {
        format!("Email terdaftar: {}", email)
    } else {
        "Email belum didaftarkan".to_string()
    }
}

/// 5. Mendemonstrasikan combinator `.map()`
/// Mengubah isi `Option<T>` menjadi `Option<U>` tanpa perlu membuka bungkus secara manual.
/// Jika `None`, fungsi pemetaan tidak dipanggil dan tetap mengembalikan `None`.
pub fn get_uppercase_email(user: &UserProfile) -> Option<String> {
    user.email.as_ref().map(|e| e.to_uppercase())
}

/// 6. Mendemonstrasikan combinator `.and_then()` (monadic bind / flat_map)
/// Menghubungkan (chaining) operasi yang sama-sama menghasilkan `Option`.
/// Jika menggunakan `.map()`, hasilnya menjadi nested `Option<Option<T>>`.
/// `.and_then()` secara otomatis meratakan (flatten) hirarki opsional.
pub fn get_user_postal_code(user_opt: Option<&UserProfile>) -> Option<String> {
    user_opt
        .and_then(|u| u.address.as_ref())
        .and_then(|addr| addr.postal_code.clone())
}

/// 7. Mendemonstrasikan defensive fallback dengan `.unwrap_or()`
/// Mengambil nilai di dalam `Some(T)` atau mengembalikan default fallback jika `None`.
/// Mencegah fatal crash / panic runtime yang diakibatkan oleh penggunaan `.unwrap()`.
pub fn get_email_or_default<'a>(user: &'a UserProfile, default_email: &'a str) -> &'a str {
    user.email.as_deref().unwrap_or(default_email)
}

/// Runner demonstrasi untuk Fase 6 Task 1
pub fn run() {
    println!("=== Fase 6 - Task 1: Robust Error Handling — Option<T> ===");
    println!("Konsep Inti: Eliminasi null pointer exception via enum Option<T> (Some & None)\n");

    let repo = UserRepository::new();

    // 1 & 2: Some & None
    println!("1. Pembuatan & Pencarian via Option (Some & None):");
    let user1 = repo.find_by_id(1);
    let user99 = repo.find_by_id(99);
    println!("   - ID 1  : {:?}", user1.map(|u| &u.username));
    println!("   - ID 99 : {:?}", user99.map(|u| &u.username));

    // 3: match pattern matching
    println!("\n2. Ekstraksi Exhaustive via `match`:");
    println!("   - match ID 1  -> {}", format_user_greeting(user1));
    println!("   - match ID 99 -> {}", format_user_greeting(user99));

    // 4: if let sugar
    println!("\n3. Ekstraksi Ringkas via `if let`:");
    if let Some(u) = user1 {
        println!("   - Alice   -> {}", check_has_email(u));
    }
    if let Some(u) = repo.find_by_id(3) {
        println!("   - Charlie -> {}", check_has_email(u));
    }

    // 5: map combinator
    println!("\n4. Transformasi Nilai via `.map()`:");
    if let Some(u) = user1 {
        let upper_email = get_uppercase_email(u);
        println!("   - Alice upper email -> {:?}", upper_email);
    }
    if let Some(u) = repo.find_by_id(3) {
        let upper_email = get_uppercase_email(u);
        println!("   - Charlie upper email -> {:?}", upper_email);
    }

    // 6: and_then combinator (chaining & flattening)
    println!("\n5. Rantai Pengecekan Bersarang via `.and_then()` (Flat Map):");
    println!("   - ID 1 (Lengkap)     postal_code: {:?}", get_user_postal_code(repo.find_by_id(1)));
    println!("   - ID 2 (Tanpa Pos)   postal_code: {:?}", get_user_postal_code(repo.find_by_id(2)));
    println!("   - ID 3 (Tanpa Alamat)postal_code: {:?}", get_user_postal_code(repo.find_by_id(3)));
    println!("   - ID 99 (None User)  postal_code: {:?}", get_user_postal_code(repo.find_by_id(99)));

    // 7: unwrap_or fallback aman
    println!("\n6. Fallback Aman via `.unwrap_or()` (Anti-Crash):");
    let fallback = "guest@system.local";
    if let Some(u) = repo.find_by_id(2) {
        println!("   - Bob email     -> {}", get_email_or_default(u, fallback));
    }
    if let Some(u) = repo.find_by_id(3) {
        println!("   - Charlie email -> {}", get_email_or_default(u, fallback));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_some_and_none() {
        let repo = UserRepository::new();
        assert!(repo.find_by_id(1).is_some());
        assert!(repo.find_by_id(99).is_none());
    }

    #[test]
    fn test_pattern_matching_with_match() {
        let repo = UserRepository::new();
        let u1 = repo.find_by_id(1);
        let u99 = repo.find_by_id(99);

        assert_eq!(format_user_greeting(u1), "Halo, alice! (ID: 1)");
        assert_eq!(format_user_greeting(u99), "User tidak ditemukan dalam sistem!");
    }

    #[test]
    fn test_if_let_construct() {
        let repo = UserRepository::new();
        let alice = repo.find_by_id(1).unwrap();
        let charlie = repo.find_by_id(3).unwrap();

        assert_eq!(check_has_email(alice), "Email terdaftar: alice@example.com");
        assert_eq!(check_has_email(charlie), "Email belum didaftarkan");
    }

    #[test]
    fn test_map_combinator() {
        let repo = UserRepository::new();
        let alice = repo.find_by_id(1).unwrap();
        let charlie = repo.find_by_id(3).unwrap();

        assert_eq!(get_uppercase_email(alice), Some("ALICE@EXAMPLE.COM".to_string()));
        assert_eq!(get_uppercase_email(charlie), None);
    }

    #[test]
    fn test_and_then_chaining() {
        let repo = UserRepository::new();

        // Kasus 1: Semua level Some
        assert_eq!(get_user_postal_code(repo.find_by_id(1)), Some("10110".to_string()));

        // Kasus 2: User ada, Alamat ada, postal_code None
        assert_eq!(get_user_postal_code(repo.find_by_id(2)), None);

        // Kasus 3: User ada, Alamat None
        assert_eq!(get_user_postal_code(repo.find_by_id(3)), None);

        // Kasus 4: User None
        assert_eq!(get_user_postal_code(repo.find_by_id(99)), None);
    }

    #[test]
    fn test_unwrap_or_safe_fallback() {
        let repo = UserRepository::new();
        let alice = repo.find_by_id(1).unwrap();
        let charlie = repo.find_by_id(3).unwrap();

        assert_eq!(get_email_or_default(alice, "fallback@mail.com"), "alice@example.com");
        assert_eq!(get_email_or_default(charlie, "fallback@mail.com"), "fallback@mail.com");
    }
}
