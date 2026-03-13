//use std:: {io, str};
//use serde::{Serialize, Deserialize};
use serde_json::json;

pub fn build_identify(username: &String) -> String {

    let identifyer = json!({
        "type": "IDENTIFY",
        "username": username
    }).to_string() + "\n";

    return identifyer;
}