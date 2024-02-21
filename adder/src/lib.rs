#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn hold (&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_hold_small(){
        let rect = Rectangle{
            width: 25,
            height: 10
        };
        let rect1 = Rectangle{
            width: 15,
            height: 5
        };
        assert!(rect.hold(&rect1));
    }

    #[test]
    fn small_not_hold_large() {
        let rect = Rectangle{
            width: 25,
            height: 10
        };
        let rect1 = Rectangle{
            width: 15,
            height: 5
        };
        assert!(!rect1.hold(&rect));
    }
}
