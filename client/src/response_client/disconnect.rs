use serde_json::json;

pub fn disconnect_request() -> String {

    let disconnect = json!({
        "type": "DISCONNECT",
    }).to_string() + "\n";

    disconnect
}

