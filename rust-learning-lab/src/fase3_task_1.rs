// Fase 3 - Task 1: Struct (Classic, Tuple, Unit-like, impl, Associated Function, &self, &mut self, Self)
// Rujukan: rust_learning_guide.md (Bagian 3.1)

// ==========================================
// 1. Classic Struct (Named-Field Struct)
// ==========================================
#[derive(Debug, Clone, PartialEq)]
pub struct UserAccount {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub balance: f64,
}

// ==========================================
// 2. Tuple Struct & Newtype Pattern
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilometers(pub f64);

// ==========================================
// 3. Unit-like Struct (Zero-Sized Type / ZST)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPrivilege;

// ==========================================
// 4. Implementasi `impl` untuk Classic Struct
// ==========================================
impl UserAccount {
    // 4a. Associated Function (Constructor) -> Mengembalikan `Self`
    // Tidak menerima parameter `self`, dipanggil via `UserAccount::new(...)`
    pub fn new(id: u64, username: &str, email: &str) -> Self {
        // Field init shorthand: saat nama variabel argumen sama dengan nama field
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            active: true,
            balance: 0.0,
        }
    }

    // Associated function dengan saldo awal
    pub fn with_initial_balance(
        id: u64,
        username: &str,
        email: &str,
        initial_balance: f64,
    ) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            active: true,
            balance: initial_balance,
        }
    }

    // 4b. Method dengan `&self` (Immutable Borrow - Membaca data)
    pub fn is_solvent(&self) -> bool {
        self.balance >= 0.0
    }

    pub fn display_summary(&self) -> String {
        format!(
            "[Account #{}] User: '{}' ({}) | Active: {} | Balance: ${:.2}",
            self.id, self.username, self.email, self.active, self.balance
        )
    }

    // 4c. Method dengan `&mut self` (Mutable Borrow - Memodifikasi data)
    pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Nominal deposit harus lebih besar dari 0".to_string());
        }
        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Nominal penarikan harus lebih besar dari 0".to_string());
        }
        if self.balance < amount {
            return Err(format!(
                "Saldo tidak mencukupi: saldo saat ini ${:.2}, ditarik ${:.2}",
                self.balance, amount
            ));
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    // 4d. Method dengan `self` (Consuming / Move - Mengambil alih hak milik)
    // Setelah fungsi ini dipanggil, instance struct di-drop / hangus dari memori pemanggil!
    pub fn close_account(self) -> String {
        format!(
            "Akun #{} milik '{}' resmi ditutup dan dihapus dari memori.",
            self.id, self.username
        )
    }
}

// ==========================================
// 5. Implementasi `impl` untuk Tuple Struct & Unit Struct
// ==========================================
impl ColorRgb {
    pub fn black() -> Self {
        Self(0, 0, 0)
    }

