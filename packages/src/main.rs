// As you write large programs, organizing your code will become increasingly important.
// By grouping related functionality and separating code with distinct features, you’ll clarify where to find code
// that implements a particular feature and where to go to change how a feature works.

// The programs we’ve written so far have been in one module in one file. As a project grows, you should organize
// code by splitting it into multiple modules and then multiple files. A package can contain multiple binary crates and
// optionally one library crate.

// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// Crates can contain modules, and the modules may be defined in other files that get compiled with the crate
//  A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs
//  you can compile to an executable that you can run, such as a command-line program or a server.
//  Each must have a function called main that defines what happens when the executable runs.
//  All the crates we’ve created so far have been binary crates.

// Library crates don’t have a main function, and they don’t compile to an executable. Instead,
// they define functionality intended to be shared with multiple projects. For example, the rand crate

// After we run cargo new, we use ls to see what Cargo creates. In the project directory, there’s a Cargo.toml 
// file, giving us a package. There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor, 
// and note there’s no mention of src/main.rs. Cargo follows a convention that src/main.rs is the crate root of a binary 
// crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, 
// the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes
//  the crate root files to rustc to build the library or binary.

// so this main.rs is our crate root which  is a binary crare and this folder is our package (Cargo.toml file) directory
// src/main.rs and src/lib.rs are called crate roots.
fn main() {
    println!("Hello, world!");
}
