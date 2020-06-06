#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn extrapolation() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let l = Rectangle { width: 8, height: 7 };
        let s = Rectangle { width: 5, height: 1 };

        assert!(l.can_hold(&s));
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width > other.width && self.height > other.height
        self.width < other.width && self.height > other.height
    }
}
