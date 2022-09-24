//! The module for all Tag-Length-Value (TLV) parsing.
//!
//! Some information for this can be found in EMV Book 3, under `Annex B`, but
//! that information is focused on BER-TLV format in particular.

// Modules
pub mod ber_tlv;
mod process_emv_tag;

// Uses
use termcolor::{ColorSpec, StandardStream, WriteColor};

use self::process_emv_tag::process_emv_tag;
use crate::{
	error::ParseError,
	non_composite_value_repr_fallible,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{print_bytes_pretty, print_bytes_small, print_indentation},
	DisplayBreakdown,
};

/// A processed block of EMV data with annotations and parsing results.
pub struct ProcessedEmvBlock<'a> {
	pub nodes: Vec<ProcessedEmvNode<'a>>,
}
impl<'a> From<Vec<ProcessedEmvNode<'a>>> for ProcessedEmvBlock<'a> {
	fn from(nodes: Vec<ProcessedEmvNode<'a>>) -> Self {
		Self { nodes }
	}
}
impl<'a> From<ProcessedEmvBlock<'a>> for Vec<ProcessedEmvNode<'a>> {
	fn from(block: ProcessedEmvBlock<'a>) -> Self {
		block.nodes
	}
}
impl<'a> Default for ProcessedEmvBlock<'a> {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

impl<'a> DisplayBreakdown for ProcessedEmvBlock<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let mut first = true;
		for node in &self.nodes {
			if first {
				first = false;
			} else {
				println!();
			}
			node.display_breakdown(stdout, indentation);
		}
	}
}

impl<'a> TryFrom<RawEmvBlock<'a>> for ProcessedEmvBlock<'a> {
	type Error = ParseError;

	fn try_from(raw_block: RawEmvBlock<'a>) -> Result<Self, Self::Error> {
		let mut nodes = Vec::with_capacity(raw_block.nodes.len());
		for raw_node in raw_block.nodes {
			nodes.push(raw_node.try_into()?);
		}

		Ok(Self { nodes })
	}
}

pub struct ProcessedEmvNode<'a> {
	pub tag: ProcessedEmvTag<'a>,
	pub child_block: ProcessedEmvBlock<'a>,
}

impl<'a> DisplayBreakdown for ProcessedEmvNode<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		// Display the tag
		self.tag.display_breakdown(stdout, indentation);

		// Display the child tags (if any)
		if !self.child_block.nodes.is_empty() {
			let header_colour_spec = header_colour_spec();

			print_indentation(indentation);
			stdout.set_color(&header_colour_spec).ok();
			println!("Constructed Data Object's Child Tags:");
			stdout.reset().ok();

			self.child_block.display_breakdown(stdout, indentation + 1);
		}
	}
}

impl<'a> TryFrom<RawEmvNode<'a>> for ProcessedEmvNode<'a> {
	type Error = ParseError;

	fn try_from(raw_node: RawEmvNode<'a>) -> Result<Self, Self::Error> {
		Ok(Self {
			tag: raw_node.tag.try_into()?,
			child_block: raw_node.child_block.try_into()?,
		})
	}
}

/// A processed EMV tag with as much information as possible about it.
///
/// The [`RawEmvTag`] is preserved in all cases because it can carry sub-tags.
pub enum ProcessedEmvTag<'a> {
	Raw {
		raw_tag: RawEmvTag<'a>,
	},
	Annotated {
		name: &'static str,
		raw_tag: RawEmvTag<'a>,
	},
	Parsed {
		name: &'static str,
		parsed: Box<dyn DisplayBreakdown>,
		raw_tag: RawEmvTag<'a>,
	},
}

impl<'a> ProcessedEmvTag<'a> {
	pub fn parse_raw<P>(
		name: &'static str,
		raw_tag: RawEmvTag<'a>,
		parsing_fn: P,
	) -> Result<Self, ParseError>
	where
		P: Fn(&'a [u8]) -> Result<Box<dyn DisplayBreakdown>, ParseError>,
	{
		match raw_tag.data {
			EmvData::Normal(data) => Ok(Self::Parsed {
				name,
				parsed: parsing_fn(data)?,
				raw_tag,
			}),
			EmvData::Masked => Ok(Self::Annotated { name, raw_tag }),
		}
	}

	/// Same as [`Self::parse_raw`], but handles an error with a different
	/// annotation if `is_unrecognised_error` returns `true`. If it returns
	/// `false`, the function returns the error instead.
	///
	/// `name_recognised` is used if the value could be successfully parsed.
	///
	/// `name_unrecognised` is used if the value could not be successfully
	/// parsed, and `parsing_fn` returned an error that matched
	/// `is_unrecognised_error`.
	pub fn parse_raw_unrecognised<P, E>(
		name_recognised: &'static str,
		name_unrecognised: &'static str,
		raw_tag: RawEmvTag<'a>,
		parsing_fn: P,
		is_unrecognised_error: E,
	) -> Result<Self, ParseError>
	where
		P: Fn(&'a [u8]) -> Result<Box<dyn DisplayBreakdown>, ParseError>,
		E: Fn(&ParseError) -> bool,
	{
		match raw_tag.data {
			EmvData::Normal(data) => match parsing_fn(data) {
				Ok(parsed) => Ok(Self::Parsed {
					name: name_recognised,
					parsed,
					raw_tag,
				}),
				Err(error) => {
					if is_unrecognised_error(&error) {
						Ok(Self::Annotated {
							name: name_unrecognised,
							raw_tag,
						})
					} else {
						Err(error)
					}
				}
			},
			EmvData::Masked => Ok(Self::Annotated {
				name: name_recognised,
				raw_tag,
			}),
		}
	}

	pub fn annotate_raw(name: &'static str, raw_tag: RawEmvTag<'a>) -> Self {
		Self::Annotated { name, raw_tag }
	}
}

impl<'a> DisplayBreakdown for ProcessedEmvTag<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		fn print_tag_name(
			stdout: &mut StandardStream,
			indentation: u8,
			header_colour_spec: &ColorSpec,
			tag: &[u8],
			length: Option<usize>,
			name_option: Option<&str>,
		) {
			let bold_colour_spec = bold_colour_spec();

			let name = name_option.unwrap_or("Unknown");

			print_indentation(indentation);
			stdout.set_color(header_colour_spec).ok();
			print!("Tag:");
			stdout.reset().ok();
			print!(" 0x");
			stdout.set_color(&bold_colour_spec).ok();
			print_bytes_small(tag);
			stdout.reset().ok();
			if let Some(len) = length {
				println!(
					" - {} byte{} - {}",
					len,
					if len == 1 { "" } else { "s" },
					name
				);
			} else {
				println!(" - ?? bytes - {}", name);
			}
		}

