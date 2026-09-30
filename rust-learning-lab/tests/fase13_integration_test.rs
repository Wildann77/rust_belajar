// Integration Test untuk Fase 13 Testing & Quality Assurance
// Lokasi: rust-learning-lab/tests/fase13_integration_test.rs
//
// File ini dikompilasi oleh Cargo sebagai crate eksternal independen
// yang menguji public API dari crate library `rust_learning_lab`.

use rust_learning_lab::fase13_task_1::{Account, AccountError, FLOAT_EPSILON};

#[test]
fn test_integration_account_lifecycle_and_transfers() {
    // 1. Success case: Pembuatan akun dengan data valid
    let mut alice =
        Account::new(101, "Alice Walker", 1500.0).expect("Inisialisasi Alice harus valid");
    let mut bob = Account::new(102, "Bob Miller", 300.0).expect("Inisialisasi Bob harus valid");

    // Validasi assertions
    assert_eq!(alice.id, 101);
    assert_eq!(bob.id, 102);
    assert_ne!(alice.id, bob.id);
    assert_ne!(alice.owner(), bob.owner());
    assert!(alice.is_active());
    assert!(bob.is_active());

    // 2. Success case: Deposit & Withdraw
    let alice_after_dep = alice.deposit(500.0).expect("Deposit Alice harus sukses");
    assert_eq!(alice_after_dep, 2000.0);
    assert_eq!(alice.balance(), 2000.0);

    let bob_after_with = bob.withdraw(100.0).expect("Penarikan Bob harus sukses");
    assert_eq!(bob_after_with, 200.0);
    assert_eq!(bob.balance(), 200.0);

    // 3. Success case: Transfer antar akun
    let transfer_res = alice.transfer(&mut bob, 600.0);
    assert!(transfer_res.is_ok(), "Transfer harus berhasil tanpa error");

    assert_eq!(alice.balance(), 1400.0);
    assert_eq!(bob.balance(), 800.0);
}

#[test]
fn test_integration_error_handling_and_guards() {
    let mut corporate_acc = Account::new(200, "PT Teknologi Maju", 500.0).unwrap();

    // 1. Error case: Overdraft / Saldo kurang
    let overdraft_err = corporate_acc.withdraw(1000.0).unwrap_err();
    assert_eq!(
        overdraft_err,
        AccountError::InsufficientFunds {
            available: 500.0,
            required: 1000.0,
        }
    );
    // Saldo tetap utuh
    assert_eq!(corporate_acc.balance(), 500.0);

    // 2. Error case: Deposit negatif / nol
    let zero_err = corporate_acc.deposit(0.0).unwrap_err();
    assert_eq!(zero_err, AccountError::NegativeOrZeroAmount(0.0));

    let neg_err = corporate_acc.deposit(-150.0).unwrap_err();
    assert_eq!(neg_err, AccountError::NegativeOrZeroAmount(-150.0));

    // 3. Error case: Akun nonaktif
    corporate_acc.set_active(false);
    assert!(!corporate_acc.is_active());

    let inactive_dep = corporate_acc.deposit(100.0).unwrap_err();
    assert_eq!(inactive_dep, AccountError::AccountInactive(200));

    let inactive_with = corporate_acc.withdraw(50.0).unwrap_err();
    assert_eq!(inactive_with, AccountError::AccountInactive(200));
}

#[test]
fn test_integration_edge_cases_and_boundaries() {
    // 1. Edge case: Nama dengan spasi / whitespace
    let whitespace_err = Account::new(301, "   \n\t   ", 100.0);
    assert!(whitespace_err.is_err());
    assert_eq!(
        whitespace_err.unwrap_err(),
        AccountError::InvalidOwnerName("Nama tidak boleh kosong".into())
    );

    // 2. Edge case: Penarikan saldo hingga sisa presisi nol
    let mut exact_acc = Account::new(302, "Exact Margin", 123.456).unwrap();
    let res = exact_acc.withdraw(123.456);
    assert!(res.is_ok());
    assert!(
        exact_acc.balance().abs() < FLOAT_EPSILON,
        "Saldo setelah ditarik habis harus mendekati nol di bawah epsilon"
    );

    // 3. Edge case: Safe division dengan pembagi nol atau mendekati nol
    let div_zero = Account::safe_divide_ratio(10.0, 0.0);
    assert_eq!(div_zero.unwrap_err(), AccountError::DivisionByZero);

    let div_tiny = Account::safe_divide_ratio(10.0, 1e-12);
    assert_eq!(div_tiny.unwrap_err(), AccountError::DivisionByZero);

    // Normal division valid
    let div_valid = Account::safe_divide_ratio(50.0, 200.0).unwrap();
    assert_eq!(div_valid, 0.25);
}

#[tokio::test]
async fn test_integration_async_settlement_workflow() {
    let mut merchant = Account::new(500, "Global Gateway", 2500.0).unwrap();

    // Jalankan async settlement
    let remaining = merchant
        .settle_transaction_async(500.0, 12.5, 20)
        .await
        .expect("Async settlement harus berhasil");

    assert_eq!(remaining, 1987.5);
    assert_eq!(merchant.balance(), 1987.5);

    // Async settlement gagal karena saldo kurang
    let fail_res = merchant.settle_transaction_async(2000.0, 10.0, 10).await;

    assert!(fail_res.is_err());
    assert_eq!(
        fail_res.unwrap_err(),
        AccountError::InsufficientFunds {
            available: 1987.5,
            required: 2010.0,
        }
    );
}
