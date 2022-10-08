//! Provides convenience macros for use in the rest of the project.

/// A non-composite enum value that doesn't assign each `value` to the
/// `variant`'s discriminant, and as such allows for multiple `value`s per
/// `variant`.
///
/// This version supports fallible conversion, so not all patterns must be
/// present.
#[macro_export]
macro_rules! enum_no_repr_fallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty, $error_type:ty, {$error_fn:expr} {
	        $(
                $(#[doc = $doc:expr])?
	            $variant:ident = $pattern:pat => $string:literal,
	        )*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($pattern))]
                $(#[doc = $doc])?
                $variant,
            )*
        }

        impl TryFrom<$typ> for $name {
            type Error = $error_type;

			fn try_from(value: $typ) -> Result<Self, $error_type> {
	            #[allow(clippy::redundant_closure_call)]
				match value {
					$($pattern => Ok(Self::$variant),)*
					_ => Err(($error_fn)(value)),
				}
			}
        }

		#[cfg(not(tarpaulin_include))]
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
			}
		}

		#[cfg(tarpaulin)]
		#[cfg(not(tarpaulin_include))]
		impl $name {
			pub fn cover_all_enum_variants() {
				$(
					Self::$variant.to_string();
					Self::$variant.clone();
				)*
			}
		}
    };
}

/// A non-composite enum value that doesn't assign each `value` to the
/// `variant`'s discriminant, and as such allows for multiple `value`s per
/// `variant`.
///
/// This version supports infallible conversion, so all patterns must be
/// present.
#[macro_export]
macro_rules! enum_no_repr_infallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty {
	        $(
                $(#[doc = $doc:expr])?
	            $variant:ident = $pattern:pat => $string:literal,
	        )*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($pattern))]
                $(#[doc = $doc])?
                $variant,
            )*
        }

        impl From<$typ> for $name {
			fn from(value: $typ) -> Self {
				match value {
					$($pattern => Self::$variant,)*
				}
			}
        }

		#[cfg(not(tarpaulin_include))]
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
			}
		}

		#[cfg(tarpaulin)]
		#[cfg(not(tarpaulin_include))]
		impl $name {
			pub fn cover_all_enum_variants() {
				$(
					Self::$variant.to_string();
					Self::$variant.clone();
				)*
			}
		}
    };
}

/// A non-composite enum value that assigns each `value` to the `variant`'s
/// discriminant, and as such requires only one `value` per `variant`.
///
/// This version supports fallible conversion, so not all patterns must be
/// present.
#[macro_export]
macro_rules! enum_repr_fallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty, $error_type:ty, {$error_fn:expr} {
	        $(
                $(#[doc = $doc:expr])?
	            $variant:ident = $value:literal => $string:literal,
	        )*
	    }
	) => {
		#[repr($typ)]
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
                $(#[doc = $doc])?
                $variant = $value,
            )*
        }

        impl TryFrom<$typ> for $name {
            type Error = $error_type;

			fn try_from(value: $typ) -> Result<Self, $error_type> {
	            #[allow(clippy::redundant_closure_call)]
				match value {
					$($value => Ok(Self::$variant),)*
					_ => Err(($error_fn)(value)),
				}
			}
        }

		impl From<$name> for $typ {
			fn from(value: $name) -> Self {
				value as $typ
			}
		}

		#[cfg(not(tarpaulin_include))]
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
			}
		}

		#[cfg(tarpaulin)]
		#[cfg(not(tarpaulin_include))]
		impl $name {
			pub fn cover_all_enum_variants() {
				$(
					Self::$variant.to_string();
					Self::$variant.clone();
				)*
			}
		}
    };
}

