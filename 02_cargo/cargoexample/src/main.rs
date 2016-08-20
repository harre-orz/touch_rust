extern crate simplelib;

use simplelib::mylib;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("no args error");
        process::exit(1)
    }
    let x = args.nth(1).unwrap().parse::<i32>().unwrap();
    let y = mylib::twice(x);
    println!("{:?} is twice of {:?}", y, x);
}
