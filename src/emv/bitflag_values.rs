//! The module for the trait that defines the interface for bitflag values.

// Uses
use derivative::Derivative;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

// Public Exports
use crate::{
	output_colours::bold_colour_spec,
	util::print_indentation,
	DisplayBreakdown,
	BITS_PER_BYTE,
};

// Utility structures for child implementations

/// Represents a single bit or bit range that's enabled, and contains the
/// meaning & severity of the enabled bit.
#[derive(Clone, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct EnabledBitRange {
	pub offset: u8,
	pub len: u8,
	#[derivative(PartialEq = "ignore")]
	#[derivative(Hash = "ignore")]
	pub explanation: String,
	pub severity: Severity,
}

/// Represents the severity of a bit being enabled.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Severity {
	Normal,
	Warning,
	Error,
}

/// A value that is stored in a bitflag-style format according to the EMV Books.
pub trait BitflagValue
where
	Self: Sized,
{
	/// The number of bytes in the raw value.
	const NUM_BYTES: usize;
	/// The bit mask for the bits that are actually used in this value.
	///
	/// Used to zero out any unused bits.
	const USED_BITS_MASK: &'static [u8];

	/// Gets the representation of the value in raw bytes.
	///
	/// TODO: Once the `generic_const_exprs` feature is stabilised, replace the
	/// return value of this with a fixed-size array.
	fn get_binary_representation(&self) -> Vec<u8>;

	/// Fetches the requisite information for display of this value.
	///
	/// The returned set is expected to be provided in left-to-right order.
	///
	/// If using this, [`Self::get_binary_representation`] is likely also
	/// required. They're separate because the latter has utility outside of
	/// displaying things.
	fn get_bit_display_information(&self) -> Vec<EnabledBitRange>;
}

impl<V> DisplayBreakdown for V
where
	V: BitflagValue,
{
	#[cfg(not(tarpaulin_include))]
	fn display_breakdown(
		&self,
		stdout: &mut StandardStream,
		indentation: u8,
		show_severity_colours: bool,
	) {
		let bold_colour_spec = bold_colour_spec();

		// Fetch the required data
		let num_bytes = V::NUM_BYTES as u8;
		let num_bits = num_bytes * BITS_PER_BYTE;
		let enabled_bit_ranges = self.get_bit_display_information();
		let binary_repr = self.get_binary_representation();

		// Print the binary representation
		print_indentation(indentation);
		stdout.set_color(&bold_colour_spec).ok();
		let mut first_byte = true;
		for byte in binary_repr {
			if !first_byte {
				print!(" ");
			}
			for offset in (0..BITS_PER_BYTE).rev() {
				if byte & (1 << offset) > 0 {
					print!("1");
				} else {
					print!("0");
				}
			}
			first_byte = false;
		}
		println!();
		stdout.reset().ok();

		// Print the breakdown
		let mut arm_bits = 0u64;
		let mut multi_bit_value = false;
		for enabled_bit_range in enabled_bit_ranges.iter().rev() {
			arm_bits |= 1 << enabled_bit_range.offset;
			if enabled_bit_range.len > 1 {
				multi_bit_value = true;
			}
		}
		// If any enabled bits are multiple bits wide, draw a header line with arms
		// denoting each one's width
		if multi_bit_value {
			let mut current_offset = num_bits - 1;
			print_indentation(indentation);
			for enabled_bit_range in &enabled_bit_ranges {
				for i in enabled_bit_range.offset..=current_offset {
					if (i + 1) % 8 == 0 && i + 1 < num_bits {
						print!(" ");
					}
					if i != enabled_bit_range.offset {
						print!(" ");
					}
				}
				if enabled_bit_range.len > 1 {
					print!("\u{251c}");
					for _ in 0..(enabled_bit_range.len - 2) {
						print!("\u{2500}");
					}
					print!("\u{2518}");
				} else {
					print!("\u{2502}");
				}
				// This somewhat bizarre condition is to handle the case of, for example:
				// offset = 7, len = 8 (1 byte, and the final segment)
				if enabled_bit_range.offset > enabled_bit_range.len {
					current_offset = enabled_bit_range.offset - enabled_bit_range.len;
				} else {
					current_offset = 0;
				}
			}
			println!();
		}
		for enabled_bit in enabled_bit_ranges.iter().rev() {
			print_indentation(indentation);
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
				.set_color(ColorSpec::new().set_fg(if show_severity_colours {
					match enabled_bit.severity {
						Severity::Normal => None,
						Severity::Warning => Some(Color::Yellow),
						Severity::Error => Some(Color::Red),
					}
				} else {
					None
				}))
				.ok();
			println!("{}", enabled_bit.explanation);
			stdout.reset().ok();
		}
	}
}
