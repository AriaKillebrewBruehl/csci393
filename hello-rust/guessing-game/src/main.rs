use rand::Rng;
use std::io; 
use std::cmp::Ordering; 
// bring into scope with the use statement 
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // range is start..=end, inclusive

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");


    let mut guess = String::new(); // creating a variable that is mutable
    // = means we want to bind to the variable now 
    // String::new returns new instance of a string 

    io::stdin()
        .read_line(&mut guess) // take user input and append into string passed as the agrument, references are immunatbale by default
        .expect("Failed to read line."); // read_line returns a resutl either Ok or Err, Result has a expect method, if Result is Err then expect will crash the program and display the message 

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // second guess is original, trim whitespace, then parse converts to new type 
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) { // match has options based on outcome
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
