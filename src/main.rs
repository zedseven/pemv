//! A utility for parsing EMV-related data.

// Linting Rules
#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::pedantic,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::clone_on_ref_ptr,
	clippy::dbg_macro,
	clippy::decimal_literal_representation,
	clippy::exit,
	clippy::filetype_is_file,
	clippy::if_then_some_else_none,
	clippy::non_ascii_literal,
	clippy::self_named_module_files,
	clippy::str_to_string,
	clippy::undocumented_unsafe_blocks,
	clippy::wildcard_enum_match_arm
)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::similar_names,
	clippy::struct_excessive_bools,
	clippy::too_many_lines,
	clippy::unnecessary_wraps,
	dead_code,
	unused_macros
)]

// Modules
mod cli;
mod config;
mod emv;
mod error;
mod macros;
mod non_emv;
mod output_colours;
mod util;

// Uses
use termcolor::{StandardStream, WriteColor};

use crate::{
	cli::build_cli,
	config::{apply_cli_arguments, colour_choice::ColourChoice, Config},
	emv::{
		auto_tlv::parse as parse_auto_tlv,
		ber_tlv::parse as parse_ber_tlv,
		ccd::{CardVerificationResults, IssuerApplicationData},
		ingenico_tlv::parse as parse_ingenico_tlv,
		CardholderVerificationMethodList,
		CardholderVerificationMethodResults,
		ProcessedEmvBlock,
		TerminalVerificationResults,
		TransactionStatusInformation,
	},
	non_emv::ServiceCode,
	output_colours::header_colour_spec,
	util::{parse_hex_str, parse_str_to_u16},
};

// Constants
pub const BITS_PER_BYTE: u8 = 8;

// Traits
/// A simple trait for displaying a comprehensive breakdown of the value.
///
/// Separate from [`Display`] because it represents a more significant operation
/// than simply printing a small value, and because it can handle coloured
/// output.
///
/// [`Display`]: core::fmt::Display
#[cfg(not(tarpaulin_include))]
pub trait DisplayBreakdown {
	/// Displays a pretty breakdown of the value and every part's meaning.
	///
	/// The indentation should be applied to every line. It's used to allow the
	/// display of nested values.
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8);

	/// Same as [`Self::display_breakdown`], but it displays as if the value is
	/// a component of a larger display.
	///
	/// This is useful for the IAC values - the TVR is rendered as part of the
	/// value, but error bits aren't really errors in the IACs.
	///
	/// The default trait implementation has no difference.
	fn display_breakdown_component_value(&self, stdout: &mut StandardStream, indentation: u8) {
		self.display_breakdown(stdout, indentation);
	}
}

// Entry Point
#[cfg(not(tarpaulin_include))]
fn main() {
	let mut cli_definition = build_cli();
	let matches = cli_definition.clone().get_matches();

	let config_figment = apply_cli_arguments(Config::figment(), &matches);

	let colour_choice = config_figment
		.extract_inner::<ColourChoice>(Config::CLI_COLOUR)
		.unwrap()
		.change_based_on_tty()
		.into();
	let masking_characters = config_figment
		.extract_inner::<Vec<char>>(Config::MASKING_CHARACTERS)
		.unwrap();
	let mut stdout = StandardStream::stdout(colour_choice);

	let parse_error = {
		// EMV Tags
		if let Some(tvr_str) = matches.get_one::<String>("tvr") {
			TerminalVerificationResults::try_from(parse_hex_str(tvr_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(iad_str) = matches.get_one::<String>("ccd-iad") {
			IssuerApplicationData::try_from(parse_hex_str(iad_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvr_str) = matches.get_one::<String>("ccd-cvr") {
			CardVerificationResults::try_from(parse_hex_str(cvr_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(tsi_str) = matches.get_one::<String>("tsi") {
			TransactionStatusInformation::try_from(parse_hex_str(tsi_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvm_results_str) = matches.get_one::<String>("cvm-results") {
			CardholderVerificationMethodResults::try_from(parse_hex_str(cvm_results_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvm_list_str) = matches.get_one::<String>("cvm-list") {
			CardholderVerificationMethodList::try_from(parse_hex_str(cvm_list_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		}
		// EMV Utilities
		else if let Some(tlv_str) = matches.get_one::<String>("auto-tlv") {
			parse_auto_tlv(tlv_str, masking_characters.as_slice())
				.and_then(|(format, v)| {
					let result = ProcessedEmvBlock::try_from(v);
					if result.is_ok() {
						stdout.set_color(&header_colour_spec()).ok();
						print!("TLV Format: ");
						stdout.reset().ok();
						println!("{}", format);
						println!();
					}
					result
				})
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(ber_tlv_str) = matches.get_one::<String>("ber-tlv") {
			parse_ber_tlv(
				parse_hex_str(ber_tlv_str).as_slice(),
				masking_characters.as_slice(),
			)
			.and_then(ProcessedEmvBlock::try_from)
			.map(|v| v.display_breakdown(&mut stdout, 0))
			.err()
		} else if let Some(ingenico_tlv_str) = matches.value_of("ingenico-tlv") {
			parse_ingenico_tlv(ingenico_tlv_str, masking_characters.as_slice())
				.and_then(ProcessedEmvBlock::try_from)
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		}
		// Non-EMV
		else if let Some(service_code_str) = matches.get_one::<String>("service-code") {
			parse_str_to_u16(service_code_str)
				.and_then(ServiceCode::try_from)
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		}
		// Default behaviour when no options are provided
		else {
			cli_definition.print_help().expect("unable to print help");
			return;
		}
	};

	if let Some(error) = parse_error {
		eprintln!("{}", error);
	}
}
