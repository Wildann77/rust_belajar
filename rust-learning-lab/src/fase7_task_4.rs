// Fase 7 - Task 4: Associated Types in Traits
// Rujukan: rust_learning_guide.md (Sub-bab 7.4) & rust_execution_tasks.md (L679-L693)

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

// ----------------------------------------------------------------------------
// 1. Error Enum & Model Data
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    NotFound(u64),
    ConnectionFailed(String),
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::NotFound(id) => write!(f, "Entity dengan ID #{id} tidak ditemukan"),
            RepoError::ConnectionFailed(msg) => write!(f, "Koneksi database gagal: {msg}"),
        }
    }
}

impl std::error::Error for RepoError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "User [ID: {}, Username: '{}', Email: '{}']", self.id, self.username, self.email)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Product [ID: {}, Name: '{}', Price: Rp{:.2}]", self.id, self.name, self.price)
    }
}

// ----------------------------------------------------------------------------
// 2. Definisi Trait dengan Associated Types (type Item; type Error;)
// ----------------------------------------------------------------------------

/// Trait `Repository` menggunakan Associated Types untuk mengikat tipe entitas (`Item`)
/// dan tipe galat (`Error`) secara tepat 1-to-1 pada struct pengimplementasi.
///
/// Keuntungan dibanding Generic Parameters (`trait Repository<Item, Error>`):
/// - Menghindari polusi signature fungsi: tidak perlu menulis `<R: Repository<User, RepoError>>`.
/// - Menjamin 1 struct hanya mengelola 1 tipe entitas pasti (tidak ambigu).
pub trait Repository {
    type Item;
    type Error;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error>;
}

// ----------------------------------------------------------------------------
// 3. Implementasi Konkret 1: InMemoryUserRepository
// ----------------------------------------------------------------------------

pub struct InMemoryUserRepository {
    storage: HashMap<u64, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn insert(&mut self, user: User) {
        self.storage.insert(user.id, user);
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for InMemoryUserRepository {
    type Item = User;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.storage.get(&id).cloned())
    }
}

// ----------------------------------------------------------------------------
// 4. Implementasi Konkret 2: InMemoryProductRepository
// ----------------------------------------------------------------------------

pub struct InMemoryProductRepository {
    storage: HashMap<u64, Product>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn insert(&mut self, product: Product) {
        self.storage.insert(product.id, product);
    }
}

impl Default for InMemoryProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for InMemoryProductRepository {
    type Item = Product;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.storage.get(&id).cloned())
    }
}

// ----------------------------------------------------------------------------
// 5. Implementasi Konkret 3: MockFaultyRepository (Simulasi Kasus Galat / Error)
// ----------------------------------------------------------------------------

pub struct MockFaultyRepository;

impl Repository for MockFaultyRepository {
    type Item = User;
    type Error = RepoError;

    fn get(&self, id: u64) -> Result<Option<Self::Item>, Self::Error> {
        if id == 0 {
            Err(RepoError::NotFound(0))
        } else {
            Err(RepoError::ConnectionFailed("Timeout server database mock".to_string()))
        }
    }
}

// ----------------------------------------------------------------------------
// 6. Fungsi Generic Menggunakan Proyeksi Associated Types (`R::Item`, `R::Error`)
// ----------------------------------------------------------------------------

