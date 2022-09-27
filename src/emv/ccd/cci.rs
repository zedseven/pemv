//! Everything for handling the Common Core Identifier (CCI).
//!
//! Information for this can be found in EMV Book 3, under section `C7.1`.

// Uses
use std::cmp::Ordering;

use derivative::Derivative;

use super::super::{BitflagValue, EnabledBitRange, Severity};
use crate::{enum_repr_fallible, error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Clone, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct CommonCoreIdentifier {
	#[derivative(PartialEq = "ignore")]
	#[derivative(Hash = "ignore")]
	pub(crate) bytes: <Self as BitflagValue>::Bytes, // TODO: Remove all this nonsense
	// Byte 1 Values
	pub iad_format_code: FormatCode,
	pub cryptogram_version: CryptogramVersion,
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

impl TryFrom<&[u8]> for CommonCoreIdentifier {
	type Error = ParseError;

	#[rustfmt::skip]
	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		if raw_bytes.len() != Self::NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: Self::NUM_BYTES,
				found: raw_bytes.len(),
			});
		}
		let mut bytes = [0u8; Self::NUM_BYTES];
		for (index, byte) in raw_bytes.iter().enumerate() {
			bytes[index] = byte & Self::USED_BITS_MASK[index];
		}

		Ok(Self {
			bytes,
			iad_format_code:          FormatCode::try_from((0b1111_0000 & bytes[0]) >> 4)?,
			cryptogram_version: CryptogramVersion::try_from(0b0000_1111 & bytes[0])?,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl BitflagValue for CommonCoreIdentifier {
	const NUM_BYTES: usize = 1;
	const USED_BITS_MASK: &'static [u8] = &[0b1111_1111];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_bit_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		enabled_bits.push(EnabledBitRange {
			offset: 7,
			len: 4,
			explanation: format!("IAD Format Code: {}", self.iad_format_code),
			severity: Severity::Normal,
		});
		enabled_bits.push(EnabledBitRange {
			offset: 3,
			len: 4,
			explanation: format!("Cryptogram Version: {}", self.cryptogram_version),
			severity: Severity::Normal,
		});

		enabled_bits
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
			bytes: [0b1010_0110],
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
