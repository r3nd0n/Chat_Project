use std::{
    env,
    net::TcpStream,
    str,
    io::{self, BufRead, BufReader, Write}
};
use crate::client_msg::new_usr;
mod client_response;

//
fn main() {

    let  arguments: Vec<String> = env::args().collect();
    let direction: &String = &arguments[1];

    let mut stream = TcpStream::connect(direction)
     .expect("No se pudo conectar a servidor");

    println!("Elige un username: ");

    //! El cliente entra en un bucle y espera 
    //! que escribamos algo por teclado con
    //! stdin.read_line().
    //! Eso deja el texto en input.
    loop {
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input)
         .expect("Err. lectura de stdin.");
        
        let input = input.trim().to_string();
        let json = new_usr(&input);

        //! el String JSON se convierte a bytes con as_bytes()
        //!esos bytes se envían por la conexión TCP al servidor
        stream.write(json.as_bytes())
         .expect("Err. escritura al servidor.");

        //! el cliente:
        //!
        //! · crea un BufReader
        //! · usa read_until(b'\n', &mut buffer)
        //! · espera hasta encontrar el \n final
        //! · convierte los bytes a texto con from_utf8
        //! · lo imprime con println!
        //! Por eso aparece en pantalla el JSON que volvió del servidor.
        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer)
         .expect("No se puede leer en el buffer");

        println!("{}", str::from_utf8(&buffer)
         .expect("No se puede escribir el buffer como un string."));
    }
}