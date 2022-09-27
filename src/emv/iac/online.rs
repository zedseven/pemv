//! The IAC value for `Online`.
//!
//! From EMV Book 3:
//! > Together, the `Issuer Action Code - Online` and the `Terminal Action
//! > Code - Online` specify the conditions that cause a transaction to be
//! > completed online. These data objects are meaningful only for terminals
//! > capable of online processing. Offline-only terminals may skip this test
//! > and proceed to checking the `Issuer Action Code - Default` and `Terminal
//! > Action Code - Default`
//!
//! and
//!
//! > For each bit in the TVR that has a value of `1`, the terminal shall check
//! > the corresponding bits in both the `Issuer Action Code - Online` and the
//! > `Terminal Action Code - Online`. If the bit in either of the action codes
//! > is set to `1`, the terminal shall complete transaction processing online
//! > and shall issue a `GENERATE AC` command requesting an `ARQC` from the ICC.
//! > Otherwise, the terminal shall issue a `GENERATE AC` command requesting a
//! > `TC` from the ICC.

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
pub struct IssuerActionCodeOnline {
	pub tvr: TerminalVerificationResults,
}

impl Default for IssuerActionCodeOnline {
	/// From EMV Book 3:
	/// > If the `Issuer Action Code - Online` is not present, a default value
	/// > with all bits set to `1` shall be used in its place.
	fn default() -> Self {
		Self {
			tvr: TerminalVerificationResults::try_from(
				[
					0b1111_1111u8,
					0b1111_1111,
					0b1111_1111,
					0b1111_1111,
					0b1111_1111,
				]
				.as_slice(),
			)
			.expect("default value for `Issuer Action Code - Online` couldn't be parsed"),
		}
	}
}

impl TryFrom<&[u8]> for IssuerActionCodeOnline {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		Ok(Self {
			tvr: TerminalVerificationResults::try_from(raw_bytes)?,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for IssuerActionCodeOnline {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("If any of the following match the TVR, complete the transaction online:");
		stdout.reset().ok();

		self.tvr
			.display_breakdown_component_value(stdout, indentation);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::IssuerActionCodeOnline;
	use crate::emv::TerminalVerificationResults;

	// Tests
	/// Ensures the parsed value here matches the same parsed value in the TVR.
	#[test]
	fn iac_matches_tvr() {
		let raw_value = [0xFF; 5];
		let expected = TerminalVerificationResults::try_from(raw_value.as_slice())
			.expect("not testing the TVR code here");
		let result = IssuerActionCodeOnline::try_from(raw_value.as_slice())
			.expect("any errors should already be tested by the TVR testing");

		assert_eq!(expected, result.tvr);
	}
	/// Ensures there's no panic.
	#[test]
	fn default_value_is_ok() {
		IssuerActionCodeOnline::default();
	}
}
