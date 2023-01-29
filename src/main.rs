mod prime_checker;
mod guess_the_number;

use std::io;

fn main() {
    loop {
        // Declare input variable
        println!("\nType a number to check: ");
        let mut user_num = String::new();
        // Read input
        io::stdin().read_line(&mut user_num)
                   .expect("Failed to read line");
        // Parse input to u32 int
        let user_num: u32 = user_num.trim().parse()
                   .expect("Please type a number");
        // Check if input number is prime
        if prime_checker::prime_checker(user_num as i32) {
            println!("Number {} is PRIME", user_num);
            break
        } else {
            println!("Number {} is NOT PRIME", user_num);
        }
    }
}
