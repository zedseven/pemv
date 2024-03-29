//! The IAC value for `Denial`.
//!
//! From EMV Book 3:
//! > Together, the `Issuer Action Code - Denial` and the `Terminal Action
//! > Code - Denial` specify the conditions that cause denial of a transaction
//! > without attempting to go online.
//!
//! and
//!
//! > For each bit in the TVR that has a value of `1`, the terminal shall check
//! > the corresponding bits in the `Issuer Action Code - Denial` and the
//! > `Terminal Action Code - Denial`. If the corresponding bit in either of the
//! > action codes is set to `1`, it indicates that the issuer or the acquirer
//! > wishes the transaction to be rejected offline. In this case, the terminal
//! > shall issue a `GENERATE AC` command to request an `AAC` from the ICC.

// Uses
use termcolor::{StandardStream, WriteColor};

use crate::{
	error::ParseError,
	output_colours::header_colour_spec,
	util::print_indentation,
	DisplayBreakdown,
	TerminalVerificationResults,
};

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IssuerActionCodeDenial {
	pub tvr: TerminalVerificationResults,
}

impl Default for IssuerActionCodeDenial {
	/// From EMV Book 3:
	/// > If the `Issuer Action Code - Denial` does not exist, a default value
	/// > with all bits set to `0` is to be used.
	fn default() -> Self {
		Self {
			tvr: TerminalVerificationResults::try_from(
				[
					0b0000_0000u8,
					0b0000_0000,
					0b0000_0000,
					0b0000_0000,
					0b0000_0000,
				]
				.as_slice(),
			)
			.expect("default value for `Issuer Action Code - Denial` couldn't be parsed"),
		}
	}
}

impl TryFrom<&[u8]> for IssuerActionCodeDenial {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		Ok(Self {
			tvr: TerminalVerificationResults::try_from(raw_bytes)?,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for IssuerActionCodeDenial {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8, _: bool) {
		let header_colour_spec = header_colour_spec();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!(
			"If any of the following match the TVR, deny the transaction without even going \
			 online:"
		);
		stdout.reset().ok();

		self.tvr.display_breakdown(stdout, indentation, false);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::IssuerActionCodeDenial;
	use crate::emv::TerminalVerificationResults;

	// Tests
	/// Ensures the parsed value here matches the same parsed value in the TVR.
	#[test]
	fn iac_matches_tvr() {
		let raw_value = [0xFF; 5];
		let expected = TerminalVerificationResults::try_from(raw_value.as_slice())
			.expect("not testing the TVR code here");
		let result = IssuerActionCodeDenial::try_from(raw_value.as_slice())
			.expect("any errors should already be tested by the TVR testing");

		assert_eq!(expected, result.tvr);
	}
	/// Ensures there's no panic.
	#[test]
	fn default_value_is_ok() {
		IssuerActionCodeDenial::default();
	}
}
