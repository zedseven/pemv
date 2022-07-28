//! Everything for handling Terminal Verification Results (TVR) values.
//!
//! Information for this can be found in EMV Book 3, under section `C5`.

// Uses
use super::{EnabledBitRange, Severity, UnitValue};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Debug)]
pub struct TerminalVerificationResults {
	bytes: <Self as UnitValue>::Bytes,
	// Byte 1 Values
	pub offline_data_authentication_not_performed: bool,
	pub sda_failed: bool,
	pub icc_data_missing: bool,
	pub terminal_card_exception: bool,
	pub dda_failed: bool,
	pub cda_failed: bool,
	// Byte 2 Values
	pub icc_terminal_version_mismatch: bool,
	pub expired_application: bool,
	pub application_not_yet_effective: bool,
	pub requested_service_not_allowed: bool,
	pub new_card: bool,
	// Byte 3 Values
	pub cardholder_verification_unsuccessful: bool,
	pub unrecognized_cvm: bool,
	pub pin_try_limit_exceeded: bool,
	pub pin_entry_required_but_no_pinpad: bool,
	pub pin_entry_required_but_no_entry: bool,
	pub online_pin_entered: bool,
	// Byte 4 Values
	pub transaction_exceeds_floor_limit: bool,
	pub consecutive_offline_limit_lower_exceeded: bool,
	pub consecutive_offline_limit_upper_exceeded: bool,
	pub transaction_selected_for_online_processing: bool,
	pub merchant_forced_transaction_online: bool,
	// Byte 5 Values
	pub default_tdol_used: bool,
	pub issuer_authentication_failed: bool,
	pub script_processing_failed_before_final_gen_ac: bool,
	pub script_processing_failed_after_final_gen_ac: bool,
}

impl TryFrom<&[u8]> for TerminalVerificationResults {
	type Error = ParseError;

	#[rustfmt::skip]
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
			offline_data_authentication_not_performed:    0b1000_0000 & bytes[0] > 0,
			sda_failed:                                   0b0100_0000 & bytes[0] > 0,
			icc_data_missing:                             0b0010_0000 & bytes[0] > 0,
			terminal_card_exception:                      0b0001_0000 & bytes[0] > 0,
			dda_failed:                                   0b0000_1000 & bytes[0] > 0,
			cda_failed:                                   0b0000_0100 & bytes[0] > 0,
			icc_terminal_version_mismatch:                0b1000_0000 & bytes[1] > 0,
			expired_application:                          0b0100_0000 & bytes[1] > 0,
			application_not_yet_effective:                0b0010_0000 & bytes[1] > 0,
			requested_service_not_allowed:                0b0001_0000 & bytes[1] > 0,
			new_card:                                     0b0000_1000 & bytes[1] > 0,
			cardholder_verification_unsuccessful:         0b1000_0000 & bytes[2] > 0,
			unrecognized_cvm:                             0b0100_0000 & bytes[2] > 0,
			pin_try_limit_exceeded:                       0b0010_0000 & bytes[2] > 0,
			pin_entry_required_but_no_pinpad:             0b0001_0000 & bytes[2] > 0,
			pin_entry_required_but_no_entry:              0b0000_1000 & bytes[2] > 0,
			online_pin_entered:                           0b0000_0100 & bytes[2] > 0,
			transaction_exceeds_floor_limit:              0b1000_0000 & bytes[3] > 0,
			consecutive_offline_limit_lower_exceeded:     0b0100_0000 & bytes[3] > 0,
			consecutive_offline_limit_upper_exceeded:     0b0010_0000 & bytes[3] > 0,
			transaction_selected_for_online_processing:   0b0001_0000 & bytes[3] > 0,
			merchant_forced_transaction_online:           0b0000_1000 & bytes[3] > 0,
			default_tdol_used:                            0b1000_0000 & bytes[4] > 0,
			issuer_authentication_failed:                 0b0100_0000 & bytes[4] > 0,
			script_processing_failed_before_final_gen_ac: 0b0010_0000 & bytes[4] > 0,
			script_processing_failed_after_final_gen_ac:  0b0001_0000 & bytes[4] > 0,
		})
	}
}

impl UnitValue for TerminalVerificationResults {
	const NUM_BYTES: usize = 5;
	const USED_BITS_MASK: &'static [u8] = &[
		0b1111_1100,
		0b1111_1000,
		0b1111_1100,
		0b1111_1000,
		0b1111_0000,
	];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		if self.offline_data_authentication_not_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 4 * 8,
				len: 1,
				explanation: "Offline data authentication was not performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.sda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 4 * 8,
				len: 1,
				explanation: "SDA failed".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.icc_data_missing {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 4 * 8,
				len: 1,
				explanation: "ICC data missing".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.terminal_card_exception {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 4 * 8,
				len: 1,
				explanation: "Card appears on terminal exception file".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.dda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 4 * 8,
				len: 1,
				explanation: "DDA failed".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.cda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 4 * 8,
				len: 1,
				explanation: "CDA failed".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.icc_terminal_version_mismatch {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 3 * 8,
				len: 1,
				explanation: "ICC and terminal have different application versions".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.expired_application {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 3 * 8,
				len: 1,
				explanation: "Expired application".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.application_not_yet_effective {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 3 * 8,
				len: 1,
				explanation: "Application not yet effective".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.requested_service_not_allowed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 3 * 8,
				len: 1,
				explanation: "Requested service not allowed for card product".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.new_card {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 3 * 8,
				len: 1,
				explanation: "New card".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.cardholder_verification_unsuccessful {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Cardholder verification was not successful".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.unrecognized_cvm {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Unrecognised CVM".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.pin_try_limit_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "PIN try limit exceeded".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.pin_entry_required_but_no_pinpad {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 2 * 8,
				len: 1,
				explanation: "PIN entry required and PIN pad not present or not working".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.pin_entry_required_but_no_entry {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 2 * 8,
				len: 1,
				explanation: "PIN entry required, PIN pad present, but PIN was not entered (PIN \
				              bypass)"
					.to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.online_pin_entered {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 2 * 8,
				len: 1,
				explanation: "Online PIN entered".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.transaction_exceeds_floor_limit {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Transaction exceeds floor limit".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.consecutive_offline_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Lower consecutive offline limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.consecutive_offline_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Upper consecutive offline limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.transaction_selected_for_online_processing {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Transaction selected randomly for online processing".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.merchant_forced_transaction_online {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Merchant forced transaction online".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.default_tdol_used {
			enabled_bits.push(EnabledBitRange {
				offset: 7,
				len: 1,
				explanation: "Default TDOL used".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_authentication_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 6,
				len: 1,
				explanation: "Issuer authentication failed".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.script_processing_failed_before_final_gen_ac {
			enabled_bits.push(EnabledBitRange {
				offset: 5,
				len: 1,
				explanation: "Script processing failed before final GENERATE AC".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.script_processing_failed_after_final_gen_ac {
			enabled_bits.push(EnabledBitRange {
				offset: 4,
				len: 1,
				explanation: "Script processing failed after final GENERATE AC".to_owned(),
				severity: Severity::Error,
			});
		}

		enabled_bits
	}
}
