fn main() {
    let mut current_leap_year = 2016;
    let mut vec = Vec::new();
    while vec.len() <= 20 {
      current_leap_year += 1;
      if (current_leap_year % 100 == 0 && current_leap_year % 400 == 0) || ( current_leap_year % 100 != 0 && current_leap_year % 4 == 0) {
        vec.push(current_leap_year);
      }
    }
    println!("{0:?}", vec);

}