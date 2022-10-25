//! Everything for handling the Common Core Identifier (CCI).
//!
//! Information for this can be found in EMV Book 3, under section `C7.1`.

// Uses
use std::{
	cmp::Ordering,
	fmt::{Display, Formatter, Result as FmtResult},
};

use super::{BitflagValue, EnabledBitRange, Severity};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Debug)]
pub struct CommonCoreIdentifier {
	bytes: <Self as BitflagValue>::Bytes,
	// Byte 1 Values
	pub iad_format_code: FormatCode,
	pub cryptogram_version: CryptogramVersion,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormatCode {
	A = 0b1010,
}
impl TryFrom<u8> for FormatCode {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b1010 => Ok(Self::A),
			_ => Err(ParseError::NonCcdCompliant),
		}
	}
}
impl Display for FormatCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::A => "Format A",
		})
	}
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CryptogramVersion {
	TripleDes = 0b0101,
	Aes = 0b0110,
}
impl TryFrom<u8> for CryptogramVersion {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b0101 => Ok(Self::TripleDes),
			0b0110 => Ok(Self::Aes),
			_ => Err(ParseError::NonCcdCompliant),
		}
	}
}
impl Display for CryptogramVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::TripleDes => "Triple DES (3DES)",
			Self::Aes => "AES",
		})
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
