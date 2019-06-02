#[derive(Debug, Eq, PartialEq)]
pub enum CSVError {
    UnequalColumns,
    SyntaxError,
}