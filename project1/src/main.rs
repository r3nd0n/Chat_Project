use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


//client_manager:
//recibe un TcpStream para establecer la conexión a un localhost
// con la dirección: 127.0.0.1, y el puerto: 8080.
//Maneja un solo cleinte.
fn client_manager(mut stream: TcpStream) {

    // lee datos del cleinte
    let mut buffer = [0; 1024];

    //lee datos desde el stream y lo almacena en el buffer
    stream.read(&mut buffer).expect("Error: failed to read from client side.");

    let request =String::from_utf8_lossy(&buffer[..]);

    println!("Recieved request: {}", request);
    let response ="Bienvenido!".as_bytes();

    stream.write(response).expect("Err: writing response");
}

fn main() {

    //let addrs = [
    //    SocketAddr::from(([127,0,0,1], 8080)),
    //    SocketAddr::from(([127,0,0,1], 1234)),
    //];
    
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(||client_manager(stream));
            }
            Err(e) => {
                eprintln!("Error to stablish connection: {}", e);
            }
        }
    }
}
