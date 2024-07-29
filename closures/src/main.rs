// Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
//  You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.

use std::thread;
use std::time::Duration;

fn simulated_expensive_caclulation(intensity: i32) -> i32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2)); // Simulate some expensive computation
    intensity
}
fn main() {
    let mut x = 4;
    // A closure in Rust is defined using a pipe syntax for parameters, followed by the closure body:
    let mut add_to_x = |y| x += y;
    add_to_x(5);
    println!("x is now: {}", x); // Output: x is now: 9
    add_to_x1(&mut x, 3);
    println!("x is now: {}", x);

    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

fn generate_workout(intensity: i32, random_number: i32) {
    // we are calling simulated_expensive_calculation three times in if else which is a time taking function

    // so we can create a variable here to call this method
    // let expensive_result = simulated_expensive_caclulation(intensity);

    // but here also in case of random number condition we dont need this function calling but we still doing
    //  this which will again take time

    // so to solve this we can make use of closure
    let expensive_closure = |num| {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2)); // Simulate some expensive computation
        num
    };
    if intensity < 25 {
        println!(
            "Today do, {} pushups",
            // simulated_expensive_caclulation(intensity)
            expensive_closure(intensity)
        );
        println!(
            "Nex do {} situps:",
            // simulated_expensive_caclulation(intensity)
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Leave for today");
        } else {
            println!(
                "Today run for {} minutes",
                // simulated_expensive_caclulation(intensity)
                expensive_closure(intensity)
            )
        }
    }
}

// Equivalent function needs to take `x` as an argument
fn add_to_x1(x: &mut i32, y: i32) {
    *x += y;
}

// Differnce between closure and function

// 1. Capturing Environment:

// Closures: Can capture variables from the surrounding scope. They can borrow (either immutably or mutably) or take ownership of the variables.
// Functions: Do not capture any variables from the surrounding scope. All variables used in a function must be passed as arguments.

// 2. Type Inference:

// Closures: Can often infer types automatically based on the context in which they are used.
// Functions: Require explicit type annotations for all parameters and return types.

// 3. Flexibility:

// Closures: Are more flexible in terms of capturing context. They can be defined in the middle of a
//           function and can make use of variables in the surrounding scope.
// Functions: Are more rigid in terms of scope. They can only use what is passed to them and must be defined separately.

// 4. Traits:
// Closures: Implement one of the Fn, FnMut, or FnOnce traits, depending on how they capture the environment.
// Functions: Implement the Fn trait.

// 5. Memory Allocation:

// Closures: Can capture environment variables which might require heap allocation if they capture non-copy types by value.
// Functions: Do not capture the environment and do not involve such allocations.

// Summary
// Closures: Capture their environment, have type inference, can be more flexible, and are used in scenarios where capturing context is beneficial.
// Functions: Do not capture environment, require explicit type annotations, and are used where capturing the context is
//            unnecessary or when defining reusable code that does not depend on specific local variables.
