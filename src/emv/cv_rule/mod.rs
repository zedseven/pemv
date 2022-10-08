//! Everything for handling Cardholder Verification (CV) Rule values.
//!
//! Information for this can be found in EMV Book 3, under section `C3`.

// Modules
mod cv_method;
mod cvm_condition;

// Uses
use std::cmp::Ordering;

pub use self::{cv_method::*, cvm_condition::*};
use crate::{bitflag_value, error::ParseError};

// Struct Implementation
bitflag_value! {
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CardholderVerificationRule: 2 {
	0 {
		pub continue_if_unsuccessful: bool =  0b0100_0000 => "Apply succeeding CV Rule if this CVM is unsuccessful",
		pub method: OptionalCvMethod =        0b0011_1111 => (Normal, "Method: {}"),
	}
	1 {
		pub condition: OptionalCvmCondition = 0b1111_1111 => (Normal, "Condition: {}"),
	}
}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::{CardholderVerificationRule, CvMethod, CvmCondition};
	use crate::wrong_byte_count;

	// Tests
	wrong_byte_count!(super::CardholderVerificationRule, 2);

	#[test]
	fn parse_from_bytes_valid() {
		let expected = Ok(CardholderVerificationRule {
			continue_if_unsuccessful: true,
			method:                   Some(CvMethod::Signature).into(),
			condition:                Some(CvmCondition::TerminalSupported).into(),
		});
		let result = CardholderVerificationRule::try_from([0b0101_1110, 0x03].as_slice());

		assert_eq!(expected, result);
	}
}
