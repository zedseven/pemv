//! Everything for handling Terminal Verification Results (TVR) values.

// Uses
use super::{display_breakdown, EnabledBitRange, StatusValue};

// Struct Implementation
pub struct TerminalVerificationResults {
	bits: u64,
	// Byte 1 Values
	offline_data_authentication_not_performed: bool,
	sda_failed: bool,
	icc_data_missing: bool,
	terminal_card_exception: bool,
	dda_failed: bool,
	cda_failed: bool,
	// Byte 2 Values
	icc_terminal_version_mismatch: bool,
	expired_application: bool,
	application_not_yet_effective: bool,
	requested_service_not_allowed: bool,
	new_card: bool,
	// Byte 3 Values
	cardholder_verification_unsuccessful: bool,
	unrecognized_cvm: bool,
	pin_try_limit_exceeded: bool,
	pin_entry_required_but_no_pinpad: bool,
	pin_entry_required_but_no_entry: bool,
	online_pin_entered: bool,
	// Byte 4 Values
	transaction_exceeds_floor_limit: bool,
	consecutive_offline_limit_lower_exceeded: bool,
	consecutive_offline_limit_upper_exceeded: bool,
	transaction_selected_for_online_processing: bool,
	merchant_forced_transaction_online: bool,
	// Byte 5 Values
	default_tdol_used: bool,
	issuer_authentication_failed: bool,
	script_processing_failed_before_final_gen_ac: bool,
	script_processing_failed_after_final_gen_ac: bool,
}

impl StatusValue<u64> for TerminalVerificationResults {
	const NUM_BITS: u8 = 40;
	const USED_BITS_MASK: u64 = 0b1111_1100_1111_1000_1111_1100_1111_1000_1111_0000;

	#[rustfmt::skip]
	fn parse_bits<B: Into<u64>>(bits: B) -> Self {
		let bits = bits.into() & Self::USED_BITS_MASK;
		Self {
			bits,
			offline_data_authentication_not_performed:    (0b1000_0000 << (4 * 8)) & bits > 0,
			sda_failed:                                   (0b0100_0000 << (4 * 8)) & bits > 0,
			icc_data_missing:                             (0b0010_0000 << (4 * 8)) & bits > 0,
			terminal_card_exception:                      (0b0001_0000 << (4 * 8)) & bits > 0,
			dda_failed:                                   (0b0000_1000 << (4 * 8)) & bits > 0,
			cda_failed:                                   (0b0000_0100 << (4 * 8)) & bits > 0,
			icc_terminal_version_mismatch:                (0b1000_0000 << (3 * 8)) & bits > 0,
			expired_application:                          (0b0100_0000 << (3 * 8)) & bits > 0,
			application_not_yet_effective:                (0b0010_0000 << (3 * 8)) & bits > 0,
			requested_service_not_allowed:                (0b0001_0000 << (3 * 8)) & bits > 0,
			new_card:                                     (0b0000_1000 << (3 * 8)) & bits > 0,
			cardholder_verification_unsuccessful:         (0b1000_0000 << (2 * 8)) & bits > 0,
			unrecognized_cvm:                             (0b0100_0000 << (2 * 8)) & bits > 0,
			pin_try_limit_exceeded:                       (0b0010_0000 << (2 * 8)) & bits > 0,
			pin_entry_required_but_no_pinpad:             (0b0001_0000 << (2 * 8)) & bits > 0,
			pin_entry_required_but_no_entry:              (0b0000_1000 << (2 * 8)) & bits > 0,
			online_pin_entered:                           (0b0000_0100 << (2 * 8)) & bits > 0,
			transaction_exceeds_floor_limit:              (0b1000_0000 << 8) & bits > 0,
			consecutive_offline_limit_lower_exceeded:     (0b0100_0000 << 8) & bits > 0,
			consecutive_offline_limit_upper_exceeded:     (0b0010_0000 << 8) & bits > 0,
			transaction_selected_for_online_processing:   (0b0001_0000 << 8) & bits > 0,
			merchant_forced_transaction_online:           (0b0000_1000 << 8) & bits > 0,
			default_tdol_used:                            0b1000_0000 & bits > 0,
			issuer_authentication_failed:                 0b0100_0000 & bits > 0,
			script_processing_failed_before_final_gen_ac: 0b0010_0000 & bits > 0,
			script_processing_failed_after_final_gen_ac:  0b0001_0000 & bits > 0,
		}
	}

