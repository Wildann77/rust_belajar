// Fase 5 - Task 4: Multi-Crate Workspace Architecture
// Rujukan: rust_learning_guide.md (Sub-bab 5.4) & rust_execution_tasks.md (L510-L534)

/// Representasi struktur multi-crate workspace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceCrateInfo {
    pub name: &'static str,
    pub path: &'static str,
    pub crate_type: &'static str,
    pub dependencies: &'static [&'static str],
}

pub fn get_workspace_crates() -> Vec<WorkspaceCrateInfo> {
    vec![
        WorkspaceCrateInfo {
            name: "core_domain",
            path: "crates/core_domain",
            crate_type: "library crate",
            dependencies: &["serde"],
        },
        WorkspaceCrateInfo {
            name: "database_adapter",
            path: "crates/database_adapter",
            crate_type: "library crate",
            dependencies: &["core_domain (path)", "serde"],
        },
        WorkspaceCrateInfo {
            name: "api_server",
            path: "crates/api_server",
            crate_type: "binary crate",
            dependencies: &[
                "core_domain (path)",
                "database_adapter (path)",
                "serde",
                "serde_json",
            ],
        },
    ]
}

pub fn run() {
    println!("=== Fase 5 - Task 4: Multi-Crate Workspace Architecture ===");

    println!("1. Struktur Workspace Terverifikasi (`rust-workspace/`):");
    println!("   rust-workspace/");
    println!("   ├── Cargo.toml                  <- Workspace Root [workspace]");
    println!("   └── crates/");
    println!("       ├── core_domain/            <- Library: Entitas Domain & Value Objects");
    println!("       │   ├── Cargo.toml");
    println!("       │   └── src/lib.rs");
    println!("       ├── database_adapter/       <- Library: Storage & Query Engine");
    println!("       │   ├── Cargo.toml          (depends: core_domain)");
    println!("       │   └── src/lib.rs");
    println!("       └── api_server/             <- Binary: HTTP Server & Entry Point");
    println!("           ├── Cargo.toml          (depends: core_domain, database_adapter)");
    println!("           └── src/main.rs");

    println!("\n2. Detail Crate Members:");
    for crate_info in get_workspace_crates() {
        println!(
            "   - [{}] {:<18} ({}) -> Dependensi: {:?}",
            crate_info.crate_type, crate_info.name, crate_info.path, crate_info.dependencies
        );
    }

    println!("\n3. Manfaat Arsitektural Cargo Workspace:");
    println!(
        "   - Single Cargo.lock: Seluruh sub-crate berbagi versi dependensi yang sama persis."
    );
    println!(
        "   - Shared target/ Directory: Output kompilasi hanya dibuat sekali untuk semua crate."
    );
    println!(
        "   - Workspace Inheritance: Versi package & dependencies dikelola terpusat via `[workspace.dependencies]`."
    );
    println!(
        "   - Modular Boundary: Pemisahan tegas domain logic, adapter, dan application runner."
    );
    println!(
        "   - Unified Commands: `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy` berjalan serentak."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_crate_members_configuration() {
        let crates = get_workspace_crates();
        assert_eq!(crates.len(), 3);

        assert_eq!(crates[0].name, "core_domain");
        assert_eq!(crates[1].name, "database_adapter");
        assert_eq!(crates[2].name, "api_server");

        // Verifikasi cross-crate dependency pada api_server
        assert!(crates[2].dependencies.contains(&"core_domain (path)"));
        assert!(crates[2].dependencies.contains(&"database_adapter (path)"));
    }
}
