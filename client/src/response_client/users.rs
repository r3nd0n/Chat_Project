use serde_json::json;

pub fn get_users() {

    let new_status = json!({
        "type": "USERS"
    }).to_string() + "\n";

}