		let header_colour_spec = header_colour_spec();

		match self {
			ProcessedEmvTag::Raw { raw_tag } => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					raw_tag.tag,
					raw_tag.data.len(),
					None,
				);

				// Display the raw value
				raw_tag.display_breakdown(stdout, indentation);
			}
			ProcessedEmvTag::Annotated { name, raw_tag } => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					raw_tag.tag,
					raw_tag.data.len(),
					Some(name),
				);

				// Display the raw value
				raw_tag.display_breakdown(stdout, indentation);
			}
			ProcessedEmvTag::Parsed {
				name,
				parsed,
				raw_tag,
			} => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					raw_tag.tag,
					raw_tag.data.len(),
					Some(name),
				);

				// Display the raw value
				raw_tag.display_breakdown(stdout, indentation);

				// Display the parsed value
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Parsed:");
				stdout.reset().ok();
				parsed.display_breakdown(stdout, indentation + 1);
			}
		}
	}
}

impl<'a> TryFrom<RawEmvTag<'a>> for ProcessedEmvTag<'a> {
	type Error = ParseError;

	fn try_from(value: RawEmvTag<'a>) -> Result<Self, Self::Error> {
		process_emv_tag(value)
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawEmvBlock<'a> {
	pub nodes: Vec<RawEmvNode<'a>>,
}
impl<'a> From<Vec<RawEmvNode<'a>>> for RawEmvBlock<'a> {
	fn from(nodes: Vec<RawEmvNode<'a>>) -> Self {
		Self { nodes }
	}
}
impl<'a> From<RawEmvBlock<'a>> for Vec<RawEmvNode<'a>> {
	fn from(block: RawEmvBlock<'a>) -> Self {
		block.nodes
	}
}
impl<'a> Default for RawEmvBlock<'a> {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawEmvNode<'a> {
	pub tag: RawEmvTag<'a>,
	pub child_block: RawEmvBlock<'a>,
}

/// A raw EMV tag-value pair, with no meaning associated with it.
///
/// This can be further parsed based on the tag value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RawEmvTag<'a> {
	pub tag: &'a [u8],
	pub class: TagClass,
	pub data_object_type: DataObjectType,
	pub data: EmvData<'a>,
}

impl<'a> DisplayBreakdown for RawEmvTag<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();
		match self.data {
			EmvData::Normal(data) => {
				if data.is_empty() {
					return;
				}

				// Display the tag value
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Raw:");
				stdout.reset().ok();
				print_bytes_pretty(data, 16, indentation + 1);
			}
			EmvData::Masked => {
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("* Masked *");
				stdout.reset().ok();
			}
		}
	}
}

non_composite_value_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TagClass: u8, ParseError::NonCompliant {
	Universal       = 0b00 => "Universal",
	Application     = 0b01 => "Application",
	ContextSpecific = 0b10 => "Context-Specific",
	Private         = 0b11 => "Private",
}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataObjectType {
	Primitive,
	Constructed,
}

/// EMV data, encoding the ability for data to be masked and therefore
/// inaccessible.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EmvData<'a> {
	Normal(&'a [u8]),
	Masked,
}

impl<'a> EmvData<'a> {
	/// Returns the data length, or `None` if unknown.
	pub fn len(self) -> Option<usize> {
		match self {
			EmvData::Normal(data) => Some(data.len()),
			EmvData::Masked => None,
		}
	}

	pub fn from(data: &'a [u8], masking_characters: &[char]) -> Self {
		for masking_char in masking_characters {
			if data.iter().all(|byte| *byte as char == *masking_char) {
				return Self::Masked;
			}
		}

		Self::Normal(data)
	}
}
