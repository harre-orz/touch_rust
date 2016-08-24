use std::env;

fn main() {
	let name = env::args().skip(1).next().unwrap_or("Someone".to_string());
	println!("hello world! {} san!", name);
}
