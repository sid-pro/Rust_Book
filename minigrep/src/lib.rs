// this is our route of our library crate
use std::fs; // to read file and to do operations with filesystem
             // this is our first rust project
use std::env;
use std::error::Error;

// now we need to declare our function and struct as public

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // this is an error type
    let contents = fs::read_to_string(config.filename)?; // the ? will return error if it not able to read file

    // println!("Reading file: {}", contents);

    // now print only that line which contains query
    // let res =search(&config.query, &contents);
    // println!("{:?}",res);

    // also print like this
    // for line in search_sensitive(&config.query, &contents){
    //     println!("{}", line);
    // }

    // Ok(())

    // Now we have two function case sensitive and insensitive so our program needs to figure out which to use
    // we will do this using enviornment variables

    let result = if config.case_sensitive {
        // in rust we dont put semi-colons in end if we are directly returning a value
        search_sensitive(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

// create structs to more clear
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// parse_config function takes a reference to a vector of strings and returns a reference to a Config struct in Result enum to handle errors
// args: &Vec<String>: This means args is a reference to a vector of strings.
// args[1]: This accesses the element at index 1 of the vector, which is of type String.
// &args[1]: This takes a reference to the String at index 1, resulting in a &String.

// Why Use &args[1] Instead of args[1]?
// args[1]: This would move the String at index 1 out of the vector, but you cannot move out of a reference (&Vec<String>),
//  so it results in a compile-time error.
// &args[1]: This borrows the String at index 1, creating a &String, which is allowed.
// we can also take args:&Vec<String> instead of args:&[String]
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone(); // clone() because we dont want to take ownership
//     let filename = args[2].clone();
//     Config { query, filename }
// }

// So now that the purpose of the parse_config function is to create a Config instance,
//  we can change parse_config from a plain function to a function named new that is associated
//   with the Config struct. Making this change will make the code more idiomatic.

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Error Handling
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // clone() because we dont want to take ownership
        let filename = args[2].clone();

        // to set CASE_INSENSITIVE variables run command 
        // export CASE_INSENSITIVE = true
        // cargo run to poem.txt     
        // to again reset variable
        // unset CASE_INSENSITIVE
        // cargo run to poem.txt
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Test Driven development

// this is a case sensitive search function
pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // need to specify the lifetime
    // vec![]

    // loop through each line

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

//case insensitive search
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // contains method always except the string slice
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // assert_eq!(vec!["safe, fast, prductive."], search(query, contents));: This checks that the result
        //  of the search function, when called with query and contents, is equal to a vector containing the strings
        //  "safe, fast, productive."

        // The test case should verify if the function correctly identifies lines that
        // contain the substring "duct", which is indeed present in "productive.".

        // so the test case pass

        assert_eq!(
            vec!["safe, fast, productive."],
            search_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], // return a vector of list with two lines "Rust:" and "Trust me."  //Trust contain query rUsT
            search_insensitive(query, contents)
        );
    }
}
