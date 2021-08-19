fn main() {
  let numbers_list = [1, 2, 3, 4, 5];
  let string_list = [
    "January", "February", "March", "April",
    "May", "June", "July", "August", 
    "Septembeber", "October", "November", "December"
  ];

  let numbers_list2:[u32;5] = [1, 2, 3, 4, 5];
  let same = ["Hi"; 10];

  println!("{}", numbers_list[0]);
  println!("{:?}", same);

  // Traversing the array
  let mut idx= 0;
  
  while idx < 12 {
    println!("The month at index {} is {}", idx, string_list[idx]);
    idx = idx + 1;
  }
  
  // Using for loop
  for i in &numbers_list2 {
    println!("{}", i);
  }

  // Using iterator
  let string_list_iter = string_list.iter();

  // The variable needs to be cloned as once it is used
  // the reference to the heap where the variable points to 
  // is already gone.
  let string_list_iter_clone = string_list_iter.clone();
  let string_list_iter_clone_reverse = string_list_iter.clone();
  println!("variable string_list has {} elements.", string_list_iter.count());

  
  for j in string_list_iter_clone {
    println!("Month {} has {} characters", j, j.len());
  }

  // Read iteration from end to start and reverse the characters as well.
  for k in string_list_iter_clone_reverse.rev() {
    let k_reversed = k.chars().rev().collect::<String>();

    println!("Reading month {} in reverse is {}", k, k_reversed);
  }

  // Call function chapter6 on tuples
  chapter6_tuples();

  // Call function to chapter6 on vectors.
  chapter6_vectors();
}

fn chapter6_tuples () {
  // Tuple allow variables inside the set to be different.
  let tuple_var = (1, 2.5, 3, "String");
  println!("Position 0 is {}", tuple_var.0);
  println!("Position 3 is {}", tuple_var.3);

  // When the type is declared for tuples, it should match.
  let tuple_declare: (u32, &str, i32) = (100, "Hello", -100);
  println!("Item at 0 is {}", tuple_declare.0);
  println!("Item at 1 is {}", tuple_declare.1);

}

fn chapter6_vectors () {
  // Declare vector.
  let mut vector_var = vec![2, 4, 6, 8, 10];
  vector_var.push(20);
  println!("Vector value is: {:?}", vector_var);

  vector_var.push(100);
  println!("After push 100 to vector, the value now is: {:?}", vector_var);

  // Similar to array, iterating through vectors can be done:
  for i in &vector_var {
    println!("Value of i is {}", i);
  }

  let vector_var_iter = vector_var.iter();

  for j in vector_var_iter {
    println!("Reading j {}", j);
  }
}