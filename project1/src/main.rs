use std::{
    io,
    net::TcpListener,
};
//fn client_manager(mut stream : TcpStream) {
//
//    let mut buff = [0, 1024];
//
//    let mut stream = TcpStream::Connect("127.0.0.1:8080")
//     .expect("No se ha podido conectar al servidor");
//
//    
//}


// Usar un while true para que no se desconecte el usuario:

fn main () {
    // Creamos un nuevo listener para establecer una conexión.
    // bind nos regresa una nueva conexión, esta función regresa un Result<T, E>
    // nos indica que bind puede fallar, podemos usar .expect() para manejar el error,
    // o bien, .unwrap() para detener el programa en caso de obtener un Err de parte de bind.
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Conexión fallida.");

    // El usuario será capaz de establecer la dirección y el puerto.
    println!("El servidor está escuchando en el puerto 8080.");
    

    //Tenemos que introducir una dirección y puerto a partir del usuario pero esto se debe manejar desde
    //el lado del cliente y conectarlo con el servidor.

    for stream in listener.incoming() {
        let stream = stream.expect("Conexión no aceptada.");

        println!("Se establecio la conexión. \n(Este mensaje puede variar con 
        el uso de JSON en el futuro)."); // Usamos JSON
    }
}