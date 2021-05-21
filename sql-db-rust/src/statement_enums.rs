/// The type of statement requested by the user
pub enum StatementType {
   InsertStatement,
   SelectStatement,
   None
}

// pub enum MetaResult {
//    CommandSuccess,
//    UnrecognizedCommand
// }

pub enum PrepareResult {
   Success,
   BadStatement(String),
   UnrecognizedStatement
}