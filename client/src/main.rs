use std::{
    env,
    io::{self, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
};

pub mod response_client;
pub mod server_reader;
pub mod usr_validation;

use crate::response_client::identify::new_usr;
use crate::response_client::public_text::send_msg;
use crate::server_reader::ServerReader;
use crate::usr_validation::usr_validation;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("Uso: cargo run -- <host:port>");
        return;
    }

    let direction: &String = &arguments[1];
    let mut stream = TcpStream::connect(direction)
     .expect("No se pudo conectar a servidor");
    let reader_stream = stream
        .try_clone()
        .expect("No se pudo clonar stream para lectura");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut worker = ServerReader::new(reader_stream, tx);
        worker.run();
    });

    println!("Elige un username: ");
    let username = usr_validation();
    let json = new_usr(&username);

    if let Err(e) = stream.write_all(json.as_bytes()) {
        eprintln!("Err. escritura al servidor: {e}");
        return;
    }

    // Esperar respuesta del servidor sobre IDENTIFY
    match rx.recv() {
        Ok(result) => {
            if result != "SUCCESS" {
                eprintln!("Identificación fallida: {}", result);
                return;
            }
        }
        Err(_) => {
            eprintln!("Error esperando respuesta de identificación.");
            return;
        }
    }

    println!("Bienvenido {}! \nComienza a chatear.", username);

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