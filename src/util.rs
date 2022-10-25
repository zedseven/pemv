//! Utility functions for internal use by other components of the crate.

// Uses
use crate::error::ParseError;

/// Parses a string into a [`u16`].
pub fn parse_str_to_u16(s: &str) -> Result<u16, ParseError> {
	s.trim().parse().map_err(|_| ParseError::InvalidNumber)
}

/// Parses a hex string into a vector of bytes.
///
/// Original function written by Jake Goulding.
///
/// https://codereview.stackexchange.com/a/201699
pub fn parse_hex_str(hex_asm: &str) -> Vec<u8> {
	let mut hex_bytes = hex_asm
		.as_bytes()
		.iter()
		.filter_map(|b| match b {
			b'0'..=b'9' => Some(b - b'0'),
			b'a'..=b'f' => Some(b - b'a' + 10),
			b'A'..=b'F' => Some(b - b'A' + 10),
			_ => None,
		})
		.fuse();

	let mut bytes = Vec::new();
	while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
		bytes.push(h << 4 | l);
	}
	bytes
}

/// Converts a raw byte slice to [`u64`].
///
/// Panics if the slice is too long.
pub fn byte_slice_to_u64(bytes: &[u8]) -> u64 {
	const BYTES_PER_64_BITS: usize = 8;

	let provided_bytes_length = bytes.len();
	assert!(provided_bytes_length <= BYTES_PER_64_BITS);

	let mut all_bytes = [0u8; BYTES_PER_64_BITS];
	for i in 0..provided_bytes_length {
		all_bytes[i + (BYTES_PER_64_BITS - provided_bytes_length)] = bytes[i];
	}

	u64::from_be_bytes(all_bytes)
}
