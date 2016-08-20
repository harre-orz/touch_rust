use std::env;

fn main() {
	let mut args = env::args();
	let mut name = "Someone".to_string();
	if args.len() > 1 {
		name = args.nth(1).unwrap();
	}
	println!("hello world! {} san!", name);
}
