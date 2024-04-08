// src/main.rs

fn main() {
    let result = add(2, 3);
    println!("The result of adding 2 and 3 is {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
