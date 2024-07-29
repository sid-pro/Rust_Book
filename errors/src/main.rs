use std::fs::File;
use std::io::ErrorKind;
// Sometimes, bad things happen in your code, and there’s nothing you can do about it.
// In these cases, Rust has the panic! macro. There are two ways to cause a panic in practice:
// by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the
// panic! macro. In both cases, we cause a panic in our program. By default, these panics will print a failure message, unwind,
//  clean up the stack, and quit. Via an environment variable, you can also have Rust display the call stack when a panic occurs
//   to make it easier to track down the source of the panic.
fn main() {
    // panic!("crash and burn");

    // Using a panic! Backtrace
    // a();

    // Result enum
    // enum Result<T, E> {
    //     Ok(T),   // success case
    //     Err(E),   // failre case

    // }

    // open return result type
    // let f = File::open("hello.txt");

    // match f{
    //     Ok(file) => println!("file open successfully {:?}",file),
    //     Err(error) => panic!("file not found :{:?}",error),
    // }

    // more details error
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) =>file,
        Err(error) => match error.kind(){
            // when file do not exist create new file
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) =>fc,
                Err(e) => panic!("Could not create file: {:?}",e),
            }
            other_error => panic!("Problem opening the file: {other_error:?}"),
        }
    };

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    //  If the Result is the Err variant, unwrap will call the panic!
    let greeting_file = File::open("hello.txt").unwrap();

    // or expect method
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    
}

fn a(){
    b();
}
fn b() {
    c(21);
}

fn c(num:i32){
    if num == 22{
        panic!("Dont pass 22:{}",num);
    }
       
}
// RUST_BACKTRACE=1 cargo run using this command we can Backtrace the error message
