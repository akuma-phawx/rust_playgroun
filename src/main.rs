use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number");

    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new(); //mutable

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = guess.trim().parse().expect("Please type a number");
        println!("You guessed: {guess}");
        
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You won");
                break;
            },
        }
    }
}
