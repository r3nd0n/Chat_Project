use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::mpsc::Sender,
};
use serde_json::Value;

use crate::response_client::identify::parse_identify_response;
use crate::response_client::users::parse_user_list;

const ANSI_RESET: &str = "\x1b[0m";
const USER_COLORS: [&str; 5] = [

    "\x1b[31m",	
    "\x1b[32m",	 
    "\x1b[33m",	 
    "\x1b[34m",	 
    "\x1b[35m",
];

pub struct ServerReader {
    reader: BufReader<TcpStream>,
    tx: Sender<String>,
    identified: bool,
    username_colors: HashMap<String, usize>,
    next_color: usize,
}

impl ServerReader {
    pub fn new(stream: TcpStream, tx: Sender<String>) -> Self {
        Self {
            reader: BufReader::new(stream),
            tx,
            identified: false,
            username_colors: HashMap::new(),
            next_color: 0,
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
                return;
            }
        }

        let value: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => {
                print!("{}", raw);
                return;
            }
        };

        let msg_type = value.get("type").and_then(Value::as_str);

        match msg_type {
            Some("NEW_USER") => {
                if let Some(username) = value.get("username").and_then(Value::as_str) {
                    println!("{} se unio al chat", self.colorized_username(username));
                } else {
                    print!("{}", raw);
                }
            }
            Some("PUBLIC_TEXT_FROM") => {
                let username = value.get("username").and_then(Value::as_str);
                let text = value.get("text").and_then(Value::as_str);
                if let (Some(username), Some(text)) = (username, text) {
                    println!("{}: {}", self.colorized_username(username), text);
                } else {
                    print!("{}", raw);
                }
            }
            Some("TEXT_FROM") => {
                let username = value.get("username").and_then(Value::as_str);
                let text = value.get("text").and_then(Value::as_str);
                if let (Some(username), Some(text)) = (username, text) {
                    println!("[privado] {}: {}", self.colorized_username(username), text);
                } else {
                    print!("{}", raw);
                }
            }
            Some("USER_LIST") => {
                if let Some(rows) = parse_user_list(trimmed) {
                    if rows.is_empty() {
                        println!("Usuarios conectados: (sin usuarios)");
                    } else {
                        println!("Usuarios conectados:");
                        for (username, status) in rows {
                            println!("- {} ({})", self.colorized_username(&username), status);
                        }
                    }
                } else {
                    print!("{}", raw);
                }
            }
            Some("NEW_STATUS") => {
                let username = value.get("username").and_then(Value::as_str);
                let status = value.get("status").and_then(Value::as_str);
                if let (Some(username), Some(status)) = (username, status) {
                    println!("{} cambio su estado a {}", self.colorized_username(username), status);
                } else {
                    print!("{}", raw);
                }
            }
            Some("DISCONNECTED") => {
                if let Some(username) = value.get("username").and_then(Value::as_str) {
                    println!("{} se desconecto del chat", self.colorized_username(username));
                } else {
                    print!("{}", raw);
                }
            }
            _ => {
                print!("{}", raw);
            }
        }
    }

    fn colorized_username(&mut self, username: &str) -> String {
        let idx = if let Some(existing) = self.username_colors.get(username) {
            *existing
        } else {
            let assigned = self.next_color;
            self.username_colors.insert(username.to_string(), assigned);
            self.next_color = (self.next_color + 1) % USER_COLORS.len();
            assigned
        };

        format!("{}{}{}", USER_COLORS[idx], username, ANSI_RESET)
    }
}