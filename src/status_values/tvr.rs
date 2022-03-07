//! Everything for handling Terminal Verification Results (TVR) values.

// Uses
use bitflags::bitflags;

use super::{display_breakdown, EnabledBit, StatusValue};

// Struct Implementation
bitflags! {
	#[repr(transparent)]
	pub struct TerminalVerificationResults: u64 {
		// Byte 1 Values
		const OFFLINE_DATA_AUTHENTICATION_NOT_PERFORMED    = 0b1000_0000 << (4 * 8);
		const SDA_FAILED                                   = 0b0100_0000 << (4 * 8);
		const ICC_DATA_MISSING                             = 0b0010_0000 << (4 * 8);
		const TERMINAL_CARD_EXCEPTION                      = 0b0001_0000 << (4 * 8);
		const DDA_FAILED                                   = 0b0000_1000 << (4 * 8);
		const CDA_FAILED                                   = 0b0000_0100 << (4 * 8);
		// Byte 2 Values
		const ICC_TERMINAL_VERSION_MISMATCH                = 0b1000_0000 << (3 * 8);
		const EXPIRED_APPLICATION                          = 0b0100_0000 << (3 * 8);
		const APPLICATION_NOT_YET_EFFECTIVE                = 0b0010_0000 << (3 * 8);
		const REQUESTED_SERVICE_NOT_ALLOWED                = 0b0001_0000 << (3 * 8);
		const NEW_CARD                                     = 0b0000_1000 << (3 * 8);
		// Byte 3 Values
		const CARDHOLDER_VERIFICATION_UNSUCCESSFUL         = 0b1000_0000 << (2 * 8);
		const UNRECOGNIZED_CVM                             = 0b0100_0000 << (2 * 8);
		const PIN_TRY_LIMIT_EXCEEDED                       = 0b0010_0000 << (2 * 8);
		const PIN_ENTRY_REQUIRED_BUT_NO_PINPAD             = 0b0001_0000 << (2 * 8);
		const PIN_ENTRY_REQUIRED_BUT_NO_ENTRY              = 0b0000_1000 << (2 * 8);
		const ONLINE_PIN_ENTERED                           = 0b0000_0100 << (2 * 8);
		// Byte 4 Values
		const TRANSACTION_EXCEEDS_FLOOR_LIMIT              = 0b1000_0000 << 8;
		const CONSECUTIVE_OFFLINE_LIMIT_LOWER_EXCEEDED     = 0b0100_0000 << 8;
		const CONSECUTIVE_OFFLINE_LIMIT_UPPER_EXCEEDED     = 0b0010_0000 << 8;
		const TRANSACTION_SELECTED_FOR_ONLINE_PROCESSING   = 0b0001_0000 << 8;
		const MERCHANT_FORCED_TRANSACTION_ONLINE           = 0b0000_1000 << 8;
		// Byte 5 Values
		const DEFAULT_TDOL_USED                            = 0b1000_0000;
		const ISSUER_AUTHENTICATION_FAILED                 = 0b0100_0000;
		const SCRIPT_PROCESSING_FAILED_BEFORE_FINAL_GEN_AC = 0b0010_0000;
		const SCRIPT_PROCESSING_FAILED_AFTER_FINAL_GEN_AC  = 0b0001_0000;
	}
}

impl StatusValue for TerminalVerificationResults {
	fn display_breakdown(&self) {
		const NUM_BITS: u8 = 40;

		let enabled_bits = RESULT_MAP
			.iter()
			.rev()
			.filter_map(|res| self.contains(res.0).then(|| &res.1))
			.collect::<Vec<_>>();

		display_breakdown(self.bits, NUM_BITS, &enabled_bits[..]);
	}
}

