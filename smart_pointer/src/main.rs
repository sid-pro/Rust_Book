use List::{Cons, Nil};
// A pointer is a general concept for a variable that contains an address in memory.
// This address refers to, or “points at,” some other data.
// The most common kind of pointer in Rust is a reference  References are indicated by the & symbol and borrow
//  the value they point to. They don’t have any special capabilities other than referring to data, and have no overhead.

// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// To explore the general concept, we’ll look at a couple of different examples of smart pointers,
//  including a reference counting smart pointer type. This pointer enables you to allow data to have multiple owners
//  by keeping track of the number of owners and, when no owners remain, cleaning up the data.

// Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers:
//  while references only borrow data, in many cases, smart pointers own the data they point to.

// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref
//  and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you
//  can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code
//  that’s run when an instance of the smart pointer goes out of scope.

// We’ll cover the most common smart pointers in the standard library:

// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

// In many cases smart pointers owns the data instead of refrence to it, unline references which simply borrow the value

fn main() {
    // 1. Box Smart Pointer

    // Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    // But they don’t have many extra capabilities either. You’ll use them most often in these situations:

    //1  When you have a type whose size can’t be known at compile time and you want to use a value of that type in a
    //  context that requires an exact size
    //2 When you have a large amount of data and you want to transfer ownership but ensure the data won’t be
    //  copied when you do so
    //3 When you want to own a value and you care only that it’s a type that implements a particular
    //  trait rather than being of a specific type

    let b = Box::new(5);
    // The most straightforward smart pointer is a box, whose type is written Box<T>.
    //  Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data

    println!("b = {}", b); //b = 5
                           // We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap
                           // when a box goes out of scope, as b does at the end of main, it will be deallocated. The deallocation happens
                           // both for the box (stored on the stack) and the data it points to (stored on the heap).

    // usage of Box in Cons Data Structure

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new((Nil)))))));
    println!("{:?}", list);

    // Deref trait

    let a = 5; // `a` is an integer with the value 5
    assert_eq!(a, 5); // Check that `a` is equal to 5

    let b = a; // `b` is assigned the value of `a`. This is a move, not a reference.
    assert_eq!(b, 5); // Check that `b` is equal to 5
                      // Unlike complex types such as vectors or strings, simple value types like integers do not require ownership
                      //  transfer or borrowing. The b variable simply gets a copy of the value from a.
                      // Implementing the Deref trait allows you to customize the behavior of the dereference operator *
    let a = 5;
    assert_eq!(a, 5);
    let b = &a;
    assert_eq!(*b, 5); // if we dont dereference it, will give error

    // At a time we can have multiple immutable references or one mutable reference
    let mut a = 5; // `a` is a mutable i32 with value 5
    assert_eq!(a, 5);

    let b = &a; // `b` is an immutable reference to `a`
    assert_eq!(*b, 5); // Dereference `b` to get the value of `a`

    let c = &mut a; // `c` is a mutable reference to `a`
    *c = *c + 1; // Modify the value of `a` through `c`

    assert_eq!(*c, 6); // `c` still points to the updated value of `a`

    assert_eq!(a, 6); // `a` has been updated to 6

    // to get rid of dereference use deref trait
    // smart pointer implemets the deref and drop trait so here we show the internal working of it we don't need to implement everytime
    let x = 5;
    let y = Box::new(x); // Box give me the reference of the value x
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // use MyBox
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // as we implement deref trait on MyBox so *y now dont give error now.
    assert_eq!(*y, 5);
    // *(y.deref())  internally it works like this

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // it will work hello method accepts the &str but we pass MyBox<String> type
               // &MyBox<String> --> &String --> &str   bcox MyBox implements deref trait and return &T
               // This is called Dered coercion

    //     Rust does deref coercion when it finds types and trait implementations in three cases:

    // &t = immutable refernce &u = another immutable refernce
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>

    // Drop trait
    // The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when
    //  a value is about to go out of scope
    // For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

    let c = CustomSmartPointer{
        value: String::from("Hello, world!"),
    };
    let c = CustomSmartPointer{
        value: String::from("Get Rusty"),
    };
    println!("CustomSmartPointer created.");
    // Drop trait run in reverse order so first d will drop and then c
}

//recursive type `List` has infinite size
#[derive(Debug)]
enum List {
    // Cons(i32, List), recursive without indirection
    // it is like a linked list Data strucutre example: (1, (2, (3, Nil)))
    Cons(i32, Box<List>), // Box<List> is recursive without indirection
    // in stack box is a fixed size pointer of data which is pointing on an heap with arbitrary size
    Nil,
}

use std::ops::Deref; //this is a trait which will implemented by the structs
struct MyBox<T>(T); // The syntax struct MyBox<T>(T); in Rust defines a tuple struct with a single field.

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //  specifies the type that MyBox<T> will dereference to. In this case, it is T.

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// implementing drop trait 
struct CustomSmartPointer{
    value:String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data: {}",&self.value);
    }
}