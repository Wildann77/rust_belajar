// Crate: database_adapter
// Lokasi: rust-workspace/crates/database_adapter/src/lib.rs

use core_domain::User;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DatabaseAdapter {
    storage: HashMap<u64, User>,
}

impl DatabaseAdapter {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert_user(&mut self, user: User) -> Result<(), &'static str> {
        if self.storage.contains_key(&user.id) {
            return Err("User dengan ID tersebut sudah terdaftar di database");
        }
        self.storage.insert(user.id, user);
        Ok(())
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn list_all(&self) -> Vec<&User> {
        self.storage.values().collect()
    }

    pub fn count(&self) -> usize {
        self.storage.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_domain::UserRole;

    #[test]
    fn test_adapter_storage_and_queries() {
        let mut db = DatabaseAdapter::new();
        let user = User::new(10, "charlie", "charlie@rust.internal", UserRole::Engineer);

        assert_eq!(db.count(), 0);
        assert!(db.insert_user(user.clone()).is_ok());
        assert_eq!(db.count(), 1);

        // Duplikat ID harus ditolak
        assert!(db.insert_user(user).is_err());

        let retrieved = db.get_user(10).expect("User harus ditemukan");
        assert_eq!(retrieved.username, "charlie");
    }
}
