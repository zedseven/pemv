//! The transaction type value, typically from EMV tag `0x9C`.
//!
//! The possible values come from the first two digits of the ISO 8583:1987
//! Processing Code.
//!
//! This could be incomplete - it's difficult to find a complete list of values
//! online.

// Uses
use std::{
	cmp::Ordering,
	fmt::{Display, Formatter, Result as FmtResult},
};

use termcolor::StandardStream;

use crate::{error::ParseError, util::print_indentation, DisplayBreakdown};

// Enum Implementation
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
	Purchase = 0x00,
	CashAdvance = 0x01,
	Void = 0x02,
	CashbackPurchase = 0x09,
	Refund = 0x20,
	BalanceInquiry = 0x31,
	MiniStatement = 0x38,
	FundTransfer = 0x40,
}
impl TryFrom<u8> for TransactionType {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Self::Purchase),
			0x01 => Ok(Self::CashAdvance),
			0x02 => Ok(Self::Void),
			0x09 => Ok(Self::CashbackPurchase),
			0x20 => Ok(Self::Refund),
			0x31 => Ok(Self::BalanceInquiry),
			0x38 => Ok(Self::MiniStatement),
			0x40 => Ok(Self::FundTransfer),
			_ => Err(ParseError::Unsupported),
		}
	}
}
impl Display for TransactionType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Purchase => "Purchase",
			Self::CashAdvance => "Cash Advance",
			Self::Void => "Void",
			Self::CashbackPurchase => "Purchase With Cashback",
			Self::Refund => "Refund",
			Self::BalanceInquiry => "Balance Inquiry",
			Self::MiniStatement => "Mini Statement",
			Self::FundTransfer => "Fund Transfer",
		})
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

impl DisplayBreakdown for TransactionType {
	fn display_breakdown(&self, _: &mut StandardStream, indentation: u8) {
		print_indentation(indentation);
		println!("{}", self);
	}
}
