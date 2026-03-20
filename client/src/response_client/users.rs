use serde_json::json;
use serde_json::Value;

pub fn get_users() -> String {
    json!({
        "type": "USERS"
    })
    .to_string()
        + "\n"
}

pub fn parse_user_list(raw: &str) -> Option<Vec<(String, String)>> {
    let value: Value = serde_json::from_str(raw).ok()?;
    let msg_type = value.get("type")?.as_str()?;
    if msg_type != "USER_LIST" {
        return None;
    }

    let users = value.get("users")?.as_object()?;
    let mut rows = Vec::with_capacity(users.len());

    for (username, status_value) in users {
        let status = status_value.as_str()?.to_string();
        rows.push((username.clone(), status));
    }

    rows.sort_by(|a, b| a.0.cmp(&b.0));
    Some(rows)
}
