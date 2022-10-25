//! Everything for handling Cardholder Verification (CV) Rule values.

// Uses
use super::{EnabledBitRange, StatusValue};

// Struct Implementation
pub struct CardholderVerificationRule {
	bits: u16,
	// Byte 1 Values
	pub continue_if_unsuccessful: bool,
	pub method: Option<CvMethod>,
	// Byte 2 Values
	pub condition: Option<CvmCondition>,
}

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

impl CardholderVerificationRule {
	pub fn new<B: Into<u16>>(bits: B) -> Self {
		Self::parse_bits(bits)
	}
}

impl StatusValue<u16> for CardholderVerificationRule {
	const NUM_BITS: u8 = 16;
	const USED_BITS_MASK: u16 = 0b0111_1111_1111_1111;

	fn parse_bits<B: Into<u16>>(bits: B) -> Self {
		let bits = bits.into() & Self::USED_BITS_MASK;
		Self {
			bits,
			continue_if_unsuccessful: (0b0100_0000 << 8) & bits > 0,
			method: {
				match ((0b0011_1111 << 8) & bits) >> 8 {
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
				match 0b1111_1111 & bits {
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
		}
	}

	fn get_bits(&self) -> u16 {
		self.bits
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.continue_if_unsuccessful {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Apply succeeding CV Rule if this CVM is unsuccessful".to_owned(),
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
		});

		enabled_bits
	}
}
