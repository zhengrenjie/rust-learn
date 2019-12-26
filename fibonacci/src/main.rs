fn main() {
    let mut a :u128 = 0;
    let mut b :u128 = 1;
    loop {
        println!("{}", a + b);
        let c :u128 = a;
        a = b;
        b = b + c;
    }
}
