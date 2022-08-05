//! The definition for Cardholder Verification Method Conditions.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// A Cardholder Verification Method Condition.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CvmCondition {
	Always,
	UnattendedCash,
	NotUnattendedNotManualNotCashback,
	TerminalSupported,
	Manual,
	Cashback,
	InApplicationCurrencyUnderX,
	InApplicationCurrencyOverX,
	InApplicationCurrencyUnderY,
	InApplicationCurrencyOverY,
}

impl Display for CvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			CvmCondition::Always => "Always",
			CvmCondition::UnattendedCash => "If unattended cash",
			CvmCondition::NotUnattendedNotManualNotCashback => {
				"If not unattended cash and not manual cash and not purchase with cashback"
			}
			CvmCondition::TerminalSupported => "If terminal supports the CVM",
			CvmCondition::Manual => "If manual cash",
			CvmCondition::Cashback => "If purchase with cashback",
			CvmCondition::InApplicationCurrencyUnderX => {
				"If transaction is in the application currency and is under X value"
			}
			CvmCondition::InApplicationCurrencyOverX => {
				"If transaction is in the application currency and is over X value"
			}
			CvmCondition::InApplicationCurrencyUnderY => {
				"If transaction is in the application currency and is under Y value"
			}
			CvmCondition::InApplicationCurrencyOverY => {
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
