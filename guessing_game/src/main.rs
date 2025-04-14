use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number!");

    // Generate secret number
    let secret_number = rand::rng().random_range(1..=100);

    // Print secret number, temporary
    // println!("The secret number is : {secret_number}");

    loop {

        // Get input from user
        println!("Please input your guess.");

        // Make a mutable string variable, using the global function new
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // read_line passes in a mutable string to be set
        // expect handles incase read_line retruns Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // sets guess as an unsigned 32 bit number
        // trim removes the whitespace at the beginning and end. also removes the "\n" character
        // parse converts the trimmed string to a u32
        // since parse can fail, it returns a Result type so expect is used

        // Outputs user's guess
        // println!("You guessed: {}", guess);

        // takes in a reference to the secret number and compares it with guess to perform certain
        // print functions
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
