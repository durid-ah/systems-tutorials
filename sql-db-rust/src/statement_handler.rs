use super::statement_enums::{StatementType, PrepareResult};
use super::table::{Row, Table};

/// The struct containing the statement data
pub struct Statement {
   pub statement_type: StatementType,
   pub row_data: Option<Row>,
}


impl Statement {
   /// Construct a statement
   pub fn new() -> Statement {
      Statement{statement_type: StatementType::None, row_data: None}
   }

   pub fn parse_user_input(&mut self, row: Vec<&str>) -> PrepareResult {
      let mut result = PrepareResult::UnrecognizedStatement;

      match self.statement_type {
         StatementType::InsertStatement => result = self._insert_parse(row),
         _ => ()
      }

      result
   }

   fn _insert_parse(&mut self, row: Vec<&str>) -> PrepareResult {
      if row.len() != 4 {
         return PrepareResult::UnrecognizedStatement
      }

      let id: u32;
      let username = row[2];
      let email = row[3].trim();
      
      if let Result::Ok(num) = row[1].trim().parse::<u32>() {
         id = num;
      } else {
         return PrepareResult::UnrecognizedStatement
      }

      let row_result = Row::new(id, username, email);
      match row_result {
         Ok(r) => {
            self.row_data = Some(r);
            return PrepareResult::Success
         }
         Err(_) => return PrepareResult::UnrecognizedStatement
      }
   }
}


/// Prcesses the incoming statement from the user
pub fn prepare_statement(command: &String, statement: &mut Statement) -> PrepareResult {
   let split_input: Vec<&str> = command.trim().split(" ").collect();
   
   let operation = split_input[0];
   match operation {
      "select" => {
         statement.statement_type = StatementType::SelectStatement;
         return PrepareResult::Success;
      },
      "insert" => {
         statement.statement_type = StatementType::InsertStatement;
         return statement.parse_user_input(split_input);
      },
      _ => return PrepareResult::UnrecognizedStatement
   }
}


/// Execute user statement
pub fn execute_statement(stmt: Statement, db: &mut Table) {
   match stmt.statement_type {
      StatementType::SelectStatement => println!("This is where we would do a select"),
      StatementType::InsertStatement => {
         db.insert_row(&stmt.row_data.unwrap());
      },
      StatementType::None => println!("Unrecognized Statement"),
   }
}


/////////////////////////////////
/// Test Section
/////////////////////////////////
#[cfg(test)]
mod tests {
   use super::*;
   use super::super::statement_enums::StatementType;
   use super::super::statement_enums::PrepareResult::{
      Success,
      UnrecognizedStatement
   };
   
   #[test]
   fn test_incorrect_stmt() {
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
         _ => assert!(false, "must return success")
      }

      match stmt.statement_type {
         StatementType::SelectStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }
   }

   #[test]
   fn test_insert_stmt() {
      let stmt_string = String::from("insert 1 stuff morestuff");
      let mut stmt = Statement::new();
      
      match prepare_statement(&stmt_string, &mut stmt) {
         Success => assert!(true),
         _ => assert!(false, "a short statement must return success")
      }

      match stmt.statement_type {
         StatementType::InsertStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }
   }
}