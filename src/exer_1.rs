use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guessing Game! ");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! The secret number is {}:", secret_number),
            Ordering::Greater => println!("Too big! The secret number is {}:", secret_number),
            Ordering::Equal => {
                println!("You Guess The Right Number!");
                break;
            }
        }
    }
}
