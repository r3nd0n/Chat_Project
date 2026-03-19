use serde_json::json;
use serde_json::Value;


pub fn send_msg(message: &str) -> String {

    let new_usr = json!({
        "type": "PUBLIC_TEXT",
        "text": message,
    }).to_string() + "\n";

    new_usr
}

pub fn format_public_text_from(raw: &str) -> Option<String> {
    let value: Value = serde_json::from_str(raw).ok()?;
    let msg_type = value.get("type")?.as_str()?;

    if msg_type != "PUBLIC_TEXT_FROM" {
        return None;
    }

    let username = value.get("username")?.as_str()?;
    let text = value.get("text")?.as_str()?;

    Some(format!("{}: {}", username, text))
}