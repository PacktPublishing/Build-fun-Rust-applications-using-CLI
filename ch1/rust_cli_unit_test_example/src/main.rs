// src/main.rs

fn main() {
    let result = concat_strings("Hello, ", "world!");
    println!("{}", result);
}

fn concat_strings(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_strings() {
        assert_eq!(concat_strings("Rust ", "is awesome"), "Rust is awesome");
    }
}
