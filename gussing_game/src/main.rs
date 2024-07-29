use rand::Rng; // to get values between range  and a randon number
use std::cmp::Ordering;
use std::io; // library to take user input // compare between two variables
fn main() {
    println!("Guess The Number!");

  

    // secret number shoould be same so it is immutable
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number is: {}", secret_number);

    // in rust variables are immutable by default
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    //  An associated function is a function that’s implemented on a type, in this case String.
    // This new function creates a new, empty string. You’ll find a new function on many types because
    //  it’s a common name for a function that makes a new value of some kind.

    // let guess = String::new();

    // let str = String::from("HELLO");
    // we can also define constant string like this

    // In full, the let mut guess = String::new(); line has created a mutable variable that is currently
    //  bound to a new, empty instance of a String. Whew!
   
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // to have mutable variables

        // The stdin function returns an instance of std::io::Stdin, which is a type
        //  that represents a handle to the standard input for your terminal.
        // the line .read_line(&mut guess) calls the read_line method on the standard input
        // handle to get input from the user. We’re also passing &mut guess as the argument to read_line
        // to tell it what string to store the user input in.
        // read_line returns success or error we can handle error using expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //This line prints the string that now contains the user’s input. The {} set of
        // curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place.
        // example : let x = 5;
        // let y = 10;
        // println!("x = {x} and y + 2 = {}", y + 2);

        // convert guess from string to number
        // the old guess variable will be shadow and its owner will be removed
       
        let guess: u32 = guess.trim().parse().expect("Please provide a number!");
        println!("your gussed: {}", guess);
       

        // cmp return an enum of type Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
}
