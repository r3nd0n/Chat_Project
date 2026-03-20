use std::io::{self, Write};
use std::net::TcpStream;

use crate::response_client::public_text::send_msg;

pub fn message_loop(mut stream: TcpStream) {
    loop {
        let mut message = String::new();
        if io::stdin().read_line(&mut message).is_err() {
            eprintln!("Err. lectura de stdin.");
            break;
        }

        let message = message.trim();
        if message.is_empty() {
            continue;
        }

        let json = match send_msg(message) {
            Ok(payload) => payload,
            Err(msg) => {
                eprintln!("{msg}");
                continue;
            }
        };
        if let Err(e) = stream.write_all(json.as_bytes()) {
            eprintln!("Err. escritura al servidor: {e}");
            break;
        }
    }
}
