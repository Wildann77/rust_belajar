// Modul Token: Autentikasi dan Claims
// Lokasi: rust-learning-lab/src/auth/token.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,            // Public: Subjek/Username
    pub role: String,           // Public: Role akun
    pub(crate) session_id: u64, // pub(crate): Hanya terlihat di crate ini
    secret_salt: String,        // Private: Hanya terlihat di file token.rs ini
}

impl Claims {
    pub fn new(sub: &str, role: &str) -> Self {
        let fake_session = (sub.len() as u64 * 1007) + 10001;
        Self {
            sub: sub.to_string(),
            role: role.to_string(),
            session_id: fake_session,
            secret_salt: Self::compute_salt(sub),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role.eq_ignore_ascii_case("admin")
    }

    pub fn summary(&self) -> String {
        format!("User: '{}' [Role: {}]", self.sub, self.role)
    }

    // pub(super): Hanya bisa diakses oleh parent module (yaitu `src/auth/mod.rs`)
    pub(super) fn internal_session_key(&self) -> String {
        format!("AUTH_SUPER_{}_{}", self.session_id, self.sub)
    }

    // pub(crate): Bisa diakses oleh modul lain di crate ini (misal services)
    pub(crate) fn is_session_active(&self) -> bool {
        self.session_id > 0
    }

    // Private helper: Hanya terlihat di dalam token.rs
    fn compute_salt(sub: &str) -> String {
        format!("salt_{}_hash", sub.to_lowercase())
    }
}
