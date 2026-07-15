use std::io;

enum Calculator {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let mut input = String::new();

    println!("enter first number");
    io::stdin().read_line(&mut input).expect("failed");
    let x: f64 = input.trim().parse().expect("invalid number");

    input.clear();

    println!("enter second number");
    io::stdin().read_line(&mut input).expect("failed");
    let y: f64 = input.trim().parse().expect("invalid number");

    input.clear();

    println!("enter operation");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("failed");

    let my_calculator = match z.trim() {
        "+" => Calculator::Add,
        "-" => Calculator::Sub,
        "*" => Calculator::Mul,
        "/" => Calculator::Div,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    match my_calculator {
        Calculator::Add => println!("Answer is {}", x + y),
        Calculator::Sub => println!("Answer is {}", x - y),
        Calculator::Mul => println!("Answer is {}", x * y),
        Calculator::Div => println!("Answer is {}", x / y),
    }
}
