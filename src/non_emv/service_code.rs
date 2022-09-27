//! Everything for handling MSR service codes.
//!
//! Information for this can be found in [ISO/IEC 7813](https://www.iso.org/standard/43317.html).

// Uses
use std::cmp::Ordering;

use termcolor::{StandardStream, WriteColor};

use crate::{
	enum_no_repr_infallible,
	error::ParseError,
	output_colours::bold_colour_spec,
	parse_str_to_u16,
	util::{bytes_to_str, print_indentation},
	DisplayBreakdown,
};

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ServiceCode {
	number: u16,
	interchange: Interchange,
	technology: Technology,
	authorisation_processing: AuthorisationProcessing,
	allowed_services: AllowedServices,
	pin_requirements: PinRequirements,
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Interchange: u8 {
	International = 1 | 2 => "International",
	National      = 5 | 6 => "National",
	Private       = 7     => "Private",
	Test          = 9     => "Test",
	Rfu           = _     => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Technology: u8 {
	IntegratedCircuitCard = 2 | 6 => "Integrated circuit card (ICC)",
	MagneticStripeOnly    = _     => "Magnetic stripe only (MSR)",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AuthorisationProcessing: u8 {
	Normal                          = 0 => "Normal",
	ByIssuer                        = 2 => "By issuer only (no offline authorisation)",
	ByIssuerUnlessExplicitAgreement = 4 => "By issuer only unless an explicit bilateral agreement \
											applies (no offline authorisation)",
	Rfu                             = _ => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AllowedServices: u8 {
	NoRestrictions       = 0 | 1 | 6 => "No restrictions",
	GoodsAndServicesOnly = 2 | 5 | 7 => "Goods and services only",
	AtmOnly              = 3         => "ATM only",
	CashOnly             = 4         => "Cash only",
	Rfu                  = _         => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PinRequirements: u8 {
	PinRequired           = 0 | 3 | 5 => "PIN required",
	PromptIfPinpadPresent = 6 | 7     => "Prompt for PIN if PIN pad is present",
	None                  = _         => "None",
}
}

impl TryFrom<u16> for ServiceCode {
	type Error = ParseError;

	fn try_from(number: u16) -> Result<Self, Self::Error> {
		if number > 999 {
			return Err(ParseError::InvalidNumber);
		}

		let position_1 = ((number % 1000) / 100) as u8;
		let position_2 = ((number % 100) / 10) as u8;
		let position_3 = (number % 10) as u8;

		Ok(Self {
			number,
			interchange: Interchange::from(position_1),
			technology: Technology::from(position_1),
			authorisation_processing: AuthorisationProcessing::from(position_2),
			allowed_services: AllowedServices::from(position_3),
			pin_requirements: PinRequirements::from(position_3),
		})
	}
}

impl TryFrom<&[u8]> for ServiceCode {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		const NUM_BYTES: usize = 2;

		if raw_bytes.len() != NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: NUM_BYTES,
				found: raw_bytes.len(),
			});
		}

		parse_str_to_u16(bytes_to_str(raw_bytes).as_str()).and_then(Self::try_from)
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for ServiceCode {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let bold_colour_spec = bold_colour_spec();

		// Print the numeric representation
		print_indentation(indentation);
		stdout.set_color(&bold_colour_spec).ok();
		println!("{:0>3}", self.number);
		stdout.reset().ok();

		// Because the structure of the service code is much more rigidly-defined, the
		// output here is much more static.
		// No less incomprehensible though, unfortunately.
		// The reason this breakdown is aligned when the others aren't, is because each
		// entry is a kind of category title, and alignment is more important.

		// Allowed Services
		print_indentation(indentation);
		print!("\u{2502}\u{2502}\u{251c} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Allowed Services:");
		stdout.reset().ok();
		println!("         {}", self.allowed_services);
		// PIN Requirements
		print_indentation(indentation);
		print!("\u{2502}\u{2502}\u{2514} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("PIN Requirements:");
		stdout.reset().ok();
		println!("         {}", self.pin_requirements);
		// Authorisation Processing
		print_indentation(indentation);
		print!("\u{2502}\u{2514}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Authorisation Processing:");
		stdout.reset().ok();
		println!(" {}", self.authorisation_processing);
		// Interchange
		print_indentation(indentation);
		print!("\u{251c}\u{2500}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Interchange:");
		stdout.reset().ok();
		println!("              {}", self.interchange);
		// Technology
		print_indentation(indentation);
		print!("\u{2514}\u{2500}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Technology:");
		stdout.reset().ok();
		println!("               {}", self.technology);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use std::cmp::Ordering;

	use super::{
		AllowedServices,
		AuthorisationProcessing,
		Interchange,
		PinRequirements,
		ServiceCode,
		Technology,
	};
	use crate::error::ParseError;

	// Tests
	/// This value is the service code of my personal Interac debit card.
	#[test]
	fn parse_interac_debit() {
		let expected = Ok(ServiceCode {
			number: 220,
			interchange: Interchange::International,
			technology: Technology::IntegratedCircuitCard,
			authorisation_processing: AuthorisationProcessing::ByIssuer,
			allowed_services: AllowedServices::NoRestrictions,
			pin_requirements: PinRequirements::PinRequired,
		});
		let result = ServiceCode::try_from(220);

		assert_eq!(expected, result);
	}

	/// This value is the service code of my personal Visa credit card.
	#[test]
	fn parse_visa_credit() {
		let expected = Ok(ServiceCode {
			number: 201,
			interchange: Interchange::International,
			technology: Technology::IntegratedCircuitCard,
			authorisation_processing: AuthorisationProcessing::Normal,
			allowed_services: AllowedServices::NoRestrictions,
			pin_requirements: PinRequirements::None,
		});
		let result = ServiceCode::try_from(201);

		assert_eq!(expected, result);
	}

	/// This value is the service code of a typical prepaid card.
	#[test]
	fn parse_prepaid() {
		let expected = Ok(ServiceCode {
			number: 121,
			interchange: Interchange::International,
			technology: Technology::MagneticStripeOnly,
			authorisation_processing: AuthorisationProcessing::ByIssuer,
			allowed_services: AllowedServices::NoRestrictions,
			pin_requirements: PinRequirements::None,
		});
		let result = ServiceCode::try_from(121);

		assert_eq!(expected, result);
	}

	/// This test simply ensures all valid values for service codes are able to
	/// be parsed without panicking or returning an error.
	#[test]
	fn parse_all_valid() {
		for n in 0..=999 {
			let result = ServiceCode::try_from(n);

			assert!(result.is_ok());
		}
	}

	#[test]
	fn parse_invalid_number() {
		let expected = Err(ParseError::InvalidNumber);
		let result = ServiceCode::try_from(1000);

		assert_eq!(expected, result);
	}

	/// This test ensures we get the same output from parsing a slice of `u8` as
	/// we do from parsing a single `u16`.
	#[test]
	fn parse_from_bytes() {
		let expected = ServiceCode::try_from(220);
		let result = ServiceCode::try_from([0x02, 0x20].as_slice());

		assert_eq!(expected, result);
	}

	#[test]
	fn parse_from_bytes_invalid() {
		let expected = Err(ParseError::ByteCountIncorrect {
			r#type: Ordering::Equal,
			expected: 2,
			found: 3,
		});
		let result = ServiceCode::try_from([0x02, 0x20, 0x00].as_slice());

		assert_eq!(expected, result);
	}
}
