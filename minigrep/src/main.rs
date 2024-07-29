use std::env;
use std::process; // help us to exit from progrma without panic

use minigrep::Config;    // minigrep is the name of our library crate

// use std::{env, fs, process};   we can also write like this

// this is a binary crate

// overview:
//Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language
// for creating command line tools(CLI), so for our project, we’ll make our own version of the classic command
// line search tool grep (globally search a regular expression and print). In the simplest use case,
// grep searches a specified file for a specified string. To do so, grep takes as its arguments a file path and a string.
// Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

// The first task is to make minigrep accept its two command line arguments:
// a string (path to a file) and a file name

// That is, we want to be able to run our program with cargo run, two hyphens to indicate
// the following arguments are for our program rather than for cargo, a string to search for,
//  and a path to a file to search in, like so:
// cargo run -- searchstring example-filename.txt

fn main() {
    let args: Vec<String> = env::args().collect();
    // args() function returns a list of command line arguments and collect()  function converts them into collection of lists

    eprintln!("{:?}", args);

    // pass arguments like this cargo run sid sid.txt
    // ouptut will be ["target/debug/minigrep", "sid", "sid.txt"]

    // now store both arguments

    // let query = &args[1]; // contains sid
    // let filename = &args[2]; // contains sid.txt

    // call function directly

    let config = Config::new(&args).unwrap_or_else(|err| {
        // this is a closure
        eprintln!("Problem passing arguments:{err}");   // to dont get error in standard output file we use eprintln! macro
        process::exit(1);
    });

    // unwarp_or_else will return the value store in Ok if this is a Ok case. in error case it will execute this closure and exit from program

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // now create a new file in the directory poem.txt to read data from this
    // use fs module for this

    //     std::fs::read_to_string
    // Purpose: Reads the entire contents of a file into a string and return that string.
    // let contents = fs::read_to_string(config.filename).expect("Should able to read file contents");

    // println!("Reading file: {}", contents);

    //  cargo run sid poem.txt
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

    // run this command to get output in output.txt file
    // cargo run to poem.txt > output.txt

    // Refactoring the codebase

    // main function does two things taking and storing the arguments. second reading the file
    // main.rs is a binary crate
    // we can create a library crate and move most of the logic thier and call it from main.rs
    // will create sepreate function for this

    // Now error handling
    // suppose our CLI conatins less than 3 arguments than our code will panic
}

// error handling
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // this is an error type
//     let contents = fs::read_to_string(config.filename)?; // the ? will return error if it not able to read file

//     println!("Reading file: {}", contents);

//     Ok(())
// }

// // create structs to more clear
// struct Config {
//     query: String,
//     filename: String,
// }

// // parse_config function takes a reference to a vector of strings and returns a reference to a Config struct in Result enum to handle errors
// // args: &Vec<String>: This means args is a reference to a vector of strings.
// // args[1]: This accesses the element at index 1 of the vector, which is of type String.
// // &args[1]: This takes a reference to the String at index 1, resulting in a &String.

// // Why Use &args[1] Instead of args[1]?
// // args[1]: This would move the String at index 1 out of the vector, but you cannot move out of a reference (&Vec<String>),
// //  so it results in a compile-time error.
// // &args[1]: This borrows the String at index 1, creating a &String, which is allowed.
// // we can also take args:&Vec<String> instead of args:&[String]
// // fn parse_config(args: &[String]) -> Config {
// //     let query = args[1].clone(); // clone() because we dont want to take ownership
// //     let filename = args[2].clone();
// //     Config { query, filename }
// // }

// // So now that the purpose of the parse_config function is to create a Config instance,
// //  we can change parse_config from a plain function to a function named new that is associated
// //   with the Config struct. Making this change will make the code more idiomatic.

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &str> {
//         // Error Handling
//         if args.len() < 3 {
//             return Err("Not enough arguments");
//         }
//         let query = args[1].clone(); // clone() because we dont want to take ownership
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }


// now we will put all the functions and struct in library crate to make your code more readable