use std::{
    net::{TcpStream, Shutdown},
    io::{Read, Write},
};

pub fn client_manager(mut stream: TcpStream) {

    let mut buff = [0 as u8; 1024];
    
    while match stream.read(&mut buff) {
        Ok(size) => {
            
            stream.write(&buff[0..size]).expect("Err: writing response");
            true
        }
        Err(_) => {
            println!("Ocurrio un error, desconectado de {}", stream
             .peer_addr().expect("Err"));
            stream.shutdown(Shutdown::Both).expect("Err");
            false
        }
    }{}

    // Usados previo al while
    //let request =String::from_utf8_lossy(&buffer[..]);
    //println!("Recieved request: {}", request);
    //let response ="Bienvenido!".as_bytes();
}

// el servidor recibe 
//Leer lo que se esta mandando y recibiendo que son cadenas, esa cadena se debe convertir en un JSON [TYPE]
//TYPE de pende del tupo de mensaje que se esta reciiemdo .
// Tenemos que leer que tipo de cadena, leer el tiṕ de cadema yu a aártor de esp va,ps eñeuemdp el tipo de dato.
// depsue ssigue la logica de la respuesta. 
//
// Usar un while true para que no se desconecte el usuario

