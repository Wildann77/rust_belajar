// Mini Project Fase 7: Repository Abstraction Pattern
// Trait Abstraction, Associated Types, Static vs Dynamic Dispatch, & Newtype Pattern
// Rujukan: rust_learning_guide.md (Sub-bab 7.6) & rust_execution_tasks.md (L707-L725)

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

// ----------------------------------------------------------------------------
// 1. Domain Model & Newtype Pattern
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(pub u64);

impl Display for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ACC-{:05}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: AccountId,
    pub holder: String,
    pub balance: f64,
    pub is_active: bool,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = if self.is_active { "AKTIF" } else { "NONAKTIF" };
        write!(
            f,
            "[{}] {} | Saldo: Rp{:.2} | Status: {}",
            self.id, self.holder, self.balance, status
        )
    }
}

// ----------------------------------------------------------------------------
// 2. Custom Error
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound(String),
    Duplicate(String),
    StorageFailure(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "[Error 404] Entitas tidak ditemukan: {msg}"),
            RepositoryError::Duplicate(msg) => write!(f, "[Error 409] Duplikasi entitas: {msg}"),
            RepositoryError::StorageFailure(msg) => write!(f, "[Error 500] Kegagalan storage: {msg}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

// ----------------------------------------------------------------------------
// 3. Abstraksi Inti: Repository Trait dengan Associated Types
// ----------------------------------------------------------------------------

/// Trait `Repository` mengabstraksikan operasi persistence data.
/// 
/// Menggunakan Associated Types (`Item`, `Id`, `Error`) untuk mengunci relasi
/// 1-ke-1 antara implementasi storage dengan model data dan tipe error-nya.
pub trait Repository {
    type Item;
    type Id;
    type Error: std::error::Error;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

// ----------------------------------------------------------------------------
// 4. Implementasi 1: InMemoryRepository
// ----------------------------------------------------------------------------

pub struct InMemoryAccountRepository {
    data: HashMap<AccountId, Account>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn find_or_err(&self, id: &AccountId) -> Result<Account, RepositoryError> {
        self.find_by_id(id)?
            .ok_or_else(|| RepositoryError::NotFound(format!("Akun {id} tidak ditemukan")))
    }
}

impl Default for InMemoryAccountRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for InMemoryAccountRepository {
    type Item = Account;
    type Id = AccountId;
    type Error = RepositoryError;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        if self.data.contains_key(&item.id) {
            return Err(RepositoryError::Duplicate(format!("Akun dengan ID {} sudah ada", item.id)));
        }
        self.data.insert(item.id, item);
        Ok(())
    }

    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.data.get(id).cloned())
    }


    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.data.remove(id).is_some())
    }

    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let mut list: Vec<Account> = self.data.values().cloned().collect();
        list.sort_by_key(|a| a.id);
        Ok(list)
    }
}

// ----------------------------------------------------------------------------
// 5. Implementasi 2: MockRepository untuk Pengujian (Testing Mock)
// ----------------------------------------------------------------------------

pub struct MockAccountRepository {
    pub accounts: HashMap<AccountId, Account>,
    pub should_fail_on_save: bool,
    pub failure_message: String,
}

impl MockAccountRepository {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            should_fail_on_save: false,
            failure_message: String::from("Simulasi I/O Disk Error pada Mock"),
        }
    }

    pub fn with_simulated_failure(failure_message: &str) -> Self {
        Self {
            accounts: HashMap::new(),
            should_fail_on_save: true,
            failure_message: failure_message.to_string(),
        }
    }
}

impl Default for MockAccountRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for MockAccountRepository {
    type Item = Account;
    type Id = AccountId;
    type Error = RepositoryError;

    fn save(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        if self.should_fail_on_save {
            return Err(RepositoryError::StorageFailure(self.failure_message.clone()));
        }
        self.accounts.insert(item.id, item);
        Ok(())
    }

    fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.accounts.get(id).cloned())
    }

    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.accounts.remove(id).is_some())
    }

    fn list_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        Ok(self.accounts.values().cloned().collect())
    }
}

// ----------------------------------------------------------------------------
// 6. Service Consumer: Static Dispatch vs Dynamic Dispatch
// ----------------------------------------------------------------------------

/// Static Dispatch via Generics (`<R: Repository>`):
/// Monomorphized saat compile-time, zero runtime cost, inlineable.
pub fn audit_repository_static<R>(repo: &R) -> Result<String, R::Error>
where
    R: Repository<Item = Account, Id = AccountId>,
{
    let accounts = repo.list_all()?;
    let total_balance: f64 = accounts.iter().map(|a| a.balance).sum();
    Ok(format!(
        "[Audit Static] Total Akun: {} | Total Aset: Rp{:.2}",
        accounts.len(),
        total_balance
    ))
}

/// Dynamic Dispatch via Trait Object (`&dyn Repository`):
/// Fleksibel via vtable fat pointer.
pub fn audit_repository_dynamic(
    repo: &dyn Repository<Item = Account, Id = AccountId, Error = RepositoryError>,
) -> Result<String, RepositoryError> {
    let accounts = repo.list_all()?;
    let active_count = accounts.iter().filter(|a| a.is_active).count();
    Ok(format!(
        "[Audit Dynamic] Akun Aktif: {} dari total {}",
        active_count,
        accounts.len()
    ))
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi Modul
// ----------------------------------------------------------------------------

pub fn run() {
    println!("============================================================");
    println!("=== Mini Project Fase 7: Repository Abstraction Pattern  ===");
    println!("============================================================");

    // 1. Demonstrasi InMemoryAccountRepository
    println!("\n1. Mengoperasikan InMemoryAccountRepository:");
    let mut in_memory_repo = InMemoryAccountRepository::new();
    
    let acc1 = Account {
        id: AccountId(101),
        holder: "Ahmad Dahlan".to_string(),
        balance: 15_000_000.0,
        is_active: true,
    };
    let acc2 = Account {
        id: AccountId(102),
        holder: "Siti Rahma".to_string(),
        balance: 27_500_000.0,
        is_active: true,
    };
    let acc3 = Account {
        id: AccountId(103),
        holder: "Budi Santoso".to_string(),
        balance: 500_000.0,
        is_active: false,
    };

    let _ = in_memory_repo.save(acc1);
    let _ = in_memory_repo.save(acc2);
    let _ = in_memory_repo.save(acc3);
    println!("   [+] Berhasil menyimpan 3 akun ke in-memory storage.");

    // Query Data via find_by_id dan find_or_err
    if let Ok(Some(found)) = in_memory_repo.find_by_id(&AccountId(102)) {
        println!("   [✓] Query Akun #102 (find_by_id): {found}");
    }
    match in_memory_repo.find_or_err(&AccountId(999)) {
        Ok(found) => println!("   - Ditemukan: {found}"),
        Err(e) => println!("   [✓] Query Akun #999 (find_or_err): {e}"),
    }

    // Simulasi Error Duplicate pada InMemory
    let duplicate_acc = Account {
        id: AccountId(101),
        holder: "Ahmad Dahlan Duplikat".to_string(),
        balance: 10_000.0,
        is_active: true,
    };
    match in_memory_repo.save(duplicate_acc) {
        Ok(_) => println!("   - Tersimpan"),
        Err(e) => println!("   [✓] Deteksi Duplikasi ID: {e}"),
    }

    // 2. Evaluasi Static Dispatch vs Dynamic Dispatch
    println!("\n2. Evaluasi Dispatch pada Service Audit:");
    match audit_repository_static(&in_memory_repo) {
        Ok(report) => println!("   - {report}"),
        Err(e) => eprintln!("   - Gagal: {e}"),
    }
    match audit_repository_dynamic(&in_memory_repo) {
        Ok(report) => println!("   - {report}"),
        Err(e) => eprintln!("   - Gagal: {e}"),
    }

    // 3. Demonstrasi MockRepository (Testing Isolation & Error Simulation)
    println!("\n3. Demonstrasi MockAccountRepository (Simulasi Failure):");
    let mut mock_faulty = MockAccountRepository::with_simulated_failure("Koneksi Database Timeout");
    let dummy_acc = Account {
        id: AccountId(999),
        holder: "Ghost Account".to_string(),
        balance: 0.0,
        is_active: false,
    };

    match mock_faulty.save(dummy_acc) {
        Ok(_) => println!("   [!] Tak terduga: Simpan berhasil"),
        Err(err) => println!("   [✓] Sukses Menangkap Simulasi Error: {err}"),
    }

    // 4. Operasi Delete & List
    println!("\n4. Operasi Modifikasi (Delete & List All):");
    let deleted = in_memory_repo.delete(&AccountId(103)).unwrap_or(false);
    println!("   - Hapus Akun #103: Status = {deleted}");

    let remaining = in_memory_repo.list_all().unwrap_or_default();
    println!("   - Sisa Akun di InMemory Storage:");
    for acc in &remaining {
        println!("     * {acc}");
    }

    // 5. Evaluasi Kriteria Lulus Fase 7
    println!("\n5. Evaluasi Kriteria Lulus Fase 7:");
    println!("   [x] Generic vs Trait: Generics adalah parameter tipe abstrak; Trait adalah kontrak antarmuka.");
    println!("   [x] Static vs Dynamic: Static = Monomorphized (Zero-Cost); Dynamic = vtable fat pointer.");
    println!("   [x] Associated Types: Repository::Item, Id, & Error terikat 1-to-1 pada struct.");
    println!("   [x] Orphan Rule & Newtype: AccountId tuple struct memberikan type safety dan enkapsulasi.");
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_repository_crud() {
        let mut repo = InMemoryAccountRepository::new();
        let acc = Account {
            id: AccountId(1),
            holder: "Alice".to_string(),
            balance: 1000.0,
            is_active: true,
        };

        // Save
        assert!(repo.save(acc.clone()).is_ok());

        // Duplicate error check
        let dup_err = repo.save(acc.clone());
        assert!(matches!(dup_err, Err(RepositoryError::Duplicate(_))));

        // Find
        let found = repo.find_by_id(&AccountId(1)).unwrap();
        assert_eq!(found, Some(acc.clone()));

        // Find or err (Success & Not Found)
        assert_eq!(repo.find_or_err(&AccountId(1)).unwrap(), acc);
        let not_found = repo.find_or_err(&AccountId(99));
        assert!(matches!(not_found, Err(RepositoryError::NotFound(_))));

        // List
        let all = repo.list_all().unwrap();
        assert_eq!(all.len(), 1);

        // Delete
        assert_eq!(repo.delete(&AccountId(1)).unwrap(), true);
        assert_eq!(repo.find_by_id(&AccountId(1)).unwrap(), None);
    }

    #[test]
    fn test_mock_repository_failure_simulation() {
        let mut mock = MockAccountRepository::with_simulated_failure("Disk Full");
        let acc = Account {
            id: AccountId(2),
            holder: "Bob".to_string(),
            balance: 500.0,
            is_active: false,
        };

        let result = mock.save(acc);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RepositoryError::StorageFailure("Disk Full".to_string())
        );
    }

    #[test]
    fn test_static_and_dynamic_dispatch() {
        let mut repo = InMemoryAccountRepository::new();
        let _ = repo.save(Account {
            id: AccountId(10),
            holder: "User1".to_string(),
            balance: 2000.0,
            is_active: true,
        });

        let static_res = audit_repository_static(&repo).unwrap();
        assert!(static_res.contains("Total Akun: 1"));
        assert!(static_res.contains("Rp2000.00"));

        let dyn_res = audit_repository_dynamic(&repo).unwrap();
        assert!(dyn_res.contains("Akun Aktif: 1 dari total 1"));
    }
}
