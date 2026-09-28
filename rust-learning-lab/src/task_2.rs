pub fn run() {
    println!("--- Task 2: Data Types ---");

    // Signed integers
    let a: i8 = -128;
    let b: i32 = 2_147_483_647;
    let c: i64 = 9_223_372_036_854_775_807;

    // Unsigned integers
    let d: u8 = 255;
    let e: u32 = 4_294_967_295;
    let f: usize = 1024;

    // Floating-point numbers
    let g: f32 = 3.5;
    let h: f64 = 42.195;

    // Boolean
    let is_rust_fast: bool = true;

    // Character (4-byte Unicode scalar)
    let crab: char = '🦀';

    // Tuple & destructuring
    let person: (i32, f64, char) = (1, 98.5, 'A');
    let (id, score, grade) = person;

    // Array & indexing
    let numbers: [i32; 4] = [10, 20, 30, 40];
    let first = numbers[0];
    let last = numbers[3];

    println!("Signed: {a}, {b}, {c}");
    println!("Unsigned: {d}, {e}, {f}");
    println!("Floats: {g}, {h}");
    println!("Bool: {is_rust_fast}");
    println!("Char: {crab}");
    println!("Tuple: id={id}, score={score}, grade={grade}");
    println!("Array: first={first}, last={last}");
}
