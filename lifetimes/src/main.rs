use std::fmt::Display;
// lifetime annotation in structs
// So far, the structs we’ve defined all hold owned types. We can define structs to hold references, 
// but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

// This struct has the single field part that holds a string slice, which is a reference.
struct ImportantExcerpt <'a> {
   part:&'a str
}
fn main() {
    // Dangaling reference: reference that points to invalid data

    //The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named
    // x with the initial value of 5. Inside the inner scope, we attempt to set the value of r as a reference to x. Then the
    //  inner scope ends, and we attempt to print the value in r. This code won’t compile because what the value r is
    //  referring to has gone out of scope before we try to use it.

    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    //     // after this x does not live long enough
    // }

    // println!("r: {r}");

    let string1 = String::from("Hello, world!");
    let string2 = String::from("Hello, world! Let Rusty");

    // as_str is same as &str
    // result will give you the smallest lifetime of pass variables here both have same lifetime
    let result = longest_string(&string1, string2.as_str());

    println!("The longest string is: {}", result);

    let string1 = String::from("long string is long");
    // here result lifetime is same as string2 because it has smaller lifetime than string1
    {
        let string2 = String::from("xyz");
        let result = longest_string(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }



    // this will give error as string2 go out of scope borrowed value does not live long enough
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest_string(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    println!("The longest string is {}",longest1(&string1,&string2));

    let novel = String::from("three men in a boat. By Harry");
    // first_sentence is a refrence of the novel
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt{
        part:first_sentence,
    };

    //Lifetimes on function or method parameters are called input lifetimes, 
    // and lifetimes on return values are called output lifetimes.

    //1 The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
    // 2 if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    //3  if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, 
    // the lifetime of self is assigned to all output lifetime parameters.

    // string literal lifetime static lifetime
    // static refrenece =  reference can live for the entire duration of the program. 
    let s: &'static str = "I have a static lifetime.";


}

// here the return type is string slice
// Rust can’t tell whether the reference being returned refers to x or y.
// Actually, we don’t know either, because the if block in the body of this function returns a
// reference to x and the else block returns a reference to y!

// fn longest_string(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// To fix this error, we’ll add generic lifetime parameters that define the relationship
//  between the references so the borrow checker can perform its analysis.

//Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start
// with an apostrophe (') and are usually all lowercase and very short, like generic types
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for
//  the parameter y, because the lifetime of y does not have any relationship with the lifetime of x
//   or the return value.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this will give error as we are returning a reference of a string whose scope will over after the function ends
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// to solve this we can return the ownership of this string instead of a reference
fn longest1(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}

// generic trait trait bound lifetime all in one function
fn longest_with_an_announcement<'a, T>(     // generics
    x: &'a str,             // lifetime
    y: &'a str,
    ann: T,            // trait
) -> &'a str
where
    T: Display,      // trait bound
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}