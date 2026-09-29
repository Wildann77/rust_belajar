// Fase 6 - Task 4: Collections — Vec<T> & HashMap Entry API
// Rujukan: rust_learning_guide.md (Sub-bab 6.4) & rust_execution_tasks.md (L594-L605)

use std::collections::HashMap;

/// Representasi data produk untuk manajemen inventaris
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub stock: u32,
    pub price: f64,
}

// ----------------------------------------------------------------------------
// 1. Vector Deep Dive: Vec<T>, with_capacity, push, get, retain
// ----------------------------------------------------------------------------

/// Membuat vector produk dengan alokasi kapasitas awal di memori heap.
/// Menghindari overhead re-alokasi berulang saat elemen ditambahkan secara masif.
pub fn init_inventory_with_capacity(capacity: usize) -> Vec<Product> {
    Vec::with_capacity(capacity)
}

/// Menambahkan elemen baru ke dalam Vector via .push()
pub fn add_product(inventory: &mut Vec<Product>, product: Product) {
    inventory.push(product);
}

/// Mengakses elemen vector secara aman via .get(index)
/// Mengembalikan Option<&Product> (Some jika ada, None jika di luar batas/out-of-bounds).
/// Mencegah fatal crash runtime (panic) yang disebabkan oleh akses index langsung `inventory[i]`.
pub fn get_product_safely(inventory: &[Product], index: usize) -> Option<&Product> {
    inventory.get(index)
}

/// Menyaring elemen vector secara in-place via .retain(predicate)
/// Menghapus semua elemen yang tidak memenuhi kondisi tanpa mengalokasikan vector baru.
pub fn filter_in_stock_only(inventory: &mut Vec<Product>) {
    inventory.retain(|p| p.stock > 0);
}

// ----------------------------------------------------------------------------
// 2. HashMap Deep Dive: HashMap, entry, or_insert, and_modify
// ----------------------------------------------------------------------------

/// Mengagregasi total stok per kategori menggunakan HashMap Entry API.
/// Menghindari double-lookup hashing dengan satu kali operasi terpadu:
/// - .entry(key) : Mengambil pointer posisi hash bucket.
/// - .and_modify(|val| ...) : Memodifikasi nilai yang sudah ada secara in-place.
/// - .or_insert(default) : Menyisipkan nilai awal jika key belum pernah ada.
pub fn calculate_stock_by_category(inventory: &[Product]) -> HashMap<String, u32> {
    let mut category_map: HashMap<String, u32> = HashMap::new();

    for product in inventory {
        category_map
            .entry(product.category.clone())
            .and_modify(|total| *total += product.stock)
            .or_insert(product.stock);
    }

    category_map
}

