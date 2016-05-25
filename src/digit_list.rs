fn main() {
  let num = 100;
  assert_eq!(to_digit_list(num), [1, 0, 0]);
  assert_eq!(add_dl(to_digit_list(num), to_digit_list(10)), [1,1,0]);
  assert_eq!(compare_dl(&to_digit_list(num), &to_digit_list(10)), 1);
  assert_eq!(compare_dl(&to_digit_list(num), &to_digit_list(1000)), -1);
  assert_eq!(compare_dl(&to_digit_list(num), &to_digit_list(100)), 0);
  assert_eq!(subtract_dl(to_digit_list(num), to_digit_list(10)), [9,0]);
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

fn compare_dl(first: &Vec<u32>, second: &Vec<u32>) -> i32 {
  let mut res: i32 = 0;
  if first.len() > second.len() {
    res = 1;
  } else if first.len() < second.len(){
    res = -1;
  } else {
    let mut cursor: usize = 0;

    while cursor != 0 || cursor != 0 {
      if first[cursor] < second[cursor] {
        res = -1;
        break;
      } else if first[cursor] > second[cursor] {
        res = 1;
        break;
      }
      cursor += 1;
    }
  }
  return res;
}

fn subtract_dl(first: Vec<u32>, second: Vec<u32>) -> Vec<u32> {
  let mut result: Vec<u32> = vec![];
  let mut first_cursor: usize = first.len();
  let mut second_cursor: usize = second.len();
  let mut borrowed: bool = false;
  
  if compare_dl(&first, &second) == -1 {
    panic!("First operand must be greater than or equal to second operand");
  }

  while first_cursor != 0  {
    let mut res: u32 = first[first_cursor - 1];
    if borrowed {
      res -= 1;
    }


    if second_cursor == 0 {
      borrowed = false;
    }

    if second_cursor > 0 && res < second[second_cursor - 1] {
      borrowed = true;
      res += 10;
      res -= second[second_cursor - 1];
      second_cursor -= 1;
    }

    if second_cursor > 0  && res >= second[second_cursor - 1] {
      res -= second[second_cursor - 1];
      second_cursor -= 1;
      borrowed = false;
    }

    
    first_cursor  -= 1;
    if first_cursor == 0 && res == 0{
      break;
    } else {
      result.insert(0, res);
    }
  }
  return result;
}