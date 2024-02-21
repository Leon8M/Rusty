#[derive(Debug)]

struct Rectangle{
    length: i32,
    width: i32,
}

impl Rectangle {
    fn new(&self) -> Rectangle{
        Rectangle { length: 5, width: 6 }
    }
}

fn main() {
    let x = Rectangle::new(&Rectangle { length: 8, width: 9 });

    println!("{:?}", x)
}
