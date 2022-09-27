//! The transaction type value, typically from EMV tag `0x9C`.
//!
//! The possible values come from the first two digits of the ISO 8583:1987
//! Processing Code.
//!
//! This could be incomplete - it's difficult to find a complete list of values
//! online.

// Uses
use std::cmp::Ordering;

use termcolor::StandardStream;

use crate::{enum_repr_fallible, error::ParseError, util::print_indentation, DisplayBreakdown};

// Enum Implementation
enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TransactionType: u8, ParseError, { |_| ParseError::Unrecognised } {
	Purchase         = 0x00 => "Purchase",
	CashAdvance      = 0x01 => "Cash Advance",
	Void             = 0x02 => "Void",
	CashbackPurchase = 0x09 => "Purchase With Cashback",
	Refund           = 0x20 => "Refund",
	BalanceInquiry   = 0x31 => "Balance Inquiry",
	MiniStatement    = 0x38 => "Mini Statement",
	FundTransfer     = 0x40 => "Fund Transfer",
}
}

impl TryFrom<&[u8]> for TransactionType {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		const NUM_BYTES: usize = 1;

		if raw_bytes.len() != NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: NUM_BYTES,
				found: raw_bytes.len(),
			});
		}

		Self::try_from(raw_bytes[0])
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for TransactionType {
	fn display_breakdown(&self, _: &mut StandardStream, indentation: u8) {
		print_indentation(indentation);
		println!("{}", self);
	}
}
