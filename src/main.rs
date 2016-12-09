extern crate reqwest;

use std::io::Read;

fn get_ip() -> String {
	let mut res = reqwest::get("https://api.ipify.org").unwrap();
	assert!(res.status().is_success(), "Recieved an invalid status from ipify");

	let mut ip = String::new();
	res.read_to_string(&mut ip).unwrap();

	ip
}

fn main() {
    println!("for thee {:?}", get_ip());
}