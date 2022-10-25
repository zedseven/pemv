//! Everything for handling Cardholder Verification Method (CVM) Results values.

// Uses
use super::{EnabledBitRange, StatusValue};
use crate::status_values::cv_rule::CardholderVerificationRule;

// Struct Implementation
pub struct CardholderVerificationMethodResults {
	bits: u32,
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

impl CardholderVerificationMethodResults {
	pub fn new<B: Into<u32>>(bits: B) -> Self {
		Self::parse_bits(bits)
	}
}

impl StatusValue<u32> for CardholderVerificationMethodResults {
	const NUM_BITS: u8 = 24;
	const USED_BITS_MASK: u32 = 0b0111_1111_1111_1111_1111_1111;

	fn parse_bits<B: Into<u32>>(bits: B) -> Self {
		let bits = bits.into() & Self::USED_BITS_MASK;
		Self {
			bits,
			cv_rule: CardholderVerificationRule::parse_bits(
				(((0b1111_1111_1111_1111 << 8) & bits) >> 8) as u16,
			),
			result: {
				#[allow(clippy::match_same_arms)]
				match 0b1111_1111 & bits {
					0b00 => CvmResult::Unknown,
					0b01 => CvmResult::Failed,
					0b10 => CvmResult::Successful,
					_ => CvmResult::Unknown,
				}
			},
		}
	}

	fn get_bits(&self) -> u32 {
		self.bits
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
		});

		enabled_bits
	}
}
