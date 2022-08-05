//! Everything for handling Card Verification Results (CVR) values.
//!
//! Information for this can be found in EMV Book 3, under section `C7.3`.

// Uses
use std::cmp::Ordering;

use super::{EnabledBitRange, Severity, UnitValue};
use crate::{error::ParseError, util::byte_slice_to_u64};

// Struct Implementation
#[derive(Debug)]
pub struct CardVerificationResults {
	bytes: <Self as UnitValue>::Bytes,
	// Byte 1 Values
	pub gen_ac_2_application_cryptogram_type: GenAc2ApplicationCryptogramType,
	pub gen_ac_1_application_cryptogram_type: GenAc1ApplicationCryptogramType,
	pub cda_performed: bool,
	pub offline_dda_performed: bool,
	pub issuer_authentication_not_performed: bool,
	pub issuer_authentication_failed: bool,
	// Byte 2 Values
	pub pin_try_count: u8,
	pub offline_pin_verification_performed: bool,
	pub offline_pin_verification_failed: bool,
	pub pin_try_limit_exceeded: bool,
	pub last_online_transaction_not_completed: bool,
	// Byte 3 Values
	pub offline_transaction_count_limit_lower_exceeded: bool,
	pub offline_transaction_count_limit_upper_exceeded: bool,
	pub offline_cumulative_amount_limit_lower_exceeded: bool,
	pub offline_cumulative_amount_limit_upper_exceeded: bool,
	pub issuer_discretionary_bit_1: bool,
	pub issuer_discretionary_bit_2: bool,
	pub issuer_discretionary_bit_3: bool,
	pub issuer_discretionary_bit_4: bool,
	// Byte 4 Values
	pub successful_issuer_script_commands_with_secure_messaging: u8,
	pub issuer_script_processing_failed: bool,
	pub offline_data_authentication_failed_on_previous_transaction: bool,
	pub go_online_on_next_transaction: bool,
	pub unable_to_go_online: bool,
}

#[derive(Debug)]
pub enum GenAc1ApplicationCryptogramType {
	Aac,
	Tc,
	Arqc,
	Rfu,
}
#[derive(Debug)]
pub enum GenAc2ApplicationCryptogramType {
	Aac,
	Tc,
	SecondGenAcNotRequested,
	Rfu,
}

impl TryFrom<&[u8]> for CardVerificationResults {
	type Error = ParseError;

	#[rustfmt::skip]
	fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
		if raw_bytes.len() != Self::NUM_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type: Ordering::Equal,
				expected: Self::NUM_BYTES,
				found: raw_bytes.len(),
			});
		}
		let mut bytes = [0u8; Self::NUM_BYTES];
		for (index, byte) in raw_bytes.iter().enumerate() {
			bytes[index] = byte & Self::USED_BITS_MASK[index];
		}

		Ok(Self {
			bytes,
			gen_ac_2_application_cryptogram_type: {
				match (0b1100_0000 & bytes[0]) >> 6 {
					0b00 => GenAc2ApplicationCryptogramType::Aac,
					0b01 => GenAc2ApplicationCryptogramType::Tc,
					0b10 => GenAc2ApplicationCryptogramType::SecondGenAcNotRequested,
					_ => GenAc2ApplicationCryptogramType::Rfu,
				}
			},
			gen_ac_1_application_cryptogram_type: {
				match (0b0011_0000 & bytes[0]) >> 4 {
					0b00 => GenAc1ApplicationCryptogramType::Aac,
					0b01 => GenAc1ApplicationCryptogramType::Tc,
					0b10 => GenAc1ApplicationCryptogramType::Arqc,
					_ => GenAc1ApplicationCryptogramType::Rfu,
				}
			},
			cda_performed:                                              0b0000_1000 & bytes[0] > 0,
			offline_dda_performed:                                      0b0000_0100 & bytes[0] > 0,
			issuer_authentication_not_performed:                        0b0000_0010 & bytes[0] > 0,
			issuer_authentication_failed:                               0b0000_0001 & bytes[0] > 0,
			pin_try_count:                                            ((0b1111_0000 & bytes[1]) >> 4) as u8,
			offline_pin_verification_performed:                         0b0000_1000 & bytes[1] > 0,
			offline_pin_verification_failed:                            0b0000_0100 & bytes[1] > 0,
			pin_try_limit_exceeded:                                     0b0000_0010 & bytes[1] > 0,
			last_online_transaction_not_completed:                      0b0000_0001 & bytes[1] > 0,
			offline_transaction_count_limit_lower_exceeded:             0b1000_0000 & bytes[2] > 0,
			offline_transaction_count_limit_upper_exceeded:             0b0100_0000 & bytes[2] > 0,
			offline_cumulative_amount_limit_lower_exceeded:             0b0010_0000 & bytes[2] > 0,
			offline_cumulative_amount_limit_upper_exceeded:             0b0001_0000 & bytes[2] > 0,
			issuer_discretionary_bit_1:                                 0b0000_1000 & bytes[2] > 0,
			issuer_discretionary_bit_2:                                 0b0000_0100 & bytes[2] > 0,
			issuer_discretionary_bit_3:                                 0b0000_0010 & bytes[2] > 0,
			issuer_discretionary_bit_4:                                 0b0000_0001 & bytes[2] > 0,
			successful_issuer_script_commands_with_secure_messaging:  ((0b1111_0000 & bytes[3]) >> 4) as u8,
			issuer_script_processing_failed:                            0b0000_1000 & bytes[3] > 0,
			offline_data_authentication_failed_on_previous_transaction: 0b0000_0100 & bytes[3] > 0,
			go_online_on_next_transaction:                              0b0000_0010 & bytes[3] > 0,
			unable_to_go_online:                                        0b0000_0001 & bytes[3] > 0,
		})
	}
}

