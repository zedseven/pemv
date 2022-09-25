//! The definition for Cardholder Verification Methods.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{enum_repr_fallible, error::ParseError};

enum_repr_fallible! {
/// A Cardholder Verification Method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvMethod: u8, ParseError, { |_| ParseError::NonCompliant } {
	FailCvmProcessing          = 0b00_0000 => "Fail CVM processing",
	PlaintextPin               = 0b00_0001 => "Plaintext PIN verification performed by ICC",
	EncipheredPinOnline        = 0b00_0010 => "Enciphered PIN verified online",
	PlaintextPinWithSignature  = 0b00_0011 => "Plaintext PIN verification performed by ICC and \
											   signature (paper)",
	EncipheredPin              = 0b00_0100 => "Enciphered PIN verification performed by ICC",
	EncipheredPinWithSignature = 0b00_0101 => "Enciphered PIN verification performed by ICC and \
											   signature (paper)",
	Signature                  = 0b01_1110 => "Signature (paper)",
	NoCvmRequired              = 0b01_1111 => "No CVM required",
	NoCvmPerformed             = 0b11_1111 => "No CVM performed",
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
