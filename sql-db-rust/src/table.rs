use std::vec::Vec;

const TABLE_MAX_PAGES: usize = 100;

pub struct Row {
   pub id: u32,
   pub username: String,
   pub email: String,
}

impl Row {
   pub fn new(id:u32, username: &str, email: &str) -> Result<Row, String> {
      let m3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

      if username.len() > 32 || username.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 32 characcters"));
      }
      
      if email.len() > 255 || email.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 255 characcters"));
      }

      Result::Ok(Row{id, username: String::from(username), email: String::from(email)})
   }
}

pub struct Table {
   pub num_rows: u32,
   pub pages: [ [u8; 10]; TABLE_MAX_PAGES]
}

// TODO: Write tests
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_row_creation() {
      let new_row = Row::new(1, "user", "email");
      match new_row {
         Result::Ok(r) => {
            assert_eq!(r.id, 1);
            assert_eq!(r.username, "user");
            assert_eq!(r.email, "email");
         }
         _ => assert!(false, "Unable to create row")
      }
   }
}