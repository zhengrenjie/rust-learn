use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess!");
    let secret_num = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("Please input: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }

    // println!("You guessed: {}", guess);
}