/// Menghitung frekuensi kemunculan log/event kata via Entry API
pub fn count_log_events(events: &[&str]) -> HashMap<String, usize> {
    let mut stats: HashMap<String, usize> = HashMap::new();

    for &event in events {
        stats
            .entry(event.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    stats
}

// ----------------------------------------------------------------------------
// Runner Demonstrasi
// ----------------------------------------------------------------------------

pub fn run() {
    println!("=== Fase 6 - Task 4: Collections — Vec<T> & HashMap Entry API ===");
    println!("Konsep Inti: Alokasi efisien with_capacity, safe access get, retain in-place, & Entry API\n");

    // 1. Vec::with_capacity & push
    println!("1. Demonstrasi Vec::with_capacity & push:");
    let mut inventory = init_inventory_with_capacity(5);
    println!("   - Initial State: len = {}, capacity = {}", inventory.len(), inventory.capacity());

    add_product(&mut inventory, Product {
        id: 1,
        name: "Mechanical Keyboard".to_string(),
        category: "Electronics".to_string(),
        stock: 15,
        price: 120.0,
    });
    add_product(&mut inventory, Product {
        id: 2,
        name: "Gaming Mouse".to_string(),
        category: "Electronics".to_string(),
        stock: 0, // Habis
        price: 60.0,
    });
    add_product(&mut inventory, Product {
        id: 3,
        name: "Ergonomic Chair".to_string(),
        category: "Furniture".to_string(),
        stock: 5,
        price: 350.0,
    });
    add_product(&mut inventory, Product {
        id: 4,
        name: "Desk Lamp".to_string(),
        category: "Furniture".to_string(),
        stock: 0, // Habis
        price: 45.0,
    });
    add_product(&mut inventory, Product {
        id: 5,
        name: "USB-C Hub".to_string(),
        category: "Electronics".to_string(),
        stock: 25,
        price: 35.0,
    });

    println!("   - After 5 Push : len = {}, capacity = {}", inventory.len(), inventory.capacity());

    // 2. Safe access via .get()
    println!("\n2. Akses Aman Index via .get():");
    let valid_item = get_product_safely(&inventory, 0);
    let invalid_item = get_product_safely(&inventory, 99);
    println!("   - Index 0  : {:?}", valid_item.map(|p| &p.name));
    println!("   - Index 99 : {:?}", invalid_item.map(|p| &p.name));

    // 3. In-place filtering via .retain()
    println!("\n3. In-Place Filtering via .retain() (Membuang stock == 0):");
    println!("   - Jumlah produk sebelum retain: {}", inventory.len());
    filter_in_stock_only(&mut inventory);
    println!("   - Jumlah produk setelah retain : {}", inventory.len());
    for p in &inventory {
        println!("     * [ID: {}] {} (Stock: {})", p.id, p.name, p.stock);
    }

    // 4. HashMap Entry API: entry, and_modify, or_insert
    println!("\n4. Agregasi Data via HashMap Entry API (.entry().and_modify().or_insert()):");
    let category_stock = calculate_stock_by_category(&inventory);
    for (category, total_stock) in &category_stock {
        println!("   - Kategori '{category}': Total Stok = {total_stock} unit");
    }

    // 5. Analisis Frekuensi Event Log
    println!("\n5. Frekuensi Event Log via Entry API:");
    let logs = ["LOGIN", "CLICK", "LOGOUT", "LOGIN", "VIEW", "LOGIN", "VIEW"];
    let event_counts = count_log_events(&logs);
    for (event, count) in &event_counts {
        println!("   - Event '{event}': {count} kali");
    }
}

// ----------------------------------------------------------------------------
// Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_with_capacity_and_push() {
        let mut vec = init_inventory_with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert!(vec.capacity() >= 10);

        add_product(&mut vec, Product {
            id: 1,
            name: "Mouse".to_string(),
            category: "Electronics".to_string(),
            stock: 10,
            price: 50.0,
        });

        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0].name, "Mouse");
    }

    #[test]
    fn test_vec_safe_get() {
        let mut vec = Vec::new();
        add_product(&mut vec, Product {
            id: 10,
            name: "Monitor".to_string(),
            category: "Electronics".to_string(),
            stock: 2,
            price: 200.0,
        });

        assert!(get_product_safely(&vec, 0).is_some());
        assert_eq!(get_product_safely(&vec, 0).unwrap().id, 10);

        // Safe out-of-bounds (tidak panic crash)
        assert!(get_product_safely(&vec, 1).is_none());
        assert!(get_product_safely(&vec, 100).is_none());
    }

    #[test]
    fn test_vec_retain_in_place() {
        let mut vec = vec![
            Product { id: 1, name: "A".to_string(), category: "C1".to_string(), stock: 5, price: 10.0 },
            Product { id: 2, name: "B".to_string(), category: "C1".to_string(), stock: 0, price: 10.0 },
            Product { id: 3, name: "C".to_string(), category: "C2".to_string(), stock: 12, price: 10.0 },
        ];

        filter_in_stock_only(&mut vec);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].name, "A");
        assert_eq!(vec[1].name, "C");
    }

    #[test]
    fn test_hashmap_entry_api_aggregation() {
        let inventory = vec![
            Product { id: 1, name: "KB".to_string(), category: "Tech".to_string(), stock: 10, price: 100.0 },
            Product { id: 2, name: "Mouse".to_string(), category: "Tech".to_string(), stock: 20, price: 50.0 },
            Product { id: 3, name: "Table".to_string(), category: "Furniture".to_string(), stock: 5, price: 200.0 },
        ];

        let agg = calculate_stock_by_category(&inventory);

        assert_eq!(agg.get("Tech"), Some(&30));
        assert_eq!(agg.get("Furniture"), Some(&5));
        assert_eq!(agg.get("Food"), None);
    }

    #[test]
    fn test_hashmap_log_event_counts() {
        let raw_events = ["WARN", "INFO", "WARN", "ERROR", "WARN"];
        let counts = count_log_events(&raw_events);

        assert_eq!(counts.get("WARN"), Some(&3));
        assert_eq!(counts.get("INFO"), Some(&1));
        assert_eq!(counts.get("ERROR"), Some(&1));
        assert_eq!(counts.get("DEBUG"), None);
    }
}
