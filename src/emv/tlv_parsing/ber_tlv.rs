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
		let (class, data_object_type) = parse_tag_metadata(tag_byte_0);

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
pub fn parse_tag_metadata(tag_byte_0: u8) -> (TagClass, DataObjectType) {
	let class = ((0b1100_0000 & tag_byte_0) >> 6).try_into().expect(
		"this operation is infallible because we already narrow the number of bits to the \
		 expected count",
	);
	let data_object_type = if 0b0010_0000 & tag_byte_0 > 0 {
		DataObjectType::Constructed
	} else {
		DataObjectType::Primitive
	};

	(class, data_object_type)
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

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		super::{DataObjectType, EmvData, RawEmvBlock, RawEmvNode, RawEmvTag, TagClass},
		parse,
		parse_tag_metadata,
	};
	use crate::error::ParseError;

	// Tests
	#[test]
	fn tag_metadata() {
		fn test_byte_0(byte_0: u8, expected: (TagClass, DataObjectType)) {
			let result = parse_tag_metadata(byte_0);
			assert_eq!(expected, result);
		}

		test_byte_0(
			0b0000_0000,
			(TagClass::Universal, DataObjectType::Primitive),
		);
		test_byte_0(
			0b0100_0000,
			(TagClass::Application, DataObjectType::Primitive),
		);
		test_byte_0(
			0b1000_0000,
			(TagClass::ContextSpecific, DataObjectType::Primitive),
		);
		test_byte_0(0b1100_0000, (TagClass::Private, DataObjectType::Primitive));
		test_byte_0(
			0b0010_0000,
			(TagClass::Universal, DataObjectType::Constructed),
		);
		test_byte_0(
			0b0110_0000,
			(TagClass::Application, DataObjectType::Constructed),
		);
		test_byte_0(
			0b1010_0000,
			(TagClass::ContextSpecific, DataObjectType::Constructed),
		);
		test_byte_0(
			0b1110_0000,
			(TagClass::Private, DataObjectType::Constructed),
		);
	}

	#[allow(clippy::needless_pass_by_value)]
	fn test_parse(bytes: &[u8], expected: Result<RawEmvBlock, ParseError>) {
		let result = parse(bytes, ['*'].as_slice());
		assert_eq!(expected, result);
	}

	#[test]
	fn parse_empty_data() {
		test_parse([].as_slice(), Ok(RawEmvBlock { nodes: vec![] }));
	}

	#[test]
	fn parse_single_byte_primitive_unmasked_tag() {
		test_parse(
			[0x5A, 0x08, 0x47, 0x61, 0x73, 0x00, 0x00, 0x00, 0x01, 0x19].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5A],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Normal(vec![0x47, 0x61, 0x73, 0x00, 0x00, 0x00, 0x01, 0x19]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_multi_byte_primitive_unmasked_tag() {
		test_parse(
			[0x5F, 0x34, 0x01, 0x01].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5F, 0x34],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Normal(vec![0x01]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_primitive_masked_tag() {
		test_parse(
			[0x5A, 0x08, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5A],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_multi_byte_primitive_masked_tag() {
		test_parse(
			[0x5F, 0x34, 0x02, 0x2A, 0x2A].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5F, 0x34],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_constructed_unmasked_tag() {
		test_parse(
			[
				0x6F, 0x09, 0x4F, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x03, 0x10, 0x10,
			]
			.as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x6F],
						class: TagClass::Application,
						data_object_type: DataObjectType::Constructed,
						data: EmvData::Normal(vec![
							0x4F, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x03, 0x10, 0x10,
						]),
					},
					child_block: RawEmvBlock {
						nodes: vec![RawEmvNode {
							tag: RawEmvTag {
								tag: vec![0x4F],
								class: TagClass::Application,
								data_object_type: DataObjectType::Primitive,
								data: EmvData::Normal(vec![
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
			[
				0x6F, 0x0B, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A,
			]
			.as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x6F],
						class: TagClass::Application,
						data_object_type: DataObjectType::Constructed,
						data: EmvData::Masked,
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_constructed_masked_child_tag() {
		test_parse(
			[
				0x6F, 0x0B, 0x5F, 0x20, 0x08, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A,
			]
			.as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x6F],
						class: TagClass::Application,
						data_object_type: DataObjectType::Constructed,
						data: EmvData::Normal(vec![
							0x5F, 0x20, 0x08, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A,
						]),
					},
					child_block: RawEmvBlock {
						nodes: vec![RawEmvNode {
							tag: RawEmvTag {
								tag: vec![0x5F, 0x20],
								class: TagClass::Application,
								data_object_type: DataObjectType::Primitive,
								data: EmvData::Masked,
							},
							child_block: RawEmvBlock::default(),
						}],
					},
				}],
			}),
		);
	}
	#[test]
	fn parse_multiple_tags() {
		test_parse(
			[
				0x4F, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10, 0x5F, 0x34, 0x08, 0x2A, 0x2A,
				0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x5F, 0x24, 0x03, 0x25, 0x12, 0x31, 0x6F, 0x06,
				0x5F, 0x24, 0x03, 0x2A, 0x2A, 0x2A,
			]
			.as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![
					RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x4F],
							class: TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data: EmvData::Normal(vec![0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10]),
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x5F, 0x34],
							class: TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data: EmvData::Masked,
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x5F, 0x24],
							class: TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data: EmvData::Normal(vec![0x25, 0x12, 0x31]),
						},
						child_block: RawEmvBlock::default(),
					},
					RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x6F],
							class: TagClass::Application,
							data_object_type: DataObjectType::Constructed,
							data: EmvData::Normal(vec![0x5F, 0x24, 0x03, 0x2A, 0x2A, 0x2A]),
						},
						child_block: RawEmvBlock {
							nodes: vec![RawEmvNode {
								tag: RawEmvTag {
									tag: vec![0x5F, 0x24],
									class: TagClass::Application,
									data_object_type: DataObjectType::Primitive,
									data: EmvData::Masked,
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
			[0x5F, 0x20, 0x00].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5F, 0x20],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Normal(vec![]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_single_byte_tag_data_is_not_masked() {
		test_parse(
			[0x5F, 0x34, 0x01, 0x2A].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x5F, 0x34],
						class: TagClass::Application,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Normal(vec![0x2A]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_3_byte_tag_name() {
		test_parse(
			[0x9F, 0xA0, 0x20, 0x03, 0x22, 0x12, 0x31].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x9F, 0xA0, 0x20],
						class: TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Normal(vec![0x22, 0x12, 0x31]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
	#[test]
	fn parse_very_long_tag_data() {
		let mut very_long_input_data = vec![0x91, 0b1000_0001, 0x80];
		// The data is '*' (0x2A) so that the result is interpreted as masked and the
		// long data doesn't need to be written twice
		very_long_input_data.extend_from_slice(vec![0x2A; 0x80].as_slice());

		test_parse(
			very_long_input_data.as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0x91],
						class: TagClass::ContextSpecific,
						data_object_type: DataObjectType::Primitive,
						data: EmvData::Masked,
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
		test_parse(
			[0x91, 0b1000_0101, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF].as_slice(),
			Err(ParseError::Unsupported),
		);
	}
	#[test]
	fn parse_invalid_tag_length_too_long() {
		test_parse([0x91, 0x02, 0x00].as_slice(), Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_tag_name_indicates_more_coming_but_ends() {
		test_parse([0x5F].as_slice(), Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_tag_length_indicates_more_coming_but_ends() {
		test_parse(
			[0x91, 0b1000_0010, 0xFF].as_slice(),
			Err(ParseError::NonCompliant),
		);
	}
	#[test]
	fn parse_invalid_no_tag_length() {
		test_parse([0x91].as_slice(), Err(ParseError::NonCompliant));
	}
	#[test]
	fn parse_invalid_no_tag_data() {
		test_parse([0x91, 0x10].as_slice(), Err(ParseError::NonCompliant));
	}
	/// Some manufacturer-custom EMV tags indicate they're constructed but don't
	/// actually store nested EMV data, which can cause problems if not properly
	/// handled. Looking at you, Verifone `E3` tag! >:(
	#[test]
	fn parse_handles_stupid_tag_names() {
		test_parse(
			[0xE3, 0x03, 0x22, 0x12, 0x31].as_slice(),
			Ok(RawEmvBlock {
				nodes: vec![RawEmvNode {
					tag: RawEmvTag {
						tag: vec![0xE3],
						class: TagClass::Private,
						data_object_type: DataObjectType::Constructed,
						data: EmvData::Normal(vec![0x22, 0x12, 0x31]),
					},
					child_block: RawEmvBlock::default(),
				}],
			}),
		);
	}
}
