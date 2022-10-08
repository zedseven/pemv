//! The definition for Cardholder Verification Method Conditions.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Uses
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{enum_repr_fallible, error::ParseError};

enum_repr_fallible! {
/// A Cardholder Verification Method Condition.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CvmCondition: u8, ParseError, { |_| ParseError::NonCompliant } {
	Always                            = 0x00 => "Always",
	UnattendedCash                    = 0x01 => "If unattended cash",
	NotUnattendedNotManualNotCashback = 0x02 => "If not unattended cash and not manual cash and \
												 not purchase with cashback",
	TerminalSupported                 = 0x03 => "If terminal supports the CVM",
	Manual                            = 0x04 => "If manual cash",
	Cashback                          = 0x05 => "If purchase with cashback",
	InApplicationCurrencyUnderX       = 0x06 => "If transaction is in the application currency and \
												 is under X value",
	InApplicationCurrencyOverX        = 0x07 => "If transaction is in the application currency and \
												 is over X value",
	InApplicationCurrencyUnderY       = 0x08 => "If transaction is in the application currency and \
												 is under Y value",
	InApplicationCurrencyOverY        = 0x09 => "If transaction is in the application currency and \
												 is over Y value",
}
}

impl CvmCondition {
	/// Simply determines if the CVM condition is one that references the X or Y
	/// value from the CVM list.
	pub fn references_x_or_y_value(self) -> bool {
		matches!(
			self,
			Self::InApplicationCurrencyUnderX
				| Self::InApplicationCurrencyOverX
				| Self::InApplicationCurrencyUnderY
				| Self::InApplicationCurrencyOverY
		)
	}
}

/// A somewhat dumb workaround to have a [`Display`] impl on
/// [`Option<CvmCondition>`].
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OptionalCvmCondition {
	pub internal: Option<CvmCondition>,
}

#[cfg(not(tarpaulin_include))]
impl From<Option<CvmCondition>> for OptionalCvmCondition {
	fn from(value: Option<CvmCondition>) -> Self {
		Self { internal: value }
	}
}

#[cfg(not(tarpaulin_include))]
impl From<OptionalCvmCondition> for Option<CvmCondition> {
	fn from(value: OptionalCvmCondition) -> Self {
		value.internal
	}
}

#[cfg(not(tarpaulin_include))]
impl From<u8> for OptionalCvmCondition {
	fn from(value: u8) -> Self {
		Self {
			internal: CvmCondition::try_from(value).ok(),
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl From<OptionalCvmCondition> for u8 {
	fn from(value: OptionalCvmCondition) -> Self {
		value.internal.map_or(0, Into::into)
	}
}

#[cfg(not(tarpaulin_include))]
impl Debug for OptionalCvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{:?}", self.internal)
	}
}

#[cfg(not(tarpaulin_include))]
impl Display for OptionalCvmCondition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if let Some(method) = &self.internal {
			write!(f, "{}", method)
		} else {
			write!(f, "Unknown (likely payment system-specific)")
		}
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::CvmCondition;

	// Tests
	#[test]
	fn references_x_or_y_value_true() {
		let expected = true;
		let result = CvmCondition::InApplicationCurrencyOverY.references_x_or_y_value();

		assert_eq!(expected, result);
	}

	#[test]
	fn references_x_or_y_value_false() {
		let expected = false;
		let result = CvmCondition::TerminalSupported.references_x_or_y_value();

		assert_eq!(expected, result);
	}
}
