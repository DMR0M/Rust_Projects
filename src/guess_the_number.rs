use std::io;
use std::cmp::Ordering;

pub fn guess_the_number(num: i32) {
    loop {
        println!("\nGuess the number: ");
        // Initialize input variable
        let mut user_guess = String::new();
        // Read input
        io::stdin().read_line(&mut user_guess)
                .expect("Failed to read line");
        // Parse string input to u32
        let user_guess: i32 = user_guess.trim().parse()
                .expect("Please type a number");
        
        match user_guess.cmp(&num) {
            Ordering::Less => println!("Number too small!"),
            Ordering::Greater => println!("Number too high"),
            Ordering::Equal => {
                println!("You guessed the correct number {}\nYOU WIN!!", num);
                break
            }
        }
    }
}
