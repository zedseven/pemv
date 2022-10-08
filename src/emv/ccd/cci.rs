//! Everything for handling the Common Core Identifier (CCI).
//!
//! Information for this can be found in EMV Book 3, under section `C7.1`.

// Uses
use std::cmp::Ordering;

use crate::{bitflag_value, enum_repr_fallible, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommonCoreIdentifier: 1 {
	0 {
		pub iad_format_code: FormatCode =          (0b1111_0000 >> 4) => (Normal, "IAD Format Code: {}"),
		pub cryptogram_version: CryptogramVersion = 0b0000_1111 => (Normal, "Cryptogram Version: {}"),
	}
}
}

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FormatCode: u8, ParseError, { |_| ParseError::NonCcdCompliant } {
	A = 0b1010 => "Format A",
}
}

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CryptogramVersion: u8, ParseError, { |_| ParseError::NonCcdCompliant } {
	TripleDes = 0b0101 => "Triple DES (3DES)",
	Aes       = 0b0110 => "AES",
}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{CommonCoreIdentifier, CryptogramVersion, FormatCode};
	use crate::{error::ParseError, wrong_byte_count};

	// Tests
	wrong_byte_count!(super::CommonCoreIdentifier, 1);

	#[test]
	fn parse_from_bytes_valid() {
		let expected = Ok(CommonCoreIdentifier {
			iad_format_code: FormatCode::A,
			cryptogram_version: CryptogramVersion::Aes,
		});
		let result = CommonCoreIdentifier::try_from([0b1010_0110].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_non_ccd_compliant() {
		let expected = Err(ParseError::NonCcdCompliant);
		let result = CommonCoreIdentifier::try_from([0b1000_1010].as_slice());

		assert_eq!(expected, result);
	}
}
