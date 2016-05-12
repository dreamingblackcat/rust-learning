fn main() {

  // Type annotated variable
  let age: i32 = 10;

  // Non-annotated with default type 

  let drinking_age = 18;

  println!("Your age is {}", age);
  println!("{} more years required for you to drink", drinking_age - age);
}