	fn display_breakdown(&self) {
		// This is an ugly mess, but these values are display-only and it doesn't make
		// sense to store them anywhere else. :/
		let mut enabled_bits = Vec::with_capacity(4);
		if self.offline_data_authentication_not_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 4 * 8,
				len: 1,
				explanation: "Offline data authentication was not performed",
			});
		}
		if self.sda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 4 * 8,
				len: 1,
				explanation: "SDA failed",
			});
		}
		if self.icc_data_missing {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 4 * 8,
				len: 1,
				explanation: "ICC data missing",
			});
		}
		if self.terminal_card_exception {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 4 * 8,
				len: 1,
				explanation: "Card appears on terminal exception file",
			});
		}
		if self.dda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 4 * 8,
				len: 1,
				explanation: "DDA failed",
			});
		}
		if self.cda_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 4 * 8,
				len: 1,
				explanation: "CDA failed",
			});
		}
		if self.icc_terminal_version_mismatch {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 3 * 8,
				len: 1,
				explanation: "ICC and terminal have different application versions",
			});
		}
		if self.expired_application {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 3 * 8,
				len: 1,
				explanation: "Expired application",
			});
		}
		if self.application_not_yet_effective {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 3 * 8,
				len: 1,
				explanation: "Application not yet effective",
			});
		}
		if self.requested_service_not_allowed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 3 * 8,
				len: 1,
				explanation: "Requested service not allowed for card product",
			});
		}
		if self.new_card {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 3 * 8,
				len: 1,
				explanation: "New card",
			});
		}
		if self.cardholder_verification_unsuccessful {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Cardholder verification was not successful",
			});
		}
		if self.unrecognized_cvm {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Unrecognised CVM",
			});
		}
		if self.pin_try_limit_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "PIN try limit exceeded",
			});
		}
		if self.pin_entry_required_but_no_pinpad {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 2 * 8,
				len: 1,
				explanation: "PIN entry required and PIN pad not present or not working",
			});
		}
		if self.pin_entry_required_but_no_entry {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 2 * 8,
				len: 1,
				explanation: "PIN entry required, PIN pad present, but PIN was not entered",
			});
		}
		if self.online_pin_entered {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 2 * 8,
				len: 1,
				explanation: "Online PIN entered",
			});
		}
		if self.transaction_exceeds_floor_limit {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 8,
				len: 1,
				explanation: "Transaction exceeds floor limit",
			});
		}
		if self.consecutive_offline_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 8,
				len: 1,
				explanation: "Lower consecutive offline limit exceeded",
			});
		}
		if self.consecutive_offline_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 8,
				len: 1,
				explanation: "Upper consecutive offline limit exceeded",
			});
		}
		if self.transaction_selected_for_online_processing {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 8,
				len: 1,
				explanation: "Transaction selected randomly for online processing",
			});
		}
		if self.merchant_forced_transaction_online {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Merchant forced transaction online",
			});
		}
		if self.default_tdol_used {
			enabled_bits.push(EnabledBitRange {
				offset: 7,
				len: 1,
				explanation: "Default TDOL used",
			});
		}
		if self.issuer_authentication_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 6,
				len: 1,
				explanation: "Issuer authentication failed",
			});
		}
		if self.script_processing_failed_before_final_gen_ac {
			enabled_bits.push(EnabledBitRange {
				offset: 5,
				len: 1,
				explanation: "Script processing failed before final GENERATE AC",
			});
		}
		if self.script_processing_failed_after_final_gen_ac {
			enabled_bits.push(EnabledBitRange {
				offset: 4,
				len: 1,
				explanation: "Script processing failed after final GENERATE AC",
			});
		}
		enabled_bits.reverse();

		display_breakdown(self.bits, Self::NUM_BITS, &enabled_bits[..]);
	}
}
