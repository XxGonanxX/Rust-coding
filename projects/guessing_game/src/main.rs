use std::io;
use rand::Rng;
use std::cmp::Ordering;

// ::Rng trait defines methods that random number generators implement.
// It's easier if we just leave it like that for now. We'll talk about
// it later. 

fn main() {

    println!("Hello, welcome to 'Guess the number'!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // thread_rng() function gives us the particular random number
    // generator that we're going to use, thje gen.range() method
    // allows us to define the range of the random number.

    println!("The secret number is: {secret_number}");
    
    println!("Please, input your guess: ");

    // let statement creates the variable, example:
    // let cars = 5;

    // mut makes the variable mutable, example:
    // let mut cars = 5;

    let mut guess = String::new();

    // ::new() is an associated function of the String type
    // it implies that there's a new isntance of a String.

    io::stdin()

        // stdin() function returns an instance of std::io::Stdin
        //which, in escense. is just a input handle for the terminal.

        .read_line(&mut guess)

        // read_line() method is to get the input of the user. The &mut
        // indicates that this argument mtuable and that it will be
        //stored in the variable guess.

        .expect("Failed to read line");

        // expect() method is to handle the Result type. If the Result
        // is an Err value, expect() will cause the program to crash
        // and display the message that you passed as an argument to
        // expect().

    println!("You guessed: {guess}");

    // {} is a placeholder for the value of the variable guess, but youc
    // can also use {} to print multiple values, example: 
    // println!("{}-{}-{}", 1, 2, 3);

    let mut guess = String::new();

    io::stdin()

        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()

        // trim() method eliminates any whitespace at the beginning
        // and end of the string. parse() method parses a string into
        // some kind of number. We need to tell Rust the type of the
        // number we expect, in this case, u32.

        .expect("Please type a number!");


    match guess.cmp(&secret_number) {

        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),

    }

}
