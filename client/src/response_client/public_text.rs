use serde_json::json;


pub fn send_msg(message: &str) -> String {

    let new_usr = json!({
        "type": "PUBLIC_TEXT",
        "text": message,
    }).to_string() + "\n";

    new_usr
} 