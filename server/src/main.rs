use std::{collections::{HashMap}, env, io, net::TcpListener, thread};
mod socket;


fn main() {
    
    //let mut book_reviews: HashMap;

    let  arguments: Vec<String> = env::args().collect();

    let direction: &String = &arguments[1];

    let listener = TcpListener::bind(direction)
        .expect("Conexión fallida.");

    println!("El servidor está escuchando en el puerto 8080.");

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("Nueva conexión; {}", stream
                 .peer_addr()
                 .expect("Err"));

                thread::spawn(move || {
                    //Aquí hay que colocar la función de "sockets.rs"
                    socket::client_manager(stream);
                });
            }
            Err(e) => {
                eprint!("Error: {}", e)
            }
        }
    }
    drop(listener);
}
