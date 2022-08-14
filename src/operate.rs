use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first_number = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator = args.nth(0).unwrap().chars().nth(0).unwrap();
    let second_number = args.nth(0).unwrap().parse::<f32>().unwrap();

    println!("{}", output(first_number, operator, second_number));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // short code

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator!"),
    }

    // long code

    // if operator == '+' {
    //     return first_number + second_number;
    // }

    // if operator == '-' {
    //     return first_number - second_number;
    // }

    // if operator == '*' {
    //     return first_number * second_number;
    // }

    // if operator == '/' {
    //     return first_number / second_number;
    // }

    // return 0.0;
}

fn output(first_number: f32, operator: char, second_number: f32) -> String {
    let result = operate(operator, first_number, second_number);
    return format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    );
}
