use std::collections::HashMap;
use crate::structs_chat::user::User;

// Crea un arregrlo de usuarios
// los usuarios provienen de un modulo con tipo
// de datos 

pub struct ListOfUsers {
  pub list: HashMap<String, User>,
}

impl ListOfUsers {

  pub fn new() -> Self {
    Self {
      list: HashMap::new(),
    }
  }

  pub fn add_usr(&mut self, username: String, user: User ) {
    // esto se hace al ingresar al usuario al struct
    // creado, i.e, en ejecución.
    //username = user.username;

    self.list.insert(username, user);
  }


  // permite eliminar un usuario dentro de la colección.
  // @return Some(User)por valor, si el usuario si existe y se elimina.
  // None en caso de no encontrar el usuario requeirdo.
  pub fn remove_usr(&mut self, username: &str ) -> Option<User> {
    // esto se hace al ingresar al usuario al struct
    // creado, i.e, en ejecución.
    //username = user.username;

    self.list.remove(username)
  }

  // permite consultar un usuario dentro de la colección.
  // @return Some(&User), si el usuario si existe (referencia).
  // None en caso de no encontrar el usuario requeirdo.
  pub fn get_usr(&self, username: &str) -> Option<&User> {
    // esto se hace al ingresar al usuario al struct
    // creado, i.e, en ejecución.
    //username = user.username;

    self.list.get(username)
  }

  pub fn count(&self) -> usize {
    self.list.len()
  } 
}



