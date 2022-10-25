//! The definition for Cardholder Verification Methods.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::error::ParseError;

/// A Cardholder Verification Method.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvMethod {
	FailCvmProcessing = 0b00_0000,
	PlaintextPin = 0b00_0001,
	EncipheredPinOnline = 0b00_0010,
	PlaintextPinWithSignature = 0b00_0011,
	EncipheredPin = 0b00_0100,
	EncipheredPinWithSignature = 0b00_0101,
	Signature = 0b01_1110,
	NoCvmRequired = 0b01_1111,
	NoCvmPerformed = 0b11_1111,
}

impl TryFrom<u8> for CvMethod {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b00_0000 => Ok(Self::FailCvmProcessing),
			0b00_0001 => Ok(Self::PlaintextPin),
			0b00_0010 => Ok(Self::EncipheredPinOnline),
			0b00_0011 => Ok(Self::PlaintextPinWithSignature),
			0b00_0100 => Ok(Self::EncipheredPin),
			0b00_0101 => Ok(Self::EncipheredPinWithSignature),
			0b01_1110 => Ok(Self::Signature),
			0b01_1111 => Ok(Self::NoCvmRequired),
			// This value isn't explicitly marked - on page 162 of EMV Book 3 it's simply
			// labelled as `This value is not available for use`
			// On page 121 of EMV Book 4, it mentions `'3F' if no CVM is performed`
			0b11_1111 => Ok(Self::NoCvmPerformed),
			_ => Err(ParseError::NonCompliant),
		}
	}
}

impl Display for CvMethod {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::FailCvmProcessing => "Fail CVM processing",
			Self::PlaintextPin => "Plaintext PIN verification performed by ICC",
			Self::EncipheredPinOnline => "Enciphered PIN verified online",
			Self::PlaintextPinWithSignature => {
				"Plaintext PIN verification performed by ICC and signature (paper)"
			}
			Self::EncipheredPin => "Enciphered PIN verification performed by ICC",
			Self::EncipheredPinWithSignature => {
				"Enciphered PIN verification performed by ICC and signature (paper)"
			}
			Self::Signature => "Signature (paper)",
			Self::NoCvmRequired => "No CVM required",
			Self::NoCvmPerformed => "No CVM performed",
		})
	}
}

/// A somewhat dumb workaround to have a [`Display`] impl on
/// [`Option<CvMethod>`].
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OptionalCvMethod {
	internal: Option<CvMethod>,
}

impl From<Option<CvMethod>> for OptionalCvMethod {
	fn from(value: Option<CvMethod>) -> Self {
		Self { internal: value }
	}
}

impl From<OptionalCvMethod> for Option<CvMethod> {
	fn from(value: OptionalCvMethod) -> Self {
		value.internal
	}
}

impl Debug for OptionalCvMethod {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{:?}", self.internal)
	}
}

impl Display for OptionalCvMethod {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if let Some(method) = &self.internal {
			write!(f, "{}", method)
		} else {
			write!(f, "Unknown (likely issuer or payment system-specific)")
		}
	}
}
