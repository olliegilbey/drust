//calculator, reverse polish notation where operator comes last
use std::io;

fn main() {
    println!("Welcome to Calculator!");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Please input your first number.");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    let num1: u32 = num1.trim().parse().expect("Please type a number!");

    println!("Please input your second number.");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num2: u32 = num2.trim().parse().expect("Please type a number!");

    println!("Please input the operator symbol, +-*/.");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let operator = operator.trim();
    if operator == "+" {
        println!("{num1}{operator}{num2} = {}", num1 + num2);
    } else if operator == "-" {
        println!("{num1}{operator}{num2} = {}", num1 - num2);
    } else if operator == "*" {
        println!("{num1}{operator}{num2} = {}", num1 * num2);
    } else if operator == "/" {
        println!("{num1}{operator}{num2} = {}", num1 / num2);
    } else {
        println!("Not a valid operator");
    }
}
