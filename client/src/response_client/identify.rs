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


