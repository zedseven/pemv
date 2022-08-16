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
		.help_expected(true)
		.mut_arg("help", |arg| {
			arg.help("Print help information. Use `--help` for more detailed descriptions.")
		})
		.mut_arg("version", |arg| arg.help("Print version information."))
		.arg(
			Arg::new("colour")
				.alias("color")
				.long("colour")
				.takes_value(true)
				.default_value("auto")
				.possible_values(["auto", "always", "ansi", "never"])
				.value_name("WHEN")
				.help("When to use colour in console output.")
				.long_help(
					"When to use colour in console output.\nThe `ansi` value is the same as \
					 `always`, except it specifies that only ANSI colour codes should be used. \
					 This means on Windows terminals for example, Windows console text attributes \
					 will not be used.",
				),
		)
		.next_help_heading("EMV UTILITIES")
		.arg(
			Arg::new("ber-tlv")
				.short('b')
				.long("ber-tlv")
				.takes_value(true)
				.value_name("EMV DATA BLOCK")
				.help("Parse a block of BER-TLV encoded data.")
				.long_help(
					"Parse a block of BER-TLV encoded data.\nThe 'BER' stands for \"Basic \
					 Encoding Rules\", and BER-TLV is the 'canonical' EMV TLV data format. That \
					 said, some PIN pad manufacturers have their own variations with slight \
					 differences.",
				),
		)
		.arg(
			Arg::new("ber-tlv-simple")
				.short('B')
				.long("ber-tlv-simple")
				.takes_value(true)
				.value_name("EMV DATA BLOCK")
				.help(
					"Parse a block of BER-TLV encoded data that doesn't support constructed \
					 (nested) EMV tags.",
				)
				.long_help(
					"Parse a block of BER-TLV encoded data that doesn't support constructed \
					 (nested) EMV tags.\nSee the description for `--ber-tlv` for more information \
					 about the BER-TLV format.\nConstructed data objects are tag values that \
					 contain more nested TLV tags, and sometimes manufacturer-custom tags (like \
					 Verifone's `E3`) don't respect this. If the `--ber-tlv` option doesn't work, \
					 try this.",
				),
		)
		.next_help_heading("INDIVIDUAL EMV TAGS")
		.arg(
			Arg::new("tvr")
				.alias("iac")
				.alias("tac")
				.short('t')
				.long("tvr")
				.takes_value(true)
				.value_name("TVR")
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
				.alias("iad")
				.long("ccd-iad")
				.takes_value(true)
				.value_name("IAD")
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
				.alias("cvr")
				.long("ccd-cvr")
				.takes_value(true)
				.value_name("CVR")
				.help("Parse CCD-compliant Card Verification Results (part of tag 0x9F10).")
				.long_help(
					"Parse CCD-compliant Card Verification Results (part of tag 0x9F10).\nSee the \
					 description for `--ccd-iad` for more information about CCD compliance.\nThis \
					 value is a part of the issuer application data, and includes information \
					 from the card application intended for the card issuer.",
				),
		)
		.arg(
			Arg::new("tsi")
				.long("tsi")
				.takes_value(true)
				.value_name("TSI")
				.help("Parse Transaction Status Information (tag 0x9B).")
				.long_help(
					"Parse Transaction Status Information (tag 0x9B).\nIndicates the functions \
					 performed during the transaction. It doesn't indicate whether they were \
					 successful or not.",
				),
		)
		.arg(
			Arg::new("cvm-results")
				.alias("cvm-result")
				.short('r')
				.long("cvm-results")
				.takes_value(true)
				.value_name("CVM RESULTS")
				.help("Parse Cardholder Verification Method (CVM) Results (tag 0x9F34).")
				.long_help(
					"Parse Cardholder Verification Method (CVM) Results (tag 0x9F34).\nThis \
					 contains the results of cardholder verification processing.",
				),
		)
		.arg(
			Arg::new("cvm-list")
				.long("cvm-list")
				.takes_value(true)
				.value_name("CVM LIST")
				.help("Parse a Cardholder Verification Method (CVM) List (tag 0x8E).")
				.long_help(
					"Parse a Cardholder Verification Method (CVM) List (tag 0x8E).\nThis list on \
					 the card defines the list of cardholder verification methods to try, in \
					 order. Some methods may only be available for certain payment environments, \
					 and some methods may specify that the transaction should continue even if \
					 they're unsuccessful.",
				),
		)
		.next_help_heading("NON-EMV")
		.arg(
			Arg::new("service-code")
				.long("service-code")
				.takes_value(true)
				.value_name("SERVICE CODE")
				.help("Parse a card Service Code (MSR, or EMV tag 0x5F30).")
				.long_help(
					"Parse a card Service Code (MSR, or EMV tag 0x5F30).\nThis specifies the \
					 restrictions on where and how the card can be used, in addition to what's \
					 required to authorise transactions with it.",
				),
		)
}
