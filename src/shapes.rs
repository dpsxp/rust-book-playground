// struct Point(u32, u32, u32);
// struct Circle(u32, u32, u32);
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn square (size: u32) -> Rectangle {
        return Rectangle { width: size, height: size };
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, shape: &Rectangle) -> bool {
        self.area() >= shape.area()
    }
}
