use std::{
        collections::HashMap,
        env,
        net::TcpListener,
        sync::{Arc, Mutex}, // Para manejar los hilos que usan la misma estructura (Atomic Reference Counted).
        };

//mod client_manager;
//mod conection_iterator;
mod response_chat;
mod main_functions;
//mod users_collection;
mod structs_chat;

use crate::main_functions::client_manager::ConnectedClients;
use crate::main_functions::conection_iterator::stream_iterator;
use crate::main_functions::users_collection::ListOfUsers;


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

    stream_iterator(listener, Arc::clone(&users), Arc::clone(&connected_clients));
}