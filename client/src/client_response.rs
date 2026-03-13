//use std:: {io, str};
//use serde::{Serialize, Deserialize};
use serde_json::json;

pub fn new_usr(username: &String) -> String {

    let identifyer = json!({
        "type": "IDENTIFY",
        "username": username
    }).to_string() + "\n";

    return identifyer;
}

pub fn new_status(username:&String, status: &String) -> String {

    let identifyer = json!({
        "type": "NEW_STATUS",
        "username": username,
        "status" :status
    }).to_string() + "\n";

    return identifyer;
}