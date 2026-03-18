use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    thread,
};

pub mod response_client;
use crate::response_client::identify::new_usr;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("Uso: cargo run -- <host:port>");
        return;
    }

    let direction: &String = &arguments[1];

    let mut stream = TcpStream::connect(direction).expect("No se pudo conectar a servidor");
    let reader_stream = stream
        .try_clone()
        .expect("No se pudo clonar stream para lectura");

    thread::spawn(move || {
        let mut reader = BufReader::new(reader_stream);

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    print!("{}", line);
                }
                Err(e) => {
                    eprintln!("Error leyendo del servidor: {e}");
                    break;
                }
            }
        }
    });

    println!("Elige un username: ");

    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Err. lectura de stdin.");
            break;
        }

        let username = input.trim();
        if username.is_empty() {
            continue;
        }

        let json = new_usr(username);
        if let Err(e) = stream.write_all(json.as_bytes()) {
            eprintln!("Err. escritura al servidor: {e}");
            break;
        }
    }
}