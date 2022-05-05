//For input and output operations
use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {


    println!("\n\t\tGuess the number\n");

    
    let mut guess : String = String::new();
    
    //Generating the random number
    let secret_number : u32 = rand::thread_rng().gen_range(1..101);

    let mut tries = 0;
    
    loop{

        tries += 1;

        println!("Please Guess A number : ");
            /*
        1) stdin allows to get the handle on the default input stream, and its coming from the terminal
        2) read_line will read the input from the terminal until `\n` and will place into the provided reference of mutable variable
        3) Expect will be called with the provided error message incase of any problems
        */
        guess.clear();
        io::stdin()
        .read_line(&mut guess)
        .expect("Error reading the input.");

        
        //Because we will be making a comparison with a random number, which is an Integer
        //Lets parse the user input to an integer
        let guess : u32 = guess.trim().parse().expect("Unable to parse number");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;

            }
        }

        //Traditional way of branching
        // if guess > secret_number{
        //     print!("Too big\n");
        // }
        // else if guess < secret_number{
        //     print!("Too Small\n");
        // }else{
        //     print!("You have successfully guessed it\n");
        //     break;
        // }
    }

    print!("It took you {} tries to successfully guess\n", tries);
    
}
