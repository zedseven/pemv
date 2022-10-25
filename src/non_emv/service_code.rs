//! Everything for handling MSR service codes.
//!
//! Information for this can be found in [ISO/IEC 7813](https://www.iso.org/standard/43317.html).

// Uses
use std::fmt::{Display, Formatter, Result as FmtResult};

use termcolor::{StandardStream, WriteColor};

use crate::{
	error::ParseError,
	output_colours::{bold_colour_spec, header_colour_spec},
	util::print_indentation,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interchange {
	International,
	National,
	Private,
	Test,
	Rfu,
}
impl From<u8> for Interchange {
	fn from(value: u8) -> Self {
		match value {
			1 | 2 => Self::International,
			5 | 6 => Self::National,
			7 => Self::Private,
			9 => Self::Test,
			_ => Self::Rfu,
		}
	}
}
impl Display for Interchange {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::International => "International",
			Self::National => "National",
			Self::Private => "Private",
			Self::Test => "Test",
			Self::Rfu => "RFU",
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Technology {
	MagneticStripeOnly,
	IntegratedCircuitCard,
}
impl From<u8> for Technology {
	fn from(value: u8) -> Self {
		match value {
			2 | 6 => Self::IntegratedCircuitCard,
			_ => Self::MagneticStripeOnly,
		}
	}
}
impl Display for Technology {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::MagneticStripeOnly => "Magnetic stripe only (MSR)",
			Self::IntegratedCircuitCard => "Integrated circuit card (ICC)",
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthorisationProcessing {
	Normal,
	ByIssuer,
	ByIssuerUnlessExplicitAgreement,
	Rfu,
}
impl From<u8> for AuthorisationProcessing {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Normal,
			2 => Self::ByIssuer,
			4 => Self::ByIssuerUnlessExplicitAgreement,
			_ => Self::Rfu,
		}
	}
}
impl Display for AuthorisationProcessing {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Normal => "Normal",
			Self::ByIssuer => "By issuer only (no offline authorisation)",
			Self::ByIssuerUnlessExplicitAgreement => {
				"By issuer only unless an explicit bilateral agreement applies (no offline \
				 authorisation)"
			}
			Self::Rfu => "RFU",
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowedServices {
	NoRestrictions,
	GoodsAndServicesOnly,
	AtmOnly,
	CashOnly,
	Rfu,
}
impl From<u8> for AllowedServices {
	fn from(value: u8) -> Self {
		match value {
			0 | 1 | 6 => Self::NoRestrictions,
			2 | 5 | 7 => Self::GoodsAndServicesOnly,
			3 => Self::AtmOnly,
			4 => Self::CashOnly,
			_ => Self::Rfu,
		}
	}
}
impl Display for AllowedServices {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::NoRestrictions => "No restrictions",
			Self::GoodsAndServicesOnly => "Goods and services only",
			Self::AtmOnly => "ATM only",
			Self::CashOnly => "Cash only",
			Self::Rfu => "RFU",
		})
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PinRequirements {
	None,
	PinRequired,
	PromptIfPinpadPresent,
}
impl From<u8> for PinRequirements {
	fn from(value: u8) -> Self {
		match value {
			0 | 3 | 5 => Self::PinRequired,
			6 | 7 => Self::PromptIfPinpadPresent,
			_ => Self::None,
		}
	}
}
impl Display for PinRequirements {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::None => "None",
			Self::PinRequired => "PIN required",
			Self::PromptIfPinpadPresent => "Prompt for PIN if PIN pad is present",
		})
	}
}

impl TryFrom<u16> for ServiceCode {
	type Error = ParseError;

	fn try_from(number: u16) -> Result<Self, Self::Error> {
		if number >= 1000 {
			return Err(ParseError::InvalidNumber);
		}

		let position_1 = (number % 1000) / 100;
		let position_2 = (number % 100) / 10;
		let position_3 = number % 10;

		Ok(Self {
			number,
			interchange: Interchange::from(position_1 as u8),
			technology: Technology::from(position_1 as u8),
			authorisation_processing: AuthorisationProcessing::from(position_2 as u8),
			allowed_services: AllowedServices::from(position_3 as u8),
			pin_requirements: PinRequirements::from(position_3 as u8),
		})
	}
}

impl DisplayBreakdown for ServiceCode {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8) {
		let header_colour_spec = header_colour_spec();
		let bold_colour_spec = bold_colour_spec();

		// Print the numeric representation
		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		print!("Value:");
		stdout.reset().ok();
		println!(" {:0>3}", self.number);

		// Print the breakdown
		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		println!("Breakdown:");
		stdout.reset().ok();
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
