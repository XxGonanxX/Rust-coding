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
    
    loop{

        println!("Please, input your guess: ");

        // let statement creates the variable, example:
        // let cars = 5;

        // mut makes the variable mutable, example:
        // let mut cars = 5;

        let mut guess = String::new();

        // ::new() is an associated function of the String type
        // it implies that there's a new isntance of a String.

        io::stdin()

            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {

            Ok(num) => num,
            Err(_) => { println!("Please input a number!");
                continue;
            }

        };

            // trim() method eliminates any whitespace at the beginning
            // and end of the string. parse() method parses a string into
            // some kind of number. We need to tell Rust the type of the
            // number we expect, in this case, u32.

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!");
                                    break;
            }

        }

    }

}

