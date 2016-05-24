fn main() {
  let num = 100;
  assert_eq!(to_digit_list(num), [1, 0, 0]);
  assert_eq!(add_dl(to_digit_list(num), to_digit_list(10)), [1,1,0]);
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

fn add_dl(first: Vec<u32>, second: Vec<u32>) -> Vec<u32> {
  let mut result: Vec<u32> = vec![];
  let mut first_cursor: usize = first.len();
  let mut second_cursor: usize = second.len();
  let mut carry: u32 = 0;

  while first_cursor != 0 || second_cursor != 0 {
    let mut sum:   u32 = carry;
    if first_cursor != 0 {
      sum += first[first_cursor - 1];
      first_cursor -= 1;
    }
    if second_cursor != 0 {
      sum += second[second_cursor - 1];
      second_cursor -= 1;
    }

    if sum > 9 {
      sum = sum % 10;
      carry = sum / 10;
    }
    result.insert(0, sum);
  }
  return result;
}