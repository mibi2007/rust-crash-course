pub mod addition {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

pub mod subtraction {
    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let result = addition::add(3, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_sub() {
        let result = subtraction::sub(3, 2);
        assert_eq!(result, 1);
    }
}
