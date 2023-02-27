use std::io;
use std::io::Write;
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    print!("Welcome, what is your name? ");
    io::stdout().flush().expect("Failed to flush");
    // By default all variables are immutable
    let greeting = "Hello";
    // To make a variable mutable, use the mut keyword
    let mut name = String::new();
    // Read input from the user
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("{} {}", greeting, name.trim_end() );
    println!("How old are you?");
    // Shadowing is a way to change the type of a variable by using the same name
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim_end().parse().expect("Please type a number!");
    println!("You are {} years old", age);
    println!("Welcome to the calculator");
    // Get console arguments
    if args.len() < 3 {
        println!("Please provide 2 numbers and an operator");
        return;
    }
    let first_element = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_element = args.nth(0).unwrap();
    println!("Operation: {} {} {}", first_element, operator, second_element);
    let first_element:f32 = first_element.parse::<f32>().unwrap();
    let second_element:f32 = second_element.parse::<f32>().unwrap();

    let result = make_operation(operator, first_element, second_element);
    println!("{}", output_result(operator, first_element, second_element, result));
}

fn make_operation(operator: char, first_number:f32, second_number:f32) -> f32{
    return match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator")
        
    }
}
// Implcit return
fn output_result(operator: char, first_number:f32, second_number:f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
// Path: src/main.rs