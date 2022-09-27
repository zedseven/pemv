//! Everything for handling Terminal Capabilities values.
//!
//! Information for this can be found in EMV Book 4, under section `A2`.

// Uses
use std::cmp::Ordering;

use derivative::Derivative;

use super::{BitflagValue, EnabledBitRange, Severity};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Clone, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct TerminalCapabilities {
	#[derivative(PartialEq = "ignore")]
	#[derivative(Hash = "ignore")]
	bytes: <Self as BitflagValue>::Bytes,
	// Byte 1 (Card Data Input Capabilities) Values
	pub input_manual_key_entry: bool,
	pub input_magnetic_stripe: bool,
	pub input_icc: bool,
	// Byte 2 (CVM Capabilities) Values
	pub cvm_plaintext_pin_for_icc_verification: bool,
	pub cvm_enciphered_pin_for_online_verification: bool,
	pub cvm_signature: bool,
	pub cvm_enciphered_pin_for_offline_verification: bool,
	pub cvm_no_cvm_required: bool,
	// Byte 3 (Security Capabilities) Values
	pub security_sda: bool,
	pub security_dda: bool,
	pub security_card_capture: bool,
	pub security_cda: bool,
}

impl TryFrom<&[u8]> for TerminalCapabilities {
	type Error = ParseError;

	#[rustfmt::skip]
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
			input_manual_key_entry:                      0b1000_0000 & bytes[0] > 0,
			input_magnetic_stripe:                       0b0100_0000 & bytes[0] > 0,
			input_icc:                                   0b0010_0000 & bytes[0] > 0,
			cvm_plaintext_pin_for_icc_verification:      0b1000_0000 & bytes[1] > 0,
			cvm_enciphered_pin_for_online_verification:  0b0100_0000 & bytes[1] > 0,
			cvm_signature:                               0b0010_0000 & bytes[1] > 0,
			cvm_enciphered_pin_for_offline_verification: 0b0001_0000 & bytes[1] > 0,
			cvm_no_cvm_required:                         0b0000_1000 & bytes[1] > 0,
			security_sda:                                0b1000_0000 & bytes[2] > 0,
			security_dda:                                0b0100_0000 & bytes[2] > 0,
			security_card_capture:                       0b0010_0000 & bytes[2] > 0,
			security_cda:                                0b0000_1000 & bytes[2] > 0,
		})
	}
}

impl BitflagValue for TerminalCapabilities {
	const NUM_BYTES: usize = 3;
	const USED_BITS_MASK: &'static [u8] = &[0b1110_0000, 0b1111_1000, 0b1110_1000];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.input_manual_key_entry {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Manual key entry".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_magnetic_stripe {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Magnetic stripe".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_icc {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "IC with contacts".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cvm_plaintext_pin_for_icc_verification {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Plaintext PIN for ICC verification".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cvm_enciphered_pin_for_online_verification {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Enciphered PIN for online verification".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cvm_signature {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Signature (paper)".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cvm_enciphered_pin_for_offline_verification {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Enciphered PIN for offline verification".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cvm_no_cvm_required {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "No CVM Required".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.security_sda {
			enabled_bits.push(EnabledBitRange {
				offset: 7,
				len: 1,
				explanation: "SDA (Static Data Authentication)".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.security_dda {
			enabled_bits.push(EnabledBitRange {
				offset: 6,
				len: 1,
				explanation: "DDA (Dynamic Data Authentication)".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.security_card_capture {
			enabled_bits.push(EnabledBitRange {
				offset: 5,
				len: 1,
				explanation: "Card capture (ATM retaining the card)".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.security_cda {
			enabled_bits.push(EnabledBitRange {
				offset: 3,
				len: 1,
				explanation: "CDA (Combined Data Authentication)".to_owned(),
				severity: Severity::Normal,
			});
		}

		enabled_bits
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use crate::{bitflag_display_bits, bitflag_unique_values, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::TerminalCapabilities, 3);
	bitflag_unique_values!(super::TerminalCapabilities, 3);
	bitflag_display_bits!(super::TerminalCapabilities, 3);
}
