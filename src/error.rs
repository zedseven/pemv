//! The error enum definition.

/// An error that occurred during parsing.
#[derive(Debug)]
pub enum ParseError {
	/// The wrong number of bytes were present.
	WrongByteCount { expected: usize, found: usize },
}
