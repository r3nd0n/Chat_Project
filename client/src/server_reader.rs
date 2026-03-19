use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::response_client::identify::parse_identify_response;
use crate::response_client::public_text::format_public_text_from;

pub struct ServerReader {
    reader: BufReader<TcpStream>,
    tx: Sender<String>,
    identified: bool,
}

impl ServerReader {
    pub fn new(stream: TcpStream, tx: Sender<String>) -> Self {
        Self {
            reader: BufReader::new(stream),
            tx,
            identified: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => self.handle_line(line.trim(), &line),
                Err(e) => {
                    eprintln!("Error leyendo del servidor: {e}");
                    break;
                }
            }
        }
    }

    fn handle_line(&mut self, trimmed: &str, raw: &str) {
        if !self.identified {
            if let Some(result) = parse_identify_response(trimmed) {
                let _ = self.tx.send(result.clone());
                self.identified = true;
                println!("{}", trimmed);
                return;
            }
        }

        if let Some(formatted) = format_public_text_from(trimmed) {
            println!("{}", formatted);
        } else {
            print!("{}", raw);
        }
    }
}