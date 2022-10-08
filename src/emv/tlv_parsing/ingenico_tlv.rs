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
/// Note: this function expects the field separator (FS) between tags to already
/// be replaced by another character, though it doesn't matter what that
/// character is.
pub fn parse(data: &str, masking_characters: &[char]) -> Result<RawEmvBlock, ParseError> {
	// The input data should only be ASCII - no Unicode data is expected.
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
		let (class, data_object_type) = parse_tag_metadata(tag_id_bytes[0]);

		// Tag Length
		let colon_index = match &data[index..].find(TAG_FIELD_SEPARATOR) {
			Some(i) => index + i,
			None => return Err(ParseError::NonCompliant),
		};
		let length_str = &data[index..colon_index];
		// Tag lengths greater than the maximum unsigned 32-bit integer value are
		// unsupported
		let length_bytes =
			parse_hex_str_strict(length_str).map_err(|_| ParseError::NonCompliant)?;
		if length_bytes.len() > BYTES_PER_32_BITS {
			return Err(ParseError::Unsupported);
		}
		let length = byte_slice_to_u32(length_bytes.as_slice()) as usize;
		index = colon_index + 1;
		if index >= data_len {
			return Err(ParseError::NonCompliant);
		}

		// Tag Data
		let data_format = data_chars[index];
		let tag_data = match data_format {
			DATA_FORMAT_ASCII => {
				index += 1;
				if index + length > data_len {
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
			tag:         RawEmvTag {
				tag: tag_id_bytes,
				class,
				data_object_type,
				data: tag_data,
			},
		});
	}

	Ok(nodes.into())
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		super::{DataObjectType, EmvData, RawEmvBlock, RawEmvNode, RawEmvTag, TagClass},
		parse,
	};
	use crate::error::ParseError;

	// Tests
	#[allow(clippy::needless_pass_by_value)]
	fn test_parse(data: &str, expected: Result<RawEmvBlock, ParseError>) {
		let result = parse(data, ['*'].as_slice());
		assert_eq!(expected, result);
	}

	#[test]
	fn parse_empty_data() {
		test_parse("", Ok(RawEmvBlock { nodes: vec![] }));
	}
	#[test]
	fn parse_single_byte_primitive_unmasked_hex_tag() {
		test_parse(
			"T5A:08:h4761730000000119",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5A],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![
							0x47, 0x61, 0x73, 0x00, 0x00, 0x00, 0x01, 0x19,
						]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_multi_byte_primitive_unmasked_hex_tag() {
		test_parse(
			"T5F34:01:h01",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0x01]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_primitive_masked_hex_tag() {
		test_parse(
			"T5A:08:h****************",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5A],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_multi_byte_primitive_masked_hex_tag() {
		test_parse(
			"T5F34:02:h****",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_primitive_unmasked_ascii_tag() {
		test_parse(
			"T8A:02:a05",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x8A],
						class:            TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0x30, 0x35]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_primitive_masked_ascii_tag() {
		test_parse(
			"T8A:02:a**",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x8A],
						class:            TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_constructed_unmasked_tag() {
		test_parse(
			"T6F:09:h4F07A0000000031010",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x6F],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Constructed,
						data:             EmvData::Normal(vec![
							0x4F, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x03, 0x10, 0x10,
						]),
					},
					child_block: RawEmvBlock {
						nodes: vec![RawEmvNode {
							tag:         RawEmvTag {
								tag:              vec![0x4F],
								class:            TagClass::Application,
								data_object_type: DataObjectType::Primitive,
								data:             EmvData::Normal(vec![
									0xA0, 0x00, 0x00, 0x00, 0x03, 0x10, 0x10,
								]),
							},
							child_block: RawEmvBlock::default(),
						}],
					},
				}],
			}),
		);
	}
	#[test]
	fn parse_constructed_masked_tag() {
		test_parse(
			"T6F:0B:h**********************",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x6F],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Constructed,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	/// This is an unsupported case. If dealing with masked data, the entire tag
	/// must be masked if it's desired to mask just some data in it.
	#[test]
	fn parse_constructed_masked_child_tag() {
		test_parse(
			"T6F:0B:h5F2008****************",
			Err(ParseError::NonCompliant),
		);
	}
	#[test]
	fn parse_multiple_tags() {
		test_parse(
			"T4F:07:hA0000000041010~T5F34:08:h****************~T5F24:03:h251231~T6F:07:\
			 h9F370434E62F92",
			Ok(RawEmvBlock {
				nodes: vec![
					RawEmvNode {
						tag:         RawEmvTag {
							tag:              vec![0x4F],
							class:            TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data:             EmvData::Normal(vec![
								0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10,
							]),
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag:         RawEmvTag {
							tag:              vec![0x5F, 0x34],
							class:            TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data:             EmvData::Masked,
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag:         RawEmvTag {
							tag:              vec![0x5F, 0x24],
							class:            TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data:             EmvData::Normal(vec![0x25, 0x12, 0x31]),
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag:         RawEmvTag {
							tag:              vec![0x6F],
							class:            TagClass::Application,
							data_object_type: DataObjectType::Constructed,
							data:             EmvData::Normal(vec![
								0x9F, 0x37, 0x04, 0x34, 0xE6, 0x2F, 0x92,
							]),
						},
						child_block: RawEmvBlock {
							nodes: vec![RawEmvNode {
								tag:         RawEmvTag {
									tag:              vec![0x9F, 0x37],
									class:            TagClass::ContextSpecific,
									data_object_type: DataObjectType::Primitive,
									data:             EmvData::Normal(vec![0x34, 0xE6, 0x2F, 0x92]),
								},
								child_block: RawEmvBlock::default(),
							}],
						},
					},
				],
			}),
		);
	}
	#[test]
	fn parse_empty_tag_data() {
		test_parse(
			"T5F20:00:h",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x20],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_tag_ascii_data_is_not_masked() {
		test_parse(
			"T5F34:01:a*",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0x2A]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	/// This is different from the BER-TLV and Ingenico ASCII cases, because the
	/// masking is more granular.
	///
	/// When this text data is masked, it takes two masking characters to mask
	/// one byte of tag data. This means there is no confusion between what is
	/// masked and what is genuine.
	#[test]
	fn parse_single_byte_tag_hex_data_is_masked() {
		test_parse(
			"T5F34:01:h**",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	/// This case is an impossibility, but still worth testing.
	#[test]
	fn parse_3_byte_tag_name() {
		test_parse(
			"T9FA020:03:h221231",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x9F, 0xA0, 0x20],
						class:            TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0x22, 0x12, 0x31]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_lowercase_where_possible() {
		test_parse(
			"T5f34:02:he6dd",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0xE6, 0xDD]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_mixed_case() {
		test_parse(
			"T5f34:02:hE6Dd",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x5F, 0x34],
						class:            TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Normal(vec![0xE6, 0xDD]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_extra_end_separator() {
		test_parse(
			"T91:08:h****************~",
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x91],
						class:            TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_very_long_tag_data() {
		let mut very_long_input_data = "T91:80:h".to_owned();
		// The data is '*' (0x2A) so that the result is interpreted as masked and the
		// long data doesn't need to be written twice
		very_long_input_data.push_str("**".repeat(0x80).as_str());

		test_parse(
			very_long_input_data.as_str(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag:         RawEmvTag {
						tag:              vec![0x91],
						class:            TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data:             EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_too_long_tag_data() {
		// This input data is actually invalid, but here we're just testing that the
		// number of bytes indicated is correctly identified as unsupported
		test_parse("T91:FFFFFFFFFF:h", Err(ParseError::Unsupported));
	}
	/// Only ASCII input data is supported for this function.
	#[test]
	fn parse_non_ascii() {
		test_parse(
			"T5F34:02:h0001\u{fffd}T91:08:****************\u{fffd}",
			Err(ParseError::NonCompliant),
		);
	}
	#[test]
	fn parse_invalid_tag_length_too_long_hex() {
		test_parse("T91:02:h00", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_tag_length_too_long_ascii() {
		test_parse("T91:02:a0", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_tag_name_ends_early() {
		test_parse("T5F", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_no_tag_name_ends_early() {
		test_parse("T", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_no_tag_name() {
		test_parse("T:", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_no_tag_length() {
		test_parse("T91:", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_no_tag_data() {
		test_parse("T91:02:", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_skips_tag_length() {
		test_parse("T91:h6E34", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_format_specifier() {
		test_parse("T91:02:u83", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_field_separator() {
		test_parse("T8A~02~a00", Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_ignores_other_tag_types() {
		test_parse("D1003:01:aD", Ok(RawEmvBlock { nodes: vec![] }));
	}
}
