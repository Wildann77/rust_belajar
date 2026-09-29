// Fase 6 - Task 2: Robust Error Handling — Result<T, E> & Operator ?
// Rujukan: rust_learning_guide.md (Sub-bab 6.2) & rust_execution_tasks.md (L568-L575)

/// Representasi akun bank untuk demonstrasi transaksi
#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub id: u64,
    pub owner: String,
    pub balance: f64,
}

/// Tanda bukti transaksi yang berhasil diproses
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionReceipt {
    pub transaction_id: &'static str,
    pub account_id: u64,
    pub amount: f64,
    pub remaining_balance: f64,
}

// ----------------------------------------------------------------------------
// 1 & 2. Demonstrasi pembuatan varian Ok(T) dan Err(E)
// ----------------------------------------------------------------------------

/// Memvalidasi dan mengurai (parse) input string nominal uang menjadi f64
/// Mengembalikan:
/// - Ok(f64) jika nominal valid dan bernilai positif (> 0.0)
/// - Err(String) jika format angka rusak atau bernilai non-positif
pub fn parse_amount(raw: &str) -> Result<f64, String> {
    let trimmed = raw.trim();
    let amount = match trimmed.parse::<f64>() {
        Ok(val) => val,
        Err(_) => return Err(format!("Nominal '{trimmed}' bukan format angka yang valid")),
    };

    if amount <= 0.0 {
        Err("Nominal transaksi harus bernilai lebih dari 0".to_string())
    } else {
        Ok(amount)
    }
}

/// Mengambil data akun berdasarkan ID
/// Mengembalikan Ok(BankAccount) jika ditemukan, atau Err(String) jika tidak ditemukan
pub fn fetch_account(id: u64) -> Result<BankAccount, String> {
    match id {
        101 => Ok(BankAccount {
            id: 101,
            owner: "Alice".to_string(),
            balance: 5000.0,
        }),
        102 => Ok(BankAccount {
            id: 102,
            owner: "Bob".to_string(),
            balance: 250.0,
        }),
        unknown_id => Err(format!("Akun dengan ID {unknown_id} tidak ditemukan")),
    }
}

/// Melakukan pemotongan saldo (debit) pada akun
/// Mengembalikan Ok(sisa_saldo) jika saldo cukup, atau Err(String) jika tidak mencukupi
pub fn debit_account(account: &mut BankAccount, amount: f64) -> Result<f64, String> {
    if account.balance < amount {
        Err(format!(
            "Saldo tidak mencukupi! Tersedia: Rp{:.2}, diminta: Rp{:.2}",
            account.balance, amount
        ))
    } else {
        account.balance -= amount;
        Ok(account.balance)
    }
}

// ----------------------------------------------------------------------------
// 3. Demonstrasi penanganan Result secara manual via exhaustive `match`
// ----------------------------------------------------------------------------

/// Mengekstrak Result menggunakan pattern matching `match` eksplisit
pub fn format_amount_inspection(raw: &str) -> String {
    match parse_amount(raw) {
        Ok(amount) => format!("[VALID] Nominal: Rp{:.2}", amount),
        Err(err_msg) => format!("[GAGAL] Validasi gagal: {}", err_msg),
    }
}

// ----------------------------------------------------------------------------
// 4 & 5. Demonstrasi Operator `?` dan Error Propagation
// ----------------------------------------------------------------------------

