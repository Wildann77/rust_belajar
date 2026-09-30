// Fase 10 - Task 2: Smart Pointer Rc<T> (Shared Ownership Single-Thread, Rc::clone, strong_count)
// Rujukan: rust_learning_guide.md (Sub-bab 10.7) & rust_execution_tasks.md (L877-L881)

use std::cell::Cell;
use std::rc::{Rc, Weak};

// ============================================================================
// 1. Struct Pelacak Siklus Hidup (Drop Tracker)
// ============================================================================

/// Struct yang mencatat saat instance di-drop dari memori heap.
///
/// Membantu membuktikan bahwa memori di heap hanya dibebaskan saat
/// `strong_count` mencapai angka 0.
#[derive(Debug)]
#[allow(dead_code)]
pub struct TrackedData {
    pub name: String,
    pub payload: Vec<i32>,
    pub drop_notifier: Option<Rc<Cell<bool>>>,
}

impl TrackedData {
    pub fn new(name: &str, payload: Vec<i32>) -> Self {
        Self {
            name: name.to_string(),
            payload,
            drop_notifier: None,
        }
    }

    pub fn with_notifier(name: &str, notifier: Rc<Cell<bool>>) -> Self {
        Self {
            name: name.to_string(),
            payload: vec![1, 2, 3],
            drop_notifier: Some(notifier),
        }
    }
}

impl Drop for TrackedData {
    fn drop(&mut self) {
        if let Some(notifier) = &self.drop_notifier {
            notifier.set(true);
        }
        // Cetak pemberitahuan drop saat instance dimusnahkan
        println!(
            "   [DROP EVENT] TrackedData '{}' dibebaskan dari heap!",
            self.name
        );
    }
}

// ============================================================================
// 2. Shared Ownership Single-Thread & strong_count Lifecycle
// ============================================================================

/// Mendemonstrasikan siklus hidup ref-count: pembuatan, pencabangan scope, dan deallokasi.
pub fn demonstrate_rc_lifecycle() -> (usize, usize, usize) {
    // 1. Buat Rc pertama: strong_count = 1
    let data_a = Rc::new(TrackedData::new("SharedConfig", vec![10, 20, 30]));
    let count_init = Rc::strong_count(&data_a);

    // 2. Clone Rc pertama: strong_count = 2 (hanya increment counter, bukan copy data!)
    let data_b = Rc::clone(&data_a);
    let count_after_b = Rc::strong_count(&data_a);

    // 3. Inner scope: strong_count naik ke 3, lalu turun lagi ke 2 saat keluar scope
    let count_inner;
    {
        let _data_c = Rc::clone(&data_b);
        count_inner = Rc::strong_count(&data_a);
        assert_eq!(count_inner, 3);
        // data_c di-drop di sini karena keluar dari inner scope
    }

    let count_after_inner = Rc::strong_count(&data_a);
    assert_eq!(count_after_inner, 2);

    (count_init, count_after_b, count_inner)
}

// ============================================================================
// 3. Struktur Data Nyata: Directed Acyclic Graph (DAG) / Multi-Parent Trees
// ============================================================================

/// Node dalam grafik terarah di mana satu simpul anak dapat dimiliki bersama
/// oleh banyak simpul orang tua (Multiple Parents).
///
/// Jika menggunakan `Box`, simpul anak harus di-copy/klon penuh.
/// Dengan `Rc`, simpul anak cukup di-share tanpa duplikasi data di heap.
#[derive(Debug)]
pub struct TreeNode {
    pub value: String,
    pub next: Option<Rc<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: &str, next: Option<Rc<TreeNode>>) -> Self {
        Self {
            value: value.to_string(),
            next,
        }
    }
}

/// Membangun grafik berbentuk Y:
/// Node C (Shared Leaf) dimiliki bersama oleh Cabang A dan Cabang B.
///
/// Branch A: Node A -> Node C
/// Branch B: Node B -> Node C
pub fn build_y_shaped_dag() -> (Rc<TreeNode>, Rc<TreeNode>, usize) {
    // 1. Buat node bersama C
    let shared_c = Rc::new(TreeNode::new("Node-C (Shared Tail)", None));
    assert_eq!(Rc::strong_count(&shared_c), 1);

    // 2. Cabang A memegang kepemilikan node C
    let branch_a = Rc::new(TreeNode::new("Node-A", Some(Rc::clone(&shared_c))));
    assert_eq!(Rc::strong_count(&shared_c), 2);

    // 3. Cabang B memegang kepemilikan node C yang sama persis
    let branch_b = Rc::new(TreeNode::new("Node-B", Some(Rc::clone(&shared_c))));
    let final_c_count = Rc::strong_count(&shared_c);
    assert_eq!(final_c_count, 3); // shared_c, branch_a.next, branch_b.next

    (branch_a, branch_b, final_c_count)
}

// ============================================================================
// 4. Mencegah Memory Leak Siklus Referensi: Weak Reference (Rc::downgrade)
// ============================================================================

/// Node pohon hierarki dengan pointer orang tua menggunakan Weak pointer.
/// Mencegah cyclic reference antara Parent dan Child.
#[allow(dead_code)]
pub struct HierarchyNode {
    pub id: u32,
    pub parent: Option<Weak<HierarchyNode>>,
}

impl HierarchyNode {
    pub fn new(id: u32, parent: Option<Weak<HierarchyNode>>) -> Self {
        Self { id, parent }
    }
}

