//! The error enum definition.

// Uses
use std::{
	cmp::Ordering,
	convert::Infallible,
	fmt::{Display, Formatter, Result as FmtResult},
};

/// An error that occurred during parsing.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseError {
	/// The wrong number of bytes were present.
	ByteCountIncorrect {
		r#type: Ordering,
		expected: usize,
		found: usize,
	},
	ByteCountNotDivisibleIntoComponents,
	/// The bytes provided are not valid.
	InvalidBytes,
	/// The string provided couldn't be parsed as a number.
	InvalidNumber,
	/// The value provided isn't compliant with the specifications in some way.
	NonCompliant,
	/// The value provided isn't compliant with the EMV CCD specifications in
	/// some way. Not necessarily a problem.
	NonCcdCompliant,
	/// Something is unsupported and cannot be processed.
	Unsupported,
	/// Something is unrecognised and cannot be processed. Not necessarily a
	/// problem.
	Unrecognised,
}

// This is for type compatibility, so that a `Result<T, Infallible>` can be
// converted to a `Result<T, ParseError>`.
#[cfg(not(tarpaulin_include))]
impl From<Infallible> for ParseError {
	fn from(_: Infallible) -> Self {
		unreachable!("because the input type is Infallible, this case should never actually occur");
	}
}

#[cfg(not(tarpaulin_include))]
impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::ByteCountIncorrect {
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
			Self::ByteCountNotDivisibleIntoComponents => write!(
				f,
				"The number of bytes provided is not divisible into the components of the value. \
				 Please check the format of the input data."
			),
			Self::InvalidBytes => write!(f, "The bytes provided are not valid."),
			Self::InvalidNumber => write!(
				f,
				"The value provided is not a valid number, or is too large."
			),
			Self::NonCompliant => write!(
				f,
				"The value provided isn't compliant with the specifications in some way, or this \
				 tool is out of date. If you have reason to believe it's the latter, please make \
				 sure you're using the latest version then open an issue on GitHub."
			),
			Self::NonCcdCompliant => write!(
				f,
				"The value provided isn't compliant with the EMV Common Core Definitions (CCD) in \
				 some way. This isn't a problem necessarily, but it does mean that the value \
				 can't be parsed."
			),
			Self::Unsupported => write!(
				f,
				"The value provided is in some way unsupported. If you have genuine need for the \
				 unsupported feature, please open an issue on GitHub."
			),
			Self::Unrecognised => write!(
				f,
				"The value provided is in some way unrecognised. This isn't a problem \
				 necessarily, but it does mean that the value can't be parsed."
			),
		}
	}
}
