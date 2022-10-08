//! Everything for handling the Issuer Application Data (IAD).
//!
//! Information for this can be found in EMV Book 3, under section `C7`.

// Uses
use termcolor::{StandardStream, WriteColor};

use super::{CardVerificationResults, CommonCoreIdentifier, FormatCode};
use crate::{
	error::ParseError,
	output_colours::header_colour_spec,
	util::{print_bytes, print_indentation},
	DisplayBreakdown,
};

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IssuerApplicationData {
	pub cci:                  CommonCoreIdentifier,
	pub format_specific_data: FormatSpecificData,
}

impl TryFrom<&[u8]> for IssuerApplicationData {
	type Error = ParseError;

	fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
		const NUM_BYTES: usize = 32;

		if bytes.len() != NUM_BYTES {
			return Err(ParseError::NonCcdCompliant);
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for IssuerApplicationData {
	fn display_breakdown(
		&self,
		stdout: &mut StandardStream,
		indentation: u8,
		show_severity_colours: bool,
	) {
		let header_colour_spec = header_colour_spec();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("Common Core Identifier:");
		stdout.reset().ok();
		self.cci
			.display_breakdown(stdout, indentation + 1, show_severity_colours);

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
				println!(" {dki:#04X}");

				// Print the CVR
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Card Verification Results:");
				stdout.reset().ok();
				cvr.display_breakdown(stdout, indentation + 1, show_severity_colours);

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

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		super::{
			CardVerificationResults,
			CommonCoreIdentifier,
			CryptogramVersion,
			FormatCode,
			GenAc1ApplicationCryptogramType,
			GenAc2ApplicationCryptogramType,
		},
		FormatSpecificData,
		IssuerApplicationData,
	};
	use crate::error::ParseError;

	// Tests
	#[test]
	fn ccd_compliant() {
		let expected = Ok(IssuerApplicationData {
			cci:                  CommonCoreIdentifier {
				iad_format_code:    FormatCode::A,
				cryptogram_version: CryptogramVersion::TripleDes,
			},
			format_specific_data: FormatSpecificData::A {
				dki: 1,
				cvr: CardVerificationResults {
					gen_ac_2_application_cryptogram_type:
						GenAc2ApplicationCryptogramType::SecondGenAcNotRequested,
					gen_ac_1_application_cryptogram_type: GenAc1ApplicationCryptogramType::Arqc,
					cda_performed: false,
					offline_dda_performed: false,
					issuer_authentication_not_performed: true,
					issuer_authentication_failed: false,
					pin_try_count: 3,
					offline_pin_verification_performed: false,
					offline_pin_verification_failed: false,
					pin_try_limit_exceeded: false,
					last_online_transaction_not_completed: false,
					offline_transaction_count_limit_lower_exceeded: false,
					offline_transaction_count_limit_upper_exceeded: false,
					offline_cumulative_amount_limit_lower_exceeded: true,
					offline_cumulative_amount_limit_upper_exceeded: true,
					issuer_discretionary_bit_1: false,
					issuer_discretionary_bit_2: false,
					issuer_discretionary_bit_3: false,
					issuer_discretionary_bit_4: false,
					successful_issuer_script_commands_with_secure_messaging: 1,
					issuer_script_processing_failed: false,
					offline_data_authentication_failed_on_previous_transaction: false,
					go_online_on_next_transaction: false,
					unable_to_go_online: false,
				},
				counter_bytes: [0x00; 8],
				issuer_discretionary_data: [
					0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					0x00, 0x00,
				],
			},
		});
		let result = IssuerApplicationData::try_from(
			[
				0x0F, 0xA5, 0x01, 0xA2, 0x30, 0x30, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x0F, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00,
			]
			.as_slice(),
		);

		assert_eq!(expected, result);
	}

	/// This tests with data that's not even the right length.
	#[test]
	fn non_ccd_compliant_wrong_byte_count() {
		let expected = Err(ParseError::NonCcdCompliant);
		let result = IssuerApplicationData::try_from([0x00; 7].as_slice());

		assert_eq!(expected, result);
	}

	/// This tests with data that is the right length, but has the wrong
	/// internal structure.
	#[test]
	fn non_ccd_compliant_invalid_structure() {
		let expected = Err(ParseError::NonCcdCompliant);
		let result = IssuerApplicationData::try_from(
			[
				0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00,
			]
			.as_slice(),
		);

		assert_eq!(expected, result);
	}

	/// This tests with data that is the right length and has the right internal
	/// structure, but the actual data to parse is invalid.
	#[test]
	fn non_ccd_compliant_valid_structure() {
		let expected = Err(ParseError::NonCcdCompliant);
		let result = IssuerApplicationData::try_from(
			[
				0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00,
			]
			.as_slice(),
		);

		assert_eq!(expected, result);
	}

	#[test]
	fn comparing_format_codes() {
		let format_code = FormatCode::A;
		let format_specific_data = FormatSpecificData::A {
			dki: 1,
			cvr: CardVerificationResults {
				gen_ac_2_application_cryptogram_type:
					GenAc2ApplicationCryptogramType::SecondGenAcNotRequested,
				gen_ac_1_application_cryptogram_type: GenAc1ApplicationCryptogramType::Arqc,
				cda_performed: false,
				offline_dda_performed: false,
				issuer_authentication_not_performed: true,
				issuer_authentication_failed: false,
				pin_try_count: 3,
				offline_pin_verification_performed: false,
				offline_pin_verification_failed: false,
				pin_try_limit_exceeded: false,
				last_online_transaction_not_completed: false,
				offline_transaction_count_limit_lower_exceeded: false,
				offline_transaction_count_limit_upper_exceeded: false,
				offline_cumulative_amount_limit_lower_exceeded: true,
				offline_cumulative_amount_limit_upper_exceeded: true,
				issuer_discretionary_bit_1: false,
				issuer_discretionary_bit_2: false,
				issuer_discretionary_bit_3: false,
				issuer_discretionary_bit_4: false,
				successful_issuer_script_commands_with_secure_messaging: 1,
				issuer_script_processing_failed: false,
				offline_data_authentication_failed_on_previous_transaction: false,
				go_online_on_next_transaction: false,
				unable_to_go_online: false,
			},
			counter_bytes: [0x00; 8],
			issuer_discretionary_data: [
				0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00,
			],
		};

		assert_eq!(format_code, format_specific_data);
		assert_eq!(format_specific_data, format_code);
	}
}
