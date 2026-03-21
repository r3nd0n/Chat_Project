use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::json;

use crate::main_functions::users_collection::ListOfUsers;

fn user_list_response(users: &Arc<Mutex<ListOfUsers>>) -> String {
    let users_guard = users.lock().expect("Error: users lock poisoned");

    let users_map: HashMap<String, String> = users_guard
        .list
        .iter()
        .map(|(username, user)| (username.clone(), user.status.clone()))
        .collect();

    let mut response = json!({
        "type": "USER_LIST",
        "users": users_map,
    })
    .to_string();
    response.push('\n');

    response
}

pub fn get_list(users: &Arc<Mutex<ListOfUsers>>) -> String {
    user_list_response(users)
}