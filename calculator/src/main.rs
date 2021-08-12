use std::io;

fn main() {
  let mut first_number = String::new();
  let mut second_number = String::new();
  let mut operation = String::new();

  println!("Please enter the first number.");
  io::stdin()
    .read_line(&mut first_number)
    .expect("Failed to read first number");
  
  let converted_first_number: f32 = first_number
    .trim()
    .parse()
    .expect("First value entered is not a number.");
  
  println!("Please enter the second number.");
  io::stdin()
    .read_line(&mut second_number)
    .expect("Failed to read second number");
  
  let converted_second_number: f32 = second_number
    .trim()
    .parse()
    .expect("Second value entered is not a number.");
  
  println!("Please enter the operation to perform (Add, Subtract, Multiply, Divide, Exponent).");
  io::stdin()
    .read_line(&mut operation)
    .expect("Failed to read operation to perform.");
  
  println!("Selected operation is {}", operation.trim().to_uppercase());
  
  if operation.trim().to_uppercase() == "ADD" {
    let sum:f32 = converted_first_number + converted_second_number;
    println!("The sum of {} and {} is {:.4}", converted_first_number, converted_second_number, sum);
  } else if operation.trim().to_uppercase() == "SUBTRACT" {
    let diff:f32 = converted_first_number - converted_second_number;
    println!("The difference of {} and {} is {:.4}", converted_first_number, converted_second_number, diff);
  } else if operation.trim().to_uppercase() == "MULTIPLY" {
    let prod:f32 = converted_first_number * converted_second_number;
    println!("The product of {} and {} is {:.4}", converted_first_number, converted_second_number, prod);
  } else if operation.trim().to_uppercase() == "DIVIDE" {
    let quotient:f32 = converted_first_number / converted_second_number;
    println!("The quotient of {} and {} is {:.4}", converted_first_number, converted_second_number, quotient);
  } else if operation.trim().to_uppercase() == "EXPONENT" {
    let exponent:f32 =  converted_first_number.powf(converted_second_number);
    println!("The number {} raised by {} is {:.4}", converted_first_number, converted_second_number, exponent);
  } else {
    println!("Operation entered {} is not yet in system", operation.trim().to_uppercase());
  }
}

// Algorithm
// Read two numbers from the input
// Read the operation to be performed
// Perform the operation
// Display the result