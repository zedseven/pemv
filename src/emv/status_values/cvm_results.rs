//! Everything for handling Cardholder Verification Method (CVM) Results values.

// Uses
use super::{cv_rule::CardholderVerificationRule, EnabledBitRange, Severity, StatusValue};
use crate::{error::ParseError, util::byte_slice_to_u64, ParseFromBytes};

// Struct Implementation
pub struct CardholderVerificationMethodResults {
	bytes: <Self as StatusValue>::Bytes,
	// CV Rule
	pub cv_rule: CardholderVerificationRule,
	// Byte 3 Values
	pub result: CvmResult,
}

pub enum CvmResult {
	Unknown,
	Failed,
	Successful,
}

impl ParseFromBytes for CardholderVerificationMethodResults {
	fn parse_bytes(raw_bytes: &[u8]) -> Result<Self, ParseError> {
		if raw_bytes.len() != Self::NUM_BYTES {
			return Err(ParseError::WrongByteCount {
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
			cv_rule: CardholderVerificationRule::parse_bytes(&bytes[0..=1])?,
			result: {
				#[allow(clippy::match_same_arms)]
				match 0b1111_1111 & bytes[2] {
					0b00 => CvmResult::Unknown,
					0b01 => CvmResult::Failed,
					0b10 => CvmResult::Successful,
					_ => CvmResult::Unknown,
				}
			},
		})
	}
}

impl StatusValue for CardholderVerificationMethodResults {
	const NUM_BYTES: usize = 3;
	const USED_BITS_MASK: &'static [u8] = &[0b0111_1111, 0b1111_1111, 0b1111_1111];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		let mut cv_rule_bits = self.cv_rule.get_display_information();
		cv_rule_bits.iter_mut().for_each(|b| b.offset += 8);
		enabled_bits.append(&mut cv_rule_bits);
		enabled_bits.push(EnabledBitRange {
			offset: 7,
			len: 8,
			explanation: format!(
				"Result: {}",
				match self.result {
					CvmResult::Unknown => "Unknown",
					CvmResult::Failed => "Failed",
					CvmResult::Successful => "Successful",
				}
			),
			severity: match self.result {
				CvmResult::Unknown | CvmResult::Successful => Severity::Normal,
				CvmResult::Failed => Severity::Error,
			},
		});

		enabled_bits
	}
}
