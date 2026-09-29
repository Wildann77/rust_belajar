// Fase 6 - Task 3: Robust Error Handling — Custom Error
// Rujukan: rust_learning_guide.md (Sub-bab 6.3) & rust_execution_tasks.md (L576-L593)

use std::fmt;
use std::fs;
use std::io;

/// 1. Custom Enum Error untuk seluruh kemungkinan kegagalan domain
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InvalidInput(String),
    Io(io::Error),
}

/// 2. Implementasi Display untuk pesan error yang ramah pengguna
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Data Tidak Ditemukan: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Input Tidak Valid: {msg}"),
            AppError::Io(err) => write!(f, "Kesalahan I/O Sistem: {err}"),
        }
    }
}

/// 3. Implementasi std::error::Error agar terintegrasi dengan ekosistem error Rust
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

/// 4. Konversi Error: Mengubah std::io::Error menjadi AppError secara otomatis
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

// ----------------------------------------------------------------------------
// Domain Functions: Demonstrasi Error Propagation via Operator ?
// ----------------------------------------------------------------------------

/// Validasi nama file konfigurasi
pub fn validate_config_filename(filename: &str) -> Result<&str, AppError> {
    let trimmed = filename.trim();
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput(
            "Nama file konfigurasi tidak boleh kosong".to_string(),
        ));
    }
    if !trimmed.ends_with(".conf") && !trimmed.ends_with(".json") {
        return Err(AppError::InvalidInput(format!(
            "Ekstensi file '{trimmed}' tidak didukung (harus .conf atau .json)"
        )));
    }
    Ok(trimmed)
}

/// Membaca file konfigurasi dan mencari key tertentu.
/// 5. Menggunakan operator `?` untuk propagasi error:
///    - `validate_config_filename(path)?` mempropagasi `AppError::InvalidInput`
///    - `fs::read_to_string(valid_path)?` otomatis dikonversi dari `io::Error` ke `AppError::Io` via `From`
///    - Jika key tidak ditemukan, mengembalikan `AppError::NotFound`
pub fn read_config_entry(path: &str, target_key: &str) -> Result<String, AppError> {
    let valid_path = validate_config_filename(path)?;
    let content = fs::read_to_string(valid_path)?;

    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == target_key {
                return Ok(v.trim().to_string());
            }
        }
    }

    Err(AppError::NotFound(format!(
        "Key '{target_key}' tidak ditemukan di '{valid_path}'"
    )))
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== Fase 6 - Task 3: Robust Error Handling — Custom Error ===");
    println!("Konsep Inti: Custom enum error dengan trait Debug, Display, Error, From, dan ?\n");

    // Persiapan file dummy sementara
    let temp_conf = std::env::temp_dir().join("app_fase6_test.conf");
    let _ = fs::write(&temp_conf, "database_url=postgres://localhost:5432/app\nport=8080\n");

    // 1. Sukses: Semua langkah valid
    println!("1. Kasus Sukses (Valid Input, Valid I/O, Key Ditemukan):");
    match read_config_entry(temp_conf.to_str().unwrap(), "database_url") {
        Ok(val) => println!("   [✓] Nilai config ditemukan: database_url = {val}"),
        Err(e) => println!("   [✗] Error tidak terduga: {e}"),
    }

    // 2. Error Kasus 1: Input Tidak Valid (InvalidInput)
    println!("\n2. Kasus Gagal 1: Validasi Input (AppError::InvalidInput):");
    match read_config_entry("invalid_format.txt", "port") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => {
            println!("   - Display : {e}");
            println!("   - Debug   : {:?}", e);
        }
    }

    // 3. Error Kasus 2: I/O File Tidak Ada (AppError::Io via From conversion)
    println!("\n3. Kasus Gagal 2: I/O File Hilang (AppError::Io via `?` & `From`):");
    match read_config_entry("berkas_ghaib_123456.conf", "port") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => {
            println!("   - Display : {e}");
            println!("   - Debug   : {:?}", e);
            if let Some(src) = std::error::Error::source(&e) {
                println!("   - Source  : {src}");
            }
        }
    }

    // 4. Error Kasus 3: Key Tidak Ada di File (AppError::NotFound)
    println!("\n4. Kasus Gagal 3: Key Tidak Ada (AppError::NotFound):");
    match read_config_entry(temp_conf.to_str().unwrap(), "secret_api_key") {
        Ok(_) => println!("   [✓] Sukses"),
        Err(e) => {
            println!("   - Display : {e}");
            println!("   - Debug   : {:?}", e);
        }
    }

    // Bersihkan file sementara
    let _ = fs::remove_file(temp_conf);
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_implementation() {
        let err1 = AppError::NotFound("User #10".to_string());
        let err2 = AppError::InvalidInput("Format salah".to_string());
        let err3 = AppError::Io(io::Error::new(io::ErrorKind::NotFound, "file hilang"));

        assert_eq!(format!("{err1}"), "Data Tidak Ditemukan: User #10");
        assert_eq!(format!("{err2}"), "Input Tidak Valid: Format salah");
        assert!(format!("{err3}").contains("Kesalahan I/O Sistem: file hilang"));
    }

    #[test]
    fn test_debug_implementation() {
        let err = AppError::NotFound("Profile".to_string());
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("NotFound(\"Profile\")"));
    }

    #[test]
    fn test_std_error_and_source() {
        use std::error::Error;

        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let app_err = AppError::Io(io_err);

        assert!(app_err.source().is_some());
        assert_eq!(
            app_err.source().unwrap().to_string(),
            "access denied"
        );

        let not_found_err = AppError::NotFound("Item".to_string());
        assert!(not_found_err.source().is_none());
    }

    #[test]
    fn test_from_io_error_conversion() {
        let raw_io = io::Error::new(io::ErrorKind::ConnectionReset, "koneksi putus");
        let converted: AppError = raw_io.into();

        match converted {
            AppError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::ConnectionReset),
            _ => panic!("Harus terkonversi menjadi AppError::Io"),
        }
    }

    #[test]
    fn test_pipeline_error_propagation() {
        let temp_file = std::env::temp_dir().join("test_custom_err.conf");
        fs::write(&temp_file, "admin_user=budi\n").unwrap();

        // Sukses
        let val = read_config_entry(temp_file.to_str().unwrap(), "admin_user").unwrap();
        assert_eq!(val, "budi");

        // InvalidInput
        let err_input = read_config_entry("file.doc", "admin_user");
        assert!(matches!(err_input, Err(AppError::InvalidInput(_))));

        // Io
        let err_io = read_config_entry("tidak_ada_di_dunia.conf", "admin_user");
        assert!(matches!(err_io, Err(AppError::Io(_))));

        // NotFound
        let err_nf = read_config_entry(temp_file.to_str().unwrap(), "non_existent_key");
        assert!(matches!(err_nf, Err(AppError::NotFound(_))));

        let _ = fs::remove_file(temp_file);
    }
}
