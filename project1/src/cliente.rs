use std::{
    net::TcpStream,
    str,
    io::{self, BufRead, BufReader, Write}
};

// Queria usar la estrada estandar para ingresar la dirección del 
// servidor pero no me está dejando
//
//pub fn conexion_dinamica() -> String {
//
//    let mut direccion = String::new();
//    io::stdin().read_line(&mut direccion).expect("Err. lectura de direccion.");
//
//    return direccion;
//}

fn main() {

    //println!("Ingresa la dirección y puerto:");
    //let mut direccion = String::new();
    //io::stdin().read_line(&mut direccion).expect("Err. lectura de direccion.");

    let mut stream = TcpStream::connect("127.0.0.1:8080")
     .expect("No se pudo conectar a servidor");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();


        io::stdin().read_line(&mut input).expect("Err. lectura de stdin.");

        stream.write(input.as_bytes()).expect("Err. escritura al servidor.");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
         .expect("No se puede leer en el buffer");

        println!("{}", str::from_utf8(&buffer)
         .expect("No se puede escribir el buffer como un string."));
    }
}