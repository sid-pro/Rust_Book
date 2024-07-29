fn main() {
    let a = [1, 2, 3];

    // it will create a new vector all collections are store in heap so they can increase or decrease the size
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // create and intialize a new vector at same time
    let v2 = vec![1, 2, 3];

    // Accessing elements in the vector
    // first way
    let third = &v2[2];
    println!("The third element is {}", third);

    // second way
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("The vector has less than 3 elements"),
    }

    // this code will give error as we learn previously that a variable cannot have mutable and immutable references at same time
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];    // immutable reference

    // v.push(6);     // try to change it

    // println!("The first element is: {first}");   // using immutable value of vector

    // iterating over vectors
    // immutable refernce
    for i in &v {
        println!("{i}");
    }

    // mutable reference to modify value
    for i in &mut v {
        *i += 5; //* to derefernce it
        println!("{}", i);
    }

    // enum with vectors

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings
    // Rust has only one string type in the core language, which is the string slice
    // str that is usually seen in its borrowed form &str
    // String literals are immutable and store in read-only memory
    // string literals are cannot be mutable
    let s: &str = "Hello, world!";
    println!("{}", s);

    //If you need a string that can be modified or resized, you use the String type.
    //  Unlike string literals, String is heap-allocated.
    // Storage: The String type allocates memory on the heap to store its contents. This allows it to be mutable and to change size dynamically.
    // Mutable: You can modify a String after it has been created.
    // Variable Size: The size of a String can change at runtime.

    let mut s = String::from("Hello, world!");
    s.push_str(" Welcome to Rust.");
    println!("{}", s);

    // we can create string slice and than convert it to String using to_string()   function

    let slice: &str = "hello";
    let mut string: String = slice.to_string();

    println!("String slice: {}", slice);
    println!("Owned String: {}", string);
    string.push_str("asd");

    // Iterating Over Characters
    let s = String::from("hello");

    for c in s.chars() {
        println!("{}", c);
    }

    // Iterating Over Bytes
    let s = String::from("hello");

    for b in s.bytes() {
        println!("{}", b);
    }

    // Hash maps

    use std::collections::HashMap;

    let blue = String::from("blue");
    let red = String::from("red");

    let mut scores = HashMap::new();

    // as we are not passing reference so ownership is move
    scores.insert(blue, 10);
    scores.insert(red, 20);

    // println!("{}", blue);

    let team_score = String::from("blue");
    // get method return option value
    let score = scores.get(&team_score);
    println!("{}", team_score);
    // get method return option value
    match scores.get(&team_score) {
        Some(score) => println!("{}", score),
        None => println!("No score found for {}", team_score),
    }

    // iteration over  hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // it will override the value of team blue
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // it does not override value if Blue key exist
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // entry("world"): "world" is not in the map, so it inserts "world" with a value of 0.
    // or_insert(0): Returns a mutable reference to the value 0.
    // *count += 1: Increments the value from 0 to 1.
    // entry("world"): "world" is already in the map with a value of 1.
    // or_insert(0): Returns a mutable reference to the value 1.
    // *count += 1: Increments the value from 1 to 2.
    // or_insert(0): Returns a mutable reference to the value count.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
