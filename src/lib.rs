mod parser;
mod error;

pub use self::error::{CSVError, SizeError};
pub use self::parser::parse_csv;