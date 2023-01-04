#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        let area1 = self.area();
        let area2 = rectangle.area();

        return area1 > area2;
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

pub fn test_method() {

    let rect1 = Rectangle {
        width: 20,
        height:30
    };

    println!(
        "The area of the rectangle is {} sqare pixels", rect1.area()
    )
}

pub fn test_can_hold() {

    let rect1 = Rectangle {
        width: 20,
        height:30
    };

    let rect2 = Rectangle {
        width: 30,
        height:40
    };

    let rect3 = Rectangle {
        width: 10,
        height:30
    };

    println!("Can rect1 hodl rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hodl rect3? {}", rect1.can_hold(&rect3));
}