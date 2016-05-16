fn main() {
    let mut arr: [i32; 10] = [3, 6, 10, 5, 4, 1, 2, 8, 9, 7];
    let n: usize = arr.len();
    let mut j: usize;
    print!("{:?}", arr);
    for i in 0..n {
      j = i;
      while j > 0 && arr[j] < arr[j-1] {
          let temp = arr[j-1];
          arr[j-1] = arr[j];
          arr[j] = temp; 
          j -= 1;
      }
    }
    print!("{:?}", arr);
}