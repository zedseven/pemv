// Provides the CLI for the program.

// Uses
use clap::{Arg, Command};

// Constants
pub const APPLICATION_PROPER_NAME: &str = "ParseEMV";
pub const APPLICATION_BIN_NAME: &str = env!("CARGO_PKG_NAME");

/// Builds the command-line interface.
pub fn build_cli() -> Command<'static> {
	Command::new(APPLICATION_PROPER_NAME)
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg_required_else_help(true)
		.arg(
			Arg::new("colour")
				.alias("color")
				.long("colour")
				.takes_value(true)
				.default_value("auto")
				.possible_values(["auto", "always", "ansi", "never"])
				.value_name("WHEN")
				.help("When to use colour in console output"),
		)
		.next_help_heading("EMV TAGS")
		.arg(
			Arg::new("tvr")
				.short('t')
				.long("tvr")
				.takes_value(true)
				.value_name("TVR")
				.help("Parse Terminal Verification Results (tag 0x95)"),
		)
		.arg(
			Arg::new("cvr")
				.short('c')
				.long("cvr")
				.takes_value(true)
				.value_name("CVR")
				.help("Parse Card Verification Results (part of tag 0x9F10)"),
		)
		.arg(
			Arg::new("tsi")
				.long("tsi")
				.takes_value(true)
				.value_name("TSI")
				.help("Parse Transaction Status Information (tag 0x9B)"),
		)
		.arg(
			Arg::new("cvm-results")
				.alias("cvm-result")
				.short('r')
				.long("cvm-results")
				.takes_value(true)
				.value_name("CVM RESULTS")
				.help("Parse Cardholder Verification Method (CVM) Results (tag 0x9F34)"),
		)
		.arg(
			Arg::new("cvm-list")
				.long("cvm-list")
				.takes_value(true)
				.value_name("CVM LIST")
				.help("Parse a Cardholder Verification Method (CVM) List (tag 0x8E)"),
		)
		.next_help_heading("NON-EMV")
		.arg(
			Arg::new("service-code")
				.long("service-code")
				.takes_value(true)
				.value_name("SERVICE CODE")
				.help("Parse a card Service Code (MSR)"),
		)
}
