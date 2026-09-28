pub fn run() {
    println!("--- Task 1: Variables ---");
    let x = 10;
    let mut y = 20;

    y += 5;

    let name = "Rust";
    let name = name.len();

    const MAX_USERS: usize = 1000;

    println!("{x}");
    println!("{y}");
    println!("{name}");
    println!("{MAX_USERS}");
}
