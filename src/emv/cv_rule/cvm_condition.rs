//! The definition for Cardholder Verification Method Conditions.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::error::ParseError;

/// A Cardholder Verification Method Condition.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvmCondition {
	Always = 0x00,
	UnattendedCash = 0x01,
	NotUnattendedNotManualNotCashback = 0x02,
	TerminalSupported = 0x03,
	Manual = 0x04,
	Cashback = 0x05,
	InApplicationCurrencyUnderX = 0x06,
	InApplicationCurrencyOverX = 0x07,
	InApplicationCurrencyUnderY = 0x08,
	InApplicationCurrencyOverY = 0x09,
}

impl TryFrom<u8> for CvmCondition {
	type Error = ParseError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Self::Always),
			0x01 => Ok(Self::UnattendedCash),
			0x02 => Ok(Self::NotUnattendedNotManualNotCashback),
			0x03 => Ok(Self::TerminalSupported),
			0x04 => Ok(Self::Manual),
			0x05 => Ok(Self::Cashback),
			0x06 => Ok(Self::InApplicationCurrencyUnderX),
			0x07 => Ok(Self::InApplicationCurrencyOverX),
			0x08 => Ok(Self::InApplicationCurrencyUnderY),
			0x09 => Ok(Self::InApplicationCurrencyOverY),
			_ => Err(ParseError::NonCompliant),
		}
	}
}

impl Display for CvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Always => "Always",
			Self::UnattendedCash => "If unattended cash",
			Self::NotUnattendedNotManualNotCashback => {
				"If not unattended cash and not manual cash and not purchase with cashback"
			}
			Self::TerminalSupported => "If terminal supports the CVM",
			Self::Manual => "If manual cash",
			Self::Cashback => "If purchase with cashback",
			Self::InApplicationCurrencyUnderX => {
				"If transaction is in the application currency and is under X value"
			}
			Self::InApplicationCurrencyOverX => {
				"If transaction is in the application currency and is over X value"
			}
			Self::InApplicationCurrencyUnderY => {
				"If transaction is in the application currency and is under Y value"
			}
			Self::InApplicationCurrencyOverY => {
				"If transaction is in the application currency and is over Y value"
			}
		})
	}
}

/// A somewhat dumb workaround to have a [`Display`] impl on
/// [`Option<CvmCondition>`].
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OptionalCvmCondition {
	internal: Option<CvmCondition>,
}

impl From<Option<CvmCondition>> for OptionalCvmCondition {
	fn from(value: Option<CvmCondition>) -> Self {
		Self { internal: value }
	}
}

impl From<OptionalCvmCondition> for Option<CvmCondition> {
	fn from(value: OptionalCvmCondition) -> Self {
		value.internal
	}
}

impl Debug for OptionalCvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{:?}", self.internal)
	}
}

impl Display for OptionalCvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if let Some(method) = &self.internal {
			write!(f, "{}", method)
		} else {
			write!(f, "Unknown (likely payment system-specific)")
		}
	}
}
