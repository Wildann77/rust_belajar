// Fase 5 - Task 1: Dependency Management (Dependencies, Dev-Dependencies, cargo tree, Cargo.lock)
// Rujukan: rust_learning_guide.md (Bagian 5.1 & 5.2)

use serde::{Deserialize, Serialize};

/// Representasi metadata package/crate untuk demonstrasi serialisasi serde
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub is_production: bool,
    pub dependencies: Vec<DependencyInfo>,
}

/// Representasi dependensi individual
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub version_req: String,
    pub kind: DependencyKind,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

impl PackageMeta {
    pub fn new(name: &str, version: &str, edition: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            edition: edition.to_string(),
            is_production: true,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dep(&mut self, name: &str, version_req: &str, kind: DependencyKind) {
        self.dependencies.push(DependencyInfo {
            name: name.to_string(),
            version_req: version_req.to_string(),
            kind,
        });
    }

    /// Serialisasi struct ke format JSON string via serde_json
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialisasi JSON string kembali menjadi struct Rust via serde_json
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

pub fn run() {
    println!("=== Fase 5 - Task 1: Dependency Management ===");

    // 1. Menggunakan Regular Dependency (serde + serde_json)
    println!("1. Inisialisasi struct PackageMeta dengan data dependensi riil Cargo.toml:");
    let mut pkg = PackageMeta::new("rust-learning-lab", "0.1.0", "2024");
    pkg.add_dep("serde", "1.0", DependencyKind::Normal);
    pkg.add_dep("serde_json", "1.0", DependencyKind::Normal);
    pkg.add_dep("pretty_assertions", "1.4", DependencyKind::Dev);

    println!(
        "   Package: {} v{} (Edition {})",
        pkg.name, pkg.version, pkg.edition
    );
    println!("   Jumlah dependensi terdaftar: {}", pkg.dependencies.len());

    // 2. Serialisasi Rust struct -> JSON string (memanfaatkan crates `serde` & `serde_json`)
    println!("\n2. Serialisasi struct Rust ke format JSON:");
    let json_output = pkg.to_json().expect("Gagal serialisasi ke JSON");
    println!("{json_output}");

    // 3. Deserialisasi JSON string -> Rust struct
    println!("\n3. Deserialisasi kembali dari JSON ke struct Rust:");
    let parsed_pkg = PackageMeta::from_json(&json_output).expect("Gagal deserialisasi dari JSON");
    println!(
        "   Verifikasi kesamaan data (parsed == original): {}",
        parsed_pkg == pkg
    );
    println!("   Nama package hasil parse: {}", parsed_pkg.name);

    // 4. Penjelasan cargo tree & Cargo.lock
    println!("\n4. Mekanisme Dependency Resolution di Cargo:");
    println!("   - [dependencies]: Dikompilasi dan dimasukkan ke dalam binary rilis produksi.");
    println!(
        "   - [dev-dependencies]: Hanya dikompilasi saat `cargo test` atau `cargo bench` (zero overhead di produksi)."
    );
    println!(
        "   - `Cargo.lock`: Mengunci versi spesifik (exact pin) dan checksum SHA-256 untuk builds deterministik."
    );
    println!(
        "   - `cargo tree`: Memvisualisasikan hierarki pohon dependensi langsung dan transitif."
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    // Menggunakan dev-dependency `pretty_assertions` untuk test assertion yang kaya warna & diff
    use pretty_assertions::assert_eq;

    #[test]
    fn test_package_meta_serialization_cycle() {
        let mut pkg = PackageMeta::new("rust-learning-lab", "0.1.0", "2024");
        pkg.add_dep("serde", "1.0", DependencyKind::Normal);
        pkg.add_dep("pretty_assertions", "1.4", DependencyKind::Dev);

        let json = pkg.to_json().expect("Serialisasi gagal");
        let restored = PackageMeta::from_json(&json).expect("Deserialisasi gagal");

        // pretty_assertions::assert_eq memberikan visual diff jelas jika data berbeda
        assert_eq!(pkg, restored);
    }

    #[test]
    fn test_dependencies_filtering_kind() {
        let mut pkg = PackageMeta::new("enterprise-core", "2.0.0", "2024");
        pkg.add_dep("tokio", "1.49", DependencyKind::Normal);
        pkg.add_dep("criterion", "0.5", DependencyKind::Dev);

        let dev_deps: Vec<_> = pkg
            .dependencies
            .iter()
            .filter(|d| d.kind == DependencyKind::Dev)
            .collect();

        assert_eq!(dev_deps.len(), 1);
        assert_eq!(dev_deps[0].name, "criterion");
        assert_eq!(dev_deps[0].version_req, "0.5");
    }
}
