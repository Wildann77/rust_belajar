pub fn run() {
    println!("--- Task 3: Functions ---");

    let sum = add(15, 27);
    println!("add(15, 27) = {sum}");

    let even_check = is_even(42);
    let odd_check = is_even(7);
    println!("is_even(42) = {even_check}, is_even(7) = {odd_check}");

    let prod = multiply(6, 7);
    let zero_prod = multiply(0, 99);
    println!("multiply(6, 7) = {prod}, multiply(0, 99) = {zero_prod}");
}

// Tail expression return (tanpa titik koma)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Tail expression return (evaluasi kondisi bool)
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Eksperimen keyword `return` eksplisit (early return & explicit return)
fn multiply(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0; // Early return
    }
    #[allow(clippy::needless_return)]
    return a * b; // Eksplisit return (Rust idola tail expression)
}
