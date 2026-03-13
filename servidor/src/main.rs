use std::{
    net::TcpListener,
    thread,
    env,
};

mod handle_client;

fn main() {

    let arguments: Vec<String> = env::args().collect();
    let direction: String = arguments[1];
    
    let listener = TcpListener::bind(direction)
     .expect("Error al leer conexión.");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("falló: {}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client::handle_client(stream).expect("Err!");
                });
            }
        }
    }
}