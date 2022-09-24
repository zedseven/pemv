//! The module for BER-TLV parsing.
//!
//! Information for this can be found in EMV Book 3, under `Annex B`.

// Uses
use super::{DataObjectType, EmvData, RawEmvBlock, RawEmvNode, RawEmvTag, TagClass};
use crate::{
	error::ParseError,
	util::{byte_slice_to_u32, BYTES_PER_32_BITS},
};

/// Parses a block of BER-TLV encoded data.
pub fn parse(bytes: &[u8], masking_characters: &[char]) -> Result<RawEmvBlock, ParseError> {
	let bytes_len = bytes.len();
	let mut nodes = Vec::new();
	let mut index = 0;
	while index < bytes_len {
		// The first byte contains some metadata about the tag
		let tag_start_index = index;
		let tag_byte_0 = bytes[index];
		let (class, data_object_type) = parse_tag_metadata(tag_byte_0)?;

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
		if index >= bytes_len {
			return Err(ParseError::NonCompliant);
		}

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

		// Push the resulting tag to the list
		let tag_data = EmvData::from_u8_check_for_masked(data.to_vec(), masking_characters);
		nodes.push(RawEmvNode {
			child_block: get_child_block(data_object_type, &tag_data, masking_characters),
			tag: RawEmvTag {
				tag: bytes[tag_start_index..=tag_end_index].to_vec(),
				class,
				data_object_type,
				data: tag_data,
			},
		});

		// Increment the index
		index += length;
	}

	Ok(nodes.into())
}

/// Parses the class and data object type of the tag from the tag ID's first
/// byte, according to the BER-TLV specification.
pub fn parse_tag_metadata(tag_byte_0: u8) -> Result<(TagClass, DataObjectType), ParseError> {
	let class = ((0b1100_0000 & tag_byte_0) >> 6).try_into()?;
	let data_object_type = if 0b0010_0000 & tag_byte_0 > 0 {
		DataObjectType::Constructed
	} else {
		DataObjectType::Primitive
	};

	Ok((class, data_object_type))
}

/// Descends into the tag data to try to parse it as a constructed data object,
/// if `data_object_type` is [`DataObjectType::Constructed`].
///
/// Otherwise, it returns [`RawEmvBlock::default`].
pub fn get_child_block(
	data_object_type: DataObjectType,
	tag_data: &EmvData,
	masking_characters: &[char],
) -> RawEmvBlock {
	match data_object_type {
		DataObjectType::Primitive => RawEmvBlock::default(),
		DataObjectType::Constructed => match tag_data {
			EmvData::Normal(data) => parse(data, masking_characters).unwrap_or_default(),
			EmvData::Masked => RawEmvBlock::default(),
		},
	}
}
