struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Прямоугольник rect1 может вместить rect2? {}", rect1.can_hold(&rect2));
    println!("Прямоугольник rect1 вмещает rect3? {}", rect1.can_hold(&rect3));
    println!("Площадь rect1 прямоугольника, {}", rect1.area());
    println!("Площадь rect2 прямоугольника, {}", rect2.area());
    println!("Площадь rect3 прямоугольника, {}", rect3.area());
}