impl UnitValue for CardVerificationResults {
	const NUM_BYTES: usize = 5;
	const USED_BITS_MASK: &'static [u8] = &[
		0b1111_1111,
		0b1111_1111,
		0b1111_1111,
		0b1111_1111,
		0b0000_0000,
	];
	type Bytes = [u8; Self::NUM_BYTES as usize];

	fn get_binary_value(&self) -> Self::Bytes {
		self.bytes
	}

	fn get_numeric_value(&self) -> u64 {
		byte_slice_to_u64(&self.bytes)
	}

	fn get_display_information(&self) -> Vec<EnabledBitRange> {
		let mut enabled_bits = Vec::with_capacity(4);

		enabled_bits.push(EnabledBitRange {
			offset: 7 + 4 * 8,
			len: 2,
			explanation: format!(
				"Application cryptogram type returned in 2nd GENERATE AC: {}",
				match self.gen_ac_2_application_cryptogram_type {
					GenAc2ApplicationCryptogramType::Aac => "AAC",
					GenAc2ApplicationCryptogramType::Tc => "TC",
					GenAc2ApplicationCryptogramType::SecondGenAcNotRequested =>
						"Second GENERATE AC not requested",
					GenAc2ApplicationCryptogramType::Rfu => "RFU",
				}
			),
			severity: Severity::Normal,
		});
		enabled_bits.push(EnabledBitRange {
			offset: 5 + 4 * 8,
			len: 2,
			explanation: format!(
				"Application cryptogram type returned in 1st GENERATE AC: {}",
				match self.gen_ac_1_application_cryptogram_type {
					GenAc1ApplicationCryptogramType::Aac => "AAC",
					GenAc1ApplicationCryptogramType::Tc => "TC",
					GenAc1ApplicationCryptogramType::Arqc => "ARQC",
					GenAc1ApplicationCryptogramType::Rfu => "RFU",
				}
			),
			severity: Severity::Normal,
		});
		if self.cda_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 4 * 8,
				len: 1,
				explanation: "CDA performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.offline_dda_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 4 * 8,
				len: 1,
				explanation: "Offline DDA performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_authentication_not_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 4 * 8,
				len: 1,
				explanation: "Issuer authentication not performed".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.issuer_authentication_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 * 8,
				len: 1,
				explanation: "Issuer authentication failed".to_owned(),
				severity: Severity::Error,
			});
		}
		enabled_bits.push(EnabledBitRange {
			offset: 7 + 3 * 8,
			len: 4,
			explanation: format!("PIN try count: {}", self.pin_try_count),
			severity: Severity::Normal,
		});
		if self.offline_pin_verification_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 3 * 8,
				len: 1,
				explanation: "Offline PIN verification performed".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.offline_pin_verification_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 3 * 8,
				len: 1,
				explanation: "Offline PIN verification performed and PIN not successfully verified"
					.to_owned(),
				severity: Severity::Error,
			});
		}
		if self.pin_try_limit_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 3 * 8,
				len: 1,
				explanation: "PIN try limit exceeded".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.last_online_transaction_not_completed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 * 8,
				len: 1,
				explanation: "Last online transaction not completed".to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.offline_transaction_count_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Lower offline transaction count limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.offline_transaction_count_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Upper offline transaction count limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.offline_cumulative_amount_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "Lower cumulative offline amount limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.offline_cumulative_amount_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 2 * 8,
				len: 1,
				explanation: "Upper cumulative offline amount limit exceeded".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_discretionary_bit_1 {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 2 * 8,
				len: 1,
				explanation: "Issuer-discretionary bit 1".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_discretionary_bit_2 {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 2 * 8,
				len: 1,
				explanation: "Issuer-discretionary bit 2".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_discretionary_bit_3 {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 2 * 8,
				len: 1,
				explanation: "Issuer-discretionary bit 3".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.issuer_discretionary_bit_4 {
			enabled_bits.push(EnabledBitRange {
				offset: 2 * 8,
				len: 1,
				explanation: "Issuer-discretionary bit 4".to_owned(),
				severity: Severity::Normal,
			});
		}
		enabled_bits.push(EnabledBitRange {
			offset: 7 + 8,
			len: 4,
			explanation: format!(
				"Number of successfully processed issuer script commands containing secure \
				 messaging: {}",
				self.successful_issuer_script_commands_with_secure_messaging
			),
			severity: Severity::Normal,
		});
		if self.issuer_script_processing_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Issuer script processing failed".to_owned(),
				severity: Severity::Error,
			});
		}
		if self.offline_data_authentication_failed_on_previous_transaction {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 8,
				len: 1,
				explanation: "Offline data authentication failed on previous transaction"
					.to_owned(),
				severity: Severity::Warning,
			});
		}
		if self.go_online_on_next_transaction {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 8,
				len: 1,
				explanation: "Go online on next transaction".to_owned(),
				severity: Severity::Normal,
			});
		}
		if self.unable_to_go_online {
			enabled_bits.push(EnabledBitRange {
				offset: 8,
				len: 1,
				explanation: "Unable to go online".to_owned(),
				severity: Severity::Warning,
			});
		}

		enabled_bits
	}
}
