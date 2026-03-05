use std::{
    io,
    thread,
    net:: TcpListener,
};

mod socket;

// Usar un while true para que no se desconecte el usuario:

fn main () {

    let listener = TcpListener::bind("127.0.0.1:8080")
     .expect("Conexión fallida.");

    println!("El servidor está escuchando en el puerto 8080.");    

    //Tenemos que introducir una dirección y puerto a partir 
    //del usuario pero esto se debe manejar desde
    //el lado del cliente y conectarlo con el servidor.
    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {

                println!("Nueva conexión; {}", stream
                 .peer_addr()
                 .expect("Err"));

                thread::spawn(move|| {
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