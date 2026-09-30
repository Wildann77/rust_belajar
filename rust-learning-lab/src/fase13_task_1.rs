// Fase 13 - Task 1: Testing & Quality Assurance Suite
// Rujukan: rust_learning_guide.md (FASE 13) & rust_execution_tasks.md (L1116-L1127)
//
// Cakupan Checklist:
// - [x] Buat unit test.
// - [x] Buat integration test di tests/.
// - [x] Buat documentation test (doc-test).
// - [x] Buat async test dengan #[tokio::test].
// - [x] assert_eq!
// - [x] assert_ne!
// - [x] assert!
// - [x] Test success case.
// - [x] Test error case.
// - [x] Test edge case.

use std::time::Duration;
use tokio::time::sleep;

/// Epsilon untuk komparasi floating point (f64) demi menghindari masalah presisi IEEE 754.
pub const FLOAT_EPSILON: f64 = 1e-7;

// ============================================================================
// 1. Tipe Data Error & Domain Account
// ============================================================================

/// Representasi varian kegagalan operasi perbankan/keuangan.
#[derive(Debug, Clone, PartialEq)]
pub enum AccountError {
    InvalidOwnerName(String),
    NegativeOrZeroAmount(f64),
    InsufficientFunds { available: f64, required: f64 },
    AccountInactive(u64),
    DivisionByZero,
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountError::InvalidOwnerName(reason) => {
                write!(f, "Nama pemilik tidak valid: {}", reason)
            }
            AccountError::NegativeOrZeroAmount(val) => {
                write!(f, "Nominal harus > 0, ditemukan: {:.2}", val)
            }
            AccountError::InsufficientFunds {
                available,
                required,
            } => {
                write!(
                    f,
                    "Saldo tidak cukup: tersedia {:.2}, diminta {:.2}",
                    available, required
                )
            }
            AccountError::AccountInactive(id) => {
                write!(f, "Akun #{} sedang nonaktif atau dibekukan", id)
            }
            AccountError::DivisionByZero => write!(f, "Pembagian dengan nilai nol dilarang"),
        }
    }
}

impl std::error::Error for AccountError {}

/// Entitas rekening finansial dengan validasi domain ketat.
///
/// # Examples
///
/// ```rust
/// use rust_learning_lab::fase13_task_1::Account;
///
/// let acc = Account::new(1, "Alice Cooper", 500.0).expect("Inisialisasi akun valid");
/// assert_eq!(acc.id, 1);
/// assert_eq!(acc.balance(), 500.0);
/// assert!(acc.is_active());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: u64,
    owner: String,
    balance: f64,
    is_active: bool,
}

