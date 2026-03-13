use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifyMessage {
    #[serde(rename = "type")]
    pub r_type: String,
    pub username: String,
}

pub fn parse_identification(raw: &str) -> Result<IdentifyMessage, serde_json::Error>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_to_struct() {
        struct test {
            type: "IDENTIFY",
            name: "Canek",
        }

        let test_json = "{ \"type\":\"IDENTIFY\", \"username\":\"Canek\"}";
        struct comparison{
            type: String,
            name: String,
        } 
        
        let c: comparison = from_json_string(test_json);
        assert_eq!(test, comparison);
    }
}