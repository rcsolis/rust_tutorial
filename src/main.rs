#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    print!("Welcome, what is your name? ");
    io::stdout().flush();
    // By default all variables are immutable
    let greeting = "Hello";
    // To make a variable mutable, use the mut keyword
    let mut name = String::new();
    // Read input from the user
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("{} {}", greeting, name.trim_end() );
    println!("Waht is your age?");
    // Constants are always immutable
    const ONE_MILL: u32 = 1_000_000;
    // Shadowing is a way to change the type of a variable by using the same name
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim_end().parse().expect("Please type a number!");
    println!("You are {} years old", age);
}
// Path: src/main.rs