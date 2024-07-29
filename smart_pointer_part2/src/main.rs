use std::rc::Rc;
use List::{Cons, Nil};
// This is second part of our smart pointer
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
//  However, there are cases when a single value might have multiple owners. For example, in graph data structures,
//  multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it

// You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation
// for reference counting. The Rc<T> type keeps track of the number of references to a value to determine whether
// or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any
// references becoming invalid.

//We’ll create list a that contains 5 and then 10. Then we’ll make two more lists:
// b that starts with 3 and c that starts with 4. Both b and c lists will then continue on to the first
// a list containing 5 and 10. In other words, both lists will share the first list containing 5 and 10.

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // // let b = Cons(3,Cons(5,Cons(10,Cons(Nil))));
    // // let c = Cons(4,Cons(5,Cons(10,Cons(Nil))));
    // // or like this
    // let b = Cons(3, Box::new(a));   // we pass ownership of a to b here
    // let c = Cons(4, Box::new(a));   // so here we will get error

    // so instead of this we can use Rc smart pointer
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Reference counter pointer only allow us to read the data not to modify because then it violates borrowing rule

    // for mutablity we use Interiour mutablity

    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable
    // references to that data; normally, this action is disallowed by the borrowing rules.

    // To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern
    // mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the rules manually instead of
    // relying on the compiler to check them for us

    // The borrowing rule we learned so far:
    //  At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
    // References must always be valid.

    // With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime.
    // Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds

    //     Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

    // Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    // Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at
    // compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    // Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T>
    //  even when the RefCell<T> is immutable.

    // Mutating the value inside an immutable value is the interior mutability pattern.

    // give error
    //     let x = Box::new(5);
    //    println!("{}",x);
    //    *x = *x+2;
    //    println!("{}",x);
    // workf fine
    let mut x = Box::new(5);
    println!("{}", x);
    *x = *x + 2;
    println!("{}", x);

    let mut x = 5;
    let y = &mut x; // Create a mutable reference to `x`

    *y = 20; // Modify `x` through the mutable reference

    println!("{}", y); // Both `x` and `y` will show the updated value of `x` but we can only acces y because we can only acces one reference at a time
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

