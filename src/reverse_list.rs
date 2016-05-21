fn main() {
  let mut arr = [1, 2, 3, 4, 5, 6];
  reverse(&mut arr);  
  println!("{0:?}", arr);
}

fn reverse(arr: &mut [u32]) {
  let mut j = arr.len() - 1;
  let mut i = 0;
  while i <= j {
   let temp = arr[i];
   arr[i] = arr[j];
   arr[j] = temp;
   i += 1;
   j -= 1;   
  }
}