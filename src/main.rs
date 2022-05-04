//For input and output operations
use std::io;

use rand::Rng;

fn main() {


    println!("\n\t\tGuess the number\n");

    println!("Please Guess A number : ");

    let mut guess : String = String::new();

    /*
    1) stdin allows to get the handle on the default input stream, and its coming from the terminal
    2) read_line will read the input from the terminal until `\n` and will place into the provided reference of mutable variable
    3) Expect will be called with the provided error message incase of any problems
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading the input.");


    //Generating the random number
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("Secret Number is {}", secret_number);
}
