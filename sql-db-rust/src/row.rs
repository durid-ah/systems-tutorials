use bincode;
use serde::{Serialize, Deserialize};

/// convert the row to a vec<u8>
pub fn serialize_row(row: &Row) -> Vec<u8> {
   bincode::serialize(&row).unwrap()
}

pub fn deserialize_row(row: &Vec<u8>) -> Row {
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
         return Result::Err(String::from("email must have between 1 and 255 characcters"));
      }

      Result::Ok(Row{id, username: String::from(username), email: String::from(email)})
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_proper_creation() {
      let r = Row::new(1, "someuser", "email");

      match r {
         Ok(val) => {
            assert_eq!(val.id, 1);
            assert_eq!(val.username, "someuser");
            assert_eq!(val.email, "email");
         },
         _ => assert!(false, "An error occured while properly instantiating a row")
      }
   }

   #[test]
   fn test_too_long_username() {
      let r = Row::new(1, "someusersssssssssssssssssssssssss", "email");

      match r {
         Err(er) => assert_eq!(er, "username must have between 1 and 32 characcters"),
         _ => assert!(false, "The row must not instantiate properly")
      }
   }

   #[test]
   fn test_too_short_username() {
      let r = Row::new(1, "", "email");

      match r {
         Err(er) => assert_eq!(er, "username must have between 1 and 32 characcters"),
         _ => assert!(false, "The row must not instantiate properly")
      }
   }

   #[test]
   fn test_too_long_email() {
      let email = String::from_utf8(vec![b'X'; 256]).unwrap();

      let r = Row::new(1, "someuser", email.as_str());

      match r {
         Err(er) => assert_eq!(er, "email must have between 1 and 255 characcters"),
         _ => assert!(false, "The row must not instantiate properly")
      }
   }

   #[test]
   fn test_too_short_email() {
      let r = Row::new(1, "someuser", "");

      match r {
         Err(er) => assert_eq!(er, "email must have between 1 and 255 characcters"),
         _ => assert!(false, "The row must not instantiate properly")
      }
   }
}