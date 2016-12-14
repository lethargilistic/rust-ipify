extern crate reqwest;

use std::io::Read;
use std::time::Duration;
use std::thread;

const MAX_RETRIES: i32 = 3;
const RETRY_WAIT_TIME : u64 = 500;
static API_URL: &'static str = "https://api.ipify.org";

fn get_response(tries_remaining:i32) -> reqwest::Response {
    match reqwest::get(API_URL) {
        Ok(response) => response,
        Err(err) => 
            if tries_remaining > 0 {
                thread::sleep(Duration::from_millis(RETRY_WAIT_TIME));
                get_response(tries_remaining-1)
            }
            else{
                panic!("There was a network error when trying to connect to ipify.\n{}", err);
            }
    }
}

pub fn get_ip() -> String {
    let mut res = get_response(MAX_RETRIES);
    
    assert!(res.status().is_success(), "Received an invalid status from ipify");

    let mut ip = String::new();
    res.read_to_string(&mut ip).unwrap();

    ip
}  
