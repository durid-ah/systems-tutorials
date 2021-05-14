use bincode;
use serde::{Serialize, Deserialize};

/// convert the row to a vec<u8>
pub fn _serialize_row(row: &Row) -> Vec<u8> {
   bincode::serialize(&row).unwrap()
}

pub fn _deserialize_row(row: &Vec<u8>) -> Row {
   bincode::deserialize(&row).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
   pub id: u32,
   pub username: String,
   pub email: String,
}

impl Row {
   pub fn new(id:u32, username: &str, email: &str) -> Result<Row, String> {

      if username.len() > 32 || username.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 32 characcters"));
      }
      
      if email.len() > 255 || email.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 255 characcters"));
      }

      Result::Ok(Row{id, username: String::from(username), email: String::from(email)})
   }
}