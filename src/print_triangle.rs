//  print below if n = 4. Write general one with configurable `n` value.
//    *
//   ***
//  *****
// *******


fn main() {
  let n = 5;
  let mut space_counter = n;
  let mut star_counter = 1;
  let mut i:u32 = 0;
  let mut space:u32;
  let mut star:u32;

  while i < n {    
    space_counter -= 1;

    space = 0;
    while space < space_counter {
        print!(" ");
        space += 1;
    }
    star = 0;
    while star < star_counter {
        print!("*");
        star += 1;
    }
    println!("");
    star_counter  += 2;
    i += 1;
  }
}