    pub fn white() -> Self {
        Self(255, 255, 255)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

impl Kilometers {
    pub fn to_miles(&self) -> f64 {
        self.0 * 0.621371
    }
}

impl AdminPrivilege {
    pub fn can_delete_users(&self) -> bool {
        true
    }
}

// ==========================================
// Runner Function
// ==========================================
pub fn run() {
    println!("=== Fase 3 - Task 1: Structs & Methods ===");

    // 1. Classic Struct & Associated Function (`Self` Constructor)
    let mut account1 = UserAccount::new(101, "boyblanco", "boy@example.com");
    println!("1. Classic Struct (via Associated Function `new`):");
    println!("   {}", account1.display_summary());

    // 2. Method `&self` (Membaca data)
    println!("2. Method `&self` (is_solvent): {}", account1.is_solvent());

    // 3. Method `&mut self` (Deposit & Withdraw)
    match account1.deposit(150.0) {
        Ok(new_balance) => println!("3. Deposit berhasil: Saldo baru = ${:.2}", new_balance),
        Err(err) => println!("   Deposit gagal: {err}"),
    }

    match account1.withdraw(50.0) {
        Ok(new_balance) => println!("   Withdraw berhasil: Saldo baru = ${:.2}", new_balance),
        Err(err) => println!("   Withdraw gagal: {err}"),
    }

    account1.deactivate();
    println!("   Setelah deactivate(): {}", account1.display_summary());

    // 4. Struct Update Syntax (`..other`)
    // Membuat struct baru dengan mewarisi field dari struct lain
    let account2 = UserAccount {
        id: 102,
        username: "ferris".to_string(),
        email: "ferris@rust-lang.org".to_string(),
        ..account1.clone() // Mengambil balance & active dari account1
    };
    println!("4. Struct Update Syntax (`..account1`):");
    println!("   {}", account2.display_summary());

    // 5. Tuple Struct & Newtype Pattern
    let red = ColorRgb(255, 0, 0);
    let black = ColorRgb::black();
    let white = ColorRgb::white();
    println!("5. Tuple Struct:");
    println!(
        "   Red: RGB({}, {}, {}) -> Hex: {}",
        red.0,
        red.1,
        red.2,
        red.to_hex()
    );
    println!("   Black Associated: Hex: {}", black.to_hex());
    println!("   White Associated: Hex: {}", white.to_hex());

    let custom_acc = UserAccount::with_initial_balance(103, "grace", "grace@example.com", 500.0);
    println!(
        "   Custom Initial Balance Account: {}",
        custom_acc.display_summary()
    );

    let distance_km = Kilometers(100.0);
    println!(
        "   Newtype Distance: {:.1} km = {:.2} miles",
        distance_km.0,
        distance_km.to_miles()
    );

    // 6. Unit-like Struct (Zero-Sized Type)
    let admin = AdminPrivilege;
    let zst_size = std::mem::size_of::<AdminPrivilege>();
    println!("6. Unit-like Struct (AdminPrivilege):");
    println!("   Ukuran memori (ZST): {} byte", zst_size);
    println!("   Privilege delete users: {}", admin.can_delete_users());

    // 7. Method `self` (Consuming / Move - Hak Milik Berpindah & Objek Di-drop)
    let closing_msg = account2.close_account();
    println!("7. Method `self` (Consuming / Move):");
    println!("   {closing_msg}");
    // account2 tidak bisa diakses lagi setelah baris ini karena ownership sudah dipindahkan!
}

// ==========================================
// Unit Tests
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consuming_self_close_account() {
        let acc = UserAccount::new(99, "terminator", "t@skynet.com");
        let msg = acc.close_account();
        assert!(msg.contains("resmi ditutup"));
        // acc sudah moved, tidak bisa diakses lagi
    }

    #[test]
    fn test_user_account_constructor() {
        let acc = UserAccount::new(1, "alice", "alice@example.com");
        assert_eq!(acc.id, 1);
        assert_eq!(acc.username, "alice");
        assert_eq!(acc.email, "alice@example.com");
        assert!(acc.active);
        assert_eq!(acc.balance, 0.0);
    }

    #[test]
    fn test_with_initial_balance() {
        let acc = UserAccount::with_initial_balance(2, "bob", "bob@example.com", 250.0);
        assert_eq!(acc.balance, 250.0);
        assert!(acc.is_solvent());
    }

    #[test]
    fn test_deposit_and_withdraw_success() {
        let mut acc = UserAccount::new(3, "charlie", "charlie@example.com");
        let dep_res = acc.deposit(100.0);
        assert!(dep_res.is_ok());
        assert_eq!(acc.balance, 100.0);

        let wd_res = acc.withdraw(40.0);
        assert!(wd_res.is_ok());
        assert_eq!(acc.balance, 60.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut acc = UserAccount::new(4, "dave", "dave@example.com");
        let res = acc.withdraw(50.0);
        assert!(res.is_err());
        assert_eq!(acc.balance, 0.0);
    }

    #[test]
    fn test_negative_or_zero_amount_error() {
        let mut acc = UserAccount::new(5, "eve", "eve@example.com");
        assert!(acc.deposit(-10.0).is_err());
        assert!(acc.deposit(0.0).is_err());
        assert!(acc.withdraw(-5.0).is_err());
        assert!(acc.withdraw(0.0).is_err());
    }

    #[test]
    fn test_deactivate_and_display_summary() {
        let mut acc = UserAccount::new(6, "frank", "frank@example.com");
        assert!(acc.active);
        acc.deactivate();
        assert!(!acc.active);
        let summary = acc.display_summary();
        assert!(summary.contains("Active: false"));
        assert!(summary.contains("frank"));
    }

    #[test]
    fn test_tuple_struct_color_rgb() {
        let color = ColorRgb(18, 52, 86);
        assert_eq!(color.0, 18);
        assert_eq!(color.1, 52);
        assert_eq!(color.2, 86);
        assert_eq!(color.to_hex(), "#123456");

        let black = ColorRgb::black();
        assert_eq!(black.to_hex(), "#000000");

        let white = ColorRgb::white();
        assert_eq!(white.to_hex(), "#FFFFFF");
    }

    #[test]
    fn test_newtype_kilometers() {
        let km = Kilometers(10.0);
        let miles = km.to_miles();
        assert!((miles - 6.21371).abs() < 1e-4);
    }

    #[test]
    fn test_unit_like_struct_zero_size() {
        let admin = AdminPrivilege;
        assert!(admin.can_delete_users());
        assert_eq!(std::mem::size_of::<AdminPrivilege>(), 0);
    }
}
