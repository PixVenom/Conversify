use std::io;

fn main() {
    println!("Welcome to Conversify");

    println!("Choose your action: \n1.Celcius to Fahrenheit\n 2. Fahrenheit to Celcius");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: i32 = choice.trim().parse().expect("Enter your choice");

    if choice == 1 {
        to_fahrenheit();
    } else if choice == 2 {
        to_celcius();
    }
}

fn to_fahrenheit() {
    let mut celcius = String::new();
    println!("Input Celcius value:");
    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to take input");
    let celcius: i32 = celcius.trim().parse().expect("Failed to accept input");
    let fahrenheit = celcius * 9 / 5 + 32;
    println!("Celcius to Fahrenheit is {fahrenheit}");
}

fn to_celcius() {
    let mut fahrenheit = String::new();
    println!("Input Fahrenheit value:");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to take input");
    let fahrenheit: i32 = fahrenheit.trim().parse().expect("Failed to accept input");
    let celcius = (fahrenheit - 32) * 5 / 9;
    println!("Fahrenheit to Celcius is {celcius}");
}