use std:: io;
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct identify{
    #[serde(rename = "type")] //deserializa json type a rust r_type
    r_type : String,
    username: String,
}

pub fn Identificador () -> Result<()> {
    
}