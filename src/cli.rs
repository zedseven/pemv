// Uses
use clap::{Arg, ArgGroup, ArgMatches, Command};

/// Parse CLI input.
pub fn parse_cli_arguments() -> ArgMatches {
	Command::new("ParseEMV")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			Arg::new("tvr")
				.short('t')
				.long("tvr")
				.takes_value(true)
				.value_name("TVR")
				.help("Parse Terminal Verification Results"),
		)
		.arg(
			Arg::new("cvr")
				.short('c')
				.long("cvr")
				.takes_value(true)
				.value_name("CVR")
				.help("Parse Card Verification Results"),
		)
		.arg(
			Arg::new("tsi")
				.long("tsi")
				.takes_value(true)
				.value_name("TSI")
				.help("Parse Transaction Status Information"),
		)
		.arg(
			Arg::new("cvm")
				.short('m')
				.long("cvm")
				.takes_value(true)
				.value_name("CVM results")
				.help("Parse Cardholder Verification Method Results"),
		)
		.group(ArgGroup::new("status-values").args(&["tvr", "cvr", "tsi", "cvm"]))
		.get_matches()
}