impl Account {
    /// Membuat instance `Account` baru dengan validasi nama dan saldo awal.
    ///
    /// # Errors
    /// Mengembalikan [`AccountError::InvalidOwnerName`] jika nama pemilik hanya whitespace atau kosong.
    /// Mengembalikan [`AccountError::NegativeOrZeroAmount`] jika `initial_balance` bernilai negatif.
    ///
    /// # Documentation Test (Success Case)
    /// ```rust
    /// use rust_learning_lab::fase13_task_1::Account;
    ///
    /// let account = Account::new(101, "Budi Santoso", 1000.0);
    /// assert!(account.is_ok());
    /// assert_eq!(account.unwrap().owner(), "Budi Santoso");
    /// ```
    ///
    /// # Documentation Test (Error Case)
    /// ```rust
    /// use rust_learning_lab::fase13_task_1::{Account, AccountError};
    ///
    /// let err = Account::new(102, "   ", 500.0);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err(), AccountError::InvalidOwnerName("Nama tidak boleh kosong".into()));
    /// ```
    pub fn new(id: u64, owner: &str, initial_balance: f64) -> Result<Self, AccountError> {
        let trimmed_owner = owner.trim();
        if trimmed_owner.is_empty() {
            return Err(AccountError::InvalidOwnerName(
                "Nama tidak boleh kosong".into(),
            ));
        }

        if initial_balance < 0.0 {
            return Err(AccountError::NegativeOrZeroAmount(initial_balance));
        }

        Ok(Self {
            id,
            owner: trimmed_owner.to_string(),
            balance: initial_balance,
            is_active: true,
        })
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Menyetorkan uang ke rekening. Nominal harus bernilai positif (> 0.0).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_learning_lab::fase13_task_1::Account;
    ///
    /// let mut acc = Account::new(1, "Citra", 100.0).unwrap();
    /// let new_balance = acc.deposit(50.0).unwrap();
    /// assert_eq!(new_balance, 150.0);
    /// ```
    pub fn deposit(&mut self, amount: f64) -> Result<f64, AccountError> {
        if !self.is_active {
            return Err(AccountError::AccountInactive(self.id));
        }
        if amount <= 0.0 {
            return Err(AccountError::NegativeOrZeroAmount(amount));
        }

        self.balance += amount;
        Ok(self.balance)
    }

    /// Menarik uang dari rekening dengan proteksi *overdraft*.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_learning_lab::fase13_task_1::Account;
    ///
    /// let mut acc = Account::new(1, "Doni", 200.0).unwrap();
    /// let new_balance = acc.withdraw(75.0).unwrap();
    /// assert_eq!(new_balance, 125.0);
    ///
    /// // Kasus saldo kurang (error)
    /// let err = acc.withdraw(500.0);
    /// assert!(err.is_err());
    /// ```
    pub fn withdraw(&mut self, amount: f64) -> Result<f64, AccountError> {
        if !self.is_active {
            return Err(AccountError::AccountInactive(self.id));
        }
        if amount <= 0.0 {
            return Err(AccountError::NegativeOrZeroAmount(amount));
        }
        if self.balance < amount {
            return Err(AccountError::InsufficientFunds {
                available: self.balance,
                required: amount,
            });
        }

        self.balance -= amount;
        Ok(self.balance)
    }

    /// Mentransfer dana dari akun ini ke akun tujuan secara atomik.
    pub fn transfer(&mut self, target: &mut Account, amount: f64) -> Result<(), AccountError> {
        if !self.is_active {
            return Err(AccountError::AccountInactive(self.id));
        }
        if !target.is_active {
            return Err(AccountError::AccountInactive(target.id));
        }

        // Tarik dari akun pengirim dahulu
        self.withdraw(amount)?;

        // Setorkan ke akun penerima (jika gagal, rollback)
        if let Err(e) = target.deposit(amount) {
            self.balance += amount; // Rollback
            return Err(e);
        }

        Ok(())
    }

    /// Menghitung rasio pembagian saldo dengan penanganan edge case pembagian nol dan floating point.
    ///
    /// # Examples
    /// ```rust
    /// use rust_learning_lab::fase13_task_1::Account;
    ///
    /// let ratio = Account::safe_divide_ratio(25.0, 100.0).unwrap();
    /// assert_eq!(ratio, 0.25);
    ///
    /// let err = Account::safe_divide_ratio(10.0, 0.0);
    /// assert!(err.is_err());
    /// ```
    pub fn safe_divide_ratio(part: f64, total: f64) -> Result<f64, AccountError> {
        if total.abs() < FLOAT_EPSILON {
            return Err(AccountError::DivisionByZero);
        }
        Ok(part / total)
    }

    /// Operasi settlement asinkron dengan simulasi latensi jaringan pembayaran.
    pub async fn settle_transaction_async(
        &mut self,
        amount: f64,
        fee: f64,
        latency_ms: u64,
    ) -> Result<f64, AccountError> {
        if !self.is_active {
            return Err(AccountError::AccountInactive(self.id));
        }

        let total_deduction = amount + fee;
        if self.balance < total_deduction {
            return Err(AccountError::InsufficientFunds {
                available: self.balance,
                required: total_deduction,
            });
        }

        // Simulasi latensi settlement perbankan
        sleep(Duration::from_millis(latency_ms)).await;

        self.balance -= total_deduction;
        Ok(self.balance)
    }
}

// ============================================================================
// 2. Fungsi Eksekusi Demo / Run
// ============================================================================

/// Fungsi utama runner demonstrasi untuk modul Fase 13.
pub fn run() {
    println!("=== FASE 13 Task 1: Testing & Quality Assurance Suite ===");

    // 1. Success case demo
    let mut acc1 = Account::new(1, "Eko Kurniawan", 1_000_000.0).expect("Inisialisasi akun gagal");
    println!(
        "1. [Success Case] Akun dibuat: #{} ({}), Saldo: Rp {:.2}",
        acc1.id,
        acc1.owner(),
        acc1.balance()
    );

    let dep_res = acc1.deposit(250_000.0).expect("Deposit gagal");
    println!("   Deposit Rp 250,000 -> Saldo Baru: Rp {:.2}", dep_res);

    let with_res = acc1.withdraw(500_000.0).expect("Penarikan gagal");
    println!("   Withdraw Rp 500,000 -> Saldo Baru: Rp {:.2}", with_res);
    println!(
        "   Status keaktifan akun #1: {}",
        if acc1.is_active() {
            "Aktif"
        } else {
            "Nonaktif"
        }
    );

    // Transfer demo
    let mut acc2 = Account::new(2, "Budi Pratama", 100_000.0).expect("Inisialisasi akun 2 gagal");
    acc1.transfer(&mut acc2, 200_000.0)
        .expect("Transfer dana gagal");
    println!(
        "   Transfer Rp 200,000 ke akun #2 -> Saldo Pengirim: Rp {:.2}, Penerima: Rp {:.2}",
        acc1.balance(),
        acc2.balance()
    );

    // Freeze account demo
    acc2.set_active(false);
    println!(
        "   Akun #2 dibekukan sementara (is_active: {})",
        acc2.is_active()
    );

    // 2. Error case demo
    println!("\n2. [Error Case] Menolak overdraft melebih saldo:");
    let overdraft_res = acc1.withdraw(2_000_000.0);
    match overdraft_res {
        Ok(_) => panic!("Harusnya transaksi ditolak karena saldo kurang!"),
        Err(e) => println!("   [Expected Error Ditangkap]: {}", e),
    }

    // 3. Edge case demo (Float epsilon & zero boundary)
    println!("\n3. [Edge Case] Pembagian rasio dengan pembagi nol:");
    let zero_div = Account::safe_divide_ratio(100.0, 0.0);
    match zero_div {
        Ok(_) => panic!("Harusnya pembagian nol menghasilkan error!"),
        Err(e) => println!("   [Expected Edge Case Ditangkap]: {}", e),
    }

    let exact_withdraw = acc1
        .withdraw(acc1.balance())
        .expect("Tarik hingga saldo tepat nol");
    println!(
        "   Penarikan presisi hingga saldo habis -> Saldo akhir: Rp {:.2}",
        exact_withdraw
    );

    // 4. Async settlement demo
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Gagal menginisialisasi Tokio runtime");

    rt.block_on(async {
        let mut async_acc = Account::new(99, "Vault Serverless", 500.0).unwrap();
        println!("\n4. [Async Settlement] Memulai settlement non-blocking (latensi 15ms)...");
        let final_bal = async_acc
            .settle_transaction_async(150.0, 2.5, 15)
            .await
            .expect("Settlement async sukses");
        println!("   Settlement berhasil! Saldo tersisa: Rp {:.2}", final_bal);
    });

    println!(
        "\n[OK] FASE 13 Task 1 (Unit, Integration, Doc, Async Tests & Assertions) tuntas & terverifikasi!\n"
    );
}

// ============================================================================
// 3. Unit Tests (White-Box Testing di Modul yang Sama)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Kategori A: Assertions Variatif (assert!, assert_eq!, assert_ne!)
    // ------------------------------------------------------------------------

