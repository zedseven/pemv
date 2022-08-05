//! The module for all status value definitions.

// Modules
mod cv_rule;
mod cvm_results;
mod cvr;
mod tsi;
mod tvr;

// Uses
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

// Public Exports
pub use self::{cv_rule::*, cvm_results::*, cvr::*, tsi::*, tvr::*};
use crate::{
	output_colours::{bold_colour_spec, header_colour_spec},
	DisplayBreakdown,
	BITS_PER_BYTE,
};

// Utility functions for child implementations
#[derive(Debug)]
pub struct EnabledBitRange {
	pub offset: u8,
	pub len: u8,
	pub explanation: String,
	pub severity: Severity,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Severity {
	Normal,
	Warning,
	Error,
}

/// An EMV value that's complete as a single unit (fixed size and
/// self-contained).
pub trait UnitValue
where
	Self: Sized,
{
	/// The number of bytes in the raw value.
	const NUM_BYTES: usize;
	/// The bit mask for the bits that are actually used in this value.
	///
	/// Used to zero out any unused bits.
	const USED_BITS_MASK: &'static [u8];
	/// The type for the fixed-size array of bytes that this value is derived
	/// from.
	type Bytes;

	/// Fetches the raw bytes of the value.
	fn get_binary_value(&self) -> Self::Bytes;

	// Fetches the numeric representation of the value.
	fn get_numeric_value(&self) -> u64;

	/// Fetches the requisite information for display of this value.
	///
	/// The returned set is expected to be provided in left-to-right order.
	fn get_display_information(&self) -> Vec<EnabledBitRange>;
}

impl<V> DisplayBreakdown for V
where
	V: UnitValue,
{
	fn display_breakdown(&self, stdout: &mut StandardStream) {
		let header_colour_spec = header_colour_spec();
		let bold_colour_spec = bold_colour_spec();

		// Fetch the required data
		let bits = self.get_numeric_value();
		let num_bytes = V::NUM_BYTES as u8;
		let num_bits = num_bytes * BITS_PER_BYTE;
		let enabled_bits = self.get_display_information();

		//dbg!(enabled_bits);

		// Print the hex representation
		stdout.set_color(&header_colour_spec).ok();
		print!("Hex:");
		stdout.reset().ok();
		println!(" {:#01$X}", bits, usize::from(num_bytes * 2 + 2));

		// Print the binary representation
		stdout.set_color(&header_colour_spec).ok();
		println!("Breakdown:");
		stdout.reset().ok();
		stdout.set_color(&bold_colour_spec).ok();
		for offset in (0..num_bits).rev() {
			if bits & (1 << offset) > 0 {
				print!("1");
			} else {
				print!("0");
			}
			if offset % BITS_PER_BYTE == 0 && offset > 0 {
				print!(" ");
			}
		}
		println!();
		stdout.reset().ok();

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
			for enabled_bit in &enabled_bits {
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
			print!("\u{2514} ");
			stdout
				.set_color(ColorSpec::new().set_fg(match enabled_bit.severity {
					Severity::Normal => None,
					Severity::Warning => Some(Color::Yellow),
					Severity::Error => Some(Color::Red),
				}))
				.ok();
			println!("{}", enabled_bit.explanation);
			stdout.reset().ok();
		}
	}
}
