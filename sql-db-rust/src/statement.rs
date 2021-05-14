use super::statement_enums::{StatementType, PrepareResult};
use super::row::Row;

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

// TODO: Test parse user input