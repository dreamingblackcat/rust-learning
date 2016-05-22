fn main() {
  let ary: [&str;3] = ["hello", "from", "world"];  
  print_frame_from_array(&ary);
}

fn print_frame_from_array(ary: &[&str]) {
  let max: usize = longest_member_of(ary);
  print_stars(max + 2);
  println!("");
  for i in 0..ary.len() {
    print_frame(ary[i], max)
  }
  print_stars(max + 2);
}

fn print_frame(text: &str, max_size: usize) {
  print!("*");
  print!("{0}", text);
  space_padded_print(text, max_size);
  print!("*");
  println!("");
}

fn space_padded_print(text: &str, max_size: usize) {
  let mut current_len = text.len();
  while current_len < max_size {
    print!(" ");
    current_len += 1;
  }
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