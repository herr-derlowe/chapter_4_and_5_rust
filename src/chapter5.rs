#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_area(&self) -> u32{
        self.width*self.height
    }
    fn can_hold(&self, other_rectangle : &Rectangle) -> bool {
        //self.new_area() > other_rectangle.new_area()
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn calculate() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The shape is {:#?}", rect1);
    dbg!(&rect1);

    println!("The area is still {}", rect1.new_area());
}

pub fn check_holding(){
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

    let rect4 = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}