//! Everything for handling Card Verification Results CVR values.
//!
//! Information for this can be found in EMV Book 3, under section `C7.3`.

// Uses
use std::{cmp::Ordering, fmt::Debug};

use crate::{bitflag_value, enum_repr_fallible, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CardVerificationResults: 5 {
	0 {
		pub gen_ac_2_application_cryptogram_type: GenAc2ApplicationCryptogramType = (0b1100_0000 >> 6)
			=> (Normal, "Application cryptogram type returned in 2nd GENERATE AC: {}"),
		pub gen_ac_1_application_cryptogram_type: GenAc1ApplicationCryptogramType = (0b0011_0000 >> 4)
			=> (Normal, "Application cryptogram type returned in 1st GENERATE AC: {}"),
		pub cda_performed: bool =                                                    0b0000_1000
			=> "CDA (Combined Data Authentication) performed",
		pub offline_dda_performed: bool =                                            0b0000_0100
			=> "Offline DDA (Dynamic Data Authentication) performed",
		pub issuer_authentication_not_performed: bool =                              0b0000_0010
			=> (Warning, "Issuer authentication not performed"),
		pub issuer_authentication_failed: bool =                                     0b0000_0001
			=> (Error, "Issuer authentication failed"),
	}
	1 {
		pub pin_try_count: u8 =                                                     (0b1111_0000 >> 4)
			=> (Normal, "PIN try count: {}"),
		pub offline_pin_verification_performed: bool =                               0b0000_1000
			=> "Offline PIN verification performed",
		pub offline_pin_verification_failed: bool =                                  0b0000_0100
			=> (Error, "Offline PIN verification performed and PIN not successfully verified"),
		pub pin_try_limit_exceeded: bool =                                           0b0000_0010
			=> (Error, "PIN try limit exceeded"),
		pub last_online_transaction_not_completed: bool =                            0b0000_0001
			=> (Warning, "Last online transaction not completed"),
	}
	2 {
		pub offline_transaction_count_limit_lower_exceeded: bool =                   0b1000_0000
			=> "Lower offline transaction count limit exceeded",
		pub offline_transaction_count_limit_upper_exceeded: bool =                   0b0100_0000
			=> "Upper offline transaction count limit exceeded",
		pub offline_cumulative_amount_limit_lower_exceeded: bool =                   0b0010_0000
			=> "Lower cumulative offline amount limit exceeded",
		pub offline_cumulative_amount_limit_upper_exceeded: bool =                   0b0001_0000
			=> "Upper cumulative offline amount limit exceeded",
		pub issuer_discretionary_bit_1: bool =                                       0b0000_1000
			=> (Normal, "Issuer-discretionary bit 1"),
		pub issuer_discretionary_bit_2: bool =                                       0b0000_0100
			=> (Normal, "Issuer-discretionary bit 2"),
		pub issuer_discretionary_bit_3: bool =                                       0b0000_0010
			=> (Normal, "Issuer-discretionary bit 3"),
		pub issuer_discretionary_bit_4: bool =                                       0b0000_0001
			=> (Normal, "Issuer-discretionary bit 4"),
	}
	3 {
		pub successful_issuer_script_commands_with_secure_messaging: u8 =           (0b1111_0000 >> 4)
			=> (Normal, "Number of successfully processed issuer script commands containing secure \
				 messaging: {}"),
		pub issuer_script_processing_failed: bool =                                  0b0000_1000
			=> (Error, "Issuer script processing failed"),
		pub offline_data_authentication_failed_on_previous_transaction: bool =       0b0000_0100
			=> (Warning, "Offline data authentication failed on previous transaction"),
		pub go_online_on_next_transaction: bool =                                    0b0000_0010
			=> "Go online on next transaction",
		pub unable_to_go_online: bool =                                              0b0000_0001
			=> (Warning, "Unable to go online"),
	}
	4 {}
}
}

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GenAc1ApplicationCryptogramType: u8, ParseError, { |_| ParseError::NonCcdCompliant } {
	Aac  = 0b00 => "AAC (Application Authentication Cryptogram)",
	Tc   = 0b01 => "TC (Transaction Certificate)",
	Arqc = 0b10 => "ARQC (Authorization Request Cryptogram)",
	Rfu  = 0b11 => "RFU (Reserved For Use)",
}
}

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GenAc2ApplicationCryptogramType: u8, ParseError, { |_| ParseError::NonCcdCompliant } {
	Aac                     = 0b00 => "AAC (Application Authentication Cryptogram)",
	Tc                      = 0b01 => "TC (Transaction Certificate)",
	SecondGenAcNotRequested = 0b10 => "Second GENERATE AC not requested",
	Rfu                     = 0b11 => "RFU (Reserved For Use)",
}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use crate::{bitflag_display_bits, bitflag_unique_values, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::CardVerificationResults, 5);
	bitflag_unique_values!(super::CardVerificationResults, 5);
	bitflag_display_bits!(super::CardVerificationResults, 5);
}
