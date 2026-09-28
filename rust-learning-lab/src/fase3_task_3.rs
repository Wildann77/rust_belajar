// Fase 3 - Task 3: Pattern Matching (match, exhaustive, destructuring, match guard, if let, while let, range, binding @)
// Rujukan: rust_learning_guide.md (Bagian 3.3)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Moderator,
    Member(u32), // Membawa level reputasi (1..=100)
    Guest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    Quit,
    MoveTo(Coordinate),
    SendMessage { sender: String, content: String },
    SetVolume(u8), // Volume suara 0..=100
}

// ==========================================
// 0. Contoh Dasar (Basic Match Fundamental)
// ==========================================
pub fn demo_basic_match(dice: u8) -> &'static str {
    match dice {
        1 => "Satu (Paling Rendah)",
        2 | 3 => "Dua atau Tiga (Rendah)",
        4 | 5 => "Empat atau Lima (Sedang)",
        6 => "Enam (Tertinggi)",
        _ => "Bukan angka dadu standar", // Wildcard catch-all
    }
}

// ==========================================
// 1. Exhaustive Match, Destructuring, Range Pattern, Binding @, & Match Guard
// ==========================================
pub fn process_command(cmd: &AppCommand) -> String {
    match cmd {
        // 1a. Match varian sederhana
        AppCommand::Quit => "Aplikasi ditutup (Quit).".to_string(),

        // 1b. Destructuring struct di dalam varian & Binding @ pada range
        AppCommand::MoveTo(Coordinate { x: x @ 0..=50, y }) => {
            format!("Berpindah ke area aman pojok kiri: x={x}, y={y}")
        }
        AppCommand::MoveTo(Coordinate { x, y }) => {
            format!("Berpindah ke koordinat target: x={x}, y={y}")
        }

        // 1c. Destructuring field bernama (struct-like) & Match Guard (if condition)
        AppCommand::SendMessage { sender, content } if content.trim().is_empty() => {
            format!("Pesan kosong dari '{sender}' diabaikan.")
        }
        AppCommand::SendMessage { sender, content } => {
            format!("Pesan dari {sender}: \"{content}\"")
        }

        // 1d. Range Pattern & Binding @ untuk mengikat nilai yang lolos filter range
        AppCommand::SetVolume(vol @ 0..=30) => {
            format!("Volume diatur rendah: {vol}%")
        }
        AppCommand::SetVolume(vol @ 31..=70) => {
            format!("Volume diatur sedang: {vol}%")
        }
        AppCommand::SetVolume(vol @ 71..=100) => {
            format!("Volume diatur tinggi: {vol}% (Peringatan pendengaran!)")
        }
        // Wildcard / Fallback untuk memastikan exhaustive matching jika ada u8 > 100
        AppCommand::SetVolume(vol) => {
            format!("Volume {vol}% melebihi batas aman 100%!")
        }
    }
}

// ==========================================
// 2. Evaluasi Role dengan Match Guard & Binding @
// ==========================================
pub fn classify_role(role: &UserRole) -> String {
    match role {
        UserRole::Admin => "Akses Penuh (Super Admin)".to_string(),
        UserRole::Moderator => "Akses Moderasi Konten".to_string(),

        // Nilai pasti (literal pattern)
        UserRole::Member(0) => "Member Belum Terverifikasi (Level 0)".to_string(),

        // Range Pattern dengan Binding @:
        UserRole::Member(level @ 1..=20) => {
            format!("Member Pemula (Level {level})")
        }
        UserRole::Member(level @ 21..=70) => {
            format!("Member Aktif (Level {level})")
        }
        // Match Guard dengan kondisi boolean runtime
        // Catatan: Compiler Rust tidak menganggap arm ber-guard exhaustive,
        // sehingga arm penutup/fallback di bawahnya wajib ada!
        UserRole::Member(level) if *level > 70 => {
            format!("Member Veteran / Elit (Level {level} - Hak Voting)")
        }
        UserRole::Member(level) => {
            format!("Member Khusus (Level {level})")
        }

        UserRole::Guest => "Akses Tamu (Read-only)".to_string(),
    }
}

