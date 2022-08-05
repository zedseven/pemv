//! The error enum definition.

// Uses
use std::{
	cmp::Ordering,
	fmt::{Display, Formatter, Result as FmtResult},
};

/// An error that occurred during parsing.
#[derive(Debug)]
pub enum ParseError {
	/// The wrong number of bytes were present.
	ByteCountIncorrect {
		r#type: Ordering,
		expected: usize,
		found: usize,
	},
	/// The string provided couldn't be parsed as a number.
	InvalidNumber,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			ParseError::ByteCountIncorrect {
				r#type,
				expected,
				found,
			} => write!(
				f,
				"The wrong number of bytes were provided for the value. Perhaps you provided the \
				 wrong value? Expected: {} {}, Found: {}",
				match r#type {
					Ordering::Less => "less than or equal to",
					Ordering::Equal => "exactly",
					Ordering::Greater => "greater than or equal to",
				},
				expected,
				found
			),
			ParseError::InvalidNumber => write!(
				f,
				"The value provided is not a valid number, or is too large."
			),
		}
	}
}
