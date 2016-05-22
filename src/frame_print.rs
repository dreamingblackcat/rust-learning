fn main() {
  let ary: [&str;3] = ["hello", "from", "world"];  
  print_frame_from_array(&ary);
}

fn print_frame_from_array(ary: &[&str]) {
  let max: usize = longest_member_of(ary);
  print_stars(max + 2);
  println!("");
  for i in 0..ary.len() {
    print!("*");
    print!("{0}", ary[i]);
    let mut current_len = ary[i].len();
    while current_len < max {
      print!(" ");
      current_len += 1;
    }
    print!("*");
    println!("");
  }
  print_stars(max + 2);
}

fn print_stars(count: usize) {
  for i in 0..count {
    print!("*");
  }
}

fn longest_member_of(ary: &[&str]) -> usize {
  let mut max: usize = 0;
    for i in 0..ary.len() {
        if ary[i].len() > max {
          max = ary[i].len();
        }
    }
    return max;
}