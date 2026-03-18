use std::{
    collections::HashMap,
    env,
    net::TcpListener,
    thread,
    sync::{Arc, Mutex}, // Para manejar los hilos que usan la misma estructura (Atomic Reference Counted).
};

mod client_manager;
mod response_chat;
mod users_collection;
mod structs_chat;

use crate::client_manager::ConnectedClients;
use crate::users_collection::ListOfUsers;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("Uso: cargo run -- <host:port>");
        return;
    }

    let direction = &arguments[1];

    let listener = TcpListener::bind(direction)
        .expect("Conexión fallida.");

    // Diccionario global de usuarios (compartido entre threads)
    let users = Arc::new(Mutex::new(ListOfUsers::new()));
    let connected_clients: Arc<Mutex<ConnectedClients>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Servidor escuchando en {}", direction);

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("Nueva conexión desde; {}", stream
                 .peer_addr()
                 .expect("Err"));

                // Clonar el Arc para pasarlo al thread
                let users_clone = Arc::clone(&users);
                let connected_clients_clone = Arc::clone(&connected_clients);
                
                thread::spawn(move || {
                    client_manager::client_manager(stream, users_clone, connected_clients_clone);
                });
            }
            Err(e) => {
                eprint!("Error: {}", e)
            }
        }
    }
    drop(listener);
}