use serde_json::json;

pub fn private_text_from(sender: &str, text: &str) -> String {
    json!({
        "type": "TEXT_FROM",
        "username": sender,
        "text": text,
    })
    .to_string() + "\n"
}

pub fn private_text_no_such_user(username: &str) -> String {
    json!({
        "type": "RESPONSE",
        "operation": "TEXT",
        "result": "NO_SUCH_USER",
        "extra": username,
    })
    .to_string() + "\n"
}
