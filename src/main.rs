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

use std::env;

// Uses
use termcolor::StandardStream;

use crate::{
	cli::build_cli,
	config::{apply_cli_arguments, colour_choice::ColourChoice, Config},
	emv::{
		ber_tlv::parse as parse_ber_tlv,
		ccd::{CardVerificationResults, IssuerApplicationData},
		CardholderVerificationMethodList,
		CardholderVerificationMethodResults,
		ProcessedEmvBlock,
		TerminalVerificationResults,
		TransactionStatusInformation,
	},
	non_emv::ServiceCode,
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
pub trait DisplayBreakdown {
	/// Displays a pretty breakdown of the value and every part's meaning.
	///
	/// The indentation should be applied to every line. It's used to allow the
	/// display of nested values.
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8);
}

// Entry Point
fn main() {
	let mut cli_definition = build_cli();
	let matches = cli_definition.clone().get_matches();

	let config_figment = apply_cli_arguments(Config::figment(), &matches);

	let colour_choice = config_figment
		.extract_inner::<ColourChoice>(Config::CLI_COLOUR)
		.unwrap()
		.change_based_on_tty()
		.into();
	let mut stdout = StandardStream::stdout(colour_choice);

	let parse_error = {
		// EMV Tags
		if let Some(tvr_str) = matches.value_of("tvr") {
			TerminalVerificationResults::try_from(parse_hex_str(tvr_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(iad_str) = matches.value_of("ccd-iad") {
			IssuerApplicationData::try_from(parse_hex_str(iad_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvr_str) = matches.value_of("ccd-cvr") {
			CardVerificationResults::try_from(parse_hex_str(cvr_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(tsi_str) = matches.value_of("tsi") {
			TransactionStatusInformation::try_from(parse_hex_str(tsi_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvm_results_str) = matches.value_of("cvm-results") {
			CardholderVerificationMethodResults::try_from(parse_hex_str(cvm_results_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(cvm_list_str) = matches.value_of("cvm-list") {
			CardholderVerificationMethodList::try_from(parse_hex_str(cvm_list_str).as_slice())
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		}
		// EMV Utilities
		else if let Some(ber_tlv_str) = matches.value_of("ber-tlv") {
			parse_ber_tlv(parse_hex_str(ber_tlv_str).as_slice(), true)
				.and_then(ProcessedEmvBlock::try_from)
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else if let Some(ber_tlv_str) = matches.value_of("ber-tlv-simple") {
			parse_ber_tlv(parse_hex_str(ber_tlv_str).as_slice(), false)
				.and_then(ProcessedEmvBlock::try_from)
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		}
		// Non-EMV
		else if let Some(service_code_str) = matches.value_of("service-code") {
			parse_str_to_u16(service_code_str)
				.and_then(ServiceCode::try_from)
				.map(|v| v.display_breakdown(&mut stdout, 0))
				.err()
		} else {
			cli_definition.print_help().expect("unable to print help");
			return;
		}
	};

	if let Some(error) = parse_error {
		eprintln!("{}", error);
	}
}
