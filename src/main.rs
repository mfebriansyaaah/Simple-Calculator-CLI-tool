use std::env;
use std::error::Error;
use std::process;
mod calculate;

fn main() -> Result<(), Box<dyn Error>> {
   let args: Vec<String> = env::args().collect();

   if args.len() != 4 {
      eprintln!("How to use: {} <first_number> <operator> <second_number>", args[0]);
      eprintln!("Supported operators: +, -, x, /");
      process::exit(1);
   }

   let num1_str = &args[1];
   let operator: &str = &args[2];
   let num2_str = &args[3];

   let num1: f64 = num1_str.parse().unwrap_or_else(|err| {
      eprintln!("Error parsing first_number '{}': {}", num1_str, err);
      process::exit(1);
   });

   let num2: f64 = num2_str.parse().unwrap_or_else(|err| {
      eprintln!("Error parsing second_number '{}': {}", num2_str, err);
      process::exit(1);
   });

   let result = calculate::calculate(num1, operator, num2);

   match result {
      Ok(value) => {
         println!("The result: {}", value);
      }
      Err(err_msg) => {
         eprintln!("Calculation error: {}", err_msg);
         process::exit(1);
      }
   }

   Ok(())
}