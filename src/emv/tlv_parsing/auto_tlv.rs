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
