use std::env::args;
use std::process::exit;

fn main() {
  let args: Vec<String> = args().collect();
  if args.len() == 4 {
    let first_number = match args[1].parse::<i32>() {
      Ok(n) => n,
      Err(_) => {
        println!("first_number must be an integer");
        exit(1);
      }
    };
    let operator = match &args[2][..] {
      "+" | "-" | "*" | "X" | "x" | "/" | "%" => args[2].clone(),
      _ => {
        println!("Invalid operator");
        exit(1);
      }
    };
    let second_number = match args[3].parse::<i32>() {
      Ok(n) => n,
      Err(_) => {
        println!("second_number must be an integer");
        exit(1);
      }
    };

    let result = operate(&operator, first_number, second_number);
    println!("{}", output(&operator, first_number, second_number, result));
  } else {
    println!("Usage: calc <first_number> <operator> <second_number>");
    exit(1);
  }
}

fn operate(operator: &str, first_number: i32, second_number: i32) -> i32 {
  match operator {
    "+" => first_number + second_number,
    "-" => first_number - second_number,
    "*" | "x" | "X" => first_number * second_number,
    "/" => first_number / second_number,
    "%" => first_number % second_number,
    _ => {
      println!("Invalid operator");
      exit(1);
    }
  }
}

fn output(operator: &str, first_number: i32, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}
