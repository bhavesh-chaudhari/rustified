// source: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
// bring Ordering(enum) into scope from standard lib
use std::cmp::Ordering; // variants are Less, Greater and Equal
// include rand crate after downloading it
use rand::Rng;

// main function is the entry point into the program
// fn declares a new function
fn main(){
    // println! is a macro and not a function
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); 
    // gen_range takes expression of the form (start..end), start inclusive;end exclusive.
    // (1..101) is similar to (1..=100)

    // println!("The secret number is: {}", secret_number);

   loop{
        println!("Please input your guess.");

        // in rust, variables are immutable by default, use mut keyword to
        // make them mutable
        let mut guess = String::new(); 
        // ::new indicates that new is an associated function of the String type
        // An associated function is implemented on a type(e.g. String in this case)
        // The new function creates a new empty string

        io::stdin() // standard input handle for terminal
            .read_line(&mut guess) 
        // call read_line method on standard input handle to get 
        // input from the user(referenced to mut guess variable)
        // like variables, references are immutable by default(hence we used mut when referencing)
        // returns an enum io::Result(variants are Ok(success) or Err(failure))
            .expect("Failed to read line");
        // display the message given as argument to expect when program has failed(handle possible error)
        // the program will compile without expect but will give a warning

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue, // underscore is a catchall value
        };
        // when we use match expression, the type of the variables 
        // to compare must be same(rust has type inference),
        // so we convert guess(String) to a number
        // Rust defaults number to i32(32-bit number)
        // method trim removes whitespaces and parse parses a string into a number.
        // parse method can parse variety of number types so we need to tell rust
        // about the exact number type we want(u32 in this case)

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            // arms and their patterns 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        // compare guess with secret number 
        // cmp method returns an Ordering between self and other(e.g. Ordering::Greater)
        // The match expression here gets a certain Ordering and then starts checking each arm's pattern(stops when pattern found)
   }
}