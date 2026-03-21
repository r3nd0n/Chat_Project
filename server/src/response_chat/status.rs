use serde_json::json;


pub fn new_status_response(status: &str, username: &str) -> String {

    let new_usr = json!({ 
        "type": "NEW_STATUS",
        "username": username,
        "status": status,
    }).to_string() + "\n";

    new_usr
} 