use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{Shutdown, TcpStream},
    sync::{Arc, Mutex},
};
use serde_json::Value;
use crate::response_chat::identify::{
    identify_new_usr,
    identify_success_response,
    identify_usr_exists,
};
use crate::structs_chat::user::{parse_identify, User};
use crate::users_collection::ListOfUsers;



pub type ConnectedClients = HashMap<String, Arc<Mutex<TcpStream>>>;



pub fn client_manager(
    mut stream: TcpStream,
    users: Arc<Mutex<ListOfUsers>>,
    connected_clients: Arc<Mutex<ConnectedClients>>,
) {
    let peer = stream.peer_addr().ok();
    let writer_stream = match stream.try_clone() {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(e) => {
            eprintln!("No se pudo clonar el socket para escritura compartida: {e}");
            return;
        }
    };

    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No se pudo clonar el socket para lectura: {e}");
            return;
        }
    };

    let mut reader = BufReader::new(reader_stream);
    let mut identified_username: Option<String> = None;

    loop {
        let mut raw = String::new();

        match reader.read_line(&mut raw) {
            Ok(0) => break,
            Ok(_) => {
                let response = handle_request(
                    raw.trim_end(),
                    &users,
                    &connected_clients,
                    &writer_stream,
                    &mut identified_username,
                );
                if let Some(message) = response {
                    if let Err(e) = stream.write_all(message.as_bytes()) {
                        eprintln!("Error al enviar respuesta: {e}");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error leyendo desde el socket: {e}");
                break;
            }
        }
    }

    if let Some(username) = identified_username {
        if let Ok(mut users_guard) = users.lock() {
            users_guard.remove_usr(&username);
        }

        if let Ok(mut clients_guard) = connected_clients.lock() {
            clients_guard.remove(&username);
        }
    }

    if let Some(addr) = peer {
        println!("Cliente desconectado: {addr}");
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        eprintln!("Error al cerrar socket: {e}");
    }
}



fn handle_request(
    raw: &str,
    users: &Arc<Mutex<ListOfUsers>>,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
    writer_stream: &Arc<Mutex<TcpStream>>,
    identified_username: &mut Option<String>,
) -> Option<String> {
    let base_msg: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let msg_type = base_msg.get("type").and_then(Value::as_str)?;

    match msg_type {
        "IDENTIFY" => {
            if identified_username.is_some() {
                return None;
            }

            let identify = parse_identify(raw).ok()?;
            let username = identify.username;

            let mut guard = users.lock().ok()?;
            if guard.get_usr(&username).is_some() {
                Some(identify_usr_exists(&username))
            } else {
                let user = User {
                    username: username.clone(),
                    status: "ACTIVE".to_string(),
                };

                guard.add_usr(username.clone(), user);
                *identified_username = Some(username.clone());
                drop(guard);

                if let Ok(mut clients_guard) = connected_clients.lock() {
                    clients_guard.insert(username.clone(), Arc::clone(writer_stream));
                }

                let new_user_message = identify_new_usr(&username);
                broadcast_except(&username, &new_user_message, connected_clients);
                Some(identify_success_response(&username))
            }
        }
        _ => None,
    }
}



fn broadcast_except(
    excluded_username: &str,
    message: &str,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
) {
    let recipients: Vec<Arc<Mutex<TcpStream>>> = match connected_clients.lock() {
        Ok(clients_guard) => clients_guard
            .iter()
            .filter(|(username, _)| username.as_str() != excluded_username)
            .map(|(_, stream)| Arc::clone(stream))
            .collect(),
        Err(_) => return,
    };

    for recipient in recipients {
        if let Ok(mut socket) = recipient.lock() {
            let _ = socket.write_all(message.as_bytes());
        }
    }
}