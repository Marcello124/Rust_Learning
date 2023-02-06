use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Pleas input your guess");
    
    let mut tries = 0;    
    loop {
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a number".red());
                continue;
            },
        };

        println!("You guessed {guess}");

        tries = tries + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Number od tries: {tries}");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
        }
    }
}