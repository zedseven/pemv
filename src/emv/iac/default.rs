//! The IAC value for `Default`.
//!
//! From EMV Book 3:
//! > Together, the `Issuer Action Code - Default` and the `Terminal Action
//! > Code - Default` specify the conditions that cause the transaction to be
//! > rejected if it might have been approved online but the terminal is for any
//! > reason unable to process the transaction online. The `Issuer Action Code -
//! > Default` and the `Terminal Action Code - Default` are used only if the
//! > `Issuer Action Code - Online` and the `Terminal Action Code - Online` were
//! > not used (for example, in case of an offline-only terminal) or indicated a
//! > desire on the part of the issuer or the acquirer to process the
//! > transaction online but the terminal was unable to go online.
//!
//! and
//!
//! > If any bit in `Issuer Action Code - Default` or the `Terminal Action
//! > Code - Default` and the corresponding bit in the TVR are both set to `1`,
//! > the transaction shall be rejected and the terminal shall request an `AAC`
//! > to complete processing. If no such condition appears, the transaction may
//! > be approved offline, and a `GENERATE AC` command shall be issued to the
//! > ICC requesting a `TC`.

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
pub struct IssuerActionCodeDefault {
	pub tvr: TerminalVerificationResults,
}

impl Default for IssuerActionCodeDefault {
	/// From EMV Book 3:
	/// > If the `Issuer Action Code - Default` does not exist, a default value
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
			.expect("default value for `Issuer Action Code - Default` couldn't be parsed"),
		}
	}
}

impl TryFrom<&[u8]> for IssuerActionCodeDefault {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		Ok(Self {
			tvr: TerminalVerificationResults::try_from(raw_bytes)?,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for IssuerActionCodeDefault {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!(
			"If not an online transaction and any of the following match the TVR, reject the \
			 transaction:"
		);
		stdout.reset().ok();

		self.tvr
			.display_breakdown_component_value(stdout, indentation);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::IssuerActionCodeDefault;
	use crate::emv::TerminalVerificationResults;

	// Tests
	/// Ensures the parsed value here matches the same parsed value in the TVR.
	#[test]
	fn iac_matches_tvr() {
		let raw_value = [0xFF; 5];
		let expected = TerminalVerificationResults::try_from(raw_value.as_slice())
			.expect("not testing the TVR code here");
		let result = IssuerActionCodeDefault::try_from(raw_value.as_slice())
			.expect("any errors should already be tested by the TVR testing");

		assert_eq!(expected, result.tvr);
	}
	/// Ensures there's no panic.
	#[test]
	fn default_value_is_ok() {
		IssuerActionCodeDefault::default();
	}
}
