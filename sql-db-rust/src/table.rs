pub struct Row {
   pub id: i32,
   pub username: String,
   pub email: String,
}

impl Row {
   fn new(id:i32, username: String, email: String) -> Result<Row, String> {
      if username.len() > 32 || username.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 32 characcters"));
      }

      if email.len() > 255 || email.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 255 characcters"));
      }

      Result::Ok(Row{id, username, email})
   }
}

// TODO: Write tests