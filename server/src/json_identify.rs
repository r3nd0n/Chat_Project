use serde_json::json;

//! success_response
//! @param un nombre de usuario.
//! Identifica, si no existe el username, a un nuevo
//! usuario en el servidor.
pub fn success_response(username: &String) -> String{
    let user = username;

    let response = json!({ 
        "type": "RESPONSE",
        "operation": "IDENTIFY",
        "result": "SUCCESS",
        "extra": user
    });

    return response.to_string();
}

//! new_usr
//! @param un nombre de usuario.
//! Manda una notificación a los usuarios conectados
//! cuando un nuevo usuario se conecta al servidor.
pub fn new_usr(username: &String) -> String{
    let user = username;

    let new_usr = json!({ 
        "type": "NEW_USER",
        "extra": user
    });

    return response.to_string();
}


//! fail_identify
//! @param un nombre de usuario.
//! Crea una respuesta cuando el intento de identificarse
//! existe el nombre de usuario en el sistema.
pub fn fail_identify(username: &String) -> String{
    let user = username;

    let fail = json!({ 
        "type": "NEW_USER",
        "extra": user
    });

    return fail.to_string();
}
