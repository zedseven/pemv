//! The POS entry mode, typically from EMV tag `0x9F39`.
//!
//! The possible values come from the first two digits of the POS Entry Mode in
//! the ISO 8583:1987 specification.
//!
//! This could be incomplete - it's difficult to find a complete list of values
//! online.

// Uses
use std::cmp::Ordering;

use termcolor::StandardStream;

use crate::{enum_no_repr_fallible, error::ParseError, util::print_indentation, DisplayBreakdown};

// Enum Implementation
enum_no_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PosEntryMode: u8, ParseError, { |_| ParseError::Unrecognised } {
	Unknown                      = 0x00        => "Unknown",
	Manual                       = 0x01        => "Manual (keyed entry)",
	MagneticStripe               = 0x02        => "Magnetic Stripe Reader (MSR)",
	Barcode                      = 0x03        => "Barcode",
	Ocr                          = 0x04        => "Optical Character Recognition (OCR)",
	ContactIcc                   = 0x05        => "Integrated Circuit Chip (ICC) - Data Reliable (contact/insert transaction) (CVV can be checked)",
	Track1                       = 0x06        => "Magnetic Stripe Track 1",
	ContactlessIcc               = 0x07 | 0x83 => "Contactless Chip (contactless/tap transaction)",
	ContactlessIccMappingApplied = 0x08 | 0x92 => "Contactless Chip (contactless/tap transaction) - Contactless Mapping Service Applied",
	EcommerceIncludingRemoteChip = 0x09        => "E-Commerce - Including Remote Chip",
	CredentialsOnFile            = 0x10        => "Merchant Has Cardholder Credentials On File (token, recurring payment, etc.)",
	Fallback                     = 0x80        => "ICC Could Not Process - Fallback to MSR",
	EcommerceIncludingChip       = 0x81        => "E-Commerce - Including Chip",
	ViaServer                    = 0x82        => "Via a Server (issuer, acquirer, third-party vendor)",
	MagneticStripeFull           = 0x90        => "Magnetic Stripe Reader (MSR) - Full Track Data - Data Reliable (CVV can be checked)",
	ContactlessMagneticStripe    = 0x91        => "Contactless Magnetic Stripe Data (MSD)",
	ContactIccUnreliable         = 0x95        => "Integrated Circuit Chip (ICC) - Data Unreliable (contact/insert transaction) (CVV cannot be checked)",
}
}

impl TryFrom<&[u8]> for PosEntryMode {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, ParseError> {
		const NUM_BYTES: usize = 1;

		if raw_bytes.len() != NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type:   Ordering::Equal,
				expected: NUM_BYTES,
				found:    raw_bytes.len(),
			});
		}

		Self::try_from(raw_bytes[0])
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for PosEntryMode {
	fn display_breakdown(&self, _: &mut StandardStream, indentation: u8, _: bool) {
		print_indentation(indentation);
		println!("{self}");
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use crate::{enum_byte_slice_result_matches_true_value_result, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::PosEntryMode, 1);
	enum_byte_slice_result_matches_true_value_result!(
		super::PosEntryMode,
		1,
		0x80,
		[0x80].as_slice()
	);
}
