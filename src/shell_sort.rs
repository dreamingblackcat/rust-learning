fn main() {
    print!("hello");
    let mut arr: [i32; 10] = [3, 6, 1,4, 9, 10, 2, 8, 5, 7];
    let n: usize = arr.len();
    let mut j: usize;
    let mut i: usize;
    let mut h: usize = 1;

    while h < (n / 3) {
      print!("{:?}", h);
      h = h * 3 + 1;
    }
    println!("{:?}", arr);

    while h >= 1 {
      println!("{:?}", h);
      i = h;
      while i < n {
        j = i;
        while j >= h && arr[j] < arr[j-h] {
            let temp = arr[j-h];
            arr[j-h] = arr[j];
            arr[j] = temp; 
            j -= h;
        }
        println!("{0} => {1:?}", i, arr );
        i += 1;
      }
      println!("Phase {0}  over => {1:?}", h, arr );
      h = (h - 1) / 3;
    }
    
    print!("{:?}", arr);
}