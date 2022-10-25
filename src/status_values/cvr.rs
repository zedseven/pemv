//! Everything for handling Card Verification Results (CVR) values.

// Uses
use super::{display_breakdown, EnabledBitRange, StatusValue};

// Struct Implementation
pub struct CardVerificationResults {
	bits: u64,
	// Byte 1 Values
	gen_ac_2_application_cryptogram_type: GenAc2ApplicationCryptogramType,
	gen_ac_1_application_cryptogram_type: GenAc1ApplicationCryptogramType,
	cda_performed: bool,
	offline_dda_performed: bool,
	issuer_authentication_not_performed: bool,
	issuer_authentication_failed: bool,
	// Byte 2 Values
	pin_try_count: u8,
	offline_pin_verification_performed: bool,
	offline_pin_verification_failed: bool,
	pin_try_limit_exceeded: bool,
	last_online_transaction_not_completed: bool,
	// Byte 3 Values
	offline_transaction_count_limit_lower_exceeded: bool,
	offline_transaction_count_limit_upper_exceeded: bool,
	offline_cumulative_amount_limit_lower_exceeded: bool,
	offline_cumulative_amount_limit_upper_exceeded: bool,
	issuer_discretionary_bit_1: bool,
	issuer_discretionary_bit_2: bool,
	issuer_discretionary_bit_3: bool,
	issuer_discretionary_bit_4: bool,
	// Byte 4 Values
	successful_issuer_script_commands_with_secure_messaging: u8,
	issuer_script_processing_failed: bool,
	offline_data_authentication_failed_on_previous_transaction: bool,
	go_online_on_next_transaction: bool,
	unable_to_go_online: bool,
}

pub enum GenAc1ApplicationCryptogramType {
	Aac,
	Tc,
	Arqc,
	Rfu,
}
pub enum GenAc2ApplicationCryptogramType {
	Aac,
	Tc,
	SecondGenAcNotRequested,
	Rfu,
}

impl StatusValue<u64> for CardVerificationResults {
	const NUM_BITS: u8 = 40;
	const USED_BITS_MASK: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_0000_0000;

	#[rustfmt::skip]
	fn parse_bits<B: Into<u64>>(bits: B) -> Self {
		let bits = bits.into() & Self::USED_BITS_MASK;
		Self {
			bits,
			gen_ac_2_application_cryptogram_type: {
				match ((0b1100_0000 << (4 * 8)) & bits) >> (4 * 8 + 6) {
					0b00 => GenAc2ApplicationCryptogramType::Aac,
					0b01 => GenAc2ApplicationCryptogramType::Tc,
					0b10 => GenAc2ApplicationCryptogramType::SecondGenAcNotRequested,
					_ => GenAc2ApplicationCryptogramType::Rfu,
				}
			},
			gen_ac_1_application_cryptogram_type: {
				match ((0b0011_0000 << (4 * 8)) & bits) >> (4 * 8 + 4) {
					0b00 => GenAc1ApplicationCryptogramType::Aac,
					0b01 => GenAc1ApplicationCryptogramType::Tc,
					0b10 => GenAc1ApplicationCryptogramType::Arqc,
					_ => GenAc1ApplicationCryptogramType::Rfu,
				}
			},
			cda_performed:                                              (0b0000_1000 << (4 * 8)) & bits > 0,
			offline_dda_performed:                                      (0b0000_0100 << (4 * 8)) & bits > 0,
			issuer_authentication_not_performed:                        (0b0000_0010 << (4 * 8)) & bits > 0,
			issuer_authentication_failed:                               (0b0000_0001 << (4 * 8)) & bits > 0,
			pin_try_count: (((0b1111_0000 << (3 * 8)) & bits) >> (3 * 8 + 4)) as u8,
			offline_pin_verification_performed:                         (0b0000_1000 << (3 * 8)) & bits > 0,
			offline_pin_verification_failed:                            (0b0000_0100 << (3 * 8)) & bits > 0,
			pin_try_limit_exceeded:                                     (0b0000_0010 << (3 * 8)) & bits > 0,
			last_online_transaction_not_completed:                      (0b0000_0001 << (3 * 8)) & bits > 0,
			offline_transaction_count_limit_lower_exceeded:             (0b1000_0000 << (2 * 8)) & bits > 0,
			offline_transaction_count_limit_upper_exceeded:             (0b0100_0000 << (2 * 8)) & bits > 0,
			offline_cumulative_amount_limit_lower_exceeded:             (0b0010_0000 << (2 * 8)) & bits > 0,
			offline_cumulative_amount_limit_upper_exceeded:             (0b0001_0000 << (2 * 8)) & bits > 0,
			issuer_discretionary_bit_1:                                 (0b0000_1000 << (2 * 8)) & bits > 0,
			issuer_discretionary_bit_2:                                 (0b0000_0100 << (2 * 8)) & bits > 0,
			issuer_discretionary_bit_3:                                 (0b0000_0010 << (2 * 8)) & bits > 0,
			issuer_discretionary_bit_4:                                 (0b0000_0001 << (2 * 8)) & bits > 0,
			successful_issuer_script_commands_with_secure_messaging:
				(((0b1111_0000 << 8) & bits) >> (8 + 4)) as u8,
			issuer_script_processing_failed:                            (0b0000_1000 << 8) & bits > 0,
			offline_data_authentication_failed_on_previous_transaction: (0b0000_0100 << 8) & bits > 0,
			go_online_on_next_transaction:                              (0b0000_0010 << 8) & bits > 0,
			unable_to_go_online:                                        (0b0000_0001 << 8) & bits > 0,
		}
	}

