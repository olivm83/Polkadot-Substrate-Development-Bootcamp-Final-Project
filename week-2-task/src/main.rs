use std::io;

fn main() {
    //first number input
    println!("Enter first number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not valid");
    let num1: f64 = input1.trim().parse().expect("Not a number");
    
    //operator input
    println!("Operator");
    let mut operator_input = String::new();
    io::stdin().read_line(&mut operator_input).expect("Not a valid operator");
    let operator: &str = operator_input.trim();
    
    //second number input
    println!("Enter second number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not valid");
    let num2: f64 = input2.trim().parse().expect("Not a number");

    //match the operator input to specific Operation enum variant
    let operation_enum = match operator {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };
    
    //call the function calculate with the variable operation_enum as its argument, and store the result in the result variable and print
    let result = calculate(operation_enum);
    println!("Result = {}", result);
}

//Operation enum with with two f64 values in each variant.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

//calculate the values according to the matched variant
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
