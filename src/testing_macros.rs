//! Provides convenience macros for use in project testing.

#![cfg(test)]

#[macro_export]
macro_rules! wrong_byte_count {
	($typ:path, $byte_count:literal) => {
		#[doc = concat!("Ensures an error is returned if the wrong number of bytes are provided when parsing [`", stringify!($typ), "`].")]
		#[test]
		fn wrong_byte_count() {
			let expected = Err($crate::error::ParseError::ByteCountIncorrect {
				r#type:   std::cmp::Ordering::Equal,
				expected: $byte_count,
				found:    $byte_count + 1,
			});
			let result = <$typ>::try_from([0x00; $byte_count + 1].as_slice());

			assert_eq!(expected, result);
		}
	};
}

#[macro_export]
macro_rules! bitflag_unique_values {
	($typ:path, $byte_count:literal) => {
		#[doc = concat!("Ensures no two bit values return the same final struct value for [`", stringify!($typ), "`].")]
		#[test]
		fn bitflag_unique_values() {
			// Get the empty result to compare against
			let empty_result = <$typ>::try_from([0x00; $byte_count].as_slice())
				.expect("error checking will be done separately");

			// Step through every bit and confirm that each one has a unique value
			let mut results_hash_set = std::collections::HashSet::new();
			for byte_index in 0..$byte_count {
				for bit_index in 0..$crate::BITS_PER_BYTE {
					let mut testing_vec = vec![0x00; $byte_count];
					testing_vec[byte_index] |= 0b1 << bit_index;

					let result = <$typ>::try_from(testing_vec.as_slice())
						.expect("error checking will be done separately");
					// If the bit we're testing is something with no meaning here, there's nothing
					// to test
					if result == empty_result {
						continue;
					}

					assert_eq!(
						testing_vec.as_slice(),
						result.get_binary_representation(),
						"the bytes retrieved later should match the input"
					);

					assert!(
						results_hash_set.insert(result.clone()),
						"two different bit inputs led to the same bitflag result: {:?}",
						result
					);
				}
			}
		}
	};
}

#[macro_export]
macro_rules! bitflag_display_bits {
	($typ:path, $byte_count:literal) => {
		// Uses
		use $crate::emv::bitflag_values::BitflagValue;

		#[doc = concat!("Ensures the display bits for [`", stringify!($typ), "`] are correct.")]
		#[test]
		fn bitflag_display_bits() {
			// Get the empty result to compare against
			let empty_result = <$typ>::try_from([0x00; $byte_count].as_slice())
				.expect("error checking will be done separately");
			let empty_result_bit_display_info = empty_result.get_bit_display_information();
			let mut empty_result_bit_offsets = std::collections::HashSet::new();
			for display_bit_range in &empty_result_bit_display_info {
				assert!(
					empty_result_bit_offsets.insert(display_bit_range.offset),
					"there shouldn't be two display bits with the same offset"
				);
			}

			// Step through every bit and confirm there are no incorrect display values
			let mut display_bit_hash_set = std::collections::HashSet::new();
			for byte_index in 0..$byte_count {
				for bit_index in 0..$crate::BITS_PER_BYTE as usize {
					let mut testing_vec = vec![0x00; $byte_count];
					testing_vec[byte_index] |= 0b1 << bit_index;

					let result = <$typ>::try_from(testing_vec.as_slice())
						.expect("error checking will be done separately");
					// If the bit we're testing is something with no meaning here, there's nothing
					// to test
					if result == empty_result {
						continue;
					}

					// Ensure there's a display bit
					let bit_display_info = result.get_bit_display_information();
					assert!(
						bit_display_info.len() >= empty_result_bit_display_info.len(),
						"there should be no fewer display bits for a value than there were for \
						 the empty result"
					);
					let new_display_bits_from_empty =
						bit_display_info.len() - empty_result_bit_display_info.len();
					assert!(
						new_display_bits_from_empty <= 1,
						"there should be at most one new display bit per value bit - found: {}",
						new_display_bits_from_empty
					);

					for enabled_display_bit in &bit_display_info {
						// Skip the display bits that are always present
						if empty_result_bit_offsets.contains(&enabled_display_bit.offset) {
							continue;
						}

						// Ensure the returned bit offset is correct
						let test_bit_offset = (($byte_count - 1) - byte_index)
							* $crate::BITS_PER_BYTE as usize
							+ bit_index;
						assert!(
							test_bit_offset <= enabled_display_bit.offset as usize,
							"the display bit offset should match the input bit (test_bit_offset \
							 {} <= display offset {})",
							test_bit_offset,
							enabled_display_bit.offset
						);
						assert!(
							(test_bit_offset + enabled_display_bit.len as usize)
								> enabled_display_bit.offset as usize,
							"the display bit offset should match the input bit (test_bit_offset \
							 {} + display len {} > display offset {})",
							test_bit_offset,
							enabled_display_bit.len,
							enabled_display_bit.offset
						);

						// Ensure the returned display bit is unique
						assert!(
							display_bit_hash_set.insert(bit_display_info),
							"two different bit inputs led to the same display bit value"
						);

						// If we get this far, we've found the one new display bit (we confirmed
						// there's only one earlier)
						break;
					}
				}
			}
		}
	};
}

#[macro_export]
macro_rules! enum_byte_slice_result_matches_true_value_result {
	($typ:path, $byte_count:literal, $test_true_value:expr, $test_byte_slice:expr) => {
		#[doc = concat!("Ensures that the byte slice parser returns the same value as the parser of the \"true\" format of the data, for [`", stringify!($typ), "`].")]
		#[test]
		fn enum_byte_slice_result_matches_true_value_result() {
			let expected = <$typ>::try_from($test_true_value);
			let result = <$typ>::try_from($test_byte_slice);

			assert_eq!(expected, result);
		}
	};
}
