//! Everything for handling Additional Terminal Capabilities values.
//!
//! Information for this can be found in EMV Book 4, under section `A3`.

// Uses
use std::cmp::Ordering;

use crate::{bitflag_value, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AdditionalTerminalCapabilities: 5 {
	// Transaction Type Capabilities
	0 {
		pub txn_cash: bool =           0b1000_0000 => "Cash",
		pub txn_goods: bool =          0b0100_0000 => "Goods",
		pub txn_services: bool =       0b0010_0000 => "Services",
		pub txn_cashback: bool =       0b0001_0000 => "Cashback",
		pub txn_inquiry: bool =        0b0000_1000
			=> "Inquiry (request for information about one of the cardholder's accounts)",
		pub txn_transfer: bool =       0b0000_0100
			=> "Transfer (between cardholder accounts at the same financial institution)",
		pub txn_payment: bool =        0b0000_0010
			=> "Payment (from a cardholder account to another party)",
		pub txn_administrative: bool = 0b0000_0001 => "Administrative",
	}
	// Transaction Type Capabilities
	1 {
		pub txn_cash_deposit: bool = 0b1000_0000
			=> "Cash Deposit (into a bank account related to an application on the card used)",
	}
	// Terminal Data Input Capabilities
	2 {
		pub input_numeric_keys: bool =                0b1000_0000 => "Numeric keys",
		pub input_alphabetic_and_special_keys: bool = 0b0100_0000
			=> "Alphabetic and special characters keys",
		pub input_command_keys: bool =                0b0010_0000 => "Command keys",
		pub input_function_keys: bool =               0b0001_0000 => "Function keys",
	}
	// Terminal Data Output Capabilities
	3 {
		pub output_print_attendant: bool =    0b1000_0000 => "Print, attendant",
		pub output_print_cardholder: bool =   0b0100_0000 => "Print, cardholder",
		pub output_display_attendant: bool =  0b0010_0000 => "Display, attendant",
		pub output_display_cardholder: bool = 0b0001_0000 => "Display, cardholder",
		pub output_code_table_10: bool =      0b0000_0010 => "ISO/IEC 8859 Code Table 10",
		pub output_code_table_9: bool =       0b0000_0001 => "ISO/IEC 8859 Code Table 9",
	}
	// Terminal Data Output Capabilities
	4 {
		pub output_code_table_8: bool = 0b1000_0000 => "ISO/IEC 8859 Code Table 8",
		pub output_code_table_7: bool = 0b0100_0000 => "ISO/IEC 8859 Code Table 7",
		pub output_code_table_6: bool = 0b0010_0000 => "ISO/IEC 8859 Code Table 6",
		pub output_code_table_5: bool = 0b0001_0000 => "ISO/IEC 8859 Code Table 5",
		pub output_code_table_4: bool = 0b0000_1000 => "ISO/IEC 8859 Code Table 4",
		pub output_code_table_3: bool = 0b0000_0100 => "ISO/IEC 8859 Code Table 3",
		pub output_code_table_2: bool = 0b0000_0010 => "ISO/IEC 8859 Code Table 2",
		pub output_code_table_1: bool = 0b0000_0001 => "ISO/IEC 8859 Code Table 1",
	}
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