/// Mendemonstrasikan penggunaan Weak pointer untuk referensi non-owning.
pub fn demonstrate_weak_reference() -> (usize, usize, bool) {
    let parent = Rc::new(HierarchyNode::new(1, None));
    let weak_parent = Rc::downgrade(&parent);

    let strong_count = Rc::strong_count(&parent);
    let weak_count = Rc::weak_count(&parent);

    // Upgrade weak pointer ke Option<Rc<T>>
    let is_alive = weak_parent.upgrade().is_some();

    (strong_count, weak_count, is_alive)
}

// ============================================================================
// 5. Entry Point Eksekusi Modul
// ============================================================================

pub fn run() {
    println!("=== FASE 10 TASK 2: SMART POINTER RC<T> ===");

    // 1. Siklus Hidup Rc & strong_count
    println!("\n1. Shared Ownership & strong_count Lifecycle:");
    let (c1, c2, c3) = demonstrate_rc_lifecycle();
    println!("   - strong_count awal (data_a)                  : {c1}");
    println!("   - strong_count setelah Rc::clone (data_b)     : {c2}");
    println!("   - strong_count di dalam inner scope (data_c)  : {c3}");

    // Pembuktian Drop hanya saat count == 0
    println!("\n2. Verifikasi Trait Drop saat strong_count == 0:");
    let drop_flag = Rc::new(Cell::new(false));
    {
        let root = Rc::new(TrackedData::with_notifier(
            "SessionToken",
            Rc::clone(&drop_flag),
        ));
        println!(
            "   - Instansiasi root: strong_count = {}",
            Rc::strong_count(&root)
        );
        let client_a = Rc::clone(&root);
        let client_b = Rc::clone(&root);
        println!(
            "   - Dibagikan ke 2 client: strong_count = {}",
            Rc::strong_count(&root)
        );
        assert_eq!(drop_flag.get(), false);
        // client_a, client_b, dan root di-drop di akhir scope ini
        drop(client_a);
        println!(
            "   - drop(client_a): strong_count = {}",
            Rc::strong_count(&root)
        );
        drop(client_b);
        println!(
            "   - drop(client_b): strong_count = {}",
            Rc::strong_count(&root)
        );
        println!("   - Menghancurkan root...");
    }
    println!(
        "   - Status Drop setelah semua owner selesai: {}",
        drop_flag.get()
    );
    assert_eq!(drop_flag.get(), true);

    // 3. Multi-Parent Graph (Y-Shaped DAG)
    println!("\n3. Multi-Parent Graph (Y-Shaped DAG):");
    let (branch_a, branch_b, count_c) = build_y_shaped_dag();
    println!("   - Cabang A: value = {}", branch_a.value);
    println!("   - Cabang B: value = {}", branch_b.value);
    println!("   - Node C yang dishare memiliki strong_count = {count_c}");

    // Buktikan alamat memori Node C di branch_a dan branch_b identik 100%
    let ptr_a = branch_a.next.as_ref().unwrap().as_ref() as *const TreeNode;
    let ptr_b = branch_b.next.as_ref().unwrap().as_ref() as *const TreeNode;
    println!("   - Alamat heap Node C via Branch A: {:p}", ptr_a);
    println!("   - Alamat heap Node C via Branch B: {:p}", ptr_b);
    assert_eq!(ptr_a, ptr_b);
    println!("   - Terbukti: Kedua cabang merujuk ke blok memori heap YANG SAMA!");

    // 4. Weak Reference & Memory Leak Prevention
    println!("\n4. Weak References (Rc::downgrade):");
    let (s_count, w_count, alive) = demonstrate_weak_reference();
    println!("   - strong_count = {s_count}, weak_count = {w_count}");
    println!("   - Weak pointer berhasil diupgrade: {alive}");

    println!("\n[OK] Task Fase 10 (Rc) selesai & terverifikasi.");
}

// ============================================================================
// Unit Tests Komprehensif
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_lifecycle_counts() {
        let (c1, c2, c3) = demonstrate_rc_lifecycle();
        assert_eq!(c1, 1);
        assert_eq!(c2, 2);
        assert_eq!(c3, 3);
    }

    #[test]
    fn test_rc_drop_behavior() {
        let is_dropped = Rc::new(Cell::new(false));
        let rc_data = Rc::new(TrackedData::with_notifier(
            "TestData",
            Rc::clone(&is_dropped),
        ));
        let clone_1 = Rc::clone(&rc_data);

        assert_eq!(Rc::strong_count(&rc_data), 2);
        assert!(!is_dropped.get());

        drop(clone_1);
        assert_eq!(Rc::strong_count(&rc_data), 1);
        assert!(!is_dropped.get());

        drop(rc_data);
        // Semua owner habis -> data drop
        assert!(is_dropped.get());
    }

    #[test]
    fn test_y_shaped_dag_shared_node() {
        let (branch_a, branch_b, count_c) = build_y_shaped_dag();
        assert_eq!(count_c, 3);

        let ptr_a = branch_a.next.as_ref().unwrap().as_ref() as *const TreeNode;
        let ptr_b = branch_b.next.as_ref().unwrap().as_ref() as *const TreeNode;
        assert_eq!(ptr_a, ptr_b);
    }

    #[test]
    fn test_weak_pointer_lifecycle() {
        let weak_ref;
        {
            let strong_val = Rc::new(42);
            weak_ref = Rc::downgrade(&strong_val);
            assert_eq!(Rc::weak_count(&strong_val), 1);

            // Saat strong_val masih ada, upgrade menghasilkan Some
            let upgraded = weak_ref.upgrade();
            assert!(upgraded.is_some());
            assert_eq!(*upgraded.unwrap(), 42);
        }
        // strong_val sudah keluar dari scope -> di-drop
        // Upgrade sekarang menghasilkan None
        assert!(weak_ref.upgrade().is_none());
    }
}
