

fn merge(mut arr: [u32;9], low: usize, mid: usize, high: usize) -> [u32;9]{
  let mut aux: [u32;9] = [0;9];
  let mut j: usize;
  let mut k: usize;
  j = low;
  k = mid + 1; 
  for i in low..high+1 {
     aux[i] = arr[i];
  }

  for i in low..high+1 {
    if j > mid {
      arr[i] = aux[k];
      k += 1; 
    } else if k > high {
      arr[i] = aux[j];
      j += 1;
    } else if aux[j] < aux[k] {
      arr[i] = aux[j];
      j += 1
    } else {
      arr[i] = aux[k];
      k += 1;
    }
  }
  return arr;
}

fn merge_test() {
  let ary: [u32; 9] = [3, 2, 9, 7, 1, 8, 4, 6, 5];
  let sorted: [u32; 9] = merge(ary, 0, 4, 8);
  assert_eq!([3, 2, 8, 4, 6, 5, 9, 7, 1], sorted);
  println!("Test Passed!");
}


fn main() {
  merge_test();
}