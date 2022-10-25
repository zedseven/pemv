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
mod emv;
mod error;
mod non_emv;
mod output_colours;
mod util;

// Uses
use atty::{is as is_tty, Stream};
use termcolor::{ColorChoice, StandardStream};

use crate::{
	cli::build_cli,
	emv::{
		bitflag_values::{
			CardVerificationResults,
			CardholderVerificationMethodResults,
			TerminalVerificationResults,
			TransactionStatusInformation,
		},
		CardholderVerificationMethodList,
		IssuerApplicationData,
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

	let choice = {
		match matches.value_of("colour").unwrap_or("auto") {
			"always" => ColorChoice::Always,
			"ansi" => ColorChoice::AlwaysAnsi,
			"auto" => {
				if is_tty(Stream::Stdout) {
					ColorChoice::Auto
				} else {
					ColorChoice::Never
				}
			}
			_ => ColorChoice::Never,
		}
	};
	let mut stdout = StandardStream::stdout(choice);

	let parse_error = if let Some(tvr_str) = matches.value_of("tvr") {
		TerminalVerificationResults::try_from(parse_hex_str(tvr_str).as_slice())
			.map(|v| v.display_breakdown(&mut stdout, 0))
			.err()
	} else if let Some(iad_str) = matches.value_of("iad") {
		IssuerApplicationData::try_from(parse_hex_str(iad_str).as_slice())
			.map(|v| v.display_breakdown(&mut stdout, 0))
			.err()
	} else if let Some(cvr_str) = matches.value_of("cvr") {
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
	} else if let Some(service_code_str) = matches.value_of("service-code") {
		parse_str_to_u16(service_code_str)
			.map(ServiceCode::try_from)
			.and_then(|v| v)
			.map(|v| v.display_breakdown(&mut stdout, 0))
			.err()
	} else {
		cli_definition.print_help().expect("unable to print help");
		return;
	};

	if let Some(error) = parse_error {
		eprintln!("{}", error);
	}
}
