//! The definition for Cardholder Verification Methods.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// A Cardholder Verification Method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvMethod {
	FailCvmProcessing,
	PlaintextPin,
	EncipheredPinOnline,
	PlaintextPinWithSignature,
	EncipheredPin,
	EncipheredPinWithSignature,
	Signature,
	NoCvmRequired,
	NoCvmPerformed,
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
