use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut path = Path::new("./pass.txt");
    let mut f = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open pass.txt: {0}", why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
      Err(_) => panic!("couldn't read string"),
      Ok(_) => println!("Password is {0}", s)
    }
    
}
