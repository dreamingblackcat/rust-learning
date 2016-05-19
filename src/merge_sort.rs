

fn merge(mut arr: &mut [u32;9], low: usize, mid: usize, high: usize) {
  let mut aux: [u32;9] = [0;9];
  let mut j: usize;
  let mut k: usize;
  j = low;
  k = mid + 1; 

  for i in 0..arr.len() {
      aux[i] = arr[i];
  }
  println!("before {:?}", arr);

  for i in low..high +1 {
    println!("{0}:{1}:{2}", i, j, k);
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
  println!("return {:?}", arr);
  return;
}

fn merge_sort(mut arr: &mut [u32;9], low: usize, high: usize) {
  if high <= low {
    println!("Loop end here with high {0} and low {1}", high, low);
    return;
  }
  let mid:usize = low + (high - low) / 2;
  println!("mid {0}:{1}:{2}", mid, low, high);
  merge_sort(&mut arr, low, mid);
  merge_sort(&mut arr, mid+1, high);
  return merge(&mut arr, low, mid, high);  

}

fn merge_test() {
  let mut ary: [u32;9] = [2, 3, 7, 9, 1, 4, 5, 6, 8];
  merge(&mut ary, 0, 3, 8);
  assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], ary);
  
}

fn merge_sort_test() {
  let mut ary: [u32;9] = [3, 2, 9, 7, 1, 8, 4, 6, 5];
  let h = ary.len() as usize;
  merge_sort(&mut ary, 0usize, h-1);
  assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], ary);
  println!("Test Passed!");
}


fn main() {
  merge_test();
  merge_sort_test();
}