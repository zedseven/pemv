//! The error enum definition.

#[derive(Debug)]
pub enum ParseError {
	WrongByteCount { expected: usize, found: usize },
}
