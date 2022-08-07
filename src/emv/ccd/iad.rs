//! Everything for handling the Issuer Application Data (IAD).
//!
//! Information for this can be found in EMV Book 3, under section `C7`.

// Uses
use std::cmp::Ordering;

use termcolor::{StandardStream, WriteColor};

use super::{CardVerificationResults, CommonCoreIdentifier, FormatCode};
use crate::{
	error::ParseError,
	output_colours::header_colour_spec,
	util::{print_bytes, print_indentation},
	DisplayBreakdown,
};

// Constants
const NUM_BYTES: usize = 32;

// Struct Implementation
#[derive(Debug)]
pub struct IssuerApplicationData {
	pub cci: CommonCoreIdentifier,
	pub format_specific_data: FormatSpecificData,
}

impl TryFrom<&[u8]> for IssuerApplicationData {
	type Error = ParseError;

	fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
		if bytes.len() != NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: NUM_BYTES,
				found: bytes.len(),
			});
		}

		// Byte 0 is the length of EMVCo-defined data in the IAD
		// Byte 16 is the length of the Issuer-Discretionary Data field in the IAD
		if bytes[0] != 0x0F || bytes[16] != 0x0F {
			return Err(ParseError::NonCcdCompliant);
		}

		let cci = CommonCoreIdentifier::try_from(&bytes[1..=1])?;
		let format_specific_data =
			FormatSpecificData::parse_format_data(cci.iad_format_code, bytes)?;

		Ok(Self {
			cci,
			format_specific_data,
		})
	}
}

#[derive(Debug)]
pub enum FormatSpecificData {
	A {
		dki: u8,
		cvr: CardVerificationResults,
		counter_bytes: [u8; 8],
		issuer_discretionary_data: [u8; 15],
	},
}

impl FormatSpecificData {
	/// Parse the IAD according to the specified format.
	///
	/// Expects the entire IAD contents, including the non-format-specific
	/// parts.
	pub fn parse_format_data(format_code: FormatCode, bytes: &[u8]) -> Result<Self, ParseError> {
		match format_code {
			FormatCode::A => {
				let dki = bytes[2];

				let cvr = CardVerificationResults::try_from(&bytes[3..8])?;

				let mut counter_bytes = [0u8; 8];
				counter_bytes.copy_from_slice(&bytes[8..16]);

				let mut issuer_discretionary_data = [0u8; 15];
				issuer_discretionary_data.copy_from_slice(&bytes[17..32]);

				Ok(Self::A {
					dki,
					cvr,
					counter_bytes,
					issuer_discretionary_data,
				})
			}
		}
	}
}

impl PartialEq<FormatCode> for FormatSpecificData {
	fn eq(&self, other: &FormatCode) -> bool {
		match self {
			Self::A { .. } => *other == FormatCode::A,
		}
	}
}
impl PartialEq<FormatSpecificData> for FormatCode {
	fn eq(&self, other: &FormatSpecificData) -> bool {
		other.eq(self)
	}
}

impl DisplayBreakdown for IssuerApplicationData {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("Common Core Identifier:");
		stdout.reset().ok();
		self.cci.display_breakdown(stdout, indentation + 1);

		match &self.format_specific_data {
			FormatSpecificData::A {
				dki,
				cvr,
				counter_bytes,
				issuer_discretionary_data,
			} => {
				// Print the DKI
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				print!("Derivation Key Index:");
				stdout.reset().ok();
				println!(" {:#04X}", dki);

				// Print the CVR
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Card Verification Results:");
				stdout.reset().ok();
				cvr.display_breakdown(stdout, indentation + 1);

				// Print the counter bytes
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Counters: (Payment System-Specific)");
				stdout.reset().ok();
				print_bytes(&counter_bytes[..], 16, indentation + 1);

				// Print the issuer-discretionary data
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Issuer-Discretionary Data");
				stdout.reset().ok();
				print_bytes(&issuer_discretionary_data[..], 16, indentation + 1);
			}
		}
	}
}
