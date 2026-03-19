use std::{
    env,
    io::{self, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
};

pub mod response_client;
pub mod server_reader;

use crate::response_client::identify::new_usr;
use crate::response_client::public_text::send_msg;
use crate::server_reader::ServerReader;

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

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Err. lectura de stdin.");
        return;
    }

    let username = input.trim();
    if username.is_empty() {
        eprintln!("Username vacio.");
        return;
    }

    let json = new_usr(username);
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

    println!("Conectado. Escribe mensajes para enviarlos como PUBLIC_TEXT.");

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

        let json = send_msg(message);
        if let Err(e) = stream.write_all(json.as_bytes()) {
            eprintln!("Err. escritura al servidor: {e}");
            break;
        }
    }
}