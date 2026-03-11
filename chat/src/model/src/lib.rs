use serde::{Deserialize, Serialize};
use serde_json::Result;

// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// 
// #[derive(Serialize, Deserialize)]
// struct Address {
//     street: String,
//     city: String,
// }
// 
// fn print_an_address() -> Result<()> {
//     // Some data structure.
//     let address = Address {
//         street: "10 Downing Street".to_owned(),
//         city: "London".to_owned(),
//     };
// 
//     // Serialize it to a JSON string.
//     let j = serde_json::to_string(&address)?;
// 
//     // Print, write to a file, or send to an HTTP server.
//     println!("{}", j);
// 
//     Ok(())
// }



pub mod string_lecture {
    pub mod json_serializer{

        //! Read the json string format to
        //! allow server side validate the 
        //! new user registration.
        pub fn identify_from_view() {

        }
    }
}

pub mod struct_lecture {

    //! Takes the keys and values in a 
    //! Rust struct to extrat data
    //! and use it to serialize it to
    //! a JSON string.
    pub fn write_json_str(struc: struct) -> Result<()> {
        serde_json::to_string(&struct)?;
        Ok(())
    }

    //! Uses a JSON String to fill values
    //! in a Rust struct type.
    pub fn write_to_struct(_str: &str) -> Result<()> {
        serde_json::from_str(_str)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_write_from_struct() {
        #[derive(Serialize, Deserialize)]
        struct Address {
            street: String,
            city: String,
        }
        
        let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
        };

        let created = write_json_str(&address);
        let test_att = r#"{"street":"10 Downing Street","city":"London"}"#
        assert_eq!(created, test_att);
    }
}
