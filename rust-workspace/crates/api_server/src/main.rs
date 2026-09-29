// Binary Crate: api_server
// Lokasi: rust-workspace/crates/api_server/src/main.rs

use core_domain::{User, UserRole};
use database_adapter::DatabaseAdapter;

pub fn handle_register_user(
    db: &mut DatabaseAdapter,
    id: u64,
    username: &str,
    email: &str,
    role: UserRole,
) -> Result<String, String> {
    let user = User::new(id, username, email, role);
    db.insert_user(user.clone())
        .map_err(|e| format!("API Error: {e}"))?;

    serde_json::to_string_pretty(&user).map_err(|e| format!("Serialization Error: {e}"))
}

fn main() {
    println!("============================================================");
    println!("=== Multi-Crate Workspace: api_server Starting...        ===");
    println!("============================================================");

    let mut db = DatabaseAdapter::new();

    println!("1. Mendaftarkan user via cross-crate logic:");
    let res1 = handle_register_user(&mut db, 1, "alice", "alice@enterprise.com", UserRole::Admin);
    let res2 = handle_register_user(&mut db, 2, "bob", "bob@enterprise.com", UserRole::Engineer);

    println!("   Payload API Response User 1:\n{}", res1.unwrap());
    println!("   Payload API Response User 2:\n{}", res2.unwrap());

    println!("\n2. Query semua user dari database_adapter:");
    println!("   Total user di database: {}", db.count());
    for u in db.list_all() {
        println!("   - {}", u.display_badge());
    }
}
