use std::io;

fn main() {
  let _variable_name = 1;
  println!("Hello, Earth! This is a Rust Application.");

  // Chapter 3 Section 5 Different Ways of using Variables
  let num1 = 5;
  let num2 = 10;
  let sum = num1 + num2;
  println!("The sum of {} and {} is {}", num1, num2, sum);

  // Chapter 3 Section 6 Mutability and Constants
  // Variable is immutable
  let immutable_var = 1;
  println!("Immutable variable value is {}", immutable_var);

  // Variable is mutable
  let mut mutable_var = 10;
  println!("Mutable variable value is {}", mutable_var);

  mutable_var = 100;
  println!("Mutable variable value after changed  is {}", mutable_var);

  // Constant
  const PI:f64 = 3.14;
  println!("Constant variable value is {}", PI);

  // Chapter 3 Section 7 Booleans and Logical Operators
  const CONSTANT_3:i32 = 3;
  const CONSTANT_ANOTHER_3:i32 = 3;
  const ARE_THEY_EQUAL:bool = CONSTANT_3 == CONSTANT_ANOTHER_3;
  println!("CONSTANT_3 with value {} is a {} equal to CONSTANT_ANOTHER_3 whose value is {}", CONSTANT_3, ARE_THEY_EQUAL, CONSTANT_ANOTHER_3);

  // Chapter 3 Section 9 If, Else, and Else If
  let test_number = 0;
  if test_number == 0 {
    println!("Test number {} is Zero", test_number);
  } else if test_number == 1 {
    println!("Test number {} is One", test_number);
  } else {
    println!("Test number {} is neither Zero nor One", test_number);
  }

  // Chapter 3 Section 10 Cool Way of Using Conditional Expressions
  let switch_flag = true;
  let switch_number = if switch_flag { 10 } else { 2 };
  println!("Switch number is {}", switch_number);

  // Chapter 3 Section 11 Loops
  let mut loop_number:i32 = 0;

  while loop_number < 5 {
    println!("The value of loop_number is {}", loop_number);
    loop_number = loop_number + 1;
  }

  // Chapter 3 Section 13 Functions
  let first_num = 100;
  let second_num = 23;
  let product_of_variables = multiply_number(first_num, second_num);
  println!("The product of {} and {} is {}", first_num, second_num, product_of_variables);

  // Chapter 3 Section 14 Introductin to Structures
  let test_structure = TestStructure{
    x: 200,
    y: 100,
    z: true
  };

  println!("The structure value for x is {}, y is {} and z is {}", 
    test_structure.x, test_structure.y, test_structure.z);

  // Chapter 3 Section 15 Rust Standard Library
  let mut string_text = String::from("Hello");
  string_text.push(',');
  string_text.push_str(" Darkness");
  let string_message = string_text + " World";
  println!("{}", string_message);

  // Chapter 3 Section 16 Reading Input from the User
  // Create empty string
  let mut input_text = String::new();

  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");
  
  let converted_to_number: u32 = input_text.trim().parse().expect("Value entered is not a number.");
  
  println!("Value entered is {}", input_text);
  println!("Value converted to number multiplied by 2 is {}", converted_to_number * 2);
}

fn multiply_number(x:i32, y:i32) -> i32 {
  // If no semi-colon is defined, this line will return the operation value. 
  // x*y
  return x*y;
}

// Chapter 3 Section 14 Introductin to Structures
struct TestStructure {
  x:u32,
  y:i32,
  z:bool
}
