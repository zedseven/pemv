//! The definition for Cardholder Verification Methods.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{enum_repr_fallible, error::ParseError};

enum_repr_fallible! {
/// A Cardholder Verification Method.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

/// A somewhat dumb workaround to have custom impls on [`Option<CvMethod>`].
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OptionalCvMethod {
	pub internal: Option<CvMethod>,
}

#[cfg(not(tarpaulin_include))]
impl From<Option<CvMethod>> for OptionalCvMethod {
	fn from(value: Option<CvMethod>) -> Self {
		Self { internal: value }
	}
}

#[cfg(not(tarpaulin_include))]
impl From<OptionalCvMethod> for Option<CvMethod> {
	fn from(value: OptionalCvMethod) -> Self {
		value.internal
	}
}

#[cfg(not(tarpaulin_include))]
impl From<u8> for OptionalCvMethod {
	fn from(value: u8) -> Self {
		Self {
			internal: CvMethod::try_from(value).ok(),
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl From<OptionalCvMethod> for u8 {
	fn from(value: OptionalCvMethod) -> Self {
		value.internal.map_or(0, Into::into)
	}
}

#[cfg(not(tarpaulin_include))]
impl Debug for OptionalCvMethod {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{:?}", self.internal)
	}
}

#[cfg(not(tarpaulin_include))]
impl Display for OptionalCvMethod {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if let Some(method) = &self.internal {
			write!(f, "{}", method)
		} else {
			write!(f, "Unknown (likely issuer or payment system-specific)")
		}
	}
}