// ==========================================
// 3. Range Pattern Mandiri
// ==========================================
pub fn classify_grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A (Istimewa)",
        80..=89 => "B (Baik)",
        70..=79 => "C (Cukup)",
        0..=69 => "D (Perlu Perbaikan)",
        _ => "Skor Tidak Valid (> 100)",
    }
}

// ==========================================
// 4. `if let` (Pencocokan Cepat 1 Pola)
// ==========================================
pub fn inspect_move(cmd: &AppCommand) -> Option<(i32, i32)> {
    if let AppCommand::MoveTo(Coordinate { x, y }) = cmd {
        Some((*x, *y))
    } else {
        None
    }
}

// ==========================================
// 5. `while let` (Looping Selama Pola Masih Cocok)
// ==========================================
pub fn process_queue(queue: &mut Vec<AppCommand>) -> Vec<String> {
    let mut logs = Vec::new();
    while let Some(cmd) = queue.pop() {
        logs.push(process_command(&cmd));
    }
    logs
}

// ==========================================
// Runner Function
// ==========================================
pub fn run() {
    println!("=== Fase 3 - Task 3: Pattern Matching ===");

    // 0. Demo Basic Match (Fundamental)
    println!("0. Basic Match (Evaluasi Angka Dadu):");
    println!("   Dadu 1: {}", demo_basic_match(1));
    println!("   Dadu 3: {}", demo_basic_match(3));
    println!("   Dadu 6: {}", demo_basic_match(6));
    println!("   Dadu 9: {}", demo_basic_match(9));

    // 1. Match, Destructuring, Range & Binding @
    let cmd1 = AppCommand::MoveTo(Coordinate { x: 25, y: 80 });
    let cmd2 = AppCommand::MoveTo(Coordinate { x: 120, y: 40 });
    let cmd3 = AppCommand::SendMessage {
        sender: "boyblanco".to_string(),
        content: "Halo Rust 2024!".to_string(),
    };
    let cmd4 = AppCommand::SendMessage {
        sender: "spammer".to_string(),
        content: "   ".to_string(), // Match guard (empty)
    };
    let cmd5 = AppCommand::SetVolume(25);
    let cmd6 = AppCommand::SetVolume(90);

    println!("1. Hasil process_command:");
    println!("   cmd1: {}", process_command(&cmd1));
    println!("   cmd2: {}", process_command(&cmd2));
    println!("   cmd3: {}", process_command(&cmd3));
    println!("   cmd4: {}", process_command(&cmd4));
    println!("   cmd5: {}", process_command(&cmd5));
    println!("   cmd6: {}", process_command(&cmd6));

    // 2. Evaluasi Role (Binding @ & Guard)
    println!("2. Hasil classify_role:");
    println!("   Admin    : {}", classify_role(&UserRole::Admin));
    println!("   Moderator: {}", classify_role(&UserRole::Moderator));
    println!("   Member 5 : {}", classify_role(&UserRole::Member(5)));
    println!("   Member 85: {}", classify_role(&UserRole::Member(85)));
    println!("   Guest   : {}", classify_role(&UserRole::Guest));

    // 3. Range Pattern
    println!("3. Hasil classify_grade:");
    println!("   Score 95: {}", classify_grade(95));
    println!("   Score 73: {}", classify_grade(73));

    // 4. `if let` Demo
    println!("4. Hasil `if let` inspect_move:");
    if let Some((x, y)) = inspect_move(&cmd1) {
        println!("   `if let` mendeteksi koordinat: ({x}, {y})");
    }

    // 5. `while let` Demo
    let mut task_queue = vec![
        AppCommand::SetVolume(15),
        AppCommand::SendMessage {
            sender: "system".to_string(),
            content: "Ready".to_string(),
        },
        AppCommand::Quit,
    ];
    println!("5. Hasil `while let` process_queue:");
    let processed_logs = process_queue(&mut task_queue);
    for (i, log) in processed_logs.iter().enumerate() {
        println!("   [{i}] {log}");
    }
}

