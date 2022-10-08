//! Everything for handling Cardholder Verification Method (CVM) Lists.
//!
//! Information for this can be found in EMV Book 3, under section `10.5`.

// Uses
use std::cmp::Ordering;

use termcolor::{StandardStream, WriteColor};

use super::{BitflagValue, CardholderVerificationRule, CvmCondition};
use crate::{
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{byte_slice_to_u32, num_dec_digits, print_indentation},
	DisplayBreakdown,
};

// Constants
const MIN_BYTES: usize = 8;

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CardholderVerificationMethodList {
	pub x_value: u32,
	pub y_value: u32,
	pub cv_rules: Vec<CardholderVerificationRule>,
}

impl TryFrom<&[u8]> for CardholderVerificationMethodList {
	type Error = ParseError;

	fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
		if bytes.len() < MIN_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Greater,
				expected: MIN_BYTES,
				found: bytes.len(),
			});
		}
		if (bytes.len() - MIN_BYTES) % CardholderVerificationRule::NUM_BYTES != 0 {
			return Err(ParseError::ByteCountNotDivisibleIntoComponents);
		}

		let x_value = byte_slice_to_u32(&bytes[0..4]);
		let y_value = byte_slice_to_u32(&bytes[4..8]);
		let mut cv_rules =
			Vec::with_capacity((bytes.len() - MIN_BYTES) / CardholderVerificationRule::NUM_BYTES);
		for byte_pair in bytes[8..].chunks(CardholderVerificationRule::NUM_BYTES) {
			cv_rules.push(CardholderVerificationRule::try_from(byte_pair)?);
		}

		Ok(Self {
			x_value,
			y_value,
			cv_rules,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for CardholderVerificationMethodList {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8, _: bool) {
		/// This value is chosen as 3 because common currency denominations have
		/// 2 digits for the cents (or equivalent) and this allows 1 additional
		/// digit to represent the whole amount. For example, `$0.00`.
		const MIN_VALUE_DIGITS: usize = 3;

		let header_colour_spec = header_colour_spec();
		let bold_colour_spec = bold_colour_spec();

		// Calculate the 0-padding length for the integer values
		let value_padding_length = num_dec_digits(self.x_value)
			.max(num_dec_digits(self.y_value))
			.max(MIN_VALUE_DIGITS);

		// Only display the X and Y values if one of them is non-zero or one of the CVM
		// conditions references them, since most of the time they go unused
		if self.x_value != 0
			|| self.y_value != 0
			|| self.cv_rules.iter().any(|cv_rule| {
				cv_rule
					.condition
					.internal
					.map_or(false, CvmCondition::references_x_or_y_value)
			}) {
			// Print the X value
			print_indentation(indentation);
			stdout.set_color(&header_colour_spec).ok();
			print!("X Value:");
			stdout.reset().ok();
			println!(
				" {:0>value_padding_length$} (implicit decimal point based on application \
				 currency)",
				self.x_value
			);

			// Print the Y value
			print_indentation(indentation);
			stdout.set_color(&header_colour_spec).ok();
			print!("Y Value:");
			stdout.reset().ok();
			println!(" {:0>value_padding_length$}", self.y_value);
		}

		// Print the CV Rules
		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("Cardholder Verification Rules:");
		stdout.reset().ok();
		for (i, cv_rule) in self.cv_rules.iter().enumerate() {
			// Print the CVM index
			print_indentation(indentation);
			stdout.set_color(&bold_colour_spec).ok();
			println!("CVM {}:", i + 1);
			stdout.reset().ok();

			// Print the method
			print_indentation(indentation + 1);
			stdout.set_color(&bold_colour_spec).ok();
			print!("Method:         ");
			stdout.reset().ok();
			println!(" {}", cv_rule.method);

			// Print the condition
			print_indentation(indentation + 1);
			stdout.set_color(&bold_colour_spec).ok();
			print!("Condition:      ");
			stdout.reset().ok();
			println!(" {}", cv_rule.condition);

			// Print whether to continue if unsuccessful
			print_indentation(indentation + 1);
			stdout.set_color(&bold_colour_spec).ok();
			print!("If Unsuccessful:");
			stdout.reset().ok();
			println!(
				" {}",
				if cv_rule.continue_if_unsuccessful {
					"Next CVM"
				} else {
					"Fail"
				}
			);
		}
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use std::cmp::Ordering;

	use super::{
		super::{CardholderVerificationRule, CvMethod, CvmCondition},
		CardholderVerificationMethodList,
	};
	use crate::error::ParseError;

	// Tests
	#[test]
	fn parse_from_bytes_empty() {
		let expected = Ok(CardholderVerificationMethodList {
			x_value: 0,
			y_value: 0,
			cv_rules: vec![],
		});
		let result = CardholderVerificationMethodList::try_from([0x00; 8].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_valid() {
		let expected = Ok(CardholderVerificationMethodList {
			x_value: 0x2A,
			y_value: 0x7B,
			cv_rules: vec![
				CardholderVerificationRule {
					continue_if_unsuccessful: false,
					method: Some(CvMethod::EncipheredPin).into(),
					condition: Some(CvmCondition::TerminalSupported).into(),
				},
				CardholderVerificationRule {
					continue_if_unsuccessful: true,
					method: Some(CvMethod::Signature).into(),
					condition: Some(CvmCondition::TerminalSupported).into(),
				},
				CardholderVerificationRule {
					continue_if_unsuccessful: false,
					method: Some(CvMethod::FailCvmProcessing).into(),
					condition: Some(CvmCondition::Always).into(),
				},
			],
		});
		let result = CardholderVerificationMethodList::try_from(
			[
				0x00,
				0x00,
				0x00,
				0x2A,
				0x00,
				0x00,
				0x00,
				0x7B,
				0b0000_0100,
				0x03,
				0b0101_1110,
				0x03,
				0b0000_0000,
				0x00,
			]
			.as_slice(),
		);

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_invalid_too_short() {
		let expected = Err(ParseError::ByteCountIncorrect {
			r#type: Ordering::Greater,
			expected: 8,
			found: 3,
		});
		let result = CardholderVerificationMethodList::try_from([0x00; 3].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_invalid_wrong_parity() {
		let expected = Err(ParseError::ByteCountNotDivisibleIntoComponents);
		let result = CardholderVerificationMethodList::try_from([0x00; 9].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_unknown_cvm_condition() {
		let expected = Ok(CardholderVerificationMethodList {
			x_value: 0,
			y_value: 0,
			cv_rules: vec![CardholderVerificationRule {
				continue_if_unsuccessful: false,
				method: Some(CvMethod::EncipheredPinOnline).into(),
				condition: None.into(),
			}],
		});
		let result = CardholderVerificationMethodList::try_from(
			[
				0x00,
				0x00,
				0x00,
				0x00,
				0x00,
				0x00,
				0x00,
				0x00,
				0b0000_0010,
				0x35,
			]
			.as_slice(),
		);

		assert_eq!(expected, result);
	}
}
