use std::{io, env,  net::TcpListener, thread};

mod socket;

// Usar un while true para que no se desconecte el usuario:

fn main() {

    let  arguments: Vec<String> = env::args().collect();

    let direction: &String = &arguments[1];

    let listener = TcpListener::bind(direction)
        .expect("Conexión fallida (no se pudo asociar al puerto \"8080\").");

    println!("El servidor está escuchando en el puerto 8080.");

    //Tenemos que introducir una dirección y puerto a partir
    //del usuario pero esto se debe manejar desde
    //el lado del cliente y conectarlo con el servidor.
    for stream in listener.incoming() {
        //incoming regresa un iterador sobre los streams que se han conectado al servidor.

        match stream {
            Ok(stream) => {
                println!("Nueva conexión; {}", stream.peer_addr().expect("Err"));

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
