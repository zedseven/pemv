// Uses
use atty::{is as is_tty, Stream};
use serde_derive::{Deserialize, Serialize};
use termcolor::ColorChoice as TermColorChoice;

use crate::error::ParseError;

/// Wraps [`termcolor`]'s [`ColorChoice`] enum, with support for
/// serialisation.
///
/// [`ColorChoice`]: termcolor::ColorChoice
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ColourChoice {
	/// Try very hard to emit colors. This includes emitting ANSI colors
	/// on Windows if the console API is unavailable.
	#[serde(rename = "always")]
	Always,
	/// `AlwaysAnsi` is like `Always`, except it never tries to use anything
	/// other than emitting ANSI color codes.
	#[serde(rename = "ansi")]
	AlwaysAnsi,
	/// Try to use colors, but don't force the issue. If the console isn't
	/// available on Windows, or if `TERM=dumb`, or if `NO_COLOR` is defined,
	/// for example, then don't use colors.
	#[serde(rename = "auto")]
	Auto,
	/// Never emit colors.
	#[serde(rename = "never")]
	Never,
}

impl Default for ColourChoice {
	fn default() -> Self {
		Self::Auto
	}
}

impl From<TermColorChoice> for ColourChoice {
	fn from(choice: TermColorChoice) -> Self {
		match choice {
			TermColorChoice::Always => Self::Always,
			TermColorChoice::AlwaysAnsi => Self::AlwaysAnsi,
			TermColorChoice::Auto => Self::Auto,
			TermColorChoice::Never => Self::Never,
		}
	}
}

impl From<ColourChoice> for TermColorChoice {
	fn from(choice: ColourChoice) -> Self {
		match choice {
			ColourChoice::Always => Self::Always,
			ColourChoice::AlwaysAnsi => Self::AlwaysAnsi,
			ColourChoice::Auto => Self::Auto,
			ColourChoice::Never => Self::Never,
		}
	}
}

impl TryFrom<&str> for ColourChoice {
	type Error = ParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value.to_lowercase().as_str() {
			"always" => Ok(Self::Always),
			"ansi" => Ok(Self::AlwaysAnsi),
			"auto" => Ok(Self::Auto),
			"never" => Ok(Self::Never),
			_ => Err(ParseError::Unsupported),
		}
	}
}

impl From<ColourChoice> for &str {
	fn from(choice: ColourChoice) -> Self {
		match choice {
			ColourChoice::Always => "always",
			ColourChoice::AlwaysAnsi => "ansi",
			ColourChoice::Auto => "auto",
			ColourChoice::Never => "never",
		}
	}
}

impl ColourChoice {
	/// Changes the value to `Never` if `stdout` isn't a tty.
	#[must_use]
	#[cfg(not(tarpaulin_include))]
	pub fn change_based_on_tty(self) -> Self {
		#[allow(clippy::wildcard_enum_match_arm)]
		match self {
			Self::Auto => {
				if is_tty(Stream::Stdout) {
					ColourChoice::Auto
				} else {
					ColourChoice::Never
				}
			}
			_ => self,
		}
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use termcolor::ColorChoice as TermColorChoice;

	use super::ColourChoice;
	use crate::error::ParseError;

	// Tests
	#[test]
	fn round_trip_conversions() {
		macro_rules! test_round_trip {
			($first_type:ty, $second_type:ty, $variant:ident) => {
				let expected = <$first_type>::$variant;
				let result: $second_type = <$first_type>::$variant.into();
				let result: $first_type = result.into();
				assert_eq!(expected, result);
			};
		}
		macro_rules! test_round_trip_second_type_try {
			($first_type:ty, $second_type:ty, $variant:ident) => {
				let expected = <$first_type>::$variant;
				let result: $second_type = <$first_type>::$variant.into();
				let result: $first_type = result
					.try_into()
					.expect("error checking will be done separately");
				assert_eq!(expected, result);
			};
		}

		test_round_trip!(ColourChoice, TermColorChoice, Always);
		test_round_trip!(ColourChoice, TermColorChoice, AlwaysAnsi);
		test_round_trip!(ColourChoice, TermColorChoice, Auto);
		test_round_trip!(ColourChoice, TermColorChoice, Never);
		test_round_trip!(TermColorChoice, ColourChoice, Always);
		test_round_trip!(TermColorChoice, ColourChoice, AlwaysAnsi);
		test_round_trip!(TermColorChoice, ColourChoice, Auto);
		test_round_trip!(TermColorChoice, ColourChoice, Never);
		test_round_trip_second_type_try!(ColourChoice, &str, Always);
		test_round_trip_second_type_try!(ColourChoice, &str, AlwaysAnsi);
		test_round_trip_second_type_try!(ColourChoice, &str, Auto);
		test_round_trip_second_type_try!(ColourChoice, &str, Never);
	}
	#[test]
	fn from_str_error() {
		let expected = Err(ParseError::Unsupported);
		let result = ColourChoice::try_from("unsupported value");

		assert_eq!(expected, result);
	}
}
