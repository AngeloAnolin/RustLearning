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

  println!("variable string_list has {} elements.", string_list_iter.count());

  
  for j in string_list_iter_clone {
    println!("Month {} has {} characters", j, j.len());
  }
}
