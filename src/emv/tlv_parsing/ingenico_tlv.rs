//! The module for Ingenico-proprietary TLV parsing.

// Uses
use super::{
	ber_tlv::{get_child_block, parse_tag_metadata},
	is_masked_str,
	EmvData,
	RawEmvBlock,
	RawEmvNode,
	RawEmvTag,
};
use crate::{
	error::ParseError,
	util::{byte_slice_to_u32, parse_hex_str_strict, BYTES_PER_32_BITS},
};

// Constants
const TAG_FIELD_SEPARATOR: char = ':';
const DATA_FORMAT_ASCII: char = 'a';
const DATA_FORMAT_HEX: char = 'h';
const TAG_TYPE_EMV: char = 'T';

/// Parses a block of Ingenico-proprietary TLV encoded data.
///
/// Note: this function expects the field separator (FS) to already be replaced
/// by another character, though it doesn't matter what that character is.
pub fn parse(data: &str, masking_characters: &[char]) -> Result<RawEmvBlock, ParseError> {
	// The input data should only be ASCII - no Unicode is expected.
	if !data.is_ascii() {
		return Err(ParseError::NonCompliant);
	}

	let data_chars = data.chars().collect::<Vec<_>>();
	let data_len = data_chars.len();
	let mut nodes = Vec::new();
	let mut index = 0;
	while index < data_len {
		// Tag Type
		let tag_type = data_chars[index];
		index += 1;
		if index >= data_len {
			return Err(ParseError::NonCompliant);
		}

		// Tag ID
		let colon_index = match &data[index..].find(TAG_FIELD_SEPARATOR) {
			Some(i) => index + i,
			None => return Err(ParseError::NonCompliant),
		};
		let tag_id_str = &data[index..colon_index];
		let tag_id_bytes =
			parse_hex_str_strict(tag_id_str).map_err(|_| ParseError::NonCompliant)?;
		index = colon_index + 1;
		if index >= data_len || tag_id_bytes.is_empty() {
			return Err(ParseError::NonCompliant);
		}
		let (class, data_object_type) = parse_tag_metadata(tag_id_bytes[0])?;

		// Tag Length
		let colon_index = match &data[index..].find(TAG_FIELD_SEPARATOR) {
			Some(i) => index + i,
			None => return Err(ParseError::NonCompliant),
		};
		let length_str = &data[index..colon_index];
		// Tag lengths greater than the maximum unsigned 32-bit integer value are
		// unsupported
		if length_str.len() > BYTES_PER_32_BITS {
			return Err(ParseError::Unsupported);
		}
		let length = byte_slice_to_u32(
			parse_hex_str_strict(length_str)
				.map_err(|_| ParseError::NonCompliant)?
				.as_slice(),
		) as usize;
		index = colon_index + 1;
		if index >= data_len {
			return Err(ParseError::NonCompliant);
		}

		// Tag Data
		let data_format = data_chars[index];
		let tag_data = match data_format {
			DATA_FORMAT_ASCII => {
				index += 1;
				if index + length >= data_len {
					return Err(ParseError::NonCompliant);
				}
				let tag_data_str = &data[index..(index + length)];
				index += length;
				if is_masked_str(tag_data_str, masking_characters) {
					EmvData::Masked
				} else {
					EmvData::Normal(tag_data_str.as_bytes().to_vec())
				}
			}
			DATA_FORMAT_HEX => {
				// The length value we got above is the number of actual bytes of data, but the
				// ASCII hex representation is twice that. (2 characters to represent 1 byte in
				// hex)
				let char_length = length * 2;
				index += 1;
				if index + char_length > data_len {
					return Err(ParseError::NonCompliant);
				}
				let tag_data_str = &data[index..(index + char_length)];
				index += char_length;
				if is_masked_str(tag_data_str, masking_characters) {
					EmvData::Masked
				} else {
					EmvData::Normal(
						parse_hex_str_strict(tag_data_str).map_err(|_| ParseError::NonCompliant)?,
					)
				}
			}
			_ => return Err(ParseError::NonCompliant),
		};

		// +1 to the index to skip the field separator (that we don't care about)
		index += 1;

		// Push the resulting tag to the list only if it's an EMV tag.
		// The reason we do this check here instead of at the top is because we need to
		// advance the index regardless of whether we use the data or not.
		if tag_type != TAG_TYPE_EMV {
			continue;
		}
		nodes.push(RawEmvNode {
			child_block: get_child_block(data_object_type, &tag_data, masking_characters),
			tag: RawEmvTag {
				tag: tag_id_bytes,
				class,
				data_object_type,
				data: tag_data,
			},
		});
	}

	Ok(nodes.into())
}
