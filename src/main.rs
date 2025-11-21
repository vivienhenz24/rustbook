struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn size_check(&self, other: &Rectangle) -> bool {

        self.width > other.width && self.height > other.height

    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 2, height: 80};
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can it hold within the other rectangle: {}", rect1.size_check(&rect2));
}