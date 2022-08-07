//! Everything for handling Cardholder Verification Method (CVM) Lists.
//!
//! Information for this can be found in EMV Book 3, under section `10.5`.

// Uses
use std::cmp::Ordering;

use termcolor::{StandardStream, WriteColor};

use super::bitflag_values::{
	BitflagValue,
	CardholderVerificationRule,
	OptionalCvMethod,
	OptionalCvmCondition,
};
use crate::{
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{byte_slice_to_u32, num_dec_digits},
	DisplayBreakdown,
};

// Constants
const MIN_BYTES: usize = 8;

// Struct Implementation
#[derive(Debug)]
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

impl DisplayBreakdown for CardholderVerificationMethodList {
	fn display_breakdown(&self, stdout: &mut StandardStream) {
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

		// Print the X value
		stdout.set_color(&header_colour_spec).ok();
		print!("X Value:");
		stdout.reset().ok();
		println!(
			" {:0>1$} (implicit decimal point based on application currency)",
			self.x_value, value_padding_length
		);

		// Print the Y value
		stdout.set_color(&header_colour_spec).ok();
		print!("Y Value:");
		stdout.reset().ok();
		println!(" {:0>1$}", self.y_value, value_padding_length);

		// Print the CV Rules
		stdout.set_color(&header_colour_spec).ok();
		println!("Cardholder Verification Rules:");
		stdout.reset().ok();
		for (i, cv_rule) in self.cv_rules.iter().enumerate() {
			// Print the CVM index
			stdout.set_color(&bold_colour_spec).ok();
			println!("CVM {}:", i + 1);
			stdout.reset().ok();

			// Print the method
			stdout.set_color(&bold_colour_spec).ok();
			print!("\tMethod:         ");
			stdout.reset().ok();
			println!(" {}", OptionalCvMethod::from(cv_rule.method));

			// Print the condition
			stdout.set_color(&bold_colour_spec).ok();
			print!("\tCondition:      ");
			stdout.reset().ok();
			println!(" {}", OptionalCvmCondition::from(cv_rule.condition));

			// Print whether to continue if unsuccessful
			stdout.set_color(&bold_colour_spec).ok();
			print!("\tIf Unsuccessful:");
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
