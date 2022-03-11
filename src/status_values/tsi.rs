//! Everything for handling Transaction Status Information (TSI) values.

// Uses
use super::{EnabledBitRange, StatusValue};
use crate::status_values::Severity;

// Struct Implementation
pub struct TransactionStatusInformation {
	bits: u16,
	// Byte 1 Values
	pub offline_data_authentication_performed: bool,
	pub cardholder_verification_performed: bool,
	pub card_risk_management_performed: bool,
	pub issuer_authentication_performed: bool,
	pub terminal_risk_management_performed: bool,
	pub script_processing_performed: bool,
}

impl TransactionStatusInformation {
	pub fn new<B: Into<u16>>(bits: B) -> Self {
		Self::parse_bits(bits)
	}
}

impl StatusValue<u16> for TransactionStatusInformation {
	const NUM_BITS: u8 = 16;
	const USED_BITS_MASK: u16 = 0b1111_1100_0000_0000;

	#[rustfmt::skip]
	fn parse_bits<B: Into<u16>>(bits: B) -> Self {
		let bits = bits.into() & Self::USED_BITS_MASK;
		Self {
			bits,
			offline_data_authentication_performed: (0b1000_0000 << 8) & bits > 0,
			cardholder_verification_performed:     (0b0100_0000 << 8) & bits > 0,
			card_risk_management_performed:        (0b0010_0000 << 8) & bits > 0,
			issuer_authentication_performed:       (0b0001_0000 << 8) & bits > 0,
			terminal_risk_management_performed:    (0b0000_1000 << 8) & bits > 0,
			script_processing_performed:           (0b0000_0100 << 8) & bits > 0,
		}
	}

	fn get_bits(&self) -> u16 {
		self.bits
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
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
