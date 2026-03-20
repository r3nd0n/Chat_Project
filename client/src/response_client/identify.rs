//use std:: {io, str};
//use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::Value;

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

pub fn parse_identify_response(json_str: &str) -> Option<String> {
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(val) => {
            let result = val.get("result").and_then(|v| v.as_str())?;
            match result {
                "SUCCESS" => Some("SUCCESS".to_string()),
                "USER_ALREADY_EXISTS" => Some("USER_ALREADY_EXISTS".to_string()),
                _ => None,
            }
        }
        Err(_) => None,
    }
}

pub fn format_new_user(json_str: &str) -> Option<String> {
    let val: Value = serde_json::from_str(json_str).ok()?;
    let msg_type = val.get("type")?.as_str()?;

    if msg_type != "NEW_USER" {
        return None;
    }

    let username = val.get("username")?.as_str()?;
    Some(format!("{} se unio al chat", username))
}

