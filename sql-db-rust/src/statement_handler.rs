use super::statement_enums::{StatementType, PrepareResult};
use super::table::{Table, ExecuteResult};
use super::statement::Statement;


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
      StatementType::SelectStatement => {
         let res = db.select_rows();
         for item in res {
            println!("{} - {} - {}", item.id, item.username, item.email)
         }
      },
      StatementType::InsertStatement => {
         match db.insert_row(&stmt.row_data.unwrap()) {
            ExecuteResult::TableFull => println!("Insert Error: the table is full"),
            ExecuteResult::Success => println!("Insert successful")
         }
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