pub fn run() {
    println!("--- Task 4: Expression vs Statement ---");

    // 1. Statement: deklarasi variabel (tidak mengembalikan nilai)
    let base = 10;

    // 2. Expression Block: block `{ ... }` dievaluasi dan menghasilkan nilai
    let calculated = {
        let multiplier = 4;
        let bonus = 2;
        // Tail expression (tanpa `;`): nilai ini di-return dari block
        (base * multiplier) + bonus
    };

    println!("Base: {base}");
    println!("Calculated via block expression: {calculated}");

    let result = add_expression(20, 22);
    println!("add_expression(20, 22): {result}");

    // Simulasi & dokumentasi error jika ditambah `;`
    explain_semicolon_behavior();
}

// Return expression tanpa `;` -> menghasilkan i32
fn add_expression(a: i32, b: i32) -> i32 {
    a + b // Expression: mengembalikan 42
}

/*
// Eksperimen Error: Menambahkan `;` pada baris return
fn add_with_semicolon_error(a: i32, b: i32) -> i32 {
    a + b;
    // ^^^ COMPILE ERROR [E0308]: mismatched types
    // expected `i32`, found `()`
    // help: remove this semicolon to return this value
}
*/

fn explain_semicolon_behavior() {
    // Block `{ ... }` diakhiri semicolon -> menghasilkan unit `()`
    let unit_val: () = {
        let _z = 100;
        // Statement dengan `;` -> membuang nilai, hasil akhir ()
    };
    println!("Block berakhiran statement menghasilkan unit type: {unit_val:?}");
}
