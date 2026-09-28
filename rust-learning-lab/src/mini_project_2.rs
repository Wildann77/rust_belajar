use std::io::{self, IsTerminal, Write};

pub fn count_bytes(s: &str) -> usize {
    s.len()
}

pub fn count_characters(s: &str) -> usize {
    s.chars().count()
}

pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn analyze_text(s: &str) -> (usize, usize, usize) {
    (count_bytes(s), count_characters(s), count_words(s))
}

fn read_line_or_default(prompt: &str, default: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => {
            println!("{default}");
            default.to_string()
        }
        Ok(_) => {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                println!("{default}");
                default.to_string()
            } else {
                if !io::stdin().is_terminal() {
                    println!("{trimmed}");
                }
                trimmed.to_string()
            }
        }
        Err(_) => {
            println!("{default}");
            default.to_string()
        }
    }
}

pub fn run() {
    println!("=== Mini Project Fase 2: Text Analyzer ===");

    let input = read_line_or_default("Masukkan teks: ", "Belajar Rust 🦀 sangat menyenangkan!");
    let (bytes, characters, words) = analyze_text(&input);

    println!("Bytes       : {bytes}");
    println!("Characters  : {characters}");
    println!("Words       : {words}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes("hello"), 5);
        assert_eq!(count_bytes("Rust 🦀"), 9); // '🦀' is 4 bytes
        assert_eq!(count_bytes(""), 0);
    }

    #[test]
    fn test_count_characters() {
        assert_eq!(count_characters("hello"), 5);
        assert_eq!(count_characters("Rust 🦀"), 6); // 5 ASCII + 1 emoji
        assert_eq!(count_characters(""), 0);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Belajar bahasa pemrograman Rust"), 4);
        assert_eq!(count_words("   spasi   banyak   di   sini   "), 4);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("Rust 🦀 mantap"), 3);
    }

    #[test]
    fn test_analyze_text() {
        let stats = analyze_text("Rust 🦀");
        assert_eq!(stats, (9, 6, 2));
    }
}