// ==========================================
// Unit Tests
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_basic_match() {
        assert_eq!(demo_basic_match(1), "Satu (Paling Rendah)");
        assert_eq!(demo_basic_match(2), "Dua atau Tiga (Rendah)");
        assert_eq!(demo_basic_match(3), "Dua atau Tiga (Rendah)");
        assert_eq!(demo_basic_match(5), "Empat atau Lima (Sedang)");
        assert_eq!(demo_basic_match(6), "Enam (Tertinggi)");
        assert_eq!(demo_basic_match(0), "Bukan angka dadu standar");
        assert_eq!(demo_basic_match(99), "Bukan angka dadu standar");
    }

    #[test]
    fn test_process_command_quit_and_move() {
        assert_eq!(process_command(&AppCommand::Quit), "Aplikasi ditutup (Quit).");

        let safe_move = AppCommand::MoveTo(Coordinate { x: 30, y: 10 });
        assert!(process_command(&safe_move).contains("area aman pojok kiri"));

        let normal_move = AppCommand::MoveTo(Coordinate { x: 80, y: 50 });
        assert!(process_command(&normal_move).contains("koordinat target"));
    }

    #[test]
    fn test_process_command_messages_with_guard() {
        let valid_msg = AppCommand::SendMessage {
            sender: "alice".to_string(),
            content: "Hello".to_string(),
        };
        assert!(process_command(&valid_msg).contains("Pesan dari alice"));

        let blank_msg = AppCommand::SendMessage {
            sender: "bob".to_string(),
            content: "   ".to_string(),
        };
        assert!(process_command(&blank_msg).contains("diabaikan"));
    }

    #[test]
    fn test_process_command_volume_binding_ranges() {
        assert!(process_command(&AppCommand::SetVolume(20)).contains("rendah"));
        assert!(process_command(&AppCommand::SetVolume(50)).contains("sedang"));
        assert!(process_command(&AppCommand::SetVolume(90)).contains("tinggi"));
        assert!(process_command(&AppCommand::SetVolume(150)).contains("melebihi batas"));
    }

    #[test]
    fn test_classify_role_levels_and_guards() {
        assert_eq!(classify_role(&UserRole::Admin), "Akses Penuh (Super Admin)");
        assert_eq!(classify_role(&UserRole::Moderator), "Akses Moderasi Konten");
        assert!(classify_role(&UserRole::Member(10)).contains("Pemula"));
        assert!(classify_role(&UserRole::Member(40)).contains("Aktif"));
        assert!(classify_role(&UserRole::Member(95)).contains("Veteran"));
        assert!(classify_role(&UserRole::Member(0)).contains("Belum Terverifikasi"));
        assert_eq!(classify_role(&UserRole::Guest), "Akses Tamu (Read-only)");
    }

    #[test]
    fn test_classify_grade_ranges() {
        assert_eq!(classify_grade(100), "A (Istimewa)");
        assert_eq!(classify_grade(85), "B (Baik)");
        assert_eq!(classify_grade(75), "C (Cukup)");
        assert_eq!(classify_grade(50), "D (Perlu Perbaikan)");
        assert_eq!(classify_grade(150), "Skor Tidak Valid (> 100)");
    }

    #[test]
    fn test_if_let_inspect_move() {
        let move_cmd = AppCommand::MoveTo(Coordinate { x: 10, y: 20 });
        assert_eq!(inspect_move(&move_cmd), Some((10, 20)));

        let quit_cmd = AppCommand::Quit;
        assert_eq!(inspect_move(&quit_cmd), None);
    }

    #[test]
    fn test_while_let_process_queue() {
        let mut queue = vec![
            AppCommand::Quit,
            AppCommand::SetVolume(10),
        ];
        let logs = process_queue(&mut queue);
        assert_eq!(logs.len(), 2);
        assert!(queue.is_empty());
    }
}
