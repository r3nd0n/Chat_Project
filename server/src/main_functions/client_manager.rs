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
use crate::response_chat::disconnect::disconnected_response;
use crate::response_chat::private_text::{
    private_text_from,
    private_text_no_such_user,
};
use crate::response_chat::public_text::msg_response;
use crate::response_chat::status::new_status_response;
use crate::response_chat::users::get_list;
use crate::structs_chat::user::{parse_identify, User};
use crate::main_functions::users_collection::ListOfUsers;

pub type ConnectedClients = HashMap<String, Arc<Mutex<TcpStream>>>;

enum HandleResult {
    Continue(Option<String>),
    Disconnect,
}

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

    io_manager(
        &mut stream,
        &mut reader,
        &users,
        &connected_clients,
        &writer_stream,
        &mut identified_username,
    );

    cleanup_client(
        &mut stream,
        &users,
        &connected_clients,
        identified_username,
        peer,
    );
}

fn handle_request(
    raw: &str,
    users: &Arc<Mutex<ListOfUsers>>,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
    writer_stream: &Arc<Mutex<TcpStream>>,
    identified_username: &mut Option<String>,
) -> HandleResult {
    let base_msg: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => return HandleResult::Continue(None),
    };

    let msg_type = match base_msg.get("type").and_then(Value::as_str) {
        Some(value) => value,
        None => return HandleResult::Continue(None),
    };

    match msg_type {
        "IDENTIFY" => {
            if identified_username.is_some() {
                return HandleResult::Continue(None);
            }

            let identify = match parse_identify(raw) {
                Ok(value) => value,
                Err(_) => return HandleResult::Continue(None),
            };
            let username = identify.username;
            let mut guard = match users.lock() {
                Ok(value) => value,
                Err(_) => return HandleResult::Continue(None),
            };
            
            if guard.get_usr(&username).is_some() {
                HandleResult::Continue(Some(identify_usr_exists(&username)))
            } else {
                let user = User {
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
                HandleResult::Continue(Some(identify_success_response(&username)))
            }
        }
        "PUBLIC_TEXT" => {
            let sender = match identified_username.as_ref() {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };
            let text = match base_msg.get("text").and_then(Value::as_str) {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };

            let public_text_message = msg_response(text, sender);
            broadcast_except(sender, &public_text_message, connected_clients);
            HandleResult::Continue(None)
        }
        "TEXT" => {
            let sender = match identified_username.as_ref() {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };
            let recipient = match base_msg.get("username").and_then(Value::as_str) {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };
            let text = match base_msg.get("text").and_then(Value::as_str) {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };

            let recipient_exists = users
                .lock()
                .ok()
                .and_then(|guard| guard.get_usr(recipient).map(|_| ()))
                .is_some();

            if !recipient_exists {
                return HandleResult::Continue(Some(private_text_no_such_user(recipient)));
            }

            let private_message = private_text_from(sender, text);
            send_to_user(recipient, &private_message, connected_clients);
            HandleResult::Continue(None)
        }
        "USERS" => {
            if identified_username.is_none() {
                return HandleResult::Continue(None);
            }
            HandleResult::Continue(Some(get_list(users)))
        }
        "STATUS" => {
            let sender = match identified_username.as_ref() {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };
            let requested_status = match base_msg.get("status").and_then(Value::as_str) {
                Some(value) => value,
                None => return HandleResult::Continue(None),
            };
            let normalized_status = requested_status.trim().to_uppercase();

            if !matches!(normalized_status.as_str(), "ACTIVE" | "AWAY" | "BUSY") {
                return HandleResult::Continue(None);
            }

            if let Ok(mut guard) = users.lock() {
                if let Some(user) = guard.get_usr_mut(sender) {
                    user.status = normalized_status.clone();
                } else {
                    return HandleResult::Continue(None);
                }
            } else {
                return HandleResult::Continue(None);
            }

            let new_status_message = new_status_response(&normalized_status, sender);
            broadcast_except(sender, &new_status_message, connected_clients);
            HandleResult::Continue(None)
        }
        "DISCONNECT" => HandleResult::Disconnect,
        _ => HandleResult::Continue(None),
    }
}

