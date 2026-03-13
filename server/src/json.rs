use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Identify {
    #[serde(rename = "type")]
    pub r_type: String,
    pub username: String,
}

pub fn parse_identify(raw: &str) -> Result<Identify, serde_json::Error> {
    serde_json::from_str(raw)
}



