//! A dedicated module for processing EMV tags and annotating them. This is in
//! its own spot because it's very long.

// Uses
use crate::{
	emv::{
		ccd::IssuerApplicationData,
		AdditionalTerminalCapabilities,
		AuthorisationResponseCode,
		CardholderVerificationMethodList,
		CardholderVerificationMethodResults,
		IssuerActionCodeDefault,
		IssuerActionCodeDenial,
		IssuerActionCodeOnline,
		ProcessedEmvTag,
		RawEmvTag,
		TerminalCapabilities,
		TerminalType,
		TerminalVerificationResults,
		TransactionStatusInformation,
		TransactionType,
	},
	error::ParseError,
	non_emv::ServiceCode,
	DisplayBreakdown,
};

/// Process a [`RawEmvTag`] into a [`ProcessedEmvTag`].
pub fn process_emv_tag(raw_tag: RawEmvTag) -> Result<ProcessedEmvTag, ParseError> {
	// Parseable tags
	Ok(match &raw_tag.tag {
		[0x5F, 0x30] => Some(ProcessedEmvTag::parse_raw(
			"Service Code",
			raw_tag,
			|data| {
				ServiceCode::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x8A] => Some(ProcessedEmvTag::parse_raw_unrecognised(
			"Authorisation Response Code",
			"Authorisation Response Code (Unrecognised - likely payment system-specific)",
			raw_tag,
			|data| {
				AuthorisationResponseCode::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::Unrecognised),
		)?),
		[0x8E] => Some(ProcessedEmvTag::parse_raw("CVM List", raw_tag, |data| {
			CardholderVerificationMethodList::try_from(data)
				.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
		})?),
		[0x95] => Some(ProcessedEmvTag::parse_raw(
			"Terminal Verification Results (TVR)",
			raw_tag,
			|data| {
				TerminalVerificationResults::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9B] => Some(ProcessedEmvTag::parse_raw(
			"Transaction Status Information (TSI)",
			raw_tag,
			|data| {
				TransactionStatusInformation::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9C] => Some(ProcessedEmvTag::parse_raw_unrecognised(
			"Transaction Type",
			"Transaction Type (Unrecognised - likely payment system-specific)",
			raw_tag,
			|data| {
				TransactionType::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::Unrecognised),
		)?),
		[0x9F, 0x0D] => Some(ProcessedEmvTag::parse_raw(
			"Issuer Action Code - Default",
			raw_tag,
			|data| {
				IssuerActionCodeDefault::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x0E] => Some(ProcessedEmvTag::parse_raw(
			"Issuer Action Code - Denial",
			raw_tag,
			|data| {
				IssuerActionCodeDenial::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x0F] => Some(ProcessedEmvTag::parse_raw(
			"Issuer Action Code - Online",
			raw_tag,
			|data| {
				IssuerActionCodeOnline::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x10] => Some(ProcessedEmvTag::parse_raw_unrecognised(
			"Issuer Application Data (CCD-Compliant)",
			"Issuer Application Data (Not CCD-Compliant)",
			raw_tag,
			|data| {
				IssuerApplicationData::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
			|error| matches!(error, ParseError::NonCcdCompliant),
		)?),
		[0x9F, 0x33] => Some(ProcessedEmvTag::parse_raw(
			"Terminal Capabilities",
			raw_tag,
			|data| {
				TerminalCapabilities::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x34] => Some(ProcessedEmvTag::parse_raw(
			"CVM Results",
			raw_tag,
			|data| {
				CardholderVerificationMethodResults::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x35] => Some(ProcessedEmvTag::parse_raw(
			"Terminal Type",
			raw_tag,
			|data| {
				TerminalType::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		[0x9F, 0x40] => Some(ProcessedEmvTag::parse_raw(
			"Additional Terminal Capabilities",
			raw_tag,
			|data| {
				AdditionalTerminalCapabilities::try_from(data)
					.map(|parsed| Box::new(parsed) as Box<dyn DisplayBreakdown>)
			},
		)?),
		_ => None,
	}
	// Recognisable tags
	.unwrap_or_else(|| {
		match &raw_tag.tag {
			[0x5F, 0x57] => Some("Account Type"),
			[0x9F, 0x01] => Some("Acquirer Identifier"),
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
			[0x88] => Some("Short File Identifier (SFI)"),
			[0x9F, 0x4B] => Some("Signed Dynamic Application Data"),
			[0x93] => Some("Signed Static Application Data"),
			[0x9F, 0x4A] => Some("Static Data Authentication Tag List"),
			[0x9F, 0x1A] => Some("Terminal Country Code"),
			[0x9F, 0x1B] => Some("Terminal Floor Limit"),
			[0x9F, 0x1C] => Some("Terminal Identification"),
			[0x9F, 0x1D] => Some("Terminal Risk Management Data"),
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
			[0x9F, 0x37] => Some("Unpredictable Number"),
			[0x9F, 0x23] => Some("Upper Consecutive Offline Limit"),
			_ => None,
		}
		.map_or_else(
			// Unrecognisable tags
			|| ProcessedEmvTag::Raw { raw_tag },
			|name| ProcessedEmvTag::Annotated { name, raw_tag },
		)
	}))
}
