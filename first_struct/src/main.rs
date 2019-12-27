struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle {
        width: 32,
        height: 32
    };

    println!("{}", rec.area());
}