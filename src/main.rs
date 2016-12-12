extern crate reqwest;

use std::io::Read;
use std::time::Duration;
use std::thread;

fn get_response(tries_remaining:i32) -> reqwest::Response {
	match reqwest::get("https://api.ipify.org") {
        Ok(response) => response,
        Err(err) => 
            if tries_remaining > 0 {
                thread::sleep(Duration::from_millis(500));
                get_response(tries_remaining-1)
            }
            else{
                panic!("There was a network error when trying to connect to ipify.\n{}", err);
            }
    }
}
fn get_ip() -> String {
    let mut res = get_response(3);
    
	assert!(res.status().is_success(), "Received an invalid status from ipify");

	let mut ip = String::new();
	res.read_to_string(&mut ip).unwrap();

	ip
}

fn main() {
    println!("for thee {:?}", get_ip());
}
