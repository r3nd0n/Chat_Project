//use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::{
    net::TcpStream,
    Error
};

pub fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Conexión desde {}", stream.peer_addr()?);
    let mut buffer = [0,1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {return Ok(());}
        stream.write(&buffer[..bytes_read])?;
    }
}
