use std::{
    env,
    net::TcpStream,
    str,
    io::{self, BufRead, BufReader, Write}
};
use crate::identify_response::build_identify;
mod identify_response;

fn main() {

    let  arguments: Vec<String> = env::args().collect();

    let direction: &String = &arguments[1];

    let mut stream = TcpStream::connect(direction)
     .expect("No se pudo conectar a servidor");

    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input).expect("Err. lectura de stdin.");
        
        //
        // Send the JSON to the server
        // Doc. in:
        // https://docs.rs/serde_json/1.0.149/serde_json/value/index.html
        //
        let json = build_identify(&input);

        stream.write(json.as_bytes()).expect("Err. escritura al servidor.");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
         .expect("No se puede leer en el buffer");

        println!("{}", str::from_utf8(&buffer)
         .expect("No se puede escribir el buffer como un string."));
    }
}