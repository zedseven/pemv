//! Everything for handling Transaction Status Information (TSI) values.
//!
//! Information for this can be found in EMV Book 3, under section `C6`.

// Uses
use std::cmp::Ordering;

use crate::{bitflag_value, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransactionStatusInformation: 2 {
	0 {
		pub offline_data_authentication_performed: bool = 0b1000_0000 => "Offline data authentication was performed",
		pub cardholder_verification_performed: bool =     0b0100_0000 => "Cardholder verification was performed",
		pub card_risk_management_performed: bool =        0b0010_0000 => "Card risk management was performed",
		pub issuer_authentication_performed: bool =       0b0001_0000 => "Issuer authentication was performed",
		pub terminal_risk_management_performed: bool =    0b0000_1000 => "Terminal risk management was performed",
		pub script_processing_performed: bool =           0b0000_0100 => "Script processing was performed",
	}
	1 {}
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
