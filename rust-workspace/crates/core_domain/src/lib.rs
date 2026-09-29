// Crate: core_domain
// Lokasi: rust-workspace/crates/core_domain/src/lib.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Engineer,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

impl User {
    pub fn new(id: u64, username: &str, email: &str, role: UserRole) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            role,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    pub fn display_badge(&self) -> String {
        let role_str = match self.role {
            UserRole::Admin => "[ADMIN]",
            UserRole::Engineer => "[ENGINEER]",
            UserRole::Guest => "[GUEST]",
        };
        format!(
            "{:<10} #{} - {} ({})",
            role_str, self.id, self.username, self.email
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_and_roles() {
        let user = User::new(1, "alex", "alex@rust.internal", UserRole::Admin);
        assert_eq!(user.id, 1);
        assert!(user.is_admin());
        assert!(user.display_badge().contains("[ADMIN]"));
    }
}
