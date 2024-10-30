fn main() {
    println!("Hello, world!");
    println!("The sum of 5 and 6 is {}", add(5, 6));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(5, 6), 11);
    }
}