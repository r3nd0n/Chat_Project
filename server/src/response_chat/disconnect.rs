use serde_json::json;

pub fn disconnected_response(username: &str) -> String {
    json!({
        "type": "DISCONNECTED",
        "username": username,
    })
    .to_string()
        + "\n"
}
