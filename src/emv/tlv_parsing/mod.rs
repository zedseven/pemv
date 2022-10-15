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
use std::{
	cmp::Ordering,
	fmt::{Display, Formatter, Result as FormatResult},
};

use termcolor::{ColorSpec, StandardStream, WriteColor};

pub use self::process_emv_tag::identify_tag;
use self::process_emv_tag::process_emv_tag;
use crate::{
	enum_repr_fallible,
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{print_bytes_pretty, print_bytes_small, print_indentation},
	DisplayBreakdown,
};

/// A processed block of EMV data with annotations and parsing results.
#[derive(Debug, Eq, PartialEq)]
pub struct ProcessedEmvBlock {
	pub nodes: Vec<ProcessedEmvNode>,
}
#[cfg(not(tarpaulin_include))]
impl From<Vec<ProcessedEmvNode>> for ProcessedEmvBlock {
	fn from(nodes: Vec<ProcessedEmvNode>) -> Self {
		Self { nodes }
	}
}
#[cfg(not(tarpaulin_include))]
impl From<ProcessedEmvBlock> for Vec<ProcessedEmvNode> {
	fn from(block: ProcessedEmvBlock) -> Self {
		block.nodes
	}
}
#[cfg(not(tarpaulin_include))]
impl Default for ProcessedEmvBlock {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

impl ProcessedEmvBlock {
	pub fn sort_nodes(&mut self) {
		self.nodes.sort();
		for node in &mut self.nodes {
			node.child_block.sort_nodes();
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvBlock {
	fn display_breakdown(
		&self,
		stdout: &mut StandardStream,
		indentation: u8,
		show_severity_colours: bool,
	) {
		let mut first = true;
		for node in &self.nodes {
			if first {
				first = false;
			} else {
				println!();
			}
			node.display_breakdown(stdout, indentation, show_severity_colours);
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

#[derive(Debug, Eq, PartialEq)]
pub struct ProcessedEmvNode {
	pub tag:         ProcessedEmvTag,
	pub child_block: ProcessedEmvBlock,
}

#[cfg(not(tarpaulin_include))]
impl Ord for ProcessedEmvNode {
	fn cmp(&self, other: &Self) -> Ordering {
		self.tag.cmp(&other.tag)
	}
}

#[cfg(not(tarpaulin_include))]
impl PartialOrd for ProcessedEmvNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvNode {
	fn display_breakdown(
		&self,
		stdout: &mut StandardStream,
		indentation: u8,
		show_severity_colours: bool,
	) {
		// Display the tag
		self.tag
			.display_breakdown(stdout, indentation, show_severity_colours);

		// Display the child tags (if any)
		if !self.child_block.nodes.is_empty() {
			let header_colour_spec = header_colour_spec();

			print_indentation(indentation);
			stdout.set_color(&header_colour_spec).ok();
			println!("Constructed Data Object's Child Tags:");
			stdout.reset().ok();

			self.child_block
				.display_breakdown(stdout, indentation + 1, show_severity_colours);
		}
	}
}

impl TryFrom<RawEmvNode> for ProcessedEmvNode {
	type Error = ParseError;

	fn try_from(raw_node: RawEmvNode) -> Result<Self, Self::Error> {
		Ok(Self {
			tag:         raw_node.tag.try_into()?,
			child_block: raw_node.child_block.try_into()?,
		})
	}
}

/// A processed EMV tag with as much information as possible about it.
#[derive(Debug)]
pub enum ProcessedEmvTag {
	Raw {
		raw_tag: RawEmvTag,
	},
	Annotated {
		name:    &'static str,
		raw_tag: RawEmvTag,
	},
	Parsed {
		name:    &'static str,
		parsed:  Box<dyn DisplayBreakdown>,
		raw_tag: RawEmvTag,
	},
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for ProcessedEmvTag {
	fn eq(&self, other: &Self) -> bool {
		self.get_raw_tag().eq(other.get_raw_tag())
	}
}

impl Eq for ProcessedEmvTag {}

#[cfg(not(tarpaulin_include))]
impl Ord for ProcessedEmvTag {
	fn cmp(&self, other: &Self) -> Ordering {
		self.get_raw_tag().cmp(other.get_raw_tag())
	}
}

#[cfg(not(tarpaulin_include))]
impl PartialOrd for ProcessedEmvTag {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl ProcessedEmvTag {
	#[cfg(not(tarpaulin_include))]
	pub fn get_raw_tag(&self) -> &RawEmvTag {
		match self {
			ProcessedEmvTag::Raw { raw_tag }
			| ProcessedEmvTag::Annotated { raw_tag, .. }
			| ProcessedEmvTag::Parsed { raw_tag, .. } => raw_tag,
		}
	}

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

	#[cfg(not(tarpaulin_include))]
	pub fn annotate_raw(name: &'static str, raw_tag: RawEmvTag) -> Self {
		Self::Annotated { name, raw_tag }
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ProcessedEmvTag {
	fn display_breakdown(
		&self,
		stdout: &mut StandardStream,
		indentation: u8,
		show_severity_colours: bool,
	) {
		fn print_tag_name(
			stdout: &mut StandardStream,
			indentation: u8,
			header_colour_spec: &ColorSpec,
			tag: &[u8],
			length: Option<usize>,
			name_option: Option<&str>,
		) {
			let bold_colour_spec = bold_colour_spec();

			let name = name_option.unwrap_or("<Unknown>");

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
				println!(" - ?? bytes - {name}");
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
				raw_tag.display_breakdown(stdout, indentation, show_severity_colours);
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
				raw_tag.display_breakdown(stdout, indentation, show_severity_colours);
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
				raw_tag.display_breakdown(stdout, indentation, show_severity_colours);

				// Display the parsed value
				print_indentation(indentation);
				stdout.set_color(&header_colour_spec).ok();
				println!("Parsed:");
				stdout.reset().ok();
				parsed.display_breakdown(stdout, indentation + 1, show_severity_colours);
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
#[cfg(not(tarpaulin_include))]
impl From<Vec<RawEmvNode>> for RawEmvBlock {
	fn from(nodes: Vec<RawEmvNode>) -> Self {
		Self { nodes }
	}
}
#[cfg(not(tarpaulin_include))]
impl From<RawEmvBlock> for Vec<RawEmvNode> {
	fn from(block: RawEmvBlock) -> Self {
		block.nodes
	}
}
#[cfg(not(tarpaulin_include))]
impl Default for RawEmvBlock {
	fn default() -> Self {
		Self {
			nodes: Vec::with_capacity(0),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawEmvNode {
	pub tag:         RawEmvTag,
	pub child_block: RawEmvBlock,
}

/// A raw EMV tag-value pair, with no meaning associated with it.
///
/// This can be further parsed based on the tag value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawEmvTag {
	pub tag:              Vec<u8>,
	pub class:            TagClass,
	pub data_object_type: DataObjectType,
	pub data:             EmvData,
}

#[cfg(not(tarpaulin_include))]
impl Ord for RawEmvTag {
	fn cmp(&self, other: &Self) -> Ordering {
		self.tag.cmp(&other.tag)
	}
}

#[cfg(not(tarpaulin_include))]
impl PartialOrd for RawEmvTag {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for RawEmvTag {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8, _: bool) {
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

#[cfg(not(tarpaulin_include))]
impl Display for DataObjectType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
		f.write_str(match self {
			Self::Primitive => "Primitive",
			Self::Constructed => "Constructed",
		})
	}
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
	#[cfg(not(tarpaulin_include))]
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

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		ber_tlv::parse as parse_ber_tlv,
		is_masked_str,
		is_masked_u8,
		DataObjectType,
		EmvData,
		ProcessedEmvBlock,
		ProcessedEmvTag,
		RawEmvTag,
		TagClass,
	};
	use crate::{
		emv::{AuthorisationResponseCode, TransactionStatusInformation},
		error::ParseError,
		DisplayBreakdown,
	};

	// Tests
	#[test]
	fn sort_nodes() {
		// This test just tests the sorting, so we don't care about manually defining
		// all the different processed data structures
		let expected = ProcessedEmvBlock::try_from(
			parse_ber_tlv(
				[
					0x7E, 0x03, 0x2A, 0x2A, 0x2A, 0x95, 0x05, 0x00, 0x80, 0x00, 0x80, 0x00, 0x9F,
					0x09, 0x02, 0x00, 0x8C,
				]
				.as_slice(),
				['*'].as_slice(),
			)
			.expect("any errors should already be tested by the BER-TLV testing"),
		)
		.expect("the testing value should be able to be processed without error");
		let mut result = ProcessedEmvBlock::try_from(
			parse_ber_tlv(
				[
					0x95, 0x05, 0x00, 0x80, 0x00, 0x80, 0x00, 0x9F, 0x09, 0x02, 0x00, 0x8C, 0x7E,
					0x03, 0x2A, 0x2A, 0x2A,
				]
				.as_slice(),
				['*'].as_slice(),
			)
			.expect("any errors should already be tested by the BER-TLV testing"),
		)
		.expect("the testing value should be able to be processed without error");
		result.sort_nodes();

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_normal() {
		let expected = &EmvData::Normal(vec![0xFC, 0x00]);
		let intermediate_result = ProcessedEmvTag::parse_raw(
			"Transaction Status Information (TSI)",
			RawEmvTag {
				tag:              vec![0x9B],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Normal(vec![0xFC, 0x00]),
			},
			|data| {
				TransactionStatusInformation::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)
		.expect("the testing value should be able to be processed without error");
		let result = &intermediate_result.get_raw_tag().data;

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_masked() {
		let expected = &EmvData::Masked;
		let intermediate_result = ProcessedEmvTag::parse_raw(
			"Transaction Status Information (TSI)",
			RawEmvTag {
				tag:              vec![0x9B],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Masked,
			},
			|data| {
				TransactionStatusInformation::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)
		.expect("the testing value should be able to be processed without error");
		let result = &intermediate_result.get_raw_tag().data;

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_unrecognised_normal_recognised() {
		let expected = "Authorisation Response Code";
		let intermediate_result = ProcessedEmvTag::parse_raw_unrecognised(
			"Authorisation Response Code",
			"Authorisation Response Code (Unrecognised - likely payment system-specific)",
			RawEmvTag {
				tag:              vec![0x8A],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Normal(b"06".to_vec()),
			},
			|data| {
				AuthorisationResponseCode::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::Unrecognised),
		)
		.expect("the testing value should be able to be processed without error");
		let result = match intermediate_result {
			ProcessedEmvTag::Annotated { name, .. } | ProcessedEmvTag::Parsed { name, .. } => name,
			ProcessedEmvTag::Raw { .. } => panic!("the testing value couldn't be parsed"),
		};

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_unrecognised_normal_unrecognised() {
		let expected =
			"Authorisation Response Code (Unrecognised - likely payment system-specific)";
		let intermediate_result = ProcessedEmvTag::parse_raw_unrecognised(
			"Authorisation Response Code",
			"Authorisation Response Code (Unrecognised - likely payment system-specific)",
			RawEmvTag {
				tag:              vec![0x8A],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Normal(b"ZZ".to_vec()),
			},
			|data| {
				AuthorisationResponseCode::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::Unrecognised),
		)
		.expect("the testing value should be able to be processed without error");
		let result = match intermediate_result {
			ProcessedEmvTag::Annotated { name, .. } | ProcessedEmvTag::Parsed { name, .. } => name,
			ProcessedEmvTag::Raw { .. } => panic!("the testing value couldn't be parsed"),
		};

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_unrecognised_error() {
		let expected = Err(ParseError::NonCompliant);
		let result = ProcessedEmvTag::parse_raw_unrecognised(
			"",
			"",
			RawEmvTag {
				tag:              vec![0x8A],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Normal(b"05".to_vec()),
			},
			|_| Err(ParseError::NonCompliant),
			|error| matches!(error, ParseError::Unrecognised),
		);

		assert_eq!(expected, result);
	}
	#[test]
	fn processed_emv_tag_parse_raw_unrecognised_masked() {
		let expected = "Authorisation Response Code";
		let intermediate_result = ProcessedEmvTag::parse_raw_unrecognised(
			"Authorisation Response Code",
			"Authorisation Response Code (Unrecognised - likely payment system-specific)",
			RawEmvTag {
				tag:              vec![0x8A],
				class:            TagClass::ContextSpecific,
				data_object_type: DataObjectType::Primitive,
				data:             EmvData::Masked,
			},
			|data| {
				AuthorisationResponseCode::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::Unrecognised),
		)
		.expect("the testing value should be able to be processed without error");
		let result = match intermediate_result {
			ProcessedEmvTag::Annotated { name, .. } | ProcessedEmvTag::Parsed { name, .. } => name,
			ProcessedEmvTag::Raw { .. } => panic!("the testing value couldn't be parsed"),
		};

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_u8_masked() {
		let expected = true;
		let result = is_masked_u8([0x2A, 0x2A, 0x2A].as_slice(), ['*'].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_u8_unmasked() {
		let expected = false;
		let result = is_masked_u8([0x23, 0x12, 0x31].as_slice(), ['*'].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_u8_too_short_to_be_sure() {
		let expected = false;
		let result = is_masked_u8([0x2A].as_slice(), ['*'].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_str_masked() {
		let expected = true;
		let result = is_masked_str("******", ['*'].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_str_unmasked() {
		let expected = false;
		let result = is_masked_str("231231", ['*'].as_slice());

		assert_eq!(expected, result);
	}
	#[test]
	fn is_masked_str_too_short_to_be_sure() {
		let expected = false;
		let result = is_masked_str("*", ['*'].as_slice());

		assert_eq!(expected, result);
	}
}
