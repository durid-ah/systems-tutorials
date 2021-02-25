use super::statement_enums::{StatementType, PrepareResult};

/// The syruct containing the statement data
pub struct Statement {
   pub st_type: StatementType 
}

impl Statement {
   fn new() -> Statement {
      Statement{st_type: StatementType::None}
   }
}


/// Prcesses the incoming statement from the user
pub fn prepare_statement(command: &String, statement: &mut Statement) -> PrepareResult {
   if command.len() < 6 {
      return PrepareResult::UnrecognizedStatement
   }

   let operation = &command[0..6];
   match operation {
      "select" => {
         statement.st_type = StatementType::SelectStatement;
         return PrepareResult::Success;
      },
      "insert" => {
         statement.st_type = StatementType::InsertStatement;
         return PrepareResult::Success;
      },
      _ => return PrepareResult::UnrecognizedStatement
   }
}

#[cfg(test)]
mod test {
   use super::*;
   use super::super::statement_enums::StatementType;
   use super::super::statement_enums::PrepareResult::{
      Success,
      UnrecognizedStatement
   };
   
   #[test]
   fn test_stmt_shorter_than_6_chars() {
     let stmt_string = String::from("selec");
     let mut stmt = Statement::new();
     match prepare_statement(&stmt_string, &mut stmt) {
        UnrecognizedStatement => assert!(true),
        _ => assert!(false, "a short statement must return unrecognized statement")
     }
   }

   #[test]
   fn test_select_stmt() {
      let stmt_string = String::from("select");
      let mut stmt = Statement::new();
      
      match prepare_statement(&stmt_string, &mut stmt) {
         Success => assert!(true),
         _ => assert!(false, "a short statement must return success")
      }

      match stmt.st_type {
         StatementType::SelectStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }
   }

   #[test]
   fn test_insert_stmt() {
      let stmt_string = String::from("insert");
      let mut stmt = Statement::new();
      
      match prepare_statement(&stmt_string, &mut stmt) {
         Success => assert!(true),
         _ => assert!(false, "a short statement must return success")
      }

      match stmt.st_type {
         StatementType::InsertStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }
   }
}