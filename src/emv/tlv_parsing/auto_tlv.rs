//! The module for automatic TLV parsing.
//!
//! This is used to parse TLV data where the exact format is unknown. It calls
//! into the other TLV format parsers.

// Uses
use super::{
	ber_tlv::parse as parse_ber_tlv,
	ingenico_tlv::parse as parse_ingenico_tlv,
	RawEmvBlock,
	TlvFormat,
};
use crate::{error::ParseError, parse_hex_str};

/// Parses a block of TLV data, attempting to find the format automatically.
pub fn parse(
	tlv_str: &str,
	masking_characters: &[char],
) -> Result<(TlvFormat, RawEmvBlock), ParseError> {
	// Don't accept empty input because it'll match to anything
	if tlv_str.is_empty() {
		return Err(ParseError::Unrecognised);
	}

	// Ingenico TLV
	if let Ok(raw_emv_block) = parse_ingenico_tlv(tlv_str, masking_characters) {
		Ok((TlvFormat::Ingenico, raw_emv_block))
	}
	// BER-TLV
	else if let Ok(raw_emv_block) =
		parse_ber_tlv(parse_hex_str(tlv_str).as_slice(), masking_characters)
	{
		Ok((TlvFormat::BerTlv, raw_emv_block))
	}
	// Error
	else {
		Err(ParseError::Unrecognised)
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{
		super::{DataObjectType, EmvData, RawEmvBlock, RawEmvNode, RawEmvTag, TagClass, TlvFormat},
		parse,
	};
	use crate::error::ParseError;

	// Tests
	#[allow(clippy::needless_pass_by_value)]
	fn test_parse(data: &str, expected: Result<(TlvFormat, RawEmvBlock), ParseError>) {
		let result = parse(data, ['*'].as_slice());
		assert_eq!(expected, result);
	}

	#[test]
	fn parse_empty_data() {
		test_parse("", Err(ParseError::Unrecognised));
	}
	#[test]
	fn parse_ber_tlv_valid() {
		test_parse(
			"5F2403251231",
			Ok((
				TlvFormat::BerTlv,
				RawEmvBlock {
					nodes: vec![RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x5F, 0x24],
							class: TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data: EmvData::Normal(vec![0x25, 0x12, 0x31]),
						},
						child_block: RawEmvBlock::default(),
					}],
				},
			)),
		);
	}
	#[test]
	fn parse_ber_tlv_invalid() {
		test_parse("5F24032531", Err(ParseError::Unrecognised));
	}
	#[test]
	fn parse_ingenico_valid() {
		test_parse(
			"T5F24:03:h251231",
			Ok((
				TlvFormat::Ingenico,
				RawEmvBlock {
					nodes: vec![RawEmvNode {
						tag: RawEmvTag {
							tag: vec![0x5F, 0x24],
							class: TagClass::Application,
							data_object_type: DataObjectType::Primitive,
							data: EmvData::Normal(vec![0x25, 0x12, 0x31]),
						},
						child_block: RawEmvBlock::default(),
					}],
				},
			)),
		);
	}
	#[test]
	fn parse_ingenico_invalid() {
		test_parse("T5F24:03:h2531", Err(ParseError::Unrecognised));
	}
}
