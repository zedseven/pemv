//! Everything for handling Terminal Capabilities values.
//!
//! Information for this can be found in EMV Book 4, under section `A2`.

// Uses
use std::cmp::Ordering;

use crate::{bitflag_value, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TerminalCapabilities: 3 {
	// Card Data Input Capabilities
	0 {
		pub input_manual_key_entry: bool = 0b1000_0000 => "Manual key entry",
		pub input_magnetic_stripe: bool =  0b0100_0000 => "Magnetic stripe",
		pub input_icc: bool =              0b0010_0000 => "IC with contacts",
	}
	// CVM Capabilities
	1 {
		pub cvm_plaintext_pin_for_icc_verification: bool =      0b1000_0000
			=> "Plaintext PIN for ICC verification",
		pub cvm_enciphered_pin_for_online_verification: bool =  0b0100_0000
			=> "Enciphered PIN for online verification",
		pub cvm_signature: bool =                               0b0010_0000
			=> "Signature (paper)",
		pub cvm_enciphered_pin_for_offline_verification: bool = 0b0001_0000
			=> "Enciphered PIN for offline verification",
		pub cvm_no_cvm_required: bool =                         0b0000_1000
			=> "No CVM Required",
	}
	// Security Capabilities
	2 {
		pub security_sda: bool =          0b1000_0000 => "SDA (Static Data Authentication)",
		pub security_dda: bool =          0b0100_0000 => "DDA (Dynamic Data Authentication)",
		pub security_card_capture: bool = 0b0010_0000 => "Card capture (ATM retaining the card)",
		pub security_cda: bool =          0b0000_1000 => "CDA (Combined Data Authentication)",
	}
}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use crate::{bitflag_display_bits, bitflag_unique_values, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::TerminalCapabilities, 3);
	bitflag_unique_values!(super::TerminalCapabilities, 3);
	bitflag_display_bits!(super::TerminalCapabilities, 3);
}
