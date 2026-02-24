use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn client_manager(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).expect("Error: failed to read from client side.");

    let request =String::from_utf8_lossy(&buffer[..]);

    println!("Recieved request: {}", request);
    
    let response ="Bienvenido!".as_bytes();

    stream.write(response).expect("Err: writing response");
} 

fn main() {

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
