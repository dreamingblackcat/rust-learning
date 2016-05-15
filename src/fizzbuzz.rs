fn main() {
   let n = 100;
   for i in 1..n {
    if i % 3 == 0 && i % 5 == 0 {
        println!("{0} : FizzBuzz", i);
    }else if i % 3 == 0 {
        println!("{0} : Fizz", i);
    }else if i % 5 == 0 {
        println!("{0} : Buzz", i);
    }
   }
}