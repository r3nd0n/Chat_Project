use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::net::TcpStream;

pub mod client_side {
    fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
        println!("Conexión desde {}", stream.peer_addr()?);

        let mut buffer = [0,1024];

        loop {
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {return Ok(());}

            stream.write(&buffer[..bytes_read])?;
        }
    }
}

pub mod ser {
    pub mod json_serializer{

        //! Takes the keys and values in a 
        //! Rust struct to extrat data
        //! and use it to serialize it to
        //! a JSON string.
        pub fn struct_to_json(struc: struct) -> Result<()> {
            serde_json::to_string(&struct)?;

            Ok(())
        }

        //! Read the json string format to
        //! allow server side validate the 
        //! new user registration.
        pub fn identify_from_view() {
            
        }
    }
}

pub mod deser {

    

    //! Uses a JSON String to fill values
    //! in a Rust struct type.
    pub fn json_to_struct(_str: &str) -> Result<()> {
        serde_json::from_str(_str)?;

        Ok(())
    }
}


pub mod json_creator {
    pub mod identify {
        
        pub fn build_success_case(username: String) {
            
        }
    }
}

pub mod struct_creator {
    pub mod identify {
        
        pub fn build_user(username: String) -> User {
            User {
                r_type: "IDENTIFY",
                username;
            }
        }
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