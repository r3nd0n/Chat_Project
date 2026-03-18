use serde_json::json;

pub fn new_status(status: &String) -> String {

    let new_status = json!({
        "type": "STATUS",
        "status" :status
    }).to_string() + "\n";

    return new_status;
}