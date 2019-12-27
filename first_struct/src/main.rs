struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rec = Rectangle {
        width: 32,
        height: 32
    };

    let rec_small = Rectangle {
        width: 32,
        height: 16,
    };

    let rec_bigger = Rectangle {
        width: 64,
        height: 16,
    };

    println!("{}", rec.area());
    println!("{}", rec.can_hold(&rec_small));
    println!("{}", rec.can_hold(&rec_bigger));
}