//! Everything for handling Cardholder Verification (CV) Rule values.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use super::{EnabledBitRange, Severity, UnitValue};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Debug)]
pub struct CardholderVerificationRule {
	bytes: <Self as UnitValue>::Bytes,
	// Byte 1 Values
	pub continue_if_unsuccessful: bool,
	pub method: Option<CvMethod>,
	// Byte 2 Values
	pub condition: Option<CvmCondition>,
}

#[derive(Debug)]
pub enum CvMethod {
	FailCvmProcessing,
	PlaintextPin,
	EncipheredPinOnline,
	PlaintextPinWithSignature,
	EncipheredPin,
	EncipheredPinWithSignature,
	Signature,
	NoCvmRequired,
	NoCvmPerformed,
}
#[derive(Debug)]
pub enum CvmCondition {
	Always,
	UnattendedCash,
	NotUnattendedNotManualNotCashback,
	TerminalSupported,
	Manual,
	Cashback,
	InApplicationCurrencyUnderX,
	InApplicationCurrencyOverX,
	InApplicationCurrencyUnderY,
	InApplicationCurrencyOverY,
}

impl TryFrom<&[u8]> for CardholderVerificationRule {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
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
			continue_if_unsuccessful: 0b0100_0000 & bytes[0] > 0,
			method: {
				match 0b0011_1111 & bytes[0] {
					0b00_0000 => Some(CvMethod::FailCvmProcessing),
					0b00_0001 => Some(CvMethod::PlaintextPin),
					0b00_0010 => Some(CvMethod::EncipheredPinOnline),
					0b00_0011 => Some(CvMethod::PlaintextPinWithSignature),
					0b00_0100 => Some(CvMethod::EncipheredPin),
					0b00_0101 => Some(CvMethod::EncipheredPinWithSignature),
					0b01_1110 => Some(CvMethod::Signature),
					0b01_1111 => Some(CvMethod::NoCvmRequired),
					// This value isn't explicitly marked - on page 162 of EMV Book 3 it's simply
					// labelled as `This value is not available for use`
					// On page 121 of EMV Book 4, it mentions `'3F' if no CVM is performed`
					0b11_1111 => Some(CvMethod::NoCvmPerformed),
					_ => None,
				}
			},
			condition: {
				match 0b1111_1111 & bytes[1] {
					0x00 => Some(CvmCondition::Always),
					0x01 => Some(CvmCondition::UnattendedCash),
					0x02 => Some(CvmCondition::NotUnattendedNotManualNotCashback),
					0x03 => Some(CvmCondition::TerminalSupported),
					0x04 => Some(CvmCondition::Manual),
					0x05 => Some(CvmCondition::Cashback),
					0x06 => Some(CvmCondition::InApplicationCurrencyUnderX),
					0x07 => Some(CvmCondition::InApplicationCurrencyOverX),
					0x08 => Some(CvmCondition::InApplicationCurrencyUnderY),
					0x09 => Some(CvmCondition::InApplicationCurrencyOverY),
					_ => None,
				}
			},
		})
	}
}

impl UnitValue for CardholderVerificationRule {
	const NUM_BYTES: usize = 2;
	const USED_BITS_MASK: &'static [u8] = &[0b0111_1111, 0b1111_1111];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.continue_if_unsuccessful {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Apply succeeding CV Rule if this CVM is unsuccessful".to_owned(),
				severity: Severity::Normal,
			});
		}
		enabled_bits.push(EnabledBitRange {
			offset: 5 + 8,
			len: 6,
			explanation: format!(
				"Method: {}",
				if let Some(method) = &self.method {
					match method {
						CvMethod::FailCvmProcessing => "Fail CVM processing",
						CvMethod::PlaintextPin => "Plaintext PIN verification performed by ICC",
						CvMethod::EncipheredPinOnline => "Enciphered PIN verified online",
						CvMethod::PlaintextPinWithSignature => {
							"Plaintext PIN verification performed by ICC and signature (paper)"
						}
						CvMethod::EncipheredPin => "Enciphered PIN verification performed by ICC",
						CvMethod::EncipheredPinWithSignature => {
							"Enciphered PIN verification performed by ICC and signature (paper)"
						}
						CvMethod::Signature => "Signature (paper)",
						CvMethod::NoCvmRequired => "No CVM required",
						CvMethod::NoCvmPerformed => "No CVM performed",
					}
				} else {
					"Unknown (likely issuer or payment system-specific)"
				}
			),
			severity: Severity::Normal,
		});
		enabled_bits.push(EnabledBitRange {
			offset: 7,
			len: 8,
			explanation: format!(
				"Condition: {}",
				if let Some(condition) = &self.condition {
					match condition {
						CvmCondition::Always => "Always",
						CvmCondition::UnattendedCash => "If unattended cash",
						CvmCondition::NotUnattendedNotManualNotCashback => {
							"If not unattended cash and not manual cash and not purchase with \
							 cashback"
						}
						CvmCondition::TerminalSupported => "If terminal supports the CVM",
						CvmCondition::Manual => "If manual cash",
						CvmCondition::Cashback => "If purchase with cashback",
						CvmCondition::InApplicationCurrencyUnderX => {
							"If transaction is in the application currency and is under X value"
						}
						CvmCondition::InApplicationCurrencyOverX => {
							"If transaction is in the application currency and is over X value"
						}
						CvmCondition::InApplicationCurrencyUnderY => {
							"If transaction is in the application currency and is under Y value"
						}
						CvmCondition::InApplicationCurrencyOverY => {
							"If transaction is in the application currency and is over Y value"
						}
					}
				} else {
					"Unknown (likely payment system-specific)"
				}
			),
			severity: Severity::Normal,
		});

		enabled_bits
	}
}