    #[test]
    fn test_assertions_equality_and_inequality() {
        let acc1 = Account::new(10, "User Alpha", 100.0).unwrap();
        let acc2 = Account::new(20, "User Beta", 200.0).unwrap();

        // assert_eq!
        assert_eq!(
            acc1.id, 10,
            "ID akun harus sesuai dengan argumen konstruktor"
        );
        assert_eq!(acc1.balance(), 100.0);

        // assert_ne!
        assert_ne!(
            acc1.id, acc2.id,
            "Dua akun dengan ID berbeda tidak boleh sama"
        );
        assert_ne!(acc1.owner(), acc2.owner());

        // assert!
        assert!(acc1.is_active(), "Akun baru harus dalam status aktif");
        assert!(acc1.balance() > 0.0, "Saldo akun baru harus positif");
    }

    // ------------------------------------------------------------------------
    // Kategori B: Test Success Cases (Happy Paths)
    // ------------------------------------------------------------------------

    #[test]
    fn test_deposit_and_withdraw_success_pipeline() {
        let mut acc = Account::new(1, "Test User", 500.0).unwrap();

        let bal1 = acc.deposit(250.0).unwrap();
        assert_eq!(bal1, 750.0);

        let bal2 = acc.withdraw(300.0).unwrap();
        assert_eq!(bal2, 450.0);
    }

