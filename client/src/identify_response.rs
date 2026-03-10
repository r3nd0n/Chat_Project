//use std:: {io, str};
//use serde::{Serialize, Deserialize};
use serde_json::json;

pub fn build_identify(username: &String) -> String {
    let user = username;

    let identifyer = json!({
        "type": "IDENTIFY",
        "username": user
    });

    return identifyer.to_string();
}