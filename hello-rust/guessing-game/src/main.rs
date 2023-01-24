use std::io; // obtain user input / output 
// bring into scope with the use statement 
fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new(); // creating a variable that is mutable
    // = means we want to bind to the variable now 
    // String::new returns new instance of a string 

    io::stdin()
        .read_line(&mut guess) // take user input and append into string passed as the agrument, references are immunatbale by default
        .expect("Failed to read line"); // read_line returns a resutl either Ok or Err, Result has a expect method, if Result is Err then expect will crash the program and display the message 

    println!("You guessed: {guess}");

}
