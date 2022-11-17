use sub::*;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    assert_eq!(add(0, 1), 1);
    assert_eq!(sub(2, 1), 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(add(0, 1), 1);
    }

    #[test]
    fn two() {
        assert_eq!(add(0, 2), 2);
    }

    #[test]
    fn lib() {
        assert_eq!(sub(1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "rip")]
    fn rip() {
        panic!("rip");
    }

    #[test]
    fn res() -> Result<(), String> {
        Ok(())
    }
}
