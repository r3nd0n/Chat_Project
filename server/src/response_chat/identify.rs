use serde_json::json;

//
// IDENTIFY
//

// identify_success_response
// @param un nombre de usuario.
// Identifica, si no existe el username, a un nuevo
// usuario en el servidor.
pub fn identify_success_response(username: &str) -> String {

    let response = json!({ 
        "type": "RESPONSE",
        "operation": "IDENTIFY",
        "result": "SUCCESS",
        "extra": username
    }).to_string() + "\n";

    response
}

// identify_new_usr
// @param un nombre de usuario.
// Manda una notificación a los usuarios conectados
// cuando un nuevo usuario se conecta al servidor.
pub fn identify_new_usr(username: &str) -> String {

    let new_usr = json!({ 
        "type": "NEW_USER",
        "username": username
    }).to_string() + "\n";

    new_usr
}


// identify_usr_exists
// @param un nombre de usuario.
// Crea una respuesta cuando el intento de identificarse
// existe el nombre de usuario en el sistema.
pub fn identify_usr_exists(username: &str) -> String {

    let usr_exists = json!({ 
        "type": "RESPONSE",
        "operation": "IDENTIFY",
        "result": "USER_ALREADY_EXISTS",
        "extra": username
    }).to_string() + "\n";

    usr_exists
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn identify_success_json_correct() {
        let raw = identify_success_response("Mateo");
        let got: Value = serde_json::from_str(raw.trim()).expect("JSON invalido");

        assert_eq!(got["type"], "RESPONSE");
        assert_eq!(got["operation"], "IDENTIFY");
        assert_eq!(got["result"], "SUCCESS");
        assert_eq!(got["extra"], "Mateo");
    }

    #[test]
    fn identify_success_equivalent_json() {
        let got: Value = serde_json::from_str(identify_success_response("Mateo").trim())
            .expect("JSON invalido de respuesta");
        let expected: Value = serde_json::from_str(
            r#"{ "type":"RESPONSE", "operation":"IDENTIFY", "result":"SUCCESS", "extra":"Mateo" }"#,
        )
        .expect("JSON invalido esperado");

        assert_eq!(got, expected);
    }

    #[test]
    fn identify_user_already_exists_json_correct() {
        let raw = identify_usr_exists("Mateo");
        let got: Value = serde_json::from_str(raw.trim()).expect("JSON invalido");

        assert_eq!(got["type"], "RESPONSE");
        assert_eq!(got["operation"], "IDENTIFY");
        assert_eq!(got["result"], "USER_ALREADY_EXISTS");
        assert_eq!(got["extra"], "Mateo");
    }

    #[test]
    fn identify_user_already_exists_equivalent_json() {
        let got: Value = serde_json::from_str(identify_usr_exists("Mateo").trim())
            .expect("JSON invalido de respuesta");
        let expected: Value = serde_json::from_str(
            r#"{ "type":"RESPONSE", "operation":"IDENTIFY", "result":"USER_ALREADY_EXISTS", "extra":"Mateo" }"#,
        )
        .expect("JSON invalido esperado");

        assert_eq!(got, expected);
    }

    #[test]
    fn identify_new_user_json_correct() {
        let raw = identify_new_usr("Luis");
        let got: Value = serde_json::from_str(raw.trim()).expect("JSON invalido");

        assert_eq!(got["type"], "NEW_USER");
        assert_eq!(got["username"], "Luis");
    }

    #[test]
    fn identify_new_user_equivalent_json() {
        let got: Value = serde_json::from_str(identify_new_usr("Luis").trim())
            .expect("JSON invalido de respuesta");
        let expected: Value =
            serde_json::from_str(r#"{ "type":"NEW_USER", "username":"Luis" }"#)
                .expect("JSON invalido esperado");

        assert_eq!(got, expected);
    }
}