//! The authorisation response code, typically from EMV tag `0x8A`.
//!
//! The possible values come from the ISO 8583:1987 specification.
//!
//! This could be incomplete - it's difficult to find a complete list of values
//! online.

// Uses
use std::{cmp::Ordering, str::from_utf8 as str_from_utf8};

use termcolor::StandardStream;

use crate::{enum_no_repr_fallible, error::ParseError, util::print_indentation, DisplayBreakdown};

// Enum Implementation
enum_no_repr_fallible! {
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AuthorisationResponseCode: &str, ParseError, { |_| ParseError::Unrecognised } {
	Approval                            = "00"        => "Approval",
	Call                                = "01"        => "Call",
	CallSpecial                         = "02"        => "Call - Special Conditions",
	TerminalIdError                     = "03"        => "Terminal ID Error",
	HoldCall                            = "04"        => "Hold Card - Call",
	Decline                             = "05"        => "Decline - Do Not Honour",
	Error                               = "06"        => "Error",
	HoldCallSpecial                     = "07"        => "Hold Card - Call - Special Conditions",
	HonourWithId                        = "08"        => "Honour With Identification",
	NoOriginalTransaction               = "09"        => "No Original Transaction",
	PartialApproval                     = "10"        => "Partial Approval",
	ApprovalVip                         = "11"        => "Approved (VIP)",
	InvalidTransaction                  = "12"        => "Invalid Transaction",
	InvalidAmount                       = "13"        => "Invalid Amount",
	InvalidCardNumber                   = "14"        => "Invalid Card Number",
	NoSuchIssuer                        = "15"        => "No Such Issuer",
	ApprovedUpdateTrack3                = "16"        => "Approved - Update Track 3",
	CustomerCancellation                = "17"        => "Customer Cancellation",
	CustomerDispute                     = "18"        => "Customer Dispute",
	RetryTransaction                    = "19"        => "Retry Transaction",
	InvalidResponse                     = "20"        => "Invalid Response",
	NoActionTaken                       = "21"        => "No Action Taken",
	SuspectedMalfunction                = "22"        => "Suspected Malfunction",
	InvalidMinimumAmount                = "23"        => "Invalid Minimum Amount",
	FileUpdateNotSupported              = "24"        => "File Update Not Supported",
	InvalidIccData                      = "25"        => "Invalid ICC Data",
	DuplicateFileUpdateRecord           = "26"        => "Duplicate File Update Record",
	FileUpdateFieldEditError            = "27"        => "File Update Field Edit Error",
	FileUpdateFileLockedOut             = "28"        => "File Update File Locked Out",
	FileUpdateNotSuccessful             = "29"        => "File Update Not Successful",
	FormatError                         = "30"        => "Format Error",
	BankNotSupportedBySwitch            = "31"        => "Bank Not Supported By Switch",
	CompletedPartially                  = "32"        => "Completed Partially",
	ExpiredCard                         = "33" | "54" => "Expired Card",
	SuspectedFraud                      = "34" | "59" => "Suspected Fraud",
	CardAcceptorContactAcquirer         = "35" | "60" => "Card Acceptor, Contact Acquirer",
	RestrictedCard                      = "36" | "62" => "Restricted Card",
	CardAcceptorCallAcquirerSecurity    = "37" | "66" => "Card Acceptor, Call Acquirer Security",
	AllowablePinRetriesExceeded         = "38" | "75" => "Allowable PIN Retries Exceeded",
	NoCreditAccount                     = "39"        => "No Credit Account",
	RequestedFunctionNotSupported       = "40"        => "Requested Function Not Supported",
	LostCard                            = "41"        => "Lost Card",
	NoUniversalAccount                  = "42"        => "No Universal Account",
	StolenCard                          = "43"        => "Stolen Card",
	NoInvestmentAccount                 = "44"        => "No Investment Account",
	InsufficientFunds                   = "51"        => "Insufficient Funds",
	NoChequingAccount                   = "52"        => "No Chequing Account",
	NoSavingsAccount                    = "53"        => "No Savings Account",
	IncorrectPin                        = "55"        => "Incorrect PIN",
	NoCardRecord                        = "56"        => "No Card Record",
	TransactionNotAllowedCardholder     = "57"        => "Transaction Not Allowed For Cardholder",
	TransactionNotAllowedTerminal       = "58"        => "Transaction Not Allowed For Terminal",
	DebitCashbackWithdrawalLimitDecline = "61"        => "Debit Cashback Withdrawal Limit Decline",
	SecurityViolation                   = "63"        => "Security Violation",
	OriginalAmountIncorrect             = "64"        => "Original Amount Incorrect",
	DeclineInsertCard                   = "65"        => "Decline - Insert Card (often due to too \
														  many contactless transactions)",
	HoldCallAtm                         = "67"        => "ATM Hard Card Capture",
	ResponseReceivedTooLate             = "68"        => "Response Received Too Late",
	IssuerTimeout                       = "91"        => "Issuer Timeout",
	IssuerRoutingProblem                = "92"        => "Issuer Routing Problem",
	TransactionNotCompletedLawViolation = "93"        => "Transaction Not Completed - Law Violation",
	DuplicateTransmission               = "94"        => "Duplicate Transmission",
	ReconciliationError                 = "95"        => "Reconciliation Error",
	SystemMalfunction                   = "96"        => "System Malfunction",
}
}

impl TryFrom<&[u8]> for AuthorisationResponseCode {
	type Error = ParseError;

	fn try_from(raw_bytes: &[u8]) -> Result<Self, ParseError> {
		const NUM_BYTES: usize = 2;

		if raw_bytes.len() != NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: NUM_BYTES,
				found: raw_bytes.len(),
			});
		}

		str_from_utf8(raw_bytes)
			.map_err(|_| ParseError::InvalidNumber)
			.and_then(Self::try_from)
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for AuthorisationResponseCode {
	fn display_breakdown(&self, _: &mut StandardStream, indentation: u8, _: bool) {
		print_indentation(indentation);
		println!("{}", self);
	}
}

// Unit Tests
#[cfg(test)]
mod tests {
	// Uses
	use super::AuthorisationResponseCode;
	use crate::{
		enum_byte_slice_result_matches_true_value_result,
		error::ParseError,
		wrong_byte_count,
	};

	// Tests
	wrong_byte_count!(super::AuthorisationResponseCode, 2);
	enum_byte_slice_result_matches_true_value_result!(
		super::AuthorisationResponseCode,
		2,
		"05",
		b"05".as_slice()
	);

	#[test]
	fn invalid_utf8() {
		let expected = Err(ParseError::InvalidNumber);
		let result = AuthorisationResponseCode::try_from([0x00, 0x9F].as_slice());

		assert_eq!(expected, result);
	}
}
