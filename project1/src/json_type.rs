use std:: io;
use serde::{Serialize, Deserialize};
use serde_json::Result;



// Crea el tipo identificdor 
//
pub fn id(mut str: String) -> String{
    let mut usr_name = String;

    // aquí tengo que recibir una entrada del ususario para nombre.
    return("{ \"type\": \"IDENTIFY\", \"username\": {} }", usr_name);
}


// envi el mensaje de nuevo usuario
//
pub fn id_new_usr(mut str: String) -> String{
    let mut usr_name = String;

    // aquí tengo que recibir una entrada del ususario para nombre.
    return("{ \"type\": \"NEW_USER\", \"username\": {} }", usr_name);
}


// Crea la repsuesta en caso de que el usuario se haya conectado con exito.
//
pub fn id_response(mut str: String) -> String{
    let mut usr_name = String;

    // aquí tengo que recibir una entrada del ususario para nombre.
    return("{ \"type\": \"RESPONSE\", 
     \"operation\": \"IDENTIFY\",
     \"result\": \"SUCCESS\",
     \"username\": {} }", usr_name);
}
