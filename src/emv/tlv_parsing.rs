//! The module for all Tag-Length-Value (TLV) parsing.
//!
//! Information for this can be found in EMV Book 3, under `Annex B`.

// Uses
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
	error::ParseError,
	util::{byte_slice_to_u32, BYTES_PER_32_BITS},
};

/// A raw EMV tag-value pair, with no meaning associated with it.
///
/// This can be further parsed based on the tag value.
#[derive(Debug, Eq, PartialEq)]
pub struct RawEmvTag<'a> {
	pub tag: &'a [u8],
	pub class: TagClass,
	pub data_object_type: DataObjectType<'a>,
	pub data: &'a [u8],
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

#[derive(Debug, Eq, PartialEq)]
pub enum DataObjectType<'a> {
	Primitive,
	Constructed { tags: Vec<RawEmvTag<'a>> },
}

pub fn parse_ber_tlv(bytes: &[u8]) -> Result<Vec<RawEmvTag>, ParseError> {
	let bytes_len = bytes.len();
	let mut tags = Vec::new();
	let mut index = 0;
	while index < bytes_len {
		// The first byte contains some metadata about the tag
		let tag_start_index = index;
		let tag_byte_0 = bytes[index];
		let class = ((0b1100_0000 & tag_byte_0) >> 6).try_into()?;
		let constructed_data_object = 0b0010_0000 & tag_byte_0 > 0;

		// The tag continues if the last 5 bits of the first byte are all 1
		let mut tag_continues = 0b0001_1111 & tag_byte_0 == 0b0001_1111;
		while tag_continues {
			index += 1;
			if index >= bytes_len {
				return Err(ParseError::NonCompliant);
			}
			// Subsequent bytes of the tag indicate if another byte follows if the first bit
			// is 1
			tag_continues = 0b1000_0000 & bytes[index] > 0;
		}
		let tag_end_index = index;
		index += 1;

		// The length is next
		let length_byte_0 = bytes[index];
		let length = if 0b1000_0000 & length_byte_0 > 0 {
			let subsequent_length_byte_count = (0b0111_1111 & length_byte_0) as usize;
			// Tag lengths greater than the maximum unsigned 32-bit integer value are
			// unsupported
			if subsequent_length_byte_count > BYTES_PER_32_BITS {
				return Err(ParseError::Unsupported);
			}
			let start_index = index;
			index += 1 + subsequent_length_byte_count;
			byte_slice_to_u32(
				&bytes[(start_index + 1)..=(start_index + subsequent_length_byte_count)],
			) as usize
		} else {
			index += 1;
			usize::from(length_byte_0)
		};
		if index + length >= bytes_len + 1 {
			return Err(ParseError::NonCompliant);
		}

		// Store a reference to the data
		let data = &bytes[index..(index + length)];

		// If it's a constructed object type, the data is more BER-TLV data that can be
		// further deconstructed
		let data_object_type = if constructed_data_object {
			DataObjectType::Constructed {
				tags: parse_ber_tlv(data)?,
			}
		} else {
			DataObjectType::Primitive
		};

		// Construct the tag
		let tag = RawEmvTag {
			tag: &bytes[tag_start_index..=tag_end_index],
			class,
			data_object_type,
			data,
		};

		// Push the resulting tag to the list
		tags.push(tag);

		// Increment the index
		index += length;
	}

	Ok(tags)
}