	fn display_breakdown(&self) {
		// This is an ugly mess, but these values are display-only and it doesn't make
		// sense to store them anywhere else. :/
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
		});
		if self.cda_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 4 * 8,
				len: 1,
				explanation: "CDA performed".to_owned(),
			});
		}
		if self.offline_dda_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 4 * 8,
				len: 1,
				explanation: "Offline DDA performed".to_owned(),
			});
		}
		if self.issuer_authentication_not_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 4 * 8,
				len: 1,
				explanation: "Issuer authentication not performed".to_owned(),
			});
		}
		if self.issuer_authentication_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 4 * 8,
				len: 1,
				explanation: "Issuer authentication failed".to_owned(),
			});
		}
		enabled_bits.push(EnabledBitRange {
			offset: 7 + 3 * 8,
			len: 4,
			explanation: format!("PIN try count: {}", self.pin_try_count),
		});
		if self.offline_pin_verification_performed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 3 * 8,
				len: 1,
				explanation: "Offline PIN verification performed".to_owned(),
			});
		}
		if self.offline_pin_verification_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 3 * 8,
				len: 1,
				explanation: "Offline PIN verification performed and PIN not successfully verified"
					.to_owned(),
			});
		}
		if self.pin_try_limit_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 3 * 8,
				len: 1,
				explanation: "PIN try limit exceeded".to_owned(),
			});
		}
		if self.last_online_transaction_not_completed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 * 8,
				len: 1,
				explanation: "Last online transaction not completed".to_owned(),
			});
		}
		if self.offline_transaction_count_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 7 + 2 * 8,
				len: 1,
				explanation: "Lower offline transaction count limit exceeded".to_owned(),
			});
		}
		if self.offline_transaction_count_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 6 + 2 * 8,
				len: 1,
				explanation: "Upper offline transaction count limit exceeded".to_owned(),
			});
		}
		if self.offline_cumulative_amount_limit_lower_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 5 + 2 * 8,
				len: 1,
				explanation: "Lower cumulative offline amount limit exceeded".to_owned(),
			});
		}
		if self.offline_cumulative_amount_limit_upper_exceeded {
			enabled_bits.push(EnabledBitRange {
				offset: 4 + 2 * 8,
				len: 1,
				explanation: "Upper cumulative offline amount limit exceeded".to_owned(),
			});
		}
		if self.issuer_discretionary_bit_1 {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 2 * 8,
				len: 1,
				explanation: "Issuer discretionary bit 1".to_owned(),
			});
		}
		if self.issuer_discretionary_bit_2 {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 2 * 8,
				len: 1,
				explanation: "Issuer discretionary bit 2".to_owned(),
			});
		}
		if self.issuer_discretionary_bit_3 {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 2 * 8,
				len: 1,
				explanation: "Issuer discretionary bit 3".to_owned(),
			});
		}
		if self.issuer_discretionary_bit_4 {
			enabled_bits.push(EnabledBitRange {
				offset: 2 * 8,
				len: 1,
				explanation: "Issuer discretionary bit 4".to_owned(),
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
		});
		if self.issuer_script_processing_failed {
			enabled_bits.push(EnabledBitRange {
				offset: 3 + 8,
				len: 1,
				explanation: "Issuer script processing failed".to_owned(),
			});
		}
		if self.offline_data_authentication_failed_on_previous_transaction {
			enabled_bits.push(EnabledBitRange {
				offset: 2 + 8,
				len: 1,
				explanation: "Offline data authentication failed on previous transaction"
					.to_owned(),
			});
		}
		if self.go_online_on_next_transaction {
			enabled_bits.push(EnabledBitRange {
				offset: 1 + 8,
				len: 1,
				explanation: "Go online on next transaction".to_owned(),
			});
		}
		if self.unable_to_go_online {
			enabled_bits.push(EnabledBitRange {
				offset: 8,
				len: 1,
				explanation: "Unable to go online".to_owned(),
			});
		}
		enabled_bits.reverse();

		display_breakdown(self.bits, Self::NUM_BITS, &enabled_bits[..]);
	}
}
