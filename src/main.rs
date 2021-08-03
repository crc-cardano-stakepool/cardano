/* Modern systems programming language
 * Provibly memory and race condition safe without garbage collection 
 * No null types or null pointers due to its rich type system
 * No exceptions, type system is used to do proper error handling
 * Modern package manager - cargo
 * CLI programs: rustc -> Rust Compiler, rustup -> Toolchain installer, Update rustc or cargo & more
 * Cargo.toml is the root of each rust project, the "package.json" of systems programming
 * In rust world, packages are called crates
 * Central crates registry -> https://crates.io
 * Install packages by appending the crate to the Cargo.toml or by typing cargo install <pkg>
 * Rust uses smart pointers -> Memory gets released when pointer address is released out of scope
 * Rust can be debugged using gdb
 * Basic datatypes are usinged integers and signed integers, from 8 to 128 bits in size incrementing in a power of 2
 * More basic datatypes are floats, either 32 bits or 64 bits in size
 * A bool is also stored in a byte 
 * A char is always 32 bits in size
 * usize, isize is architecture dependent, either 32 bits or 64 bits
 * u8, i8 - u128, i128 ; f32, f64 ; char ; bool  
 * Function naming convention is snake_case
 * main is always the first called function in a program -> The entry point
 * TypeScript type syntax and variable declaration syntax
 * Functions are declared with fn and their return is declared with "->" instead of ":"
 * Last expression in a function without a semicolon is the expression the function shall return, no return keyword needed
 * Semicolons are otherwise mandatory
 * Function calls with "!" at the end are not functions but macros
 * Macros can be called with a varying amount of arguments, functions have a defined and fixed amount of arguments
 * Anytime a macro receives a string with curly braces inside it, this indicates a variable to be inserted there
 * These variables can have mixed data types
 */

use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}