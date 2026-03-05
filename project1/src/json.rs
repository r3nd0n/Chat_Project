use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]

struct Users{
    username: String, 
    status: String,
}

// Tengo que usar una clase que ya tenga los elementos de 
//los json que tenemos que parsear.
fn crear_json(mut str: String) {
    
}

// '{"type": "IDENTFY",
//    "username":"nombre",
//   }'