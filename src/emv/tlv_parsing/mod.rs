//! The module for all Tag-Length-Value (TLV) parsing.
//!
//! Some information for this can be found in EMV Book 3, under `Annex B`, but
//! that information is focused on BER-TLV format in particular.

// Modules
pub mod ber_tlv;

// Uses
use std::fmt::{Display, Formatter, Result as FmtResult};

use termcolor::{ColorSpec, StandardStream, WriteColor};

use crate::{
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::{print_bytes_pretty, print_bytes_small, print_indentation},
	CardholderVerificationMethodList,
	CardholderVerificationMethodResults,
	DisplayBreakdown,
	IssuerApplicationData,
	TerminalVerificationResults,
	TransactionStatusInformation,
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
		value: RawEmvTag<'a>,
	},
}

impl<'a> DisplayBreakdown for ProcessedEmvTag<'a> {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		fn print_tag_name(
			stdout: &mut StandardStream,
			indentation: u8,
			header_colour_spec: &ColorSpec,
			tag: &[u8],
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
			println!(" - {}", name);
		}

		let header_colour_spec = header_colour_spec();

		match self {
			ProcessedEmvTag::Raw { value } => {
				// Display the tag name
				print_tag_name(stdout, indentation, &header_colour_spec, value.tag, None);

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
					Some(name),
				);

				// Display the raw value
				value.display_breakdown(stdout, indentation);
			}
			ProcessedEmvTag::Parsed {
				name,
				parsed,
				value,
			} => {
				// Display the tag name
				print_tag_name(
					stdout,
					indentation,
					&header_colour_spec,
					value.tag,
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

	fn try_from(raw_tag: RawEmvTag<'a>) -> Result<Self, Self::Error> {
		// Parseable tags
		Ok(match &raw_tag.tag {
			[0x8E] => Some(Self::Parsed {
				name: "CVM List",
				parsed: Box::new(CardholderVerificationMethodList::try_from(raw_tag.data)?),
				value: raw_tag,
			}),
			[0x95] => Some(Self::Parsed {
				name: "Terminal Verification Results (TVR)",
				parsed: Box::new(TerminalVerificationResults::try_from(raw_tag.data)?),
				value: raw_tag,
			}),
			[0x9B] => Some(Self::Parsed {
				name: "Transaction Status Information (TSI)",
				parsed: Box::new(TransactionStatusInformation::try_from(raw_tag.data)?),
				value: raw_tag,
			}),
			[0x9F, 0x10] => match IssuerApplicationData::try_from(raw_tag.data) {
				Ok(ccd_iad) => Some(Self::Parsed {
					name: "Issuer Application Data (CCD-Compliant)",
					parsed: Box::new(ccd_iad),
					value: raw_tag,
				}),
				Err(ParseError::NonCcdCompliant) => Some(Self::Annotated {
					name: "Issuer Application Data (Not CCD-Compliant)",
					value: raw_tag,
				}),
				Err(error) => return Err(error),
			},
			[0x9F, 0x34] => Some(Self::Parsed {
				name: "CVM Results",
				parsed: Box::new(CardholderVerificationMethodResults::try_from(raw_tag.data)?),
				value: raw_tag,
			}),
			_ => None,
		}
		// Recognisable tags
		.unwrap_or_else(|| {
			match &raw_tag.tag {
				[0x5F, 0x57] => Some("Account Type"),
				[0x9F, 0x01] => Some("Acquirer Identifier"),
				[0x9F, 0x40] => Some("Additional Terminal Capabilities"),
				[0x81] => Some("Amount, Authorised (Binary)"),
				[0x9F, 0x02] => Some("Amount, Authorised (Numeric)"),
				[0x9F, 0x04] => Some("Amount, Other (Binary)"),
				[0x9F, 0x03] => Some("Amount, Other (Numeric)"),
				[0x9F, 0x3A] => Some("Amount, Reference Currency (Binary)"),
				[0x9F, 0x26] => Some("Application Cryptogram"),
				[0x9F, 0x42] => Some("Application Currency Code"),
				[0x9F, 0x44] => Some("Application Currency Exponent"),
				[0x9F, 0x05] => Some("Application Discretionary Data"),
				[0x5F, 0x25] => Some("Application Effective Date"),
				[0x5F, 0x24] => Some("Application Expiration Date"),
				[0x94] => Some("Application File Locator (AFL)"),
				[0x4F] => Some("Application Dedicated File (ADF) Name"),
				[0x9F, 0x06] => Some("Application Identifier (AID)"),
				[0x82] => Some("Application Interchange Profile"),
				[0x50] => Some("Application Label"),
				[0x9F, 0x12] => Some("Application Preferred Name"),
				[0x5A] => Some("Application Primary Account Number (PAN)"),
				[0x5F, 0x34] => Some("Application Primary Account Number (PAN) Sequence Number"),
				[0x87] => Some("Application Priority Indicator"),
				[0x9F, 0x3B] => Some("Application Reference Currency"),
				[0x9F, 0x43] => Some("Application Reference Currency Exponent"),
				[0x61] => Some("Application Template"),
				[0x9F, 0x36] => Some("Application Transaction Counter (ATC)"),
				[0x9F, 0x07] => Some("Application Usage Control"),
				[0x9F, 0x08] => Some("Application Version Number (ICC)"),
				[0x9F, 0x09] => Some("Application Version Number (Terminal)"),
				[0x89] => Some("Authorisation Code"),
				[0x8A] => Some("Authorisation Response Code"),
				[0x5F, 0x54] => Some("Bank Identifier Code (BIC)"),
				[0x8C] => Some("Card Risk Management Data Object List 1 (CDOL1)"),
				[0x8D] => Some("Card Risk Management Data Object List 2 (CDOL2)"),
				[0x5F, 0x20] => Some("Cardholder Name"),
				[0x9F, 0x0B] => Some("Cardholder Name Extended"),
				[0x8F] => Some("Certification Authority Public Key Index (ICC)"),
				[0x9F, 0x22] => Some("Certification Authority Public Key Index (Terminal)"),
				[0x83] => Some("Command Template"),
				[0x9F, 0x27] => Some("Cryptogram Information Data (CID)"),
				[0x9F, 0x45] => Some("Data Authentication Code"),
				[0x84] => Some("Dedicated File (DF) Name"),
				[0x9D] => Some("Directory Definition File (DDF) Name"),
				[0x73] => Some("Directory Discretionary Template"),
				[0x9F, 0x49] => Some("Dynamic Data Authentication Data Object List (DDOL)"),
				[0xBF, 0x0C] => Some("File Control Information (FCI) Issuer Discretionary Data"),
				[0xA5] => Some("File Control Information (FCI) Proprietary Template"),
				[0x6F] => Some("File Control Information (FCI) Template"),
				[0x9F, 0x4C] => Some("ICC Dynamic Number"),
				[0x9F, 0x2D] => Some("ICC PIN Encipherment Public Key Certificate"),
				[0x9F, 0x2E] => Some("ICC PIN Encipherment Public Key Exponent"),
				[0x9F, 0x2F] => Some("ICC PIN Encipherment Public Key Remainder"),
				[0x9F, 0x46] => Some("ICC Public Key Certificate"),
				[0x9F, 0x47] => Some("ICC Public Key Exponent"),
				[0x9F, 0x48] => Some("ICC Public Key Remainder"),
				[0x9F, 0x1E] => Some("Interface Device (IFD/Terminal) Serial Number"),
				[0x5F, 0x53] => Some("International Bank Account Number (IBAN)"),
				[0x9F, 0x0D] => Some("Issuer Action Code - Default"),
				[0x9F, 0x0E] => Some("Issuer Action Code - Denial"),
				[0x9F, 0x0F] => Some("Issuer Action Code - Online"),
				[0x91] => Some("Issuer Authentication Data"),
				[0x9F, 0x11] => Some("Issuer Code Table Index"),
				[0x5F, 0x28] => Some("Issuer Country Code"),
				[0x5F, 0x55] => Some("Issuer Country Code (alpha2 format)"),
				[0x5F, 0x56] => Some("Issuer Country Code (alpha3 format)"),
				[0x42] => Some("Issuer Identification Number (IIN)"),
				[0x90] => Some("Issuer Public Key Certificate"),
				[0x9F, 0x32] => Some("Issuer Public Key Exponent"),
				[0x92] => Some("Issuer Public Key Remainder"),
				[0x86] => Some("Issuer Script Command"),
				[0x9F, 0x18] => Some("Issuer Script Identifier"),
				[0x71] => Some("Issuer Script Template 1"),
				[0x72] => Some("Issuer Script Template 2"),
				[0x5F, 0x50] => Some("Issuer URL"),
				[0x5F, 0x2D] => Some("Language Preference"),
				[0x9F, 0x13] => Some("Last Online Application Transaction Counter (ATC) Register"),
				[0x9F, 0x4D] => Some("Log Entry"),
				[0x9F, 0x4F] => Some("Log Format"),
				[0x9F, 0x14] => Some("Lower Consecutive Offline Limit"),
				[0x9F, 0x15] => Some("Merchant Category Code"),
				[0x9F, 0x16] => Some("Merchant Identifier"),
				[0x9F, 0x4E] => Some("Merchant Name and Location"),
				[0x9F, 0x17] => Some("PIN Try Counter"),
				[0x9F, 0x39] => Some("POS Entry Mode"),
				[0x9F, 0x38] => Some("Processing Options Data Object List (PDOL)"),
				[0x70] => Some("READ RECORD Response Message Template"),
				[0x80] => Some("Response Message Template Format 1"),
				[0x77] => Some("Response Message Template Format 2"),
				[0x5F, 0x30] => Some("Service Code"),
				[0x88] => Some("Short File Identifier (SFI)"),
				[0x9F, 0x4B] => Some("Signed Dynamic Application Data"),
				[0x93] => Some("Signed Static Application Data"),
				[0x9F, 0x4A] => Some("Static Data Authentication Tag List"),
				[0x9F, 0x33] => Some("Terminal Capabilities"),
				[0x9F, 0x1A] => Some("Terminal Country Code"),
				[0x9F, 0x1B] => Some("Terminal Floor Limit"),
				[0x9F, 0x1C] => Some("Terminal Identification"),
				[0x9F, 0x1D] => Some("Terminal Risk Management Data"),
				[0x9F, 0x35] => Some("Terminal Type"),
				[0x9F, 0x1F] => Some("Track 1 Discretionary Data"),
				[0x9F, 0x20] => Some("Track 2 Discretionary Data"),
				[0x57] => Some("Track 2 Equivalent Data"),
				[0x97] => Some("Transaction Certificate Data Object List (TDOL)"),
				[0x98] => Some("Transaction Certificate (TC) Hash Value"),
				[0x5F, 0x2A] => Some("Transaction Currency Code"),
				[0x5F, 0x36] => Some("Transaction Currency Exponent"),
				[0x9A] => Some("Transaction Date"),
				[0x99] => Some("Transaction PIN Data"),
				[0x9F, 0x3C] => Some("Transaction Reference Currency Code"),
				[0x9F, 0x3D] => Some("Transaction Reference Currency Exponent"),
				[0x9F, 0x41] => Some("Transaction Sequence Counter"),
				[0x9F, 0x21] => Some("Transaction Time"),
				[0x9C] => Some("Transaction Type"),
				[0x9F, 0x37] => Some("Unpredictable Number"),
				[0x9F, 0x23] => Some("Upper Consecutive Offline Limit"),
				_ => None,
			}
			.map_or_else(
				// Unrecognisable tags
				|| Self::Raw { value: raw_tag },
				|name| Self::Annotated {
					name,
					value: raw_tag,
				},
			)
		}))
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
