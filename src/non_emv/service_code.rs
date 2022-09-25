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
#[derive(Clone, Debug)]
pub struct ServiceCode {
	number: u16,
	interchange: Interchange,
	technology: Technology,
	authorisation_processing: AuthorisationProcessing,
	allowed_services: AllowedServices,
	pin_requirements: PinRequirements,
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interchange: u8 {
	International = 1 | 2 => "International",
	National      = 5 | 6 => "National",
	Private       = 7     => "Private",
	Test          = 9     => "Test",
	Rfu           = _     => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Technology: u8 {
	IntegratedCircuitCard = 2 | 6 => "Integrated circuit card (ICC)",
	MagneticStripeOnly    = _     => "Magnetic stripe only (MSR)",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthorisationProcessing: u8 {
	Normal                          = 0 => "Normal",
	ByIssuer                        = 2 => "By issuer only (no offline authorisation)",
	ByIssuerUnlessExplicitAgreement = 4 => "By issuer only unless an explicit bilateral agreement \
											applies (no offline authorisation)",
	Rfu                             = _ => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowedServices: u8 {
	NoRestrictions       = 0 | 1 | 6 => "No restrictions",
	GoodsAndServicesOnly = 2 | 5 | 7 => "Goods and services only",
	AtmOnly              = 3         => "ATM only",
	CashOnly             = 4         => "Cash only",
	Rfu                  = _         => "RFU",
}
}

enum_no_repr_infallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PinRequirements: u8 {
	None                  = 0 | 3 | 5 => "None",
	PinRequired           = 6 | 7     => "PIN required",
	PromptIfPinpadPresent = _         => "Prompt for PIN if PIN pad is present",
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
