//! Everything for handling Cardholder Verification Method (CVM) Results values.
//!
//! Information for this can be found in EMV Book 4, under section `A4`.

// Uses
use std::{
	cmp::Ordering,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use super::{cv_rule::CardholderVerificationRule, BitflagValue, EnabledBitRange, Severity};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Debug)]
pub struct CardholderVerificationMethodResults {
	bytes: <Self as BitflagValue>::Bytes,
	// CV Rule
	pub cv_rule: CardholderVerificationRule,
	// Byte 3 Values
	pub result: CvmResult,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvmResult {
	Unknown = 0b00,
	Failed = 0b01,
	Successful = 0b10,
}
impl TryFrom<u8> for CvmResult {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b00 => Ok(Self::Unknown),
			0b01 => Ok(Self::Failed),
			0b10 => Ok(Self::Successful),
			_ => Err(ParseError::NonCompliant),
		}
	}
}
impl Display for CvmResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Unknown => "Unknown",
			Self::Failed => "Failed",
			Self::Successful => "Successful",
		})
	}
}

impl TryFrom<&[u8]> for CardholderVerificationMethodResults {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		if raw_bytes.len() != Self::NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: Self::NUM_BYTES,
				found: raw_bytes.len(),
			});
		}
		let mut bytes = [0u8; Self::NUM_BYTES];
		for (index, byte) in raw_bytes.iter().enumerate() {
			bytes[index] = byte & Self::USED_BITS_MASK[index];
		}

		Ok(Self {
			bytes,
			cv_rule: CardholderVerificationRule::try_from(&bytes[0..2])?,
			result: CvmResult::try_from(bytes[2])?,
		})
	}
}

impl BitflagValue for CardholderVerificationMethodResults {
	const NUM_BYTES: usize = 3;
	const USED_BITS_MASK: &'static [u8] = &[0b0111_1111, 0b1111_1111, 0b1111_1111];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		let mut cv_rule_bits = self.cv_rule.get_bit_display_information();
		cv_rule_bits.iter_mut().for_each(|b| b.offset += 8);
		enabled_bits.append(&mut cv_rule_bits);
		enabled_bits.push(EnabledBitRange {
			offset: 7,
			len: 8,
			explanation: format!("Result: {}", self.result),
			severity: match self.result {
				CvmResult::Unknown | CvmResult::Successful => Severity::Normal,
				CvmResult::Failed => Severity::Error,
			},
		});

		enabled_bits
	}
}
