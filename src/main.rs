extern crate reqwest;

use std::io::Read;

fn get_ip() -> String {
    
	let mut res = match reqwest::get("https://api.ipify.org") {
        Ok(val) => val,
        Err(err) => panic!("There was a network error when trying to connect to ipify.")
    };
	assert!(res.status().is_success(), "Received an invalid status from ipify");

	let mut ip = String::new();
	res.read_to_string(&mut ip).unwrap();

	ip
}

fn main() {
    println!("for thee {:?}", get_ip());
}
