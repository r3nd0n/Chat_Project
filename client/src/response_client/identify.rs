//use std:: {io, str};
//use serde::{Serialize, Deserialize};
use serde_json::json;

fn identify_request(username: &str) -> String {

    let identifyer = json!({
        "type": "IDENTIFY",
        "username": username
    }).to_string() + "\n";

    identifyer
}

pub fn new_usr(username: &str) -> String {

    identify_request(username)
}

