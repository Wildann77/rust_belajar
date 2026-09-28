use std::io::{self, IsTerminal, Write};

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Pembagian dengan nol tidak valid"))
    } else {
        Ok(a / b)
    }
}

pub fn parse_number(input: &str) -> Result<f64, String> {
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Input '{input}' bukan angka yang valid"))
}

pub fn parse_operator(input: &str) -> Result<char, String> {
    let trimmed = input.trim();
    if trimmed.len() == 1 {
        let ch = trimmed.chars().next().unwrap();
        if matches!(ch, '+' | '-' | '*' | '/') {
            return Ok(ch);
        }
    }
    Err(format!("Operator '{trimmed}' tidak valid (+, -, *, /)"))
}

pub fn calculate(a: f64, op: char, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(add(a, b)),
        '-' => Ok(subtract(a, b)),
        '*' => Ok(multiply(a, b)),
        '/' => divide(a, b),
        _ => Err(format!("Operator '{op}' tidak didukung")),
    }
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
    println!("=== Rust Calculator ===");

    let raw_num1 = read_line_or_default("Input angka 1: ", "10");
    let num1 = match parse_number(&raw_num1) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    let raw_op = read_line_or_default("Input operator (+ - * /): ", "+");
    let op = match parse_operator(&raw_op) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    let raw_num2 = read_line_or_default("Input angka 2: ", "5");
    let num2 = match parse_number(&raw_num2) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    match calculate(num1, op, num2) {
        Ok(result) => println!("Hasil: {result}"),
        Err(err) => println!("Hasil: Error ({err})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10.0, 5.0), 15.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10.0, 5.0), 5.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(10.0, 5.0), 50.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(10.0, '+', 5.0).unwrap(), 15.0);
        assert_eq!(calculate(10.0, '-', 5.0).unwrap(), 5.0);
        assert_eq!(calculate(10.0, '*', 5.0).unwrap(), 50.0);
        assert_eq!(calculate(10.0, '/', 5.0).unwrap(), 2.0);
        assert!(calculate(10.0, 'x', 5.0).is_err());
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number(" 42.5 ").unwrap(), 42.5);
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_parse_operator() {
        assert_eq!(parse_operator(" + ").unwrap(), '+');
        assert!(parse_operator("%").is_err());
    }
}
