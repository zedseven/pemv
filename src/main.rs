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
	clippy::identity_op,
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
mod util;

// Uses
use atty::{is as is_atty, Stream};
use termcolor::{ColorChoice, StandardStream};

use crate::{
	cli::parse_cli_arguments,
	emv::unit_values::{
		CardVerificationResults,
		CardholderVerificationMethodResults,
		TerminalVerificationResults,
		TransactionStatusInformation,
	},
	error::ParseError,
	non_emv::ServiceCode,
	util::{parse_hex_str, parse_str_to_u16},
};

// Constants
pub const BITS_PER_BYTE: u8 = 8;

// Traits
pub trait DisplayBreakdown {
	/// Displays a pretty breakdown of the value and every part's meaning.
	fn display_breakdown(&self, stdout: &mut StandardStream);
}

// Entry Point
fn main() {
	let matches = parse_cli_arguments();

	let choice = {
		match matches.value_of("colour").unwrap_or("auto") {
			"always" => ColorChoice::Always,
			"ansi" => ColorChoice::AlwaysAnsi,
			"auto" => {
				if is_atty(Stream::Stdout) {
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
			.map(|v| v.display_breakdown(&mut stdout))
			.err()
	} else if let Some(cvr_str) = matches.value_of("cvr") {
		CardVerificationResults::try_from(parse_hex_str(cvr_str).as_slice())
			.map(|v| v.display_breakdown(&mut stdout))
			.err()
	} else if let Some(tsi_str) = matches.value_of("tsi") {
		TransactionStatusInformation::try_from(parse_hex_str(tsi_str).as_slice())
			.map(|v| v.display_breakdown(&mut stdout))
			.err()
	} else if let Some(cvm_str) = matches.value_of("cvm") {
		CardholderVerificationMethodResults::try_from(parse_hex_str(cvm_str).as_slice())
			.map(|v| v.display_breakdown(&mut stdout))
			.err()
	} else if let Some(service_code_str) = matches.value_of("service-code") {
		parse_str_to_u16(service_code_str)
			.map(ServiceCode::try_from)
			.and_then(|v| v)
			.map(|v| v.display_breakdown(&mut stdout))
			.err()
	} else {
		unreachable!();
	};

	if let Some(error) = parse_error {
		match error {
			ParseError::WrongByteCount { expected, found } => eprintln!(
				"The wrong number of bytes were provided for the value. Perhaps you provided the \
				 wrong value? Expected: {}, Found: {}",
				expected, found
			),
			ParseError::InvalidNumber => {
				eprintln!("The value provided is not a valid number, or is too large.");
			}
		}
	}
}
