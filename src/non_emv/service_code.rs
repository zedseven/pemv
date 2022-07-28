//! Everything for handling MSR service codes.
//!
//! Information for this can be found in [ISO/IEC 7813](https://www.iso.org/standard/43317.html).

// Uses
use std::fmt::{Display, Formatter, Result as FmtResult};

use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::{error::ParseError, DisplayBreakdown};

// Struct Implementation
#[derive(Debug)]
pub struct ServiceCode {
	number: u16,
	interchange: Interchange,
	technology: Technology,
	authorisation_processing: AuthorisationProcessing,
	allowed_services: AllowedServices,
	pin_requirements: PinRequirements,
}

#[derive(Debug)]
pub enum Interchange {
	International,
	National,
	Private,
	Test,
	Rfu,
}
impl Display for Interchange {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Interchange::International => "International",
			Interchange::National => "National",
			Interchange::Private => "Private",
			Interchange::Test => "Test",
			Interchange::Rfu => "RFU",
		})
	}
}

#[derive(Debug)]
pub enum Technology {
	MagneticStripeOnly,
	IntegratedCircuitCard,
}
impl Display for Technology {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Technology::MagneticStripeOnly => "Magnetic stripe only (MSR)",
			Technology::IntegratedCircuitCard => "Integrated circuit card (ICC)",
		})
	}
}

#[derive(Debug)]
pub enum AuthorisationProcessing {
	Normal,
	ByIssuer,
	ByIssuerUnlessExplicitAgreement,
	Rfu,
}
impl Display for AuthorisationProcessing {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			AuthorisationProcessing::Normal => "Normal",
			AuthorisationProcessing::ByIssuer => "By issuer only (no offline authorisation)",
			AuthorisationProcessing::ByIssuerUnlessExplicitAgreement => {
				"By issuer only unless an explicit bilateral agreement applies (no offline \
				 authorisation)"
			}
			AuthorisationProcessing::Rfu => "RFU",
		})
	}
}

#[derive(Debug)]
pub enum AllowedServices {
	NoRestrictions,
	GoodsAndServicesOnly,
	AtmOnly,
	CashOnly,
	Rfu,
}
impl Display for AllowedServices {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			AllowedServices::NoRestrictions => "No restrictions",
			AllowedServices::GoodsAndServicesOnly => "Goods and services only",
			AllowedServices::AtmOnly => "ATM only",
			AllowedServices::CashOnly => "Cash only",
			AllowedServices::Rfu => "RFU",
		})
	}
}

#[derive(Debug)]
pub enum PinRequirements {
	None,
	PinRequired,
	PromptIfPinpadPresent,
}
impl Display for PinRequirements {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			PinRequirements::None => "None",
			PinRequirements::PinRequired => "PIN required",
			PinRequirements::PromptIfPinpadPresent => "Prompt for PIN if PIN pad is present",
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
			interchange: match position_1 {
				1 | 2 => Interchange::International,
				5 | 6 => Interchange::National,
				7 => Interchange::Private,
				9 => Interchange::Test,
				_ => Interchange::Rfu,
			},
			technology: match position_1 {
				2 | 6 => Technology::IntegratedCircuitCard,
				_ => Technology::MagneticStripeOnly,
			},
			authorisation_processing: match position_2 {
				0 => AuthorisationProcessing::Normal,
				2 => AuthorisationProcessing::ByIssuer,
				4 => AuthorisationProcessing::ByIssuerUnlessExplicitAgreement,
				_ => AuthorisationProcessing::Rfu,
			},
			allowed_services: match position_3 {
				0 | 1 | 6 => AllowedServices::NoRestrictions,
				2 | 5 | 7 => AllowedServices::GoodsAndServicesOnly,
				3 => AllowedServices::AtmOnly,
				4 => AllowedServices::CashOnly,
				_ => AllowedServices::Rfu,
			},
			pin_requirements: match position_3 {
				0 | 3 | 5 => PinRequirements::PinRequired,
				6 | 7 => PinRequirements::PromptIfPinpadPresent,
				_ => PinRequirements::None,
			},
		})
	}
}

impl DisplayBreakdown for ServiceCode {
	fn display_breakdown(&self, stdout: &mut StandardStream) {
		// Print the numeric representation
		stdout
			.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Cyan)))
			.ok();
		print!("Value:");
		stdout.reset().ok();
		println!(" {:0>3}", self.number);

		// Print the breakdown
		let mut bold_colour_spec = ColorSpec::new();
		bold_colour_spec.set_bold(true);
		stdout
			.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Cyan)))
			.ok();
		println!("Breakdown:");
		stdout.reset().ok();
		stdout.set_color(&bold_colour_spec).ok();
		println!("{:0>3}", self.number);
		stdout.reset().ok();

		// Because the structure of the service code is much more rigidly-defined, the
		// output here is much more static
		// No less incomprehensible though, unfortunately
		// The reason this breakdown is aligned when the others aren't, is because each
		// entry is a kind of category title, and alignment is more important

		// Allowed Services
		print!("\u{2502}\u{2502}\u{251c} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Allowed Services:");
		stdout.reset().ok();
		println!("         {}", self.allowed_services);
		// PIN Requirements
		print!("\u{2502}\u{2502}\u{2514} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("PIN Requirements:");
		stdout.reset().ok();
		println!("         {}", self.pin_requirements);
		// Authorisation Processing
		print!("\u{2502}\u{2514}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Authorisation Processing:");
		stdout.reset().ok();
		println!(" {}", self.authorisation_processing);
		// Interchange
		print!("\u{251c}\u{2500}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Interchange:");
		stdout.reset().ok();
		println!("              {}", self.interchange);
		// Technology
		print!("\u{2514}\u{2500}\u{2500} ");
		stdout.set_color(&bold_colour_spec).ok();
		print!("Technology:");
		stdout.reset().ok();
		println!("               {}", self.technology);
	}
}
