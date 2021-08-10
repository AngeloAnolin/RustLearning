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
}
