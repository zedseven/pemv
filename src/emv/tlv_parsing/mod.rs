//! The module for all Tag-Length-Value (TLV) parsing.
//!
//! Some information for this can be found in EMV Book 3, under `Annex B`, but
//! that information is focused on BER-TLV format in particular.

// Modules
pub mod ber_tlv;
mod process_emv_tag;

// Uses
use std::fmt::{Display, Formatter, Result as FmtResult};

use termcolor::{ColorSpec, StandardStream, WriteColor};

use self::process_emv_tag::process_emv_tag;
use crate::{
	error::ParseError,
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
		value: RawEmvTag<'a>,
	},
	Annotated {
		name: &'static str,
		value: RawEmvTag<'a>,
	},
	Parsed {
		name: &'static str,
		parsed: Box<dyn DisplayBreakdown>,
		raw_tag: RawEmvTag<'a>,
	},
}

impl<'a> DisplayBreakdown for ProcessedEmvTag<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		fn print_tag_name(
			stdout: &mut StandardStream,
			indentation: u8,
			header_colour_spec: &ColorSpec,
			tag: &[u8],
			length: usize,
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
			println!(
				" - {} byte{} - {}",
				length,
				if length == 1 { "" } else { "s" },
				name
			);
		}

		let header_colour_spec = header_colour_spec();

		match self {
			ProcessedEmvTag::Raw { value } => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					value.tag,
					value.data.len(),
					None,
				);

				// Display the raw value
				value.display_breakdown(stdout, indentation);
			}
			ProcessedEmvTag::Annotated { name, value } => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					value.tag,
					value.data.len(),
					Some(name),
				);

				// Display the raw value
				value.display_breakdown(stdout, indentation);
			}
			ProcessedEmvTag::Parsed {
				name,
				parsed,
				raw_tag: value,
			} => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					value.tag,
					value.data.len(),
					Some(name),
				);

				// Display the raw value
				value.display_breakdown(stdout, indentation);

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
	pub data: &'a [u8],
}

impl<'a> DisplayBreakdown for RawEmvTag<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		if self.data.is_empty() {
			return;
		}

		let header_colour_spec = header_colour_spec();

		// Display the tag value
		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("Raw:");
		stdout.reset().ok();
		print_bytes_pretty(self.data, 16, indentation + 1);
	}
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TagClass {
	Universal = 0b00,
	Application = 0b01,
	ContextSpecific = 0b10,
	Private = 0b11,
}
impl TryFrom<u8> for TagClass {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b00 => Ok(Self::Universal),
			0b01 => Ok(Self::Application),
			0b10 => Ok(Self::ContextSpecific),
			0b11 => Ok(Self::Private),
			_ => Err(ParseError::NonCompliant),
		}
	}
}
impl Display for TagClass {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Universal => "Universal",
			Self::Application => "Application",
			Self::ContextSpecific => "Context-Specific",
			Self::Private => "Private",
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataObjectType {
	Primitive,
	Constructed,
}
