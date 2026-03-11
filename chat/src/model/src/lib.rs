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



pub mod read_from_string {
    pub mod json_serializer{

        //! Takes the information in a json 
        //! string format to extrat data
        //! and use it to create a Rust
        //! struct.
        pub fn from_json_string() {

        }

        //! Read the json string format to
        //! allow server side validate the 
        //! new user registration.
        pub fn identify_from_view() {

        }
    }
}

mod read_from_struct {
    mod json_deserializer{

        //! Uses the data in a Rust struct to 
        //! create a json string format to use
        //! in the server side as a response for
        //! the client side.
        fn from_struct() {

        }
    }
}

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
