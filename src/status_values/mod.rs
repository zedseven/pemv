//! The module for all status value definitions.

// Modules
mod cv_rule;
mod cvm_results;
mod cvr;
mod tsi;
mod tvr;

// Public Exports
pub use cvm_results::*;
pub use cvr::*;
pub use tsi::*;
pub use tvr::*;

/// An EMV status value.
pub trait StatusValue<I: Into<u64>> {
	const NUM_BITS: u8;
	const USED_BITS_MASK: I;

	/// Parses a raw integer into the status value.
	fn parse_bits<B: Into<I>>(bits: B) -> Self;

	/// Fetches the raw bits of the value.
	fn get_bits(&self) -> I;

	/// Fetches the requisite information for display of this value.
	///
	/// The returned set is expected to be provided in left-to-right order.
	fn get_display_information(&self) -> Vec<EnabledBitRange>;
}

// Utility functions for child implementations
#[derive(Debug)]
pub struct EnabledBitRange {
	pub offset: u8,
	pub len: u8,
	pub explanation: String,
}

/// Displays a pretty breakdown of the bits and their meaning.
pub fn display_breakdown<V: StatusValue<I>, I: Into<u64>>(value: &V) {
	// Fetch the required data
	let bits = value.get_bits().into();
	let num_bits = V::NUM_BITS;
	let enabled_bits = &value.get_display_information();

	//dbg!(enabled_bits);

	// Print the hex representation
	println!("Hex: {:#01$X}", bits, usize::from((num_bits / 8) * 2 + 2));

	// Print the binary representation
	println!("Breakdown:");
	for offset in (0..num_bits).rev() {
		if bits & (1 << offset) > 0 {
			print!("1");
		} else {
			print!("0");
		}
		if offset % 8 == 0 && offset > 0 {
			print!("_");
		}
	}
	println!();

	// Print the breakdown
	let mut arm_bits = 0u64;
	let mut multi_bit_value = false;
	for enabled_bit in enabled_bits.iter().rev() {
		arm_bits |= 1 << enabled_bit.offset;
		if enabled_bit.len > 1 {
			multi_bit_value = true;
		}
	}
	// If any enabled bits are multiple bits wide, draw a header line with arms
	// denoting each one's width
	if multi_bit_value {
		let mut current_offset = num_bits - 1;
		for enabled_bit in enabled_bits {
			for i in enabled_bit.offset..=current_offset {
				if (i + 1) % 8 == 0 && i + 1 < num_bits {
					print!(" ");
				}
				if i != enabled_bit.offset {
					print!(" ");
				}
			}
			if enabled_bit.len > 1 {
				print!("\u{251c}");
				for _ in 0..(enabled_bit.len - 2) {
					print!("\u{2500}");
				}
				print!("\u{2518}");
			} else {
				print!("\u{2502}");
			}
			// This somewhat bizarre condition is to handle the case of, for example:
			// offset = 7, len = 8 (1 byte, and the final segment)
			if enabled_bit.offset > enabled_bit.len {
				current_offset = enabled_bit.offset - enabled_bit.len;
			} else {
				current_offset = 0;
			}
		}
		println!();
	}
	for enabled_bit in enabled_bits.iter().rev() {
		// Print leading space
		for i in 1..(num_bits - enabled_bit.offset) {
			if arm_bits & (1 << (num_bits - i)) > 0 {
				print!("\u{2502}");
			} else {
				print!(" ");
			}
			if (num_bits - i) % 8 == 0 {
				print!(" ");
			}
		}
		println!("\u{2514} {}", enabled_bit.explanation);
	}
}
