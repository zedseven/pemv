//! Everything for handling Terminal Verification Results (TVR) values.
//!
//! Information for this can be found in EMV Book 3, under section `C5`.

// Uses
use std::cmp::Ordering;

use crate::{bitflag_value, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TerminalVerificationResults: 5 {
	0 {
		pub offline_data_authentication_not_performed: bool = 0b1000_0000
			=> "Offline data authentication was not performed",
		pub sda_failed: bool =                                0b0100_0000
			=> (Error, "SDA (Static Data Authentication) failed"),
		pub icc_data_missing: bool =                          0b0010_0000
			=> (Error, "ICC data missing"),
		pub terminal_card_exception: bool =                   0b0001_0000
			=> (Error, "Card appears on terminal exception file"),
		pub dda_failed: bool =                                0b0000_1000
			=> (Error, "DDA (Dynamic Data Authentication) failed"),
		pub cda_failed: bool =                                0b0000_0100
			=> (Error, "CDA (Combined Data Authentication) failed"),
	}
	1 {
		pub icc_terminal_version_mismatch: bool = 0b1000_0000
			=> (Warning, "ICC and terminal have different application versions"),
		pub expired_application: bool =           0b0100_0000
			=> (Error, "Expired application"),
		pub application_not_yet_effective: bool = 0b0010_0000
			=> (Error, "Application not yet effective"),
		pub requested_service_not_allowed: bool = 0b0001_0000
			=> (Error, "Requested service not allowed for card product"),
		pub new_card: bool =                      0b0000_1000
			=> (Warning, "New card"),
	}
	2 {
		pub cardholder_verification_unsuccessful: bool = 0b1000_0000
			=> (Warning, "Cardholder verification was not successful"),
		pub unrecognized_cvm: bool =                     0b0100_0000
			=> (Warning, "Unrecognised CVM (Cardholder Verification Method)"),
		pub pin_try_limit_exceeded: bool =               0b0010_0000
			=> (Error, "PIN try limit exceeded"),
		pub pin_entry_required_but_no_pinpad: bool =     0b0001_0000
			=> (Error, "PIN entry required and PIN pad not present or not working"),
		pub pin_entry_required_but_no_entry: bool =      0b0000_1000
			=> (Warning, "PIN entry required, PIN pad present, but PIN was not entered (PIN \
							  bypass)"),
		pub online_pin_entered: bool =                   0b0000_0100
			=> "Online PIN entered",
	}
	3 {
		pub transaction_exceeds_floor_limit: bool =            0b1000_0000
			=> "Transaction exceeds floor limit",
		pub consecutive_offline_limit_lower_exceeded: bool =   0b0100_0000
			=> "Lower consecutive offline limit exceeded",
		pub consecutive_offline_limit_upper_exceeded: bool =   0b0010_0000
			=> "Upper consecutive offline limit exceeded",
		pub transaction_selected_for_online_processing: bool = 0b0001_0000
			=> "Transaction selected randomly for online processing",
		pub merchant_forced_transaction_online: bool =         0b0000_1000
			=> "Merchant forced transaction online",
	}
	4 {
		pub default_tdol_used: bool =                            0b1000_0000
			=> "Default TDOL (Transaction Certificate Data Object List) used",
		pub issuer_authentication_failed: bool =                 0b0100_0000
			=> (Error, "Issuer authentication failed"),
		pub script_processing_failed_before_final_gen_ac: bool = 0b0010_0000
			=> (Error, "Script processing failed before final GENERATE AC"),
		pub script_processing_failed_after_final_gen_ac: bool =  0b0001_0000
			=> (Error, "Script processing failed after final GENERATE AC"),
	}
}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use crate::{bitflag_display_bits, bitflag_unique_values, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::TerminalVerificationResults, 5);
	bitflag_unique_values!(super::TerminalVerificationResults, 5);
	bitflag_display_bits!(super::TerminalVerificationResults, 5);
}
