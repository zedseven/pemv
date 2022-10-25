//! Utility functions for internal use by other components of the crate.

// Uses
use anyhow::{Context, Result};

/// Converts a hex string into a raw integer, of size `u16`.
pub fn hex_str_to_u16(hex_str: &str) -> Result<u16> {
	let trimmed_str = hex_str.trim_start_matches("0x");
	u16::from_str_radix(trimmed_str, 16)
		.with_context(|| "unable to parse string as a 16-bit integer")
}

/// Converts a hex string into a raw integer, of size `u32`.
pub fn hex_str_to_u32(hex_str: &str) -> Result<u32> {
	let trimmed_str = hex_str.trim_start_matches("0x");
	u32::from_str_radix(trimmed_str, 16)
		.with_context(|| "unable to parse string as a 32-bit integer")
}

/// Converts a hex string into a raw integer, of size `u64`.
pub fn hex_str_to_u64(hex_str: &str) -> Result<u64> {
	let trimmed_str = hex_str.trim_start_matches("0x");
	u64::from_str_radix(trimmed_str, 16)
		.with_context(|| "unable to parse string as a 64-bit integer")
}
