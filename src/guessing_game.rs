use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println!(msg) - macro for printing out 
    // to standard output 
    println!("Guess the number!");
    
    // generating random number with "Rng" from rand lib
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // infinite loop
    loop {
        println!("Please input your guess:");

        // string mutable variable - call new() on string type
        let mut guess = String::new();

        // standard input (passing address as argument to readline())
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        // cast string to u32 using parse()
        // using match { } to do different function
        // depends on parse() returned enum value
        // Ok - parsed
        // Err - type can't be parsed 
        let guess: u32 = match guess.trim().parse() {
            // return parsed number
            Ok(num) => num,
            // if it's error return message and continue loop
            Err(_) => {
                println!("Input must be a number. Try again!");
                continue;
            },
        };
        
        // compare guessed number with random number
        // using std::cmp::Ordering to compare
        // if cmp() returns Ordering::Equal beak the loop
        // else print message
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
