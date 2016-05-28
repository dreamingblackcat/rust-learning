fn main() {
  let ary: Vec<u32> = vec!(1,2,3,4,5,6);
  let res = rotate_list(ary, 2);
  assert_eq!(res, [3,4,5,6,1,2]);
  assert_eq!(rotate_list(vec!(1,2,3,4,5,6), 4), [5,6,1,2,3,4]);
  assert_eq!(rotate_list(vec!(1,2,3,4,5,6), 0), [1,2,3,4,5,6]);
  println!("Test Passed!");
}


fn rotate_list(mut ary: Vec<u32>, turn_count: u32) -> Vec<u32> {
    let mut i = 1;
    while i <= turn_count {
      let temp = ary[0];
      let last_index = ary.len() - 1;
      for j in 1..ary.len() {
        ary[j-1] = ary[j];
      }
      ary[last_index] = temp;
      i += 1;
    }
    return ary;
}