fn io_manager(
    stream: &mut TcpStream,
    reader: &mut BufReader<TcpStream>,
    users: &Arc<Mutex<ListOfUsers>>,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
    writer_stream: &Arc<Mutex<TcpStream>>,
    identified_username: &mut Option<String>,
) {
    loop {
        let mut raw = String::new();

        match reader.read_line(&mut raw) {
            Ok(0) => break,
            Ok(_) => {
                let response = handle_request(
                    raw.trim_end(),
                    users,
                    connected_clients,
                    writer_stream,
                    identified_username,
                );

                match response {
                    HandleResult::Continue(Some(message)) => {
                        if let Err(e) = stream.write_all(message.as_bytes()) {
                            eprintln!("Error al enviar respuesta: {e}");
                            break;
                        }
                    }
                    HandleResult::Continue(None) => {}
                    HandleResult::Disconnect => {
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
}

fn cleanup_client(
    stream: &mut TcpStream,
    users: &Arc<Mutex<ListOfUsers>>,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
    identified_username: Option<String>,
    peer: Option<std::net::SocketAddr>,
) {
    if let Some(username) = identified_username {
        let disconnected_message = disconnected_response(&username);

        if let Ok(mut users_guard) = users.lock() {
            users_guard.remove_usr(&username);
        }

        if let Ok(mut clients_guard) = connected_clients.lock() {
            clients_guard.remove(&username);
        }

        broadcast_except(&username, &disconnected_message, connected_clients);
    }

    if let Some(addr) = peer {
        println!("Cliente desconectado: {addr}");
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        eprintln!("Error al cerrar socket: {e}");
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

fn send_to_user(
    username: &str,
    message: &str,
    connected_clients: &Arc<Mutex<ConnectedClients>>,
) {
    let recipient = match connected_clients.lock() {
        Ok(clients_guard) => clients_guard.get(username).cloned(),
        Err(_) => None,
    };

    if let Some(socket) = recipient {
        if let Ok(mut guard) = socket.lock() {
            let _ = guard.write_all(message.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;
    use std::time::Duration;

    fn tcp_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("No se pudo crear listener de prueba");
        let addr = listener
            .local_addr()
            .expect("No se pudo obtener direccion local");

        let client = TcpStream::connect(addr).expect("No se pudo conectar cliente de prueba");
        let (server, _) = listener.accept().expect("No se pudo aceptar conexion");

        (server, client)
    }

    #[test]
    fn private_text_returns_no_such_user_when_recipient_missing() {
        let (writer_server, _writer_client) = tcp_pair();

        let users = Arc::new(Mutex::new(ListOfUsers::new()));
        let connected_clients: Arc<Mutex<ConnectedClients>> = Arc::new(Mutex::new(HashMap::new()));
        let writer_stream = Arc::new(Mutex::new(writer_server));
        let mut identified_username = Some("Alice".to_string());

        let raw = r#"{ "type":"TEXT", "username":"Bob", "text":"Hola" }"#;
        let response = handle_request(
            raw,
            &users,
            &connected_clients,
            &writer_stream,
            &mut identified_username,
        );

        let payload = match response {
            HandleResult::Continue(Some(message)) => message,
            _ => panic!("Debio regresar respuesta NO_SUCH_USER"),
        };
        let json: Value = serde_json::from_str(payload.trim()).expect("JSON invalido en respuesta");

        assert_eq!(json["type"], "RESPONSE");
        assert_eq!(json["operation"], "TEXT");
        assert_eq!(json["result"], "NO_SUCH_USER");
        assert_eq!(json["extra"], "Bob");
    }

    #[test]
    fn private_text_sends_text_from_to_connected_recipient() {
        let (writer_server, _writer_client) = tcp_pair();
        let (recipient_server, mut recipient_client) = tcp_pair();

        recipient_client
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("No se pudo configurar timeout");

        let users = Arc::new(Mutex::new(ListOfUsers::new()));
        {
            let mut guard = users.lock().expect("No se pudo bloquear users");
            guard.add_usr(
                "Alice".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
            guard.add_usr(
                "Bob".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
        }

        let connected_clients: Arc<Mutex<ConnectedClients>> = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut guard = connected_clients
                .lock()
                .expect("No se pudo bloquear connected_clients");
            guard.insert("Bob".to_string(), Arc::new(Mutex::new(recipient_server)));
        }

        let writer_stream = Arc::new(Mutex::new(writer_server));
        let mut identified_username = Some("Alice".to_string());

        let raw = r#"{ "type":"TEXT", "username":"Bob", "text":"Secreto" }"#;
        let response = handle_request(
            raw,
            &users,
            &connected_clients,
            &writer_stream,
            &mut identified_username,
        );

        assert!(matches!(response, HandleResult::Continue(None)));

        let mut buffer = [0_u8; 512];
        let n = recipient_client
            .read(&mut buffer)
            .expect("No se pudo leer mensaje privado");
        assert!(n > 0);

        let payload = String::from_utf8_lossy(&buffer[..n]);
        let json: Value = serde_json::from_str(payload.trim()).expect("JSON invalido recibido");

        assert_eq!(json["type"], "TEXT_FROM");
        assert_eq!(json["username"], "Alice");
        assert_eq!(json["text"], "Secreto");
    }

    #[test]
    fn status_updates_user_and_broadcasts_new_status() {
        let (writer_server, _writer_client) = tcp_pair();
        let (recipient_server, mut recipient_client) = tcp_pair();

        recipient_client
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("No se pudo configurar timeout");

        let users = Arc::new(Mutex::new(ListOfUsers::new()));
        {
            let mut guard = users.lock().expect("No se pudo bloquear users");
            guard.add_usr(
                "Alice".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
            guard.add_usr(
                "Bob".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
        }

        let connected_clients: Arc<Mutex<ConnectedClients>> = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut guard = connected_clients
                .lock()
                .expect("No se pudo bloquear connected_clients");
            guard.insert("Bob".to_string(), Arc::new(Mutex::new(recipient_server)));
        }

        let writer_stream = Arc::new(Mutex::new(writer_server));
        let mut identified_username = Some("Alice".to_string());

        let raw = r#"{ "type":"STATUS", "status":"away" }"#;
        let response = handle_request(
            raw,
            &users,
            &connected_clients,
            &writer_stream,
            &mut identified_username,
        );

        assert!(matches!(response, HandleResult::Continue(None)));

        let current_status = users
            .lock()
            .expect("No se pudo bloquear users")
            .get_usr("Alice")
            .expect("Alice debe existir")
            .status
            .clone();
        assert_eq!(current_status, "AWAY");

        let mut buffer = [0_u8; 512];
        let n = recipient_client
            .read(&mut buffer)
            .expect("No se pudo leer mensaje de status");
        assert!(n > 0);

        let payload = String::from_utf8_lossy(&buffer[..n]);
        let json: Value = serde_json::from_str(payload.trim()).expect("JSON invalido recibido");

        assert_eq!(json["type"], "NEW_STATUS");
        assert_eq!(json["username"], "Alice");
        assert_eq!(json["status"], "AWAY");
    }

    #[test]
    fn disconnect_request_triggers_disconnect_and_broadcast_on_cleanup() {
        let (writer_server, _writer_client) = tcp_pair();
        let (recipient_server, mut recipient_client) = tcp_pair();

        recipient_client
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("No se pudo configurar timeout");

        let users = Arc::new(Mutex::new(ListOfUsers::new()));
        {
            let mut guard = users.lock().expect("No se pudo bloquear users");
            guard.add_usr(
                "Alice".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
            guard.add_usr(
                "Bob".to_string(),
                User {
                    status: "ACTIVE".to_string(),
                },
            );
        }

        let connected_clients: Arc<Mutex<ConnectedClients>> = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut guard = connected_clients
                .lock()
                .expect("No se pudo bloquear connected_clients");
            guard.insert("Alice".to_string(), Arc::new(Mutex::new(writer_server)));
            guard.insert("Bob".to_string(), Arc::new(Mutex::new(recipient_server)));
        }

        let (placeholder_server, _placeholder_client) = tcp_pair();
        let writer_stream = Arc::new(Mutex::new(placeholder_server));
        let mut identified_username = Some("Alice".to_string());

        let response = handle_request(
            r#"{ "type":"DISCONNECT" }"#,
            &users,
            &connected_clients,
            &writer_stream,
            &mut identified_username,
        );

        assert!(matches!(response, HandleResult::Disconnect));

        let (mut cleanup_server, _cleanup_client) = tcp_pair();
        cleanup_client(
            &mut cleanup_server,
            &users,
            &connected_clients,
            Some("Alice".to_string()),
            None,
        );

        let mut buffer = [0_u8; 512];
        let n = recipient_client
            .read(&mut buffer)
            .expect("No se pudo leer mensaje de desconexion");
        assert!(n > 0);

        let payload = String::from_utf8_lossy(&buffer[..n]);
        let json: Value = serde_json::from_str(payload.trim()).expect("JSON invalido recibido");

        assert_eq!(json["type"], "DISCONNECTED");
        assert_eq!(json["username"], "Alice");
    }
}