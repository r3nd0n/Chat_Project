use std::{
    env,
    io::Write,
    net::TcpStream,
    sync::mpsc,
    thread,
};

mod main_functions;
mod response_client;

use crate::main_functions::message_loop::message_loop;
use crate::main_functions::server_reader::ServerReader;
use crate::main_functions::usr_validation::usr_validation;
use crate::response_client::identify::new_usr;

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

    message_loop(stream);
}