/// Mengambil entitas dari sembarang repository dan memformatnya menjadi String.
/// 
/// Menggunakan proyeksi `R::Item` dan `R::Error`:
/// - `R::Item: Display` memastikan item repository bisa dicetak via `{}`.
pub fn fetch_and_display<R>(repo: &R, id: u64) -> Result<String, R::Error>
where
    R: Repository,
    R::Item: Display,
{
    match repo.get(id)? {
        Some(item) => Ok(format!("Ditemukan: {item}")),
        None => Ok(format!("Entity ID #{id} tidak ditemukan di repository")),
    }
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Fase 7 - Task 4: Associated Types in Traits          ===");
    println!("============================================================");

    // 1. Setup InMemoryUserRepository
    println!("\n1. InMemoryUserRepository (type Item = User):");
    let mut user_repo = InMemoryUserRepository::new();
    user_repo.insert(User {
        id: 1,
        username: "alice_crypto".to_string(),
        email: "alice@rust.dev".to_string(),
    });
    user_repo.insert(User {
        id: 2,
        username: "bob_engineer".to_string(),
        email: "bob@systems.id".to_string(),
    });

    if let Ok(Some(user)) = user_repo.get(1) {
        println!("   - Direct get(1) : {user}");
    }
    if let Ok(None) = user_repo.get(99) {
        println!("   - Direct get(99): None (Aman, data tidak ada)");
    }

    // 2. Setup InMemoryProductRepository
    println!("\n2. InMemoryProductRepository (type Item = Product):");
    let mut product_repo = InMemoryProductRepository::new();
    product_repo.insert(Product {
        id: 101,
        name: "Mechanical Keyboard 75%".to_string(),
        price: 1250000.0,
    });
    product_repo.insert(Product {
        id: 102,
        name: "Wireless Ergonomic Mouse".to_string(),
        price: 650000.0,
    });

    if let Ok(Some(prod)) = product_repo.get(101) {
        println!("   - Direct get(101): {prod}");
    }

    // 3. Proyeksi Generic via fetch_and_display<R>
    println!("\n3. Generic Function Menggunakan Proyeksi R::Item & R::Error:");
    match fetch_and_display(&user_repo, 2) {
        Ok(msg) => println!("   - User Repo Query   -> {msg}"),
        Err(err) => eprintln!("   - Gagal: {err}"),
    }
    match fetch_and_display(&product_repo, 102) {
        Ok(msg) => println!("   - Product Repo Query-> {msg}"),
        Err(err) => eprintln!("   - Gagal: {err}"),
    }
    match fetch_and_display(&product_repo, 888) {
        Ok(msg) => println!("   - Missing Item Query-> {msg}"),
        Err(err) => eprintln!("   - Gagal: {err}"),
    }

    // 4. Simulasi Error via MockFaultyRepository
    println!("\n4. Simulasi Error via MockFaultyRepository:");
    let faulty_repo = MockFaultyRepository;
    match fetch_and_display(&faulty_repo, 0) {
        Ok(msg) => println!("   - ID 0 -> {msg}"),
        Err(err) => println!("   - ID 0 -> Error Ditangkap: {err}"),
    }
    match fetch_and_display(&faulty_repo, 100) {
        Ok(msg) => println!("   - ID 100 -> {msg}"),
        Err(err) => println!("   - ID 100 -> Error Ditangkap: {err}"),
    }
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_repository_get() {
        let mut repo = InMemoryUserRepository::new();
        repo.insert(User {
            id: 1,
            username: "tester".to_string(),
            email: "test@example.com".to_string(),
        });

        let found = repo.get(1).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "tester");

        let missing = repo.get(99).unwrap();
        assert!(missing.is_none());
    }

    #[test]
    fn test_product_repository_get() {
        let mut repo = InMemoryProductRepository::new();
        repo.insert(Product {
            id: 10,
            name: "Monitor".to_string(),
            price: 2500000.0,
        });

        let found = repo.get(10).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Monitor");
    }

    #[test]
    fn test_fetch_and_display_helper() {
        let mut repo = InMemoryUserRepository::new();
        repo.insert(User {
            id: 5,
            username: "clippy".to_string(),
            email: "clippy@rust.org".to_string(),
        });

        let res_found = fetch_and_display(&repo, 5).unwrap();
        assert!(res_found.contains("Ditemukan: User [ID: 5, Username: 'clippy'"));

        let res_missing = fetch_and_display(&repo, 999).unwrap();
        assert!(res_missing.contains("Entity ID #999 tidak ditemukan"));
    }

    #[test]
    fn test_mock_faulty_repository_errors() {
        let faulty = MockFaultyRepository;
        let err_not_found = fetch_and_display(&faulty, 0).unwrap_err();
        assert_eq!(err_not_found, RepoError::NotFound(0));

        let err_conn = fetch_and_display(&faulty, 42).unwrap_err();
        assert!(matches!(err_conn, RepoError::ConnectionFailed(_)));
    }
}

