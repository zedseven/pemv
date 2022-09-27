//! Everything for handling Additional Terminal Capabilities values.
//!
//! Information for this can be found in EMV Book 4, under section `A3`.

// Uses
use std::cmp::Ordering;

use derivative::Derivative;

use super::{BitflagValue, EnabledBitRange, Severity};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Clone, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct AdditionalTerminalCapabilities {
	#[derivative(PartialEq = "ignore")]
	#[derivative(Hash = "ignore")]
	bytes: <Self as BitflagValue>::Bytes,
	// Byte 1 (Transaction Type Capabilities) Values
	pub txn_cash: bool,
	pub txn_goods: bool,
	pub txn_services: bool,
	pub txn_cashback: bool,
	pub txn_inquiry: bool,
	pub txn_transfer: bool,
	pub txn_payment: bool,
	pub txn_administrative: bool,
	// Byte 2 (Transaction Type Capabilities) Values
	pub txn_cash_deposit: bool,
	// Byte 3 (Terminal Data Input Capabilities) Values
	pub input_numeric_keys: bool,
	pub input_alphabetic_and_special_keys: bool,
	pub input_command_keys: bool,
	pub input_function_keys: bool,
	// Byte 4 (Terminal Data Output Capabilities) Values
	pub output_print_attendant: bool,
	pub output_print_cardholder: bool,
	pub output_display_attendant: bool,
	pub output_display_cardholder: bool,
	pub output_code_table_10: bool,
	pub output_code_table_9: bool,
	// Byte 5 (Terminal Data Output Capabilities) Values
	pub output_code_table_8: bool,
	pub output_code_table_7: bool,
	pub output_code_table_6: bool,
	pub output_code_table_5: bool,
	pub output_code_table_4: bool,
	pub output_code_table_3: bool,
	pub output_code_table_2: bool,
	pub output_code_table_1: bool,
}

impl TryFrom<&[u8]> for AdditionalTerminalCapabilities {
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
			txn_cash:                          0b1000_0000 & bytes[0] > 0,
			txn_goods:                         0b0100_0000 & bytes[0] > 0,
			txn_services:                      0b0010_0000 & bytes[0] > 0,
			txn_cashback:                      0b0001_0000 & bytes[0] > 0,
			txn_inquiry:                       0b0000_1000 & bytes[0] > 0,
			txn_transfer:                      0b0000_0100 & bytes[0] > 0,
			txn_payment:                       0b0000_0010 & bytes[0] > 0,
			txn_administrative:                0b0000_0001 & bytes[0] > 0,
			txn_cash_deposit:                  0b1000_0000 & bytes[1] > 0,
			input_numeric_keys:                0b1000_0000 & bytes[2] > 0,
			input_alphabetic_and_special_keys: 0b0100_0000 & bytes[2] > 0,
			input_command_keys:                0b0010_0000 & bytes[2] > 0,
			input_function_keys:               0b0001_0000 & bytes[2] > 0,
			output_print_attendant:            0b1000_0000 & bytes[3] > 0,
			output_print_cardholder:           0b0100_0000 & bytes[3] > 0,
			output_display_attendant:          0b0010_0000 & bytes[3] > 0,
			output_display_cardholder:         0b0001_0000 & bytes[3] > 0,
			output_code_table_10:              0b0000_0010 & bytes[3] > 0,
			output_code_table_9:               0b0000_0001 & bytes[3] > 0,
			output_code_table_8:               0b1000_0000 & bytes[4] > 0,
			output_code_table_7:               0b0100_0000 & bytes[4] > 0,
			output_code_table_6:               0b0010_0000 & bytes[4] > 0,
			output_code_table_5:               0b0001_0000 & bytes[4] > 0,
			output_code_table_4:               0b0000_1000 & bytes[4] > 0,
			output_code_table_3:               0b0000_0100 & bytes[4] > 0,
			output_code_table_2:               0b0000_0010 & bytes[4] > 0,
			output_code_table_1:               0b0000_0001 & bytes[4] > 0,
		})
	}
}

impl BitflagValue for AdditionalTerminalCapabilities {
	const NUM_BYTES: usize = 5;
	const USED_BITS_MASK: &'static [u8] = &[
		0b1111_1111,
		0b1000_0000,
		0b1111_0000,
		0b1111_0011,
		0b1111_1111,
	];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.txn_cash {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 4 * 8,
				len: 1,
				explanation: "Cash".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_goods {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 4 * 8,
				len: 1,
				explanation: "Goods".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_services {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 4 * 8,
				len: 1,
				explanation: "Services".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_cashback {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 4 * 8,
				len: 1,
				explanation: "Cashback".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_inquiry {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 4 * 8,
				len: 1,
				explanation: "Inquiry (request for information about one of the cardholder's \
				              accounts)"
					.to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_transfer {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 4 * 8,
				len: 1,
				explanation: "Transfer (between cardholder accounts at the same financial \
				              institution)"
					.to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_payment {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 4 * 8,
				len: 1,
				explanation: "Payment (from a cardholder account to another party)".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_administrative {
			enabled_bits.push(EnabledBitRange {
				offset: 4 * 8,
				len: 1,
				explanation: "Administrative".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.txn_cash_deposit {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 3 * 8,
				len: 1,
				explanation: "Cash Deposit (into a bank account related to an application on the \
				              card used)"
					.to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_numeric_keys {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Numeric keys".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_alphabetic_and_special_keys {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Alphabetic and special characters keys".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_command_keys {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "Command keys".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.input_function_keys {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 2 * 8,
				len: 1,
				explanation: "Function keys".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_print_attendant {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Print, attendant".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_print_cardholder {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Print, cardholder".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_display_attendant {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Display, attendant".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_display_cardholder {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Display, cardholder".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_10 {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 8,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 10".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_9 {
			enabled_bits.push(EnabledBitRange {
				offset: 8,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 9".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_8 {
			enabled_bits.push(EnabledBitRange {
				offset: 7,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 8".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_7 {
			enabled_bits.push(EnabledBitRange {
				offset: 6,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 7".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_6 {
			enabled_bits.push(EnabledBitRange {
				offset: 5,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 6".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_5 {
			enabled_bits.push(EnabledBitRange {
				offset: 4,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 5".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_4 {
			enabled_bits.push(EnabledBitRange {
				offset: 3,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 4".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_3 {
			enabled_bits.push(EnabledBitRange {
				offset: 2,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 3".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_2 {
			enabled_bits.push(EnabledBitRange {
				offset: 1,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 2".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.output_code_table_1 {
			enabled_bits.push(EnabledBitRange {
				offset: 0,
				len: 1,
				explanation: "ISO/IEC 8859 Code Table 1".to_owned(),
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
	wrong_byte_count!(super::AdditionalTerminalCapabilities, 5);
	bitflag_unique_values!(super::AdditionalTerminalCapabilities, 5);
	bitflag_display_bits!(super::AdditionalTerminalCapabilities, 5);
}
