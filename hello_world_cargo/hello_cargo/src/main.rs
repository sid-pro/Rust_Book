fn main() {
    println!("Hello, world!");
    let a = 5;
   let b = add_five(a);
   
   println!("{}",b);
}

fn add_five(a:i32)->i32{
    a+5
}

// cargo.toml is like a package.json file which contains all dependencies
// now to run this file first we need to build it using cargo build which will generate target file which is like node_modules
// and cargo.lock which is like a package.json file which contains all dependencies.

// now run cargo run to execute the program

// cargo new name --> create a new project
// cargo add name --> add a dependency package
// cargo check --> check if our code has any errors or not
// cargo build --> build a project
// crago run --> We can build and run a project in one step using