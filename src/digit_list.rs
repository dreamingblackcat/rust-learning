fn main() {
  let num = 100;
  assert_eq!(to_digit_list(num), [1, 0, 0]);
  println!("test passed!");
}

fn to_digit_list(num: u32) -> Vec<u32> {
  let mut current = num;
  let mut list = Vec::new();

  while current > 0 {
    list.insert(0, current % 10);
    current = current / 10;  
  }

  return list;
}