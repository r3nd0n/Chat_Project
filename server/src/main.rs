use std::{
    env,
    net::TcpListener,
    thread,
    //collections::HashMap,
};
mod client_manager;
pub mod structs;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("Uso: cargo run -- <host:port>");
        return;
    }

    let direction = &arguments[1];

    let listener = TcpListener::bind(direction)
        .expect("Conexión fallida.");

    println!("Servidor escuchando en {}", direction);

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("Nueva conexión desde; {}", stream
                 .peer_addr()
                 .expect("Err"));

                thread::spawn(move || {
                    client_manager::client_manager(stream);
                });
            }
            Err(e) => {
                eprint!("Error: {}", e)
            }
        }
    }
    drop(listener);
}