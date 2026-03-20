use serde_json::json;
use serde_json::Value;
use crate::response_client::users::get_users;

pub fn send_msg(message: &str) -> Result<String, &'static str> {
    if message == "/users" {
        return Ok(get_users());
    }

    if message.starts_with("/w") {
        return parse_private_command(message).ok_or("Uso: /w <usuario> <mensaje>");
    }

    Ok(
        json!({
        "type": "PUBLIC_TEXT",
        "text": message,
    })
    .to_string()
            + "\n",
    )
}

fn parse_private_command(message: &str) -> Option<String> {
    let rest = message.strip_prefix("/w ")?;
    let mut parts = rest.splitn(2, ' ');
    let username = parts.next()?.trim();
    let text = parts.next()?.trim();

    if username.is_empty() || text.is_empty() {
        return None;
    }

    Some(
        json!({
            "type": "TEXT",
            "username": username,
            "text": text,
        })
        .to_string()
            + "\n",
    )
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

pub fn format_private_text_from(raw: &str) -> Option<String> {
    let value: Value = serde_json::from_str(raw).ok()?;
    let msg_type = value.get("type")?.as_str()?;

    if msg_type != "TEXT_FROM" {
        return None;
    }

    let username = value.get("username")?.as_str()?;
    let text = value.get("text")?.as_str()?;

    Some(format!("[privado] {}: {}", username, text))
}