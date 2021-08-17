fn main() {
  let numbers_list = [1, 2, 3, 4, 5];
  let string_list = [
    "January", "February", "March", "April",
    "May", "June", "July", "August", 
    "Septembeber", "October", "November", "December"
  ];

  let _numbers_list2:[u32;5] = [1, 2, 3, 4, 5];
  let same = ["Hi"; 10];

  println!("{}", numbers_list[0]);
  println!("{:?}", same);

  // Traversing the array
  let mut idx= 0;
  
  while idx < 12 {
    println!("The month at index {} is {}", idx, string_list[idx]);
    idx = idx + 1;
  }
}
