// Provides the CLI for the program.

// Uses
use clap::{builder::NonEmptyStringValueParser, value_parser, Arg, ArgAction, Command};

// Constants
pub const APPLICATION_PROPER_NAME: &str = "ParseEMV";
pub const APPLICATION_BIN_NAME: &str = env!("CARGO_PKG_NAME");
const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

/// Builds the command-line interface.
pub fn build_cli() -> Command {
	Command::new(APPLICATION_PROPER_NAME)
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.help_template(HELP_TEMPLATE)
		.arg_required_else_help(true)
		.help_expected(true)
		.disable_help_flag(true)
		.disable_version_flag(true)
		.arg(
			Arg::new("help")
				.short('h')
				.long("help")
				.short_alias('?')
				.global(true)
				.action(ArgAction::Help)
				.help("Print help information. Use `--help` for more detailed descriptions."),
		)
		.arg(
			Arg::new("version")
				.short('V')
				.long("version")
				.global(true)
				.action(ArgAction::Version)
				.help("Print version information."),
		)
		.arg(
			Arg::new("colour")
				.long("colour")
				.alias("color")
				.num_args(1)
				.default_value("auto")
				.default_missing_value("always")
				.action(ArgAction::Set)
				.value_name("WHEN")
				.value_parser(["auto", "always", "ansi", "never"])
				.help("When to use colour in console output.")
				.long_help(
					"When to use colour in console output.\nThe `ansi` value is the same as \
					 `always`, except it specifies that only ANSI colour codes should be used. \
					 This means on Windows terminals for example, Windows console text attributes \
					 will not be used.",
				),
		)
		.arg(
			Arg::new("masking-character")
				.short('m')
				.long("masking-character")
				.alias("masking-char")
				.visible_alias("masking")
				.num_args(1..)
				.default_values(["*"])
				.action(ArgAction::Append)
				.value_name("CHARACTER")
				.value_parser(value_parser!(char))
				.requires("tlv-parsers")
				.help(
					"The character that will be treated as masked data when parsing. This \
					 argument can be provided multiple times to specify multiple masking \
					 characters.",
				)
				.long_help(
					"The character that will be treated as masked data when parsing. This \
					 argument can be provided multiple times to specify multiple masking \
					 characters.\nA tag is considered to be masked if the entire contents match \
					 the masking character.",
				),
		)
		.arg(
			Arg::new("sort-parsed-tags")
				.long("sort-parsed-tags")
				.visible_alias("sort")
				.num_args(0..=1)
				.default_value("true")
				.default_missing_value("true")
				.action(ArgAction::Set)
				.value_name("TRUE/FALSE")
				.value_parser(value_parser!(bool))
				.conflicts_with("no-sort-parsed-tags")
				.requires("tlv-parsers")
				.help("Whether to sort parsed tags from TLV parsers like `--ber-tlv`."),
		)
		.arg(
			Arg::new("no-sort-parsed-tags")
				.long("no-sort-parsed-tags")
				.visible_alias("no-sort")
				.num_args(0..=1)
				.default_value("false")
				.default_missing_value("true")
				.action(ArgAction::Set)
				.value_name("TRUE/FALSE")
				.value_parser(value_parser!(bool))
				.requires("tlv-parsers")
				.help(
					"Whether not to sort parsed tags from TLV parsers like `--ber-tlv`. This is \
					 the inverse to `--sort-parsed-tags`.",
				),
		)
		.next_help_heading("EMV Utilities")
		.arg(
			Arg::new("identify")
				.group("operations")
				.long("identify")
				.alias("id")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("TAG")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Attempt to identify an EMV tag by name.")
				.long_help(
					"Attempt to identify an EMV tag by name.\nThe class and data object type are \
					 properties of tags that indicate what context they're designed to be used \
					 in, and what kind of data they store. Constructed data objects contain \
					 nested EMV TLV data.",
				),
		)
		.arg(
			Arg::new("auto-tlv")
				.group("operations")
				.group("tlv-parsers")
				.short('a')
				.long("auto-tlv")
				.visible_alias("auto")
				.visible_alias("parse-tlv")
				.visible_alias("parse")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("EMV DATA BLOCK")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse a block of TLV data, attempting to find the format automatically."),
		)
		.arg(
			Arg::new("ber-tlv")
				.group("operations")
				.group("tlv-parsers")
				.short('b')
				.long("ber-tlv")
				.alias("ber")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("EMV DATA BLOCK")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse a block of BER-TLV encoded data.")
				.long_help(
					"Parse a block of BER-TLV encoded data.\nThe 'BER' stands for \"Basic \
					 Encoding Rules\", and BER-TLV is the 'canonical' EMV TLV data format. That \
					 said, some PIN pad manufacturers have their own variations with slight \
					 differences.",
				),
		)
		.arg(
			Arg::new("ingenico-tlv")
				.group("operations")
				.group("tlv-parsers")
				.short('i')
				.long("ingenico-tlv")
				.alias("ingenico")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("EMV DATA BLOCK")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse a block of TLV data encoded in the proprietary Ingenico format.")
				.long_help(
					"Parse a block of TLV data encoded in the proprietary Ingenico format.\nNote \
					 that this tool ignores non-EMV tags in the input data.",
				),
		)
		.next_help_heading("Individual EMV Tags")
		.arg(
			Arg::new("tvr")
				.group("operations")
				.short('t')
				.long("tvr")
				.visible_alias("95")
				.visible_alias("iac")
				.visible_alias("tac")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("TVR")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse Terminal Verification Results (tag 0x95).")
				.long_help(
					"Parse Terminal Verification Results (tag 0x95).\nIndicates the results of \
					 almost everything in the transaction, and often elucidates the cause of a \
					 chip-related issue.\nIssuer Action Codes (IAC, tags 0x9F0D, 0x9F0E, 0x9F0F) \
					 and Terminal Action Codes (TAC, no EMV tags but often in terminal EMV \
					 configuration files) can also be parsed using this same option.",
				),
		)
		.arg(
			Arg::new("ccd-iad")
				.group("operations")
				.long("ccd-iad")
				.visible_alias("9F10")
				.alias("9f10")
				.visible_alias("iad")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("IAD")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse CCD-compliant Issuer Application Data (tag 0x9F10).")
				.long_help(
					"Parse CCD-compliant Issuer Application Data (tag 0x9F10).\nCommon Core \
					 Definitions (CCD) are an EMV standard that allows issuers to use the same \
					 standard format for multiple card applications. Not all card brands use \
					 this, however.\nThe issuer application data is sent to the card issuer, and \
					 isn't usually very helpful.",
				),
		)
		.arg(
			Arg::new("ccd-cvr")
				.group("operations")
				.long("ccd-cvr")
				.visible_alias("cvr")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("CVR")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse CCD-compliant Card Verification Results (part of tag 0x9F10).")
				.long_help(
					"Parse CCD-compliant Card Verification Results (part of tag 0x9F10).\nSee the \
					 description of `--ccd-iad` for more information about CCD compliance.\nThis \
					 value is a part of the issuer application data, and includes information \
					 from the card application intended for the card issuer.",
				),
		)
		.arg(
			Arg::new("tsi")
				.group("operations")
				.long("tsi")
				.visible_alias("9B")
				.alias("9b")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("TSI")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse Transaction Status Information (tag 0x9B).")
				.long_help(
					"Parse Transaction Status Information (tag 0x9B).\nIndicates the functions \
					 performed during the transaction. It doesn't indicate whether they were \
					 successful or not.",
				),
		)
		.arg(
			Arg::new("cvm-results")
				.group("operations")
				.short('r')
				.long("cvm-results")
				.visible_alias("9F34")
				.alias("9f34")
				.alias("cvm-result")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("CVM RESULTS")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse Cardholder Verification Method (CVM) Results (tag 0x9F34).")
				.long_help(
					"Parse Cardholder Verification Method (CVM) Results (tag 0x9F34).\nThis \
					 contains the results of cardholder verification processing.",
				),
		)
		.arg(
			Arg::new("cvm-list")
				.group("operations")
				.long("cvm-list")
				.visible_alias("8E")
				.alias("8e")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("CVM LIST")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse a Cardholder Verification Method (CVM) List (tag 0x8E).")
				.long_help(
					"Parse a Cardholder Verification Method (CVM) List (tag 0x8E).\nThis list on \
					 the card defines the list of cardholder verification methods to try, in \
					 order. Some methods may only be available for certain payment environments, \
					 and some methods may specify that the transaction should continue even if \
					 they're unsuccessful.",
				),
		)
		.next_help_heading("Non-EMV")
		.arg(
			Arg::new("service-code")
				.group("operations")
				.long("service-code")
				.visible_alias("5F30")
				.alias("5f30")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("SERVICE CODE")
				.value_parser(NonEmptyStringValueParser::new())
				.help("Parse a card Service Code (MSR, or EMV tag 0x5F30).")
				.long_help(
					"Parse a card Service Code (MSR, or EMV tag 0x5F30).\nThis specifies the \
					 restrictions on where and how the card can be used, in addition to what's \
					 required to authorise transactions with it.",
				),
		)
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::build_cli;

	// Tests
	#[test]
	fn cli_debug_assertions() {
		build_cli().debug_assert();
	}
}
