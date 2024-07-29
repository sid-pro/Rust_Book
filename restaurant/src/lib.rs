// this is our library crate to create this run commant (cargo new --lib name)

// so in library crate we can create different modules like:

// Modules let us organize code within a crate for readability and easy reuse.
// Modules also allow us to control the privacy of items, because code within a module is private by default. 
// Private items are internal implementation details not available for outside use. We can choose to make modules and 
// the items within them public, which exposes them to allow external code to use and depend on them.

// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.

mod front_of_house{

    // one module can contain other modules enums,structs ,contants ,traits
    // mod hosting{
    //     fn add_to_waitlist(){

    //     }
    //     fn seat_at_table(){

    //     }
    // }

    // we need to make this module public to use it in another module/function
    pub mod hosting{
        pub fn add_to_waitlist(){

        }
        fn seat_at_table(){

        }
    }

    mod serving{
        fn take_order(){

        }

        fn serve_order(){

        }

        fn take_payment(){

        }
    }
}

// To show Rust where to find an item in a module tree, we use a path in the same way we
//  use a path when navigating a filesystem. To call a function, we need to know its path.

// A path can take two forms:

// An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path
//  begins with the crate name, and for code from the current crate, it starts with the literal crate.
// A relative path starts from the current module and uses self, super, or an identifier in the current module.
// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Starting Relative Paths with super

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // relative path
        super::deliver_order();
    }

    fn cook_order() {}
}

// Making Structs and Enums Public

mod back_of_house1 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// enum public
mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}

// use keyword
mod front_of_house1 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// now we can directly use hosting module
pub use crate::front_of_house::hosting;
// we can mark this as public so it can be used by external code also

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}