// Method are functions, but defined within a struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// this is known as an IMPLEMENTATION
// would have to use &mut self if changing something

// you *can* have multiple implementation blocks of the same struct, but they just
// add onto each other, so it doesn't really matter either way
impl Rectangle {
    // associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
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

    // :: means it's associated with the implementation, and not an individual instance
    // kinda like can be both static and non-static in Java terms
    // you could call it associatively, but would have to pass in the instance as a parameter for &self
    let square1 = Rectangle::square(40);

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );

}
