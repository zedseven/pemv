//! Everything for handling Transaction Status Information (TSI) values.
//!
//! Information for this can be found in EMV Book 3, under section `C6`.

// Uses
use std::cmp::Ordering;

use derivative::Derivative;

use super::{BitflagValue, EnabledBitRange, Severity};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Clone, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct TransactionStatusInformation {
	#[derivative(PartialEq = "ignore")]
	#[derivative(Hash = "ignore")]
	bytes: <Self as BitflagValue>::Bytes,
	// Byte 1 Values
	pub offline_data_authentication_performed: bool,
	pub cardholder_verification_performed: bool,
	pub card_risk_management_performed: bool,
	pub issuer_authentication_performed: bool,
	pub terminal_risk_management_performed: bool,
	pub script_processing_performed: bool,
}

impl TryFrom<&[u8]> for TransactionStatusInformation {
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
			offline_data_authentication_performed: 0b1000_0000 & bytes[0] > 0,
			cardholder_verification_performed:     0b0100_0000 & bytes[0] > 0,
			card_risk_management_performed:        0b0010_0000 & bytes[0] > 0,
			issuer_authentication_performed:       0b0001_0000 & bytes[0] > 0,
			terminal_risk_management_performed:    0b0000_1000 & bytes[0] > 0,
			script_processing_performed:           0b0000_0100 & bytes[0] > 0,
		})
	}
}

impl BitflagValue for TransactionStatusInformation {
	const NUM_BYTES: usize = 2;
	const USED_BITS_MASK: &'static [u8] = &[0b1111_1100, 0b0000_0000];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.offline_data_authentication_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Offline data authentication was performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.cardholder_verification_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Cardholder verification was performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.card_risk_management_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Card risk management was performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_authentication_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Issuer authentication was performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.terminal_risk_management_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Terminal risk management was performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.script_processing_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 8,
				len: 1,
				explanation: "Script processing was performed".to_owned(),
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
	wrong_byte_count!(super::TransactionStatusInformation, 2);
	bitflag_unique_values!(super::TransactionStatusInformation, 2);
	bitflag_display_bits!(super::TransactionStatusInformation, 2);
}
