use std::fmt;
use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum CSVError {
    UnequalColumns(SizeError),
    SyntaxError(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct SizeError {
    pub line_no: usize,
    pub expected_size: usize,
    pub actual_size: usize,
}

impl fmt::Display for CSVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CSVError::UnequalColumns(v) => write!(
                f,
                "Unequal number of columns at line {}, expected: {} columns, got: {}",
                v.line_no, v.expected_size, v.actual_size
            ),
            CSVError::SyntaxError(l) => write!(f, "Syntax error at line number: {}", l),
        }

    }
}

impl Error for CSVError {}
