/// The type of statement requested by the user
pub enum StatementType {
   InsertStatement,
   SelectStatement
}

/// The syruct containing the statement data
pub struct Statement {
   pub st_type: StatementType 
}

pub fn prepare_statement(command: &String, statement: &mut Statement) -> Result<(), String> {
   let operation = &command[0..6];
   match operation {
       "select" => {
           statement.st_type = StatementType::SelectStatement;
           return Ok(())
       },
       "insert" => {
           statement.st_type = StatementType::InsertStatement;
           return Ok(())
       },
       _ => return Err(String::from("Unrecognized Statement"))
   }
}