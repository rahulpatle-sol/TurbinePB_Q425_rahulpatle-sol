use std::io;

fn main() {
    println!("🎯 Welcome to the Counter Program in Rust!");
    let mut x: i32 = 0;

    loop {
        println!("\nCurrent value of counter: {}", x);
        println!("Choose an option:");
        println!("1. Add to counter");
        println!("2. Subtract from counter");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter value to add:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                if let Ok(val) = input.trim().parse::<i32>() {
                    x += val;
                    println!("✅ Added {} to counter.", val);
                } else {
                    println!("⚠️ Invalid input. Please enter a number.");
                }
            }
            "2" => {
                println!("Enter value to subtract:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                if let Ok(val) = input.trim().parse::<i32>() {
                    x -= val;
                    println!("✅ Subtracted {} from counter.", val);
                } else {
                    println!("⚠️ Invalid input. Please enter a number.");
                }
            }
            "3" => {
                println!("👋 Exiting. Final counter value: {}", x);
                break;
            }
            _ => println!("⚠️ Invalid choice. Please select 1, 2, or 3."),
        }
    }
}
