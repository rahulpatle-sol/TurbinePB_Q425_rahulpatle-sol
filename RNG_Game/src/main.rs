use rand::Rng;
use std::io;

fn main() {
    println!("🎯 Welcome to the Random Number Guess Game!");
    println!("I'm thinking of a number between 1 and 100...");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("🔢 Enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        if guess < secret {
            println!("📉 Too low!");
        } else if guess > secret {
            println!("📈 Too high!");
        } else {
            println!("🎉 Bingo! You guessed it right: {}", secret);
            break;
        }
    }
}
