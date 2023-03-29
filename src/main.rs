pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_with_negative_number() {
        assert_eq!(add(-1, -2), -3);
    }
}

fn main() -> i32 {
    add(5, 5)
}