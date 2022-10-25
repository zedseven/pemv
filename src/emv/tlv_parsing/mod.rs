//! The module for all Tag-Length-Value (TLV) parsing.
//!
//! Some information for this can be found in EMV Book 3, under `Annex B`, but
//! that information is focused on BER-TLV format in particular.

// Modules
pub mod auto_tlv;
pub mod ber_tlv;
pub mod ingenico_tlv;
mod process_emv_tag;

// Uses
use std::fmt::{Display, Formatter, Result as FormatResult};

use termcolor::{ColorSpec, StandardStream, WriteColor};

use self::process_emv_tag::process_emv_tag;
use crate::{
	enum_repr_fallible,
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{print_bytes_pretty, print_bytes_small, print_indentation},
	DisplayBreakdown,
};

/// A processed block of EMV data with annotations and parsing results.
pub struct ProcessedEmvBlock {
	pub nodes: Vec<ProcessedEmvNode>,
}
impl From<Vec<ProcessedEmvNode>> for ProcessedEmvBlock {
	fn from(nodes: Vec<ProcessedEmvNode>) -> Self {
		Self { nodes }
	}
}
impl From<ProcessedEmvBlock> for Vec<ProcessedEmvNode> {
	fn from(block: ProcessedEmvBlock) -> Self {
		block.nodes
	}
}
impl Default for ProcessedEmvBlock {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvBlock {
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

impl TryFrom<RawEmvBlock> for ProcessedEmvBlock {
	type Error = ParseError;

	fn try_from(raw_block: RawEmvBlock) -> Result<Self, Self::Error> {
		let mut nodes = Vec::with_capacity(raw_block.nodes.len());
		for raw_node in raw_block.nodes {
			nodes.push(raw_node.try_into()?);
		}

		Ok(Self { nodes })
	}
}

pub struct ProcessedEmvNode {
	pub tag: ProcessedEmvTag,
	pub child_block: ProcessedEmvBlock,
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvNode {
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

impl TryFrom<RawEmvNode> for ProcessedEmvNode {
	type Error = ParseError;

	fn try_from(raw_node: RawEmvNode) -> Result<Self, Self::Error> {
		Ok(Self {
			tag: raw_node.tag.try_into()?,
			child_block: raw_node.child_block.try_into()?,
		})
	}
}

/// A processed EMV tag with as much information as possible about it.
pub enum ProcessedEmvTag {
	Raw {
		raw_tag: RawEmvTag,
	},
	Annotated {
		name: &'static str,
		raw_tag: RawEmvTag,
	},
	Parsed {
		name: &'static str,
		parsed: Box<dyn DisplayBreakdown>,
		raw_tag: RawEmvTag,
	},
}

impl ProcessedEmvTag {
	pub fn parse_raw<P>(
		name: &'static str,
		raw_tag: RawEmvTag,
		parsing_fn: P,
	) -> Result<Self, ParseError>
	where
		P: Fn(&[u8]) -> Result<Box<dyn DisplayBreakdown>, ParseError>,
	{
		match &raw_tag.data {
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
		raw_tag: RawEmvTag,
		parsing_fn: P,
		is_unrecognised_error: E,
	) -> Result<Self, ParseError>
	where
		P: Fn(&[u8]) -> Result<Box<dyn DisplayBreakdown>, ParseError>,
		E: Fn(&ParseError) -> bool,
	{
		match &raw_tag.data {
			EmvData::Normal(data) => match parsing_fn(data.as_slice()) {
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

	pub fn annotate_raw(name: &'static str, raw_tag: RawEmvTag) -> Self {
		Self::Annotated { name, raw_tag }
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvTag {
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
					raw_tag.tag.as_slice(),
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
					raw_tag.tag.as_slice(),
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
					raw_tag.tag.as_slice(),
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

impl TryFrom<RawEmvTag> for ProcessedEmvTag {
	type Error = ParseError;

	fn try_from(value: RawEmvTag) -> Result<Self, Self::Error> {
		process_emv_tag(value)
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawEmvBlock {
	pub nodes: Vec<RawEmvNode>,
}
impl From<Vec<RawEmvNode>> for RawEmvBlock {
	fn from(nodes: Vec<RawEmvNode>) -> Self {
		Self { nodes }
	}
}
impl From<RawEmvBlock> for Vec<RawEmvNode> {
	fn from(block: RawEmvBlock) -> Self {
		block.nodes
	}
}
impl Default for RawEmvBlock {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawEmvNode {
	pub tag: RawEmvTag,
	pub child_block: RawEmvBlock,
}

/// A raw EMV tag-value pair, with no meaning associated with it.
///
/// This can be further parsed based on the tag value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawEmvTag {
	pub tag: Vec<u8>,
	pub class: TagClass,
	pub data_object_type: DataObjectType,
	pub data: EmvData,
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for RawEmvTag {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();
		match &self.data {
			EmvData::Normal(data) => {
				if data.is_empty() {
					return;
				}

				// Display the tag value
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Raw:");
				stdout.reset().ok();
				print_bytes_pretty(data.as_slice(), 16, indentation + 1);
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

enum_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TagClass: u8, ParseError, { |_| ParseError::NonCompliant } {
	Universal       = 0b00 => "Universal",
	Application     = 0b01 => "Application",
	ContextSpecific = 0b10 => "Context-Specific",
	Private         = 0b11 => "Private",
}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DataObjectType {
	Primitive,
	Constructed,
}

/// EMV data, encoding the ability for data to be masked and therefore
/// inaccessible.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EmvData {
	Normal(Vec<u8>),
	Masked,
}

impl EmvData {
	/// Returns the data length, or `None` if unknown.
	pub fn len(&self) -> Option<usize> {
		match self {
			EmvData::Normal(data) => Some(data.len()),
			EmvData::Masked => None,
		}
	}

	pub fn from_u8_check_for_masked(data: Vec<u8>, masking_characters: &[char]) -> Self {
		if is_masked_u8(data.as_slice(), masking_characters) {
			Self::Masked
		} else {
			Self::Normal(data)
		}
	}
}

pub const MASKING_CHAR_MINIMUM: usize = 2;

pub fn is_masked_u8(data: &[u8], masking_characters: &[char]) -> bool {
	if data.len() < MASKING_CHAR_MINIMUM {
		return false;
	}

	for masking_char in masking_characters {
		if data.iter().all(|byte| *byte as char == *masking_char) {
			return true;
		}
	}

	false
}

pub fn is_masked_str(data: &str, masking_characters: &[char]) -> bool {
	if data.len() < MASKING_CHAR_MINIMUM {
		return false;
	}

	for masking_char in masking_characters {
		if data.chars().all(|c| c == *masking_char) {
			return true;
		}
	}

	false
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TlvFormat {
	BerTlv,
	Ingenico,
}

#[cfg(not(tarpaulin_include))]
impl Display for TlvFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
		f.write_str(match self {
			TlvFormat::BerTlv => "BER-TLV",
			TlvFormat::Ingenico => "Ingenico",
		})
	}
}
