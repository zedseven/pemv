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
/// <https://codereview.stackexchange.com/a/201699>
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
/// Does the exact same thing as [`parse_hex_str`], but it throws an error if
/// there are any non-hex ASCII characters in the string.
pub fn parse_hex_str_strict(hex_asm: &str) -> Result<Vec<u8>, ParseError> {
	if !hex_asm.is_ascii()
		|| hex_asm.contains(|c| !matches!(c as u8, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'))
	{
		Err(ParseError::InvalidBytes)
	} else {
		Ok(parse_hex_str(hex_asm))
	}
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

/// Converts bytes to a string.
pub fn bytes_to_str(bytes: &[u8]) -> String {
	fn nibble_to_char(num: u8) -> char {
		(match num {
			0..=9 => num + 0x30,
			10..=15 => (num - 10) + 0x41,
			_ => unreachable!("there should be nothing higher than 0xF"),
		}) as char
	}

	let mut result = String::with_capacity(bytes.len() * 2);
	for &byte in bytes {
		result.push(nibble_to_char((0b1111_0000 & byte) >> 4));
		result.push(nibble_to_char(0b0000_1111 & byte));
	}
	result
}

/// Sourced from <https://stackoverflow.com/a/69302957>.
///
/// Once the `int_log` feature becomes stable, this can be replaced with
/// [`u32::log10`].
pub fn num_dec_digits(value: u32) -> usize {
	successors(Some(value), |&n| (n >= 10).then_some(n / 10)).count()
}

/// Prints the specified amount of indentation on the current line.
#[cfg(not(tarpaulin_include))]
pub fn print_indentation(indentation: u8) {
	for _ in 0..indentation {
		print!("\t");
	}
}

/// Pretty-prints bytes as hex.
#[cfg(not(tarpaulin_include))]
pub fn print_bytes(bytes: &[u8], bytes_per_line: usize, indentation: u8) {
	for line in bytes.chunks(bytes_per_line) {
		// Print the hex
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

		// End the line
		println!();
	}
}

/// Pretty-prints bytes as hex with an ASCII readout next to the hex on each
/// line.
#[cfg(not(tarpaulin_include))]
pub fn print_bytes_pretty(bytes: &[u8], bytes_per_line: usize, indentation: u8) {
	for line in bytes.chunks(bytes_per_line) {
		// Print the hex
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

		// Add padding to the end if this is the last line
		for _ in 0..(bytes_per_line - line.len()) {
			print!("   ");
		}

		// Add padding between the hex and ASCII sections
		print!("  ");

		// Print the ASCII readout, replacing unprintable characters
		for &byte in line {
			let printable_char = match byte {
				0x20..=0x7E => byte as char,
				_ => '.',
			};
			print!("{}", printable_char);
		}

		// End the line
		println!();
	}
}

/// Pretty-prints bytes as hex.
///
/// This does not add a line ending afterwards, and all bytes are printed on one
/// line.
#[cfg(not(tarpaulin_include))]
pub fn print_bytes_small(bytes: &[u8]) {
	for byte in bytes {
		print!("{:0>2X}", byte);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		byte_slice_to_u32,
		byte_slice_to_u64,
		bytes_to_str,
		num_dec_digits,
		parse_hex_str,
		parse_hex_str_strict,
		parse_str_to_u16,
	};
	use crate::error::ParseError;

	// Tests
	#[test]
	fn parse_str_to_u16_valid() {
		let expected = Ok(1234);
		let result = parse_str_to_u16("1234");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_str_to_u16_invalid() {
		let expected = Err(ParseError::InvalidNumber);
		let result = parse_str_to_u16("65536");

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_hex_str_uppercase_no_spaces() {
		let expected = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
		let result = parse_hex_str("DEADBEEF");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_uppercase_with_spaces() {
		let expected = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
		let result = parse_hex_str("DE AD  BE EF");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_lowercase_no_spaces() {
		let expected = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
		let result = parse_hex_str("deadbeef");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_lowercase_with_spaces() {
		let expected = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
		let result = parse_hex_str("de ad  be ef");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_mixed() {
		let expected = vec![0x0Au8, 0x6E, 0x42];
		let result = parse_hex_str("  . 0a 6E  42    t ");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_mixed_non_ascii() {
		let expected = vec![0x0Au8, 0x6E, 0x42];
		let result = parse_hex_str("  . 0a 6E  \u{2764}\u{fe0f} 42    t ");

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_hex_str_strict_uppercase_no_spaces() {
		let expected = Ok(vec![0xDEu8, 0xAD, 0xBE, 0xEF]);
		let result = parse_hex_str_strict("DEADBEEF");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_strict_lowercase_no_spaces() {
		let expected = Ok(vec![0xDEu8, 0xAD, 0xBE, 0xEF]);
		let result = parse_hex_str_strict("deadbeef");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_with_spaces() {
		let expected = Err(ParseError::InvalidBytes);
		let result = parse_hex_str_strict("de ad  be ef");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_strict_mixed() {
		let expected = Err(ParseError::InvalidBytes);
		let result = parse_hex_str_strict("  . 0a 6E  42    t ");

		assert_eq!(expected, result);
	}
	#[test]
	fn parse_hex_str_strict_mixed_non_ascii() {
		let expected = Err(ParseError::InvalidBytes);
		let result = parse_hex_str_strict("  . 0a 6E  \u{2764}\u{fe0f} 42    t ");

		assert_eq!(expected, result);
	}

	#[test]
	fn byte_slice_to_u32_single_byte() {
		let expected = 0x25u32;
		let result = byte_slice_to_u32([0x25u8].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn byte_slice_to_u32_multi_byte_not_full() {
		let expected = 0x0012_3456_u32;
		let result = byte_slice_to_u32([0x12u8, 0x34, 0x56].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn byte_slice_to_u32_multi_byte_full() {
		let expected = 0x1234_5678_u32;
		let result = byte_slice_to_u32([0x12u8, 0x34, 0x56, 0x78].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	#[should_panic]
	fn byte_slice_to_u32_too_many_bytes() {
		byte_slice_to_u32([0x12u8, 0x34, 0x56, 0x78, 0x90].as_slice());
	}

	#[test]
	fn byte_slice_to_u64_single_byte() {
		let expected = 0x25u64;
		let result = byte_slice_to_u64([0x25u8].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn byte_slice_to_u64_multi_byte_not_full() {
		let expected = 0x0012_3456_u64;
		let result = byte_slice_to_u64([0x12u8, 0x34, 0x56].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn byte_slice_to_u64_multi_byte_full() {
		let expected = 0x1234_5678_9012_3456_u64;
		let result =
			byte_slice_to_u64([0x12u8, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	#[should_panic]
	fn byte_slice_to_u64_too_many_bytes() {
		byte_slice_to_u64([0x12u8, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78].as_slice());
	}

	#[test]
	fn bytes_to_str_single_byte() {
		let expected = "1A";
		let result = bytes_to_str([0x1Au8].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn bytes_to_str_multi_byte() {
		let expected = "C6A2FF3E2B";
		let result = bytes_to_str([0xC6u8, 0xA2, 0xFF, 0x3E, 0x2B].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn bytes_to_str_multi_byte_all_nibbles() {
		let expected = "0123456789ABCDEF";
		let result = bytes_to_str([0x01u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn num_dec_digits_0() {
		let expected = 1;
		let result = num_dec_digits(0);

		assert_eq!(expected, result);
	}
	#[test]
	fn num_dec_digits_powers_of_10() {
		for n in 0..=7 {
			let expected = n as usize + 1;
			let result = num_dec_digits(10u32.pow(n));

			assert_eq!(expected, result);
		}
	}
	#[test]
	fn num_dec_digits_non_multiple_of_10() {
		let expected = 6;
		let result = num_dec_digits(123_456);

		assert_eq!(expected, result);
	}
}