/// Memproses pembayaran lengkap dari string nominal ke pemotongan akun bank.
/// Menggunakan operator `?` untuk melakukan early-return (propagasi error otomatis).
/// Jika ada langkah yang gagal, fungsi ini langsung keluar dan mengembalikan Err.
pub fn process_payment(account_id: u64, amount_str: &str) -> Result<TransactionReceipt, String> {
    // 1. Parse nominal - jika error, langsung return Err dari fungsi ini
    let amount = parse_amount(amount_str)?;

    // 2. Ambil akun - jika ID tidak ada, langsung return Err
    let mut account = fetch_account(account_id)?;

    // 3. Debet saldo - jika saldo kurang, langsung return Err
    let remaining = debit_account(&mut account, amount)?;

    // 4. Semua langkah sukses: bungkus hasil akhir ke dalam Ok
    Ok(TransactionReceipt {
        transaction_id: "TXN-2026-001",
        account_id: account.id,
        amount,
        remaining_balance: remaining,
    })
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== Fase 6 - Task 2: Robust Error Handling — Result<T, E> & Operator ? ===");
    println!("Konsep Inti: Penanganan kemungkinan gagal secara eksplisit via Ok & Err serta operator ?\n");

    // 1 & 2: Ok & Err via match
    println!("1. Validasi Input via `match` (Ok vs Err):");
    println!("   - {}", format_amount_inspection("1500.50"));
    println!("   - {}", format_amount_inspection("-500"));
    println!("   - {}", format_amount_inspection("seribu_rupiah"));

    // 3: Error Propagation via ? (Sukses)
    println!("\n2. Eksekusi Pipeline Pembayaran Sukses (Operator ?):");
    match process_payment(101, "1200.00") {
        Ok(receipt) => {
            println!(
                "   [✓] Transaksi Berhasil! ID: {}, Akun: {}, Debet: Rp{:.2}, Sisa: Rp{:.2}",
                receipt.transaction_id, receipt.account_id, receipt.amount, receipt.remaining_balance
            );
        }
        Err(e) => println!("   [✗] Transaksi Gagal: {}", e),
    }

    // 4: Error Propagation: Nominal Tidak Valid
    println!("\n3. Propagasi Error Tahap 1: Format Nominal Tidak Valid:");
    match process_payment(101, "abc") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => println!("   [Propagated Err] {}", e),
    }

    // 5: Error Propagation: Akun Tidak Ditemukan
    println!("\n4. Propagasi Error Tahap 2: Akun Tidak Terdaftar:");
    match process_payment(999, "500.00") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => println!("   [Propagated Err] {}", e),
    }

    // 6: Error Propagation: Saldo Tidak Mencukupi
    println!("\n5. Propagasi Error Tahap 3: Saldo Akun Kurang:");
    match process_payment(102, "1000.00") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => println!("   [Propagated Err] {}", e),
    }
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_and_err_creation() {
        assert_eq!(parse_amount("500"), Ok(500.0));
        assert!(parse_amount("0").is_err());
        assert!(parse_amount("-10").is_err());
        assert!(parse_amount("invalid").is_err());
    }

    #[test]
    fn test_match_formatting() {
        let valid = format_amount_inspection("250.0");
        let invalid = format_amount_inspection("xyz");

        assert!(valid.contains("[VALID]"));
        assert!(invalid.contains("[GAGAL]"));
    }

    #[test]
    fn test_process_payment_success() {
        let res = process_payment(101, "1000.0");
        assert!(res.is_ok());

        let receipt = res.unwrap();
        assert_eq!(receipt.account_id, 101);
        assert_eq!(receipt.amount, 1000.0);
        assert_eq!(receipt.remaining_balance, 4000.0);
    }

    #[test]
    fn test_error_propagation_invalid_input() {
        let res = process_payment(101, "bukan_angka");
        assert_eq!(
            res,
            Err("Nominal 'bukan_angka' bukan format angka yang valid".to_string())
        );
    }

    #[test]
    fn test_error_propagation_account_not_found() {
        let res = process_payment(888, "100.0");
        assert_eq!(
            res,
            Err("Akun dengan ID 888 tidak ditemukan".to_string())
        );
    }

    #[test]
    fn test_error_propagation_insufficient_funds() {
        let res = process_payment(102, "500.0");
        assert_eq!(
            res,
            Err("Saldo tidak mencukupi! Tersedia: Rp250.00, diminta: Rp500.00".to_string())
        );
    }
}
