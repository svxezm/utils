use std::io::{self, Write};

fn input_loop(text: &str) -> f64 {
    let result: f64 = loop {
        print!("{}", text);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please, insert only numbers."),
        }
    };

    result
}

fn main() {
    let mut first_number = input_loop("Insert the first number: ");

    loop {
        let operator = loop {
            print!("Insert the operator (+, -, *, /): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let op = input.trim().chars().next();

            match op {
                Some('+') | Some('-') | Some('*') | Some('/') => break op.unwrap(),
                _ => println!("Operator not available. Please, use +, -, * or /."),
            };
        };

        let last_number = loop {
            let num = input_loop("Insert the second number: ");
            if operator == '/' && num == 0.0 {
                println!("Cannot divide by zero. Try another number.");
            } else {
                break num;
            }
        };

        let result = match operator {
            '+' => first_number + last_number,
            '-' => first_number - last_number,
            '*' => first_number * last_number,
            '/' => first_number / last_number,
            _ => unreachable!(),
        };

        println!(
            "\n{} {} {} = {}",
            first_number, operator, last_number, result
        );
        print!("\nDo you wish to continue the operation? [y/N] ");
        io::stdout().flush().unwrap();

        let mut continue_operation = String::new();
        io::stdin().read_line(&mut continue_operation).unwrap();
        match continue_operation.trim() {
            "Y" | "y" => first_number = result,
            _ => {
                println!("\nFinal value: {}", result);
                break;
            }
        };
    }
}
