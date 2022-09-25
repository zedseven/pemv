//! The terminal type value, typically from EMV tag `0x9F35`.
//!
//! Information for this can be found in EMV Book 4, under section `A1`.

// Uses
use std::cmp::Ordering;

use termcolor::StandardStream;

use crate::{enum_repr_fallible, error::ParseError, util::print_indentation, DisplayBreakdown};

// Enum Implementation
enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TerminalType: u8, ParseError, { |_| ParseError::Unrecognised } {
	AttendedOnlineOnlyFinancialInstitution = 0x11
		=> "Attended, Online-Only, Controlled by a Financial Institution",
	AttendedOfflineWithOnlineCapabilitiesFinancialInstitution = 0x12
		=> "Attended, Offline With Online Capabilities, Controlled by a Financial Institution",
	AttendedOfflineOnlyFinancialInstitution = 0x13
		=> "Attended, Offline-Only, Controlled by a Financial Institution",
	UnattendedOnlineOnlyFinancialInstitution = 0x14
		=> "Unattended, Online-Only, Controlled by a Financial Institution (ATM if it supports \
			Cash Disbursement)",
	UnattendedOfflineWithOnlineCapabilitiesFinancialInstitution = 0x15
		=> "Unattended, Offline With Online Capabilities, Controlled by a Financial Institution \
			(ATM if it supports Cash Disbursement)",
	UnattendedOfflineOnlyFinancialInstitution = 0x16
		=> "Unattended, Offline-Only, Controlled by a Financial Institution (ATM if it supports \
			Cash Disbursement)",
	AttendedOnlineOnlyMerchant = 0x21
		=> "Attended, Online-Only, Controlled by a Merchant",
	AttendedOfflineWithOnlineCapabilitiesMerchant = 0x22
		=> "Attended, Offline With Online Capabilities, Controlled by a Merchant",
	AttendedOfflineOnlyMerchant = 0x23
		=> "Attended, Offline-Only, Controlled by a Merchant",
	UnattendedOnlineOnlyMerchant = 0x24
		=> "Unattended, Online-Only, Controlled by a Merchant",
	UnattendedOfflineWithOnlineCapabilitiesMerchant = 0x25
		=> "Unattended, Offline With Online Capabilities, Controlled by a Merchant",
	UnattendedOfflineOnlyMerchant = 0x26
		=> "Unattended, Offline-Only, Controlled by a Merchant",
	UnattendedOnlineOnlyCardholder = 0x34
		=> "Unattended, Online-Only, Controlled by the Cardholder (self-attended, home PC, etc.)",
	UnattendedOfflineWithOnlineCapabilitiesCardholder = 0x35
		=> "Unattended, Offline With Online Capabilities, Controlled by the Cardholder \
			(self-attended, home PC, etc.)",
	UnattendedOfflineOnlyCardholder = 0x36
		=> "Unattended, Offline-Only, Controlled by the Cardholder (self-attended, home PC, etc.)",
}
}

impl TryFrom<&[u8]> for TerminalType {
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

impl DisplayBreakdown for TerminalType {
	fn display_breakdown(&self, _: &mut StandardStream, indentation: u8) {
		print_indentation(indentation);
		println!("{}", self);
	}
}
