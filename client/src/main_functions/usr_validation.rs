use std::io;

const MAX_USERNAME_LEN: usize = 8;

pub fn usr_validation() -> String {
    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Err. lectura de stdin.");
            continue;
        }

        let candidate = input.trim().to_string();
        let len = candidate.chars().count();

        if len == 0 {
            eprintln!("Username vacio. \nElige un username:");
            continue;
        }

        if len > MAX_USERNAME_LEN {
            eprintln!("Username invalido: maximo 8 caracteres. \nElige un username:");
            continue;
        }

        break candidate;
    }
}