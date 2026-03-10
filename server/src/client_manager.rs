use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

pub fn client_manager(mut stream: TcpStream) {
    let mut buf = [0 as u8; 5120];

    while match stream.read(&mut buf) {
        Ok(size) => {
            stream
             .write(&buf[0..size])
             .expect("Err: writing response");
            true
        }
        Err(_) => {
            println!(
                "Ocurrio un error, desconectado de {}",
                stream.peer_addr().expect("Err")
            );
            stream
             .shutdown(Shutdown::Both)
             .expect("Err");
            false
        }
    } {}

}

// el servidor recibe
//Leer lo que se esta mandando y recibiendo que son cadenas, esa cadena se debe convertir en un JSON [TYPE]
//TYPE de pende del tupo de mensaje que se esta reciiemdo .
// Tenemos que leer que tipo de cadena, leer el tiṕ de cadema yu a aártor de esp va,ps eñeuemdp el tipo de dato.
// depsue ssigue la logica de la respuesta.
//
// Usar un while true para que no se desconecte el usuario
