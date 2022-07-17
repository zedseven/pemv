// Uses
use clap::{Arg, ArgMatches, Command};

/// Parse CLI input.
pub fn parse_cli_arguments() -> ArgMatches {
	Command::new("ParseEMV")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
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
			Arg::new("cvm")
				.short('m')
				.long("cvm")
				.takes_value(true)
				.value_name("CVM results")
				.help("Parse Cardholder Verification Method Results (tag 0x9F34)"),
		)
		.get_matches()
}
