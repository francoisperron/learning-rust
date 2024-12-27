use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
    println!("Guess the secret number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too big"),
            Ordering::Equal => {
                println!("{guess} was the secret number. You win 😎");
                break;
            }
        }
    }
}
