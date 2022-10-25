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
mod status_values;
mod util;

// Uses
use anyhow::{Context, Result};

use crate::{
	cli::parse_cli_arguments,
	status_values::{
		display_breakdown,
		CardVerificationResults,
		StatusValue,
		TerminalVerificationResults,
		TransactionStatusInformation,
	},
	util::{hex_str_to_u16, hex_str_to_u64},
};

// Entry Point
fn main() -> Result<()> {
	let matches = parse_cli_arguments();
	if let Some(tvr_value) = matches.value_of("tvr") {
		let status_value = TerminalVerificationResults::new(
			hex_str_to_u64(tvr_value).with_context(|| "unable to parse hex value")?,
		);
		display_breakdown(&status_value);
	}
	if let Some(cvr_value) = matches.value_of("cvr") {
		let status_value = CardVerificationResults::new(
			hex_str_to_u64(cvr_value).with_context(|| "unable to parse hex value")?,
		);
		display_breakdown(&status_value);
	}
	if let Some(tsi_value) = matches.value_of("tsi") {
		let status_value = TransactionStatusInformation::new(
			hex_str_to_u16(tsi_value).with_context(|| "unable to parse hex value")?,
		);
		display_breakdown(&status_value);
	}

	Ok(())
}
