//! Everything for handling Transaction Status Information (TSI) values.

// Uses
use super::{display_breakdown, EnabledBitRange, StatusValue};

// Struct Implementation
pub struct TransactionStatusInformation {
	bits: u16,
	// Byte 1 Values
	offline_data_authentication_performed: bool,
	cardholder_verification_performed: bool,
	card_risk_management_performed: bool,
	issuer_authentication_performed: bool,
	terminal_risk_management_performed: bool,
	script_processing_performed: bool,
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

	fn display_breakdown(&self) {
		// This is an ugly mess, but these values are display-only and it doesn't make
		// sense to store them anywhere else. :/
		let mut enabled_bits = Vec::with_capacity(4);
		if self.offline_data_authentication_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Offline data authentication was performed",
			});
		}
		if self.cardholder_verification_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Cardholder verification was performed",
			});
		}
		if self.card_risk_management_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Card risk management was performed",
			});
		}
		if self.issuer_authentication_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Issuer authentication was performed",
			});
		}
		if self.terminal_risk_management_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Terminal risk management was performed",
			});
		}
		if self.script_processing_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 8,
				len: 1,
				explanation: "Script processing was performed",
			});
		}
		enabled_bits.reverse();

		display_breakdown(u64::from(self.bits), Self::NUM_BITS, &enabled_bits[..]);
	}
}