const RESULT_MAP: &[(TerminalVerificationResults, EnabledBit)] = &[
	(
		TerminalVerificationResults::OFFLINE_DATA_AUTHENTICATION_NOT_PERFORMED,
		EnabledBit {
			offset: 7 + 4 * 8,
			explanation: "Offline data authentication was not performed",
		},
	),
	(
		TerminalVerificationResults::SDA_FAILED,
		EnabledBit {
			offset: 6 + 4 * 8,
			explanation: "SDA failed",
		},
	),
	(
		TerminalVerificationResults::ICC_DATA_MISSING,
		EnabledBit {
			offset: 5 + 4 * 8,
			explanation: "ICC data missing",
		},
	),
	(
		TerminalVerificationResults::TERMINAL_CARD_EXCEPTION,
		EnabledBit {
			offset: 4 + 4 * 8,
			explanation: "Card appears on terminal exception file",
		},
	),
	(
		TerminalVerificationResults::DDA_FAILED,
		EnabledBit {
			offset: 3 + 4 * 8,
			explanation: "DDA failed",
		},
	),
	(
		TerminalVerificationResults::CDA_FAILED,
		EnabledBit {
			offset: 2 + 4 * 8,
			explanation: "CDA failed",
		},
	),
	(
		TerminalVerificationResults::ICC_TERMINAL_VERSION_MISMATCH,
		EnabledBit {
			offset: 7 + 3 * 8,
			explanation: "ICC and terminal have different application versions",
		},
	),
	(
		TerminalVerificationResults::EXPIRED_APPLICATION,
		EnabledBit {
			offset: 6 + 3 * 8,
			explanation: "Expired application",
		},
	),
	(
		TerminalVerificationResults::APPLICATION_NOT_YET_EFFECTIVE,
		EnabledBit {
			offset: 5 + 3 * 8,
			explanation: "Application not yet effective",
		},
	),
	(
		TerminalVerificationResults::REQUESTED_SERVICE_NOT_ALLOWED,
		EnabledBit {
			offset: 4 + 3 * 8,
			explanation: "Requested service not allowed for card product",
		},
	),
	(
		TerminalVerificationResults::NEW_CARD,
		EnabledBit {
			offset: 3 + 3 * 8,
			explanation: "New card",
		},
	),
	(
		TerminalVerificationResults::CARDHOLDER_VERIFICATION_UNSUCCESSFUL,
		EnabledBit {
			offset: 7 + 2 * 8,
			explanation: "Cardholder verification was not successful",
		},
	),
	(
		TerminalVerificationResults::UNRECOGNIZED_CVM,
		EnabledBit {
			offset: 6 + 2 * 8,
			explanation: "Unrecognised CVM",
		},
	),
	(
		TerminalVerificationResults::PIN_TRY_LIMIT_EXCEEDED,
		EnabledBit {
			offset: 5 + 2 * 8,
			explanation: "PIN Try Limit exceeded",
		},
	),
	(
		TerminalVerificationResults::PIN_ENTRY_REQUIRED_BUT_NO_PINPAD,
		EnabledBit {
			offset: 4 + 2 * 8,
			explanation: "PIN entry required and PIN pad not present or not working",
		},
	),
	(
		TerminalVerificationResults::PIN_ENTRY_REQUIRED_BUT_NO_ENTRY,
		EnabledBit {
			offset: 3 + 2 * 8,
			explanation: "PIN entry required, PIN pad present, but PIN was not entered",
		},
	),
	(
		TerminalVerificationResults::ONLINE_PIN_ENTERED,
		EnabledBit {
			offset: 2 + 2 * 8,
			explanation: "Online PIN entered",
		},
	),
	(
		TerminalVerificationResults::TRANSACTION_EXCEEDS_FLOOR_LIMIT,
		EnabledBit {
			offset: 7 + 8,
			explanation: "Transaction exceeds floor limit",
		},
	),
	(
		TerminalVerificationResults::CONSECUTIVE_OFFLINE_LIMIT_LOWER_EXCEEDED,
		EnabledBit {
			offset: 6 + 8,
			explanation: "Lower consecutive offline limit exceeded",
		},
	),
	(
		TerminalVerificationResults::CONSECUTIVE_OFFLINE_LIMIT_UPPER_EXCEEDED,
		EnabledBit {
			offset: 5 + 8,
			explanation: "Upper consecutive offline limit exceeded",
		},
	),
	(
		TerminalVerificationResults::TRANSACTION_SELECTED_FOR_ONLINE_PROCESSING,
		EnabledBit {
			offset: 4 + 8,
			explanation: "Transaction selected randomly for online processing",
		},
	),
	(
		TerminalVerificationResults::MERCHANT_FORCED_TRANSACTION_ONLINE,
		EnabledBit {
			offset: 3 + 8,
			explanation: "Merchant forced transaction online",
		},
	),
	(
		TerminalVerificationResults::DEFAULT_TDOL_USED,
		EnabledBit {
			offset: 7,
			explanation: "Default TDOL used",
		},
	),
	(
		TerminalVerificationResults::ISSUER_AUTHENTICATION_FAILED,
		EnabledBit {
			offset: 6,
			explanation: "Issuer authentication failed",
		},
	),
	(
		TerminalVerificationResults::SCRIPT_PROCESSING_FAILED_BEFORE_FINAL_GEN_AC,
		EnabledBit {
			offset: 5,
			explanation: "Script processing failed before final GENERATE AC",
		},
	),
	(
		TerminalVerificationResults::SCRIPT_PROCESSING_FAILED_AFTER_FINAL_GEN_AC,
		EnabledBit {
			offset: 4,
			explanation: "Script processing failed after final GENERATE AC",
		},
	),
];
