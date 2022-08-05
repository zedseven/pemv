//! Everything for handling Cardholder Verification Method (CVM) Lists.
//!
//! Information for this can be found in EMV Book 3, under section `10.5`.

// Uses
use std::cmp::Ordering;

use termcolor::StandardStream;

use super::unit_values::{CardholderVerificationRule, UnitValue};
use crate::{error::ParseError, util::byte_slice_to_u32, DisplayBreakdown};

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
		todo!()
	}
}
