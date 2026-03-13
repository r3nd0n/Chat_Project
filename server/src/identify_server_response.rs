use serde_json::json;

//
// IDENTIFY
//

//! identify_success_response
//! @param un nombre de usuario.
//! Identifica, si no existe el username, a un nuevo
//! usuario en el servidor.
pub fn identify_success_response(username: &String) -> String{

    let response = json!({ 
        "type": "RESPONSE",
        "operation": "IDENTIFY",
        "result": "SUCCESS",
        "extra": username
    }).to_string() + "\n";

    return response;
}

//! identify_new_usr
//! @param un nombre de usuario.
//! Manda una notificación a los usuarios conectados
//! cuando un nuevo usuario se conecta al servidor.
pub fn identify_new_usr(username: &String) -> String{

    let new_usr = json!({ 
        "type": "NEW_USER",
        "extra": user
    }).to_string() + "\n";

    return new_usr;
}


//! identify_usr_exists
//! @param un nombre de usuario.
//! Crea una respuesta cuando el intento de identificarse
//! existe el nombre de usuario en el sistema.
pub fn identify_usr_exists(username: &String) -> String{

    let usr_exists = json!({ 
        "type": "RESPONSE",
        "operation": "IDENTIFY",
        "result": "USER_ALREADY_EXISTS",
        "extra": username
    }).to_string() + "\n";

    return usr_exists;
}

//
// NEW USER
//

pub fn new_usr_response(username: &String) -> String{

    let new_usr = json!({ 
        "type": "NEW_USER",
        "username": username
    }).to_string() + "\n";

    return usr_exists;
}

//
// NEW STATUS
//

pub fn new_status_response(status: &String, username: &String) -> String{

    let new_usr = json!({ 
        "type": "NEW_STATUS",
        "username": username,
        "status": status,
    }).to_string() + "\n";

    return usr_exists;
}