    #[test]
    fn test_transfer_between_accounts_success() {
        let mut sender = Account::new(1, "Sender", 1000.0).unwrap();
        let mut receiver = Account::new(2, "Receiver", 200.0).unwrap();

        let res = sender.transfer(&mut receiver, 400.0);
        assert!(res.is_ok());

        assert_eq!(sender.balance(), 600.0);
        assert_eq!(receiver.balance(), 600.0);
    }

    // ------------------------------------------------------------------------
    // Kategori C: Test Error Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_insufficient_funds_error() {
        let mut acc = Account::new(1, "Alice", 100.0).unwrap();

        let err = acc.withdraw(150.0).unwrap_err();
        assert_eq!(
            err,
            AccountError::InsufficientFunds {
                available: 100.0,
                required: 150.0,
            }
        );
        // Saldo tidak boleh berubah jika gagal
        assert_eq!(acc.balance(), 100.0);
    }

    #[test]
    fn test_negative_deposit_rejected() {
        let mut acc = Account::new(1, "Alice", 100.0).unwrap();

        let err = acc.deposit(-25.0).unwrap_err();
        assert_eq!(err, AccountError::NegativeOrZeroAmount(-25.0));
    }

    #[test]
    fn test_inactive_account_operations_fail() {
        let mut acc = Account::new(1, "Frozen User", 100.0).unwrap();
        acc.set_active(false);

        assert!(!acc.is_active());
        assert_eq!(
            acc.deposit(50.0).unwrap_err(),
            AccountError::AccountInactive(1)
        );
        assert_eq!(
            acc.withdraw(50.0).unwrap_err(),
            AccountError::AccountInactive(1)
        );
    }

    // ------------------------------------------------------------------------
    // Kategori D: Test Edge Cases (Batas Ekstrem, Presisi, Nilai Kosong)
    // ------------------------------------------------------------------------

    #[test]
    fn test_edge_case_empty_or_whitespace_owner_name() {
        let empty_err = Account::new(1, "", 100.0);
        assert!(empty_err.is_err());
        assert_eq!(
            empty_err.unwrap_err(),
            AccountError::InvalidOwnerName("Nama tidak boleh kosong".into())
        );

        let space_err = Account::new(2, "   \t\n  ", 100.0);
        assert!(space_err.is_err());
    }

    #[test]
    fn test_edge_case_exact_balance_withdrawal_to_zero() {
        let mut acc = Account::new(1, "Zero Balance Target", 250.75).unwrap();

        // Menarik tepat seluruh sisa saldo
        let res = acc.withdraw(250.75);
        assert!(res.is_ok());

        // Verifikasi saldo mencapai 0.0 dengan batas toleransi epsilon
        assert!(acc.balance().abs() < FLOAT_EPSILON);
    }

    #[test]
    fn test_edge_case_safe_divide_by_zero_and_near_zero() {
        // Pembagian dengan 0.0 murni
        let err1 = Account::safe_divide_ratio(50.0, 0.0);
        assert_eq!(err1.unwrap_err(), AccountError::DivisionByZero);

        // Pembagian dengan angka sangat kecil di bawah epsilon
        let err2 = Account::safe_divide_ratio(50.0, 1e-9);
        assert_eq!(err2.unwrap_err(), AccountError::DivisionByZero);

        // Pembagian normal mendekati batas atas
        let normal = Account::safe_divide_ratio(1.0, 3.0).unwrap();
        assert!((normal - (1.0 / 3.0)).abs() < FLOAT_EPSILON);
    }

    // ------------------------------------------------------------------------
    // Kategori E: Async Test dengan #[tokio::test]
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_async_settlement_success_with_fee() {
        let mut acc = Account::new(55, "Async Merchant", 1000.0).unwrap();

        let remaining = acc
            .settle_transaction_async(200.0, 5.0, 10)
            .await
            .expect("Settlement asinkron harus sukses");

        assert_eq!(remaining, 795.0);
        assert_eq!(acc.balance(), 795.0);
    }

    #[tokio::test]
    async fn test_async_settlement_insufficient_funds_rejected() {
        let mut acc = Account::new(56, "Poor Merchant", 100.0).unwrap();

        let err = acc
            .settle_transaction_async(95.0, 10.0, 5) // Total 105.0 > 100.0
            .await
            .unwrap_err();

        assert_eq!(
            err,
            AccountError::InsufficientFunds {
                available: 100.0,
                required: 105.0,
            }
        );
        // Saldo tidak terpotong
        assert_eq!(acc.balance(), 100.0);
    }

    #[test]
    fn test_sync_runner_smoke_test() {
        run();
    }
}
