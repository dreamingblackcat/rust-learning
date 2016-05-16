
fn main() {
    let mut arr: [i32; 10] = [3, 6, 10, 5, 4, 1, 2, 8, 9, 7];
    let n: usize = arr.len();
    let mut min: i32;
    print!("{:?}", arr);
    for i in 0..n {
      min = arr[i];
      for j in (i+1)..n {
          if arr[j] < min {
            let temp = min;
            min = arr[j];
            arr[j] = temp; 
          }
      }
      arr[i] = min;
    }
    print!("{:?}", arr);
}