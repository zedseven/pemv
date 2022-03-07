//! The module for all status value definitions.

// Modules
mod tvr;

// Public Exports
pub use tvr::*;

/// An EMV status value.
pub trait StatusValue {
	/// Prints a breakdown of all enabled flags to stdout.
	fn display_breakdown(&self);
}

// Utility functions for child implementations
struct EnabledBit {
	pub offset: u8,
	pub explanation: &'static str,
}

/// Displays a pretty breakdown of the bits and their meaning.
///
/// `enabled_bits` is expected to be provided in right-to-left order.
fn display_breakdown(bits: u64, num_bits: u8, enabled_bits: &[&EnabledBit]) {
	// Print the binary representation
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
	for enabled_bit in enabled_bits {
		// Print leading space
		for i in 1..(num_bits - enabled_bit.offset) {
			if bits & (1 << (num_bits - i)) > 0 {
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
