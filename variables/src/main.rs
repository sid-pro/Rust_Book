fn main() {
    // variables are immutable by default in rust
    let x = 5;
    println!("x = {x}");
    // will return error as x is immutable by default
    //    x = x+1;
    //    println!("x = {x}"); 

    // to create variables muttable use mut keyword
    let mut y = 5;
    println!("y = {y}");
    y = y + 1;
    println!("y = {y}");

    // shadowing allows us to create a new variable with the same name

    // constants values
    // Constants require a type annotation here u means unsigned we can also use i32 which is signed integer
    // constants cannot be mutable that is the main difference between constants and variables in rust
    // constants always require a type annotation whereas variables not requiring a type annotation always
    const COUNT: u32 = 10000;
    println!("count = {COUNT}");

    // shadowing variables
    // This program first binds a to a value of 5. Then it creates a new variable a by repeating let a =,
    //  taking the original value and adding 1 so the value of a is then 6. Then, within an inner scope created with
    //  the curly brackets, the third let statement also shadows a and creates a new variable, multiplying the previous value by
    //  2 to give a a value of 12. When that scope is over, the inner shadowing ends and a returns to being 6.
    //   When we run this program, it will output the following:

    let a = 5;   // this will shadow after line no 31

    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");

    // it will work because a is immutable and it will create a new a with type string it will give error if a is muttable
    let a = "sid";
    println!("The value of a is: {a}");

    // Data Types
    // Scaler Data Types
    // Integer
    // Floating point
    // boolean
    // character

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // Compound Types
    // Tuple Data Types
    //  A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    //  Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // tuple of string and number types
    let tup = ("Hello",1000);
    let (string,number) = tup;
    println!("{}, {}", string, number);

    let count = tup.1;
    println!("count = {count}");

    // Array Data Types
    let arr = [1,2,3,4,5];
    let first = arr[0];
    let second = arr[1];
    println!("{}, {}", first, second);

    // by default tuples and arrays are also immutable by default

    //u8 maximum value store = 255
   // i8 maximum value store = 127  (because it has both positive and negative)

    // functions
    another_funtion();
    with_arguments(10, 11);
    let sum = add(10,20);
    println!("sum = {sum}");


    // control flow
    // if else
    let no = 10;
    if no>10 {
        println!("Number is greater than 10");
    }else if no<10{
        println!("Number is smaller than 10");
    }else{
        println!("Number is equal to 10");
    }

    // loop
    let mut counter = 1;

    let res = loop{
        counter += 1;
        println!("again!");
        if counter == 5{
            break counter; // break the loop and return counter
        }
       
    };
    println!("Loop result: {}", res);

    let mut counter = 3;   // shadows previous counter
    while counter != 0{
       println!("again!");
       counter -= 1;
    }

    let array = [10,20,30,40,50];
    for element in array{
        println!("element is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
     
}
fn another_funtion(){
    println!("Another function");
}
// need to write type annotations
// this function is statement as it not return anything
fn with_arguments(x:i32,y:i32){
    println!("value of x is {x} and y is {y}");
}

// expression function which return a value
fn add(x:i32,y:i32)->i32{
    return x+y;
}