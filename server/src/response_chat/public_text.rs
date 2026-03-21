use serde_json::json;


pub fn msg_response(message: &str, username: &str) -> String {

    let new_usr = json!({ 
        "type": "PUBLIC_TEXT_FROM",
        "username": username,
        "text": message,
    }).to_string() + "\n";

    new_usr
} 