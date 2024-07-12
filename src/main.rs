use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    println!("{}", "Guess the number!".blue().bold());
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess it should be a number between 1 and 100:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect(format!("{}", "Failed to read the guess".red()).as_str());

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > 100 {
            continue;
        }

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Too small!".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
}
