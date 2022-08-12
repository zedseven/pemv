//! Utility functions for internal use by other components of the crate.

// Uses
use std::iter::successors;

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

/// The number of bytes per 32 bits.
pub const BYTES_PER_32_BITS: usize = 4;
/// Converts a raw byte slice to [`u32`].
///
/// Panics if the slice is too long.
pub fn byte_slice_to_u32(bytes: &[u8]) -> u32 {
	let provided_bytes_length = bytes.len();
	assert!(provided_bytes_length <= BYTES_PER_32_BITS);

	let mut all_bytes = [0u8; BYTES_PER_32_BITS];
	for i in 0..provided_bytes_length {
		all_bytes[(BYTES_PER_32_BITS - provided_bytes_length) + i] = bytes[i];
	}

	u32::from_be_bytes(all_bytes)
}

/// The number of bytes per 64 bits.
pub const BYTES_PER_64_BITS: usize = 8;
/// Converts a raw byte slice to [`u64`].
///
/// Panics if the slice is too long.
pub fn byte_slice_to_u64(bytes: &[u8]) -> u64 {
	let provided_bytes_length = bytes.len();
	assert!(provided_bytes_length <= BYTES_PER_64_BITS);

	let mut all_bytes = [0u8; BYTES_PER_64_BITS];
	for i in 0..provided_bytes_length {
		all_bytes[(BYTES_PER_64_BITS - provided_bytes_length) + i] = bytes[i];
	}

	u64::from_be_bytes(all_bytes)
}

/// Sourced from https://stackoverflow.com/a/69302957.
///
/// Once the `int_log` feature becomes stable, this can be replaced with
/// [`u32::log10`].
pub fn num_dec_digits(value: u32) -> usize {
	successors(Some(value), |&n| (n >= 10).then_some(n / 10)).count()
}

/// Prints the specified amount of indentation on the current line.
pub fn print_indentation(indentation: u8) {
	for _ in 0..indentation {
		print!("\t");
	}
}

/// Pretty-prints bytes as hex.
pub fn print_bytes(bytes: &[u8], bytes_per_line: usize, indentation: u8) {
	for line in bytes.chunks(bytes_per_line) {
		print_indentation(indentation);
		let mut first = true;
		for byte in line {
			if first {
				first = false;
			} else {
				print!(" ");
			}
			print!("{:0>2X}", byte);
		}
		println!();
	}
}
