//! Everything for handling Cardholder Verification Method (CVM) Results values.
//!
//! Information for this can be found in EMV Book 4, under section `A4`.

// Uses
use std::{cmp::Ordering, fmt::Debug};

use super::{cv_rule::CardholderVerificationRule, BitflagValue, EnabledBitRange, Severity};
use crate::{enum_repr_fallible, error::ParseError};

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CardholderVerificationMethodResults {
	// CV Rule
	pub cv_rule: CardholderVerificationRule,
	// Byte 3 Values
	pub result:  CvmResult,
}

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CvmResult: u8, ParseError, { |_| ParseError::NonCompliant } {
	Unknown    = 0b00 => "Unknown",
	Failed     = 0b01 => "Failed",
	Successful = 0b10 => "Successful",
}
}

impl TryFrom<&[u8]> for CardholderVerificationMethodResults {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		if raw_bytes.len() != Self::NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type:   Ordering::Equal,
				expected: Self::NUM_BYTES,
				found:    raw_bytes.len(),
			});
		}
		let mut bytes = [0u8; Self::NUM_BYTES];
		for (index, byte) in raw_bytes.iter().enumerate() {
			bytes[index] = byte & Self::USED_BITS_MASK[index];
		}

		Ok(Self {
			cv_rule: CardholderVerificationRule::try_from(&bytes[0..2])?,
			result:  CvmResult::try_from(bytes[2])?,
		})
	}
}

impl BitflagValue for CardholderVerificationMethodResults {
	const NUM_BYTES: usize = 3;
	const USED_BITS_MASK: &'static [u8] = &[0b0111_1111, 0b1111_1111, 0b1111_1111];

	fn get_binary_representation(&self) -> Vec<u8> {
		let mut result = self.cv_rule.get_binary_representation();
		result.extend([self.result as u8]);

		result
	}

	#[cfg(not(tarpaulin_include))]
	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		let mut cv_rule_bits = self.cv_rule.get_bit_display_information();
		cv_rule_bits.iter_mut().for_each(|b| b.offset += 8);
		enabled_bits.append(&mut cv_rule_bits);
		enabled_bits.push(EnabledBitRange {
			offset:      7,
			len:         8,
			explanation: format!("Result: {}", self.result),
			severity:    match self.result {
				CvmResult::Unknown | CvmResult::Successful => Severity::Normal,
				CvmResult::Failed => Severity::Error,
			},
		});

		enabled_bits
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		super::{CardholderVerificationRule, CvMethod, CvmCondition},
		CardholderVerificationMethodResults,
		CvmResult,
	};
	use crate::wrong_byte_count;

	// Tests
	wrong_byte_count!(super::CardholderVerificationMethodResults, 3);

	#[test]
	fn parse_from_bytes_valid() {
		let expected = Ok(CardholderVerificationMethodResults {
			cv_rule: CardholderVerificationRule {
				continue_if_unsuccessful: true,
				method:                   Some(CvMethod::Signature).into(),
				condition:                Some(CvmCondition::TerminalSupported).into(),
			},
			result:  CvmResult::Successful,
		});
		let result =
			CardholderVerificationMethodResults::try_from([0b0101_1110, 0x03, 0b10].as_slice());

		assert_eq!(expected, result);
	}
}
