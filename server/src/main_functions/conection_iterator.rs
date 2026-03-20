use std::{
    net::TcpListener,
    thread,
    sync::{Arc, Mutex}
};

use crate::main_functions::client_manager;
use crate::main_functions::client_manager::ConnectedClients;
use crate::main_functions::users_collection::ListOfUsers;

pub fn stream_iterator(
    listener: TcpListener,
    users: Arc<Mutex<ListOfUsers>>,
    connected_clients: Arc<Mutex<ConnectedClients>>,
) {

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
}