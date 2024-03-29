use crate::statement::Statement;
use crate::table::{
   TableRefExt, Row, row_util, ExecuteResult, TableRef, Cursor };
use crate::statement_enums::{
   StatementType, PrepareResult };

use crate::size_constants::TABLE_MAX_ROWS;

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
pub fn execute_statement(stmt: Statement,mut db: TableRef) {
   match stmt.statement_type {
      StatementType::SelectStatement => {
         let cursor = db.start_cursor();
         let res = select_rows(cursor);

         for item in res {
            println!("{} - {} - {}", item.id, item.username, item.email)
         }
      },
      StatementType::InsertStatement => {
         let cursor = db.end_cursor();
         match insert_row(cursor, &stmt.row_data.unwrap()) {
            ExecuteResult::TableFull => println!("Insert Error: the table is full"),
            ExecuteResult::Success => println!("Insert successful")
         }
      },
      StatementType::None => println!("Unrecognized Statement"),
   }
}

pub fn select_rows(mut cursor: Cursor) -> Vec<Row> {
   let mut res: Vec<Row> = Vec::new();
   while !cursor.end_of_table {
      let row_ref = cursor.get_val();
      let borrowed_row = row_ref.borrow();
      let r = borrowed_row.as_ref();

      if let Option::Some(row) = r {
         let deserialized_r = row_util::deserialize_row(&row);
         res.push(deserialized_r);
      }

      cursor.advance();
   }

   res
}

/// Insert the row into the next available slot
pub fn insert_row(mut cursor: Cursor, row: &Row) -> ExecuteResult {
   if cursor.row_num >= TABLE_MAX_ROWS {
      return ExecuteResult::TableFull;
   }

   let bin_row = row_util::serialize_row(row);
   cursor.set_val(bin_row);

   ExecuteResult::Success
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

      match stmt.statement_type {
         StatementType::None => assert!(true),
         _ => assert!(false, "statement type must be None")
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
         _ => assert!(false, "the insert statement must return success")
      }

      match stmt.statement_type {
         StatementType::InsertStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }

      let row = stmt.row_data.unwrap();
      assert_eq!(row.id, 1);
      assert_eq!(row.username, "stuff");
      assert_eq!(row.email, "morestuff");
   }

   #[test]
   fn test_bad_insert_stmt() {
      let stmt_string = String::from("insert 1 stuff");
      let mut stmt = Statement::new();

      match prepare_statement(&stmt_string, &mut stmt) {
         UnrecognizedStatement => assert!(true),
         _ => assert!(false, "a short statement must return success")
      }

      match stmt.statement_type {
         StatementType::InsertStatement => assert!(true),
         _ => assert!(false, "Wrong statement type")
      }

      match stmt.row_data {
         None => assert!(true),
         _ => assert!(false, "The row should not have been parsed")
      }
   }
}