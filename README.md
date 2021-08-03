# Notes on Rust

## General notes on the language

* Rust is a modern systems programming language

* Provibly memory and race condition safe without garbage collection

* No null types or null pointers due to its rich type system

* No exceptions, type system is used to do proper error handling

* Modern package manager - cargo

* Rust uses smart pointers -> Memory gets released when pointer address is released out of scope

* Rust can be debugged using gdb

* Variables are immutable by default (constants), unless specified with "mut" keyword, to make the variable mutable

## Notes on Rust projects, packages and tools

* CLI programs: rustc -> Rust Compiler, rustup -> Toolchain installer, Update rustc or cargo & more

* Cargo.toml is the root of each rust project, the "package.json" of systems programming

* In rust world, packages are called crates

* Central crates registry -> <https://crates.io>

* Install packages by appending the crate to the Cargo.toml or by typing cargo install {package-name}

## Notes on datatypes

* Basic datatypes are usinged integers and signed integers, from 8 to 128 bits in size incrementing in a power of 2

* More basic datatypes are floats, either 32 bits or 64 bits in size

* A bool is also stored in a byte

* A char is always 32 bits in size

* usize, isize is architecture dependent, either 32 bits or 64 bits

* u8, i8 - u128, i128 ; f32, f64 ; char ; bool  

## Notes on syntax

* Function and variable naming convention is snake_case

* main is always the first called function in a program -> The entry point

* TypeScript type syntax and variable declaration syntax

* Functions are declared with fn and their return is declared with "->" instead of ":"

* Last expression in a function without a semicolon is the expression the function shall return, no return keyword needed

* Semicolons are otherwise mandatory

* Function calls with "!" at the end are not functions but macros

## Notes on macros

* Macros can be called with a varying amount of arguments, functions have a defined and fixed amount of arguments

* Anytime a macro receives a string with curly braces inside it, this indicates a variable to be inserted there

* These variables can have mixed data types

## Notes on the compiler

* Compiler does type inference when declaration and initialization happens on the same line

* The standard library can be easily imported in the scope with the "use" keyword

## Notes on ownership

* Rust has three ownership rules that spread through the entire design of Rust
    1. Each value in Rust is owned by a variable

    2. When the owner goes out of scope, the value will be deallocated

    3. There can only be ONE owner at a time

        * Rule 3 solves the double free vulnerability of many C programs

* When a function is called with a variable as an argument the parameter of the function will become the owner of the value (rule 1) and rule 2 will deallocate the variable from the heap after the function returns

* To keep using variables in functions, the concept of referencing and borrowing is used

* Referencing allows using values without taking ownership of it

* Referencing values is indicated with "&" in front of the datatype of the function paramter and the argument of the function call

* In Rust, passing references as parameters is called "borrowing" 

* References are immutable by default

* To mutate references one has to declare "&mut" in front of the datatype of the parameter as well as the argument of the function call

* &mut is a mutable reference

* In the same scope there can be as many immutable references as desired or a single mutable reference

* This means that borrows are either mutable or immutable but not both in the same scope, if a borrow is mutated the others would be as well

* These restrictions to data mutations solve data races at compile time -> super powerful

* If the code compiles, data races are impossible!
