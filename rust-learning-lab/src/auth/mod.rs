// Modul Auth: Entry point module auth
// Lokasi: rust-learning-lab/src/auth/mod.rs

pub mod token;

// Re-export Claims agar pemanggil modul auth bisa langsung `use crate::auth::Claims;`
pub use token::Claims;

/// Fungsi autentikasi sederhana berbasis token mock.
/// Mengembalikan Claims jika valid.
pub fn authenticate(token_str: &str) -> Result<Claims, &'static str> {
    if token_str.is_empty() {
        return Err("Token kosong tidak valid");
    }

    // Format token mock: "username:role"
    let parts: Vec<&str> = token_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Format token tidak valid (harus 'username:role')");
    }

    let claims = Claims::new(parts[0], parts[1]);

    // Memanggil method `pub(super)` dari submodul token
    // Ini legal karena `mod.rs` adalah super (parent) dari `token.rs`
    let _session_key = claims.internal_session_key();

    Ok(claims)
}
