pub fn main() {
  // Print to console
  println!("Hello from print.rs file");

  // Basic formatting
  println!("{} is from {}", "Fadhil", "Mass");

  // Positional arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Fadhil", "Mass", "Code"
  );

  // Named arguments
  println!(
    "{name} likes to play {activity}",
    name = "Fadhil",
    activity = "Baseball"
  );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
