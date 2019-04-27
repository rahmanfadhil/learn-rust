// Variables are immutable by default

pub fn main() {
  let name = "Fadhil";
  let mut age = 15;
  println!("My name is {} and i am {}", name, age);
  age = 16;
  println!("My name is {} and i am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Fadhil", 15);
  println!("My name is {} and i am {}", my_name, my_age);
}
