//Ownership is Rust’s most unique feature and has deep implications for the rest of the language.
// It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to
// understand how ownership works.
// Thats why rust is faster in runtime but slower in compile time because it throws lots of errors
// and rust is also memory safe

// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through
//  the examples that illustrate them:
// Each value in Rust has a variable thats called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    {
        // s is not valid here as its not declared
        // let s:&str = "hello"; // string literal are fixed in size and stored in binary format
        let s = String::from("hello"); // s is valid from here this string store on heap thid is also fix but we can make this muttable
        println!("{}", { s });
    } // here s scope is over and is no longer valid

    // integers are store on stack as we know the size of them during compile time. so move opreation is faster
    // but string will store on heap as we dont know the size of strings
    let x = 10;
    let y = x; // copy
    println!("The value of x is {x} and y is {y}"); // both will assign to 10

    let s1 = String::from("hello"); // store in heap of fix size
                                    // let s2 = s1; // s1 is moved to s2 so s1 is no longer valid
                                    // it gives error as s1 is not valid now
                                    // print!("{}",{s1})

    // we can use clone method if we want to move the data from heap and also persist it.

    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // ownership and functions
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("{}",{s}) // error as s is no longer valid

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{}", x);

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);

    // now s2 goes out of scope
    println!("{} {}", s1, s3);

    // if we want to persist the value than pass the refernce
    let s1 = String::from("hello");

    // refrences dont take ownership
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    change(&mut s);
    println!("The value of string is {s}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    //     At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.

    // Slice type
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // for this we have a solution with string slices
    //A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");
    // this is the immutable reference of string s
    let word = first_word_slice(&s); // word will get the value 5

    // word is a string slice which is a immutable refernce of string s
    // and as we discuss there can be any number of immutable references or only one mutable refernce at a time
    // so s.clear() will mutate the string so we cannot perform this operation
    // s.clear();
    println!("The first word is: {}", word);

    // string literals are basically string slices only
    let s2:&str = "hello world";
    let word = first_word_slice(s2);

    // other slicees
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for &element in slice {
        println!("{}", element);
    }
}
fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
//reference are immutable by default
fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    length // length is returned and moves out to the calling function
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    //Because we need to go through the String element by element and check whether a value is a space,
    //  we’ll convert our String to an array of bytes using the as_bytes method.

    // For now, know that iter is a method that returns each element in a collection and that
    //  enumerate wraps the result of iter and returns each element as part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to
    // the element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // here b represents the byte
            return i;
        }
    }

    return s.len();

    // We now have a way to find out the index of the end of the first word in the string, but there’s a problem.
    //  We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String.
    //  In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid
    //   in the future.
}

// &str will take both String and &str type
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // whole string is a first word
}
