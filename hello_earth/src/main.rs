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

}