/// A non-composite enum value that assigns each `value` to the `variant`'s
/// discriminant, and as such requires only one `value` per `variant`.
///
/// This version supports infallible conversion, so all patterns must be
/// present.
#[macro_export]
macro_rules! enum_repr_infallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty {
	        $(
                $(#[doc = $doc:expr])?
	            $variant:ident = $value:literal => $string:literal,
	        )*
	    }
	) => {
		#[repr($typ)]
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
                $(#[doc = $doc])?
                $variant = $value,
            )*
        }

        impl From<$typ> for $name {
			fn from(value: $typ) -> Self {
				match value {
					$($value => Self::$variant,)*
				}
			}
        }

		impl From<$name> for $typ {
			fn from(value: $name) -> Self {
				value as $typ
			}
		}

		#[cfg(not(tarpaulin_include))]
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
			}
		}

		#[cfg(tarpaulin)]
		#[cfg(not(tarpaulin_include))]
		impl $name {
			pub fn cover_all_enum_variants() {
				$(
					Self::$variant.to_string();
					Self::$variant.clone();
				)*
			}
		}
    };
}

#[macro_export]
macro_rules! bitflag_value {
	// Main Definition
	// The `$typ` has to be a token tree so that it can be passed to an internal rule and matched
	// against literal strings. This is unfortunate.
	// More info: https://doc.rust-lang.org/reference/macros-by-example.html#transcribing
	(
		$(#[$outer:meta])*
		$visibility:vis struct $name:ident: $byte_count:literal {
	        $(
	            $byte_num:literal {
		            $(
		                $(#[doc = $doc:expr])?
		                $field_visibility:vis $field:ident: $typ:tt = $field_tt:tt => $display_tt:tt,
		            )*
	            }
	        )*
	    }
	) => {
		$(#[$outer])*
        $visibility struct $name {
            $($(
                $(#[doc = $doc])?
                #[doc = concat!("From byte ", $byte_num)]
                $field_visibility $field: $typ,
            )*)*
        }

		impl TryFrom<&[u8]> for $name {
			type Error = ParseError;

			fn try_from(raw_bytes: &[u8]) -> Result<Self, Self::Error> {
				use $crate::emv::bitflag_values::BitflagValue;

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
					$($(
						$field: bitflag_value!(@parse: $typ, bytes, $byte_num, $field_tt),
					)*)*
				})
			}
		}

		impl $crate::emv::bitflag_values::BitflagValue for $name {
			const NUM_BYTES: usize = $byte_count;
			const USED_BITS_MASK: &'static [u8] = &[
				$(
					$(
						bitflag_value!(@byte_bits: $field_tt) |
					)* 0b0000_0000,
				)*
			];

			fn get_binary_representation(&self) -> Vec<u8> {
				vec![
					$(
						$(
							bitflag_value!(@field_binary_repr: self, $field, $typ, $field_tt) |
						)* 0b0000_0000,
					)*
				]
			}

			fn get_bit_display_information(&self) -> Vec<$crate::emv::bitflag_values::EnabledBitRange> {
				let mut enabled_bits = Vec::new();

				$($(
					bitflag_value!(@display_field_bits: self, enabled_bits, $byte_count, $byte_num, $field, $typ, $field_tt, $display_tt);
				)*)*

				enabled_bits
			}
		}
    };

	// Internal Rules
	// Pulling the binary literal out of the token tree
	(@byte_bits: $byte_bits:literal) => {
		$byte_bits
	};
	(@byte_bits: ($byte_bits:literal >> $shift_bit_count:literal)) => {
		$byte_bits
	};

	// Bit-length calculations
	(@bit_len: $byte_bits:literal) => {
		(($byte_bits >> ($byte_bits as u8).trailing_zeros()) as u8).trailing_ones() as u8
	};
	(@bit_len: ($byte_bits:literal >> $shift_bit_count:literal)) => {
		(($byte_bits >> $shift_bit_count) as u8).trailing_ones() as u8
	};

	// Parsing
	(@parse: bool, $bytes:ident, $byte_num:literal, $byte_bits:literal) => {
		$byte_bits & $bytes[$byte_num] > 0
	};
	(@parse: u8, $bytes:ident, $byte_num:literal, $byte_bits:literal) => {
		($byte_bits & $bytes[$byte_num]) as u8
	};
	(@parse: u8, $bytes:ident, $byte_num:literal, ($byte_bits:literal >> $shift_bit_count:literal)) => {
		(($byte_bits & $bytes[$byte_num]) >> $shift_bit_count) as u8
	};
	(@parse: $typ:ty, $bytes:ident, $byte_num:literal, $byte_bits:literal) => {
		<$typ>::try_from($byte_bits & $bytes[$byte_num])?
	};
	(@parse: $typ:ty, $bytes:ident, $byte_num:literal, ($byte_bits:literal >> $shift_bit_count:literal)) => {
		<$typ>::try_from(($byte_bits & $bytes[$byte_num]) >> $shift_bit_count)?
	};

	// Generating binary representations
	(@field_binary_repr: $self:ident, $field:ident, bool, $byte_bits:literal) => {
		if $self.$field {
			$byte_bits
		} else {
			0b0000_0000
		}
	};
	(@field_binary_repr: $self:ident, $field:ident, $typ:ty, $byte_bits:literal) => {
		u8::from($self.$field)
	};
	(@field_binary_repr: $self:ident, $field:ident, $typ:ty, ($byte_bits:literal >> $shift_bit_count:literal)) => {
		u8::from($self.$field) << $shift_bit_count
	};

	// Displays
	(@display_field_bits: $self:ident, $enabled_bits:ident, $byte_count:literal, $byte_num:literal, $field:ident, bool, $field_tt:tt, $display:literal) => {
		if $self.$field {
			$enabled_bits.push($crate::emv::bitflag_values::EnabledBitRange {
				offset: (7 - (bitflag_value!(@byte_bits: $field_tt) as u8).leading_zeros() as u8) + ($byte_count - $byte_num - 1) * $crate::BITS_PER_BYTE,
				len: bitflag_value!(@bit_len: $field_tt),
				explanation: $display.to_owned(),
				severity: $crate::emv::bitflag_values::Severity::Normal,
			});
		};
	};
	(@display_field_bits: $self:ident, $enabled_bits:ident, $byte_count:literal, $byte_num:literal, $field:ident, bool, $field_tt:tt, ($severity:ident, $display:literal)) => {
		if $self.$field {
			$enabled_bits.push($crate::emv::bitflag_values::EnabledBitRange {
				offset: (7 - (bitflag_value!(@byte_bits: $field_tt) as u8).leading_zeros() as u8) + ($byte_count - $byte_num - 1) * $crate::BITS_PER_BYTE,
				len: bitflag_value!(@bit_len: $field_tt),
				explanation: $display.to_owned(),
				severity: $crate::emv::bitflag_values::Severity::$severity,
			});
		};
	};
	(@display_field_bits: $self:ident, $enabled_bits:ident, $byte_count:literal, $byte_num:literal, $field:ident, $typ:ty, $field_tt:tt, $display:literal) => {
		$enabled_bits.push($crate::emv::bitflag_values::EnabledBitRange {
			offset: (7 - (bitflag_value!(@byte_bits: $field_tt) as u8).leading_zeros() as u8) + ($byte_count - $byte_num - 1) * $crate::BITS_PER_BYTE,
			len: bitflag_value!(@bit_len: $field_tt),
			explanation: format!($display, $self.$field),
			severity: $crate::emv::bitflag_values::Severity::Normal,
		});
	};
	(@display_field_bits: $self:ident, $enabled_bits:ident, $byte_count:literal, $byte_num:literal, $field:ident, $typ:ty, $field_tt:tt, ($severity:ident, $display:literal)) => {
		$enabled_bits.push($crate::emv::bitflag_values::EnabledBitRange {
			offset: (7 - (bitflag_value!(@byte_bits: $field_tt) as u8).leading_zeros() as u8) + ($byte_count - $byte_num - 1) * $crate::BITS_PER_BYTE,
			len: bitflag_value!(@bit_len: $field_tt),
			explanation: format!($display, $self.$field),
			severity: $crate::emv::bitflag_values::Severity::$severity,
		});
	};
}
