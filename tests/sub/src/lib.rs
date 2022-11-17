pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(sub(2, 1), 1);
    }

    #[test]
    fn two() {
        assert_eq!(sub(3, 1), 2);
    }
}
