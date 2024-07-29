#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height * self.width > other.height * other.width
    }
}

// To change a function into a test function, add #[test] on the line before fn.
//  When you run your tests with the cargo test command, Rust builds a test runner binary that runs the
//   annotated functions and reports on whether each test function passes or fails.

//cargo new adder --lib  create this project

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)] // attribute marks the tests module as a module that should only be compiled and run when running tests.
mod tests {
    // use super::*;

    // #[test]   // this represents that it is a test function
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    //     // The example function body uses the assert_eq! macro to assert that result, which contains the
    //     // result of adding 2 and 2, equals 4.
    // }

    //   // failed test
    //   #[test]
    //   fn failed_test() {
    //       panic!("failed test");    // whenever we write panic macro in test function it fails
    //   }

    // test the original functions
    use super::*; // In Rust, use super::*; is a way to import all items from the parent module into the current scope. take Recatangle struct

    // assert! macro
    // this macro returns boolean value
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 8,
            width: 12,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller)); // this will return true
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 8,
            width: 12,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger)); // this will return true because we write ! in front of the smaller
    }

    // if a test fails it creates a panic

    //assert_eq! macro
    // this macro will compare two values and returns boolean

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    // assert_ne! macro
    // this macro is opposite of the assert_eq!

    #[test]
    fn it_adds_two_second() {
        assert_ne!(!add_two(2), 4);
    }

    // custom error message

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));   without message

        // if test fails we get this message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name {}",
            result
        );
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!") // format macro will Returns a String containing the formatted text.
    // format!("Hello ! {}",name)
                               //  return Hello ! Carol
}
