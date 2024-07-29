pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// By default, All test are run in parallel with different threads so the execution is not fixed
//cargo test -- --test-threads=1 = this command will take only one thread and run all tests
#[cfg(test)] // configuration
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        //  Result<T, E>
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Test failed"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // while running test if test case pass than we dont see the print statements on terminal to see that use command
    // cargo test -- --show-output
    // #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // we can also run only one test by using the command cargo test fn_name

    // we can also ignore the test by writing #[ignore] in front of the test

    // in rust test can be of two types unit tests and integration tests

    // Unit tests
    // The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint
    //  where code is and isn’t working as expected. You’ll put unit tests in the src directory in each file with the code
    //  that they’re testing. The convention is to create a module named tests in each file to contain the test functions
    //  and to annotate the module with cfg(test).

    // You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)]
    //  annotation. However, because unit tests go in the same files as the code, you’ll use #[cfg(test)]
    //  to specify that they shouldn’t be included in the compiled result.

    // Unit Tests: Tests that are defined within the same file or module as the code they are testing,
    // typically using #[cfg(test)] and #[test] annotations.
    // Integration Tests: Tests that are placed in the tests directory of your project.
    // They test the public interface of your library in a more holistic way.
}
