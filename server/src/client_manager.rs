use std::{
    io::{Read,BufRead, BufReader, Write},
    net::{Shutdown, TcpStream},
};
use serde_json::Value;
use crate::structs;

//! el servidor usa stream.read(&mut buf).
//! 
//! Eso hace:
//! 
//! · leer bytes que llegan del cliente
//! · guardarlos en buffer
//! · "size" dice cuántos bytes reales llegaron
//! Hasta aquí, el servidor todavía no está interpretando JSON.
//! Solo está leyendo bytes.
pub fn client_manager(mut stream: TcpStream) {

    //! para JSON por mensajes, suele ser mejor leer por líneas (con BufReader + read_line) 
    //! y deserializar cada línea, en lugar de depender solo de buffer fijo.
    let mut buffer = [0 as u8; 5120];

    while match stream.read(&mut buffer) {

        //!  El servidor genera la respuesta
        //! Ahora mismo, en client_manager.rs:13-15, el servidor responde así:
        //! 
        //! · toma exactamente los bytes recibidos: &buffer[0..size]
        //! · los vuelve a escribir en el socket con stream.write(...)
        //! O sea: no construye una respuesta nueva.
        //! Lo que hace es un echo: devuelve exactamente el mismo JSON que recibió.
        //! 
        //! Por eso se ve impreso lo mismo en el cliente.
        Ok(size) => {
            stream
             .write(&buffer[0..size])
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
    }{}
}