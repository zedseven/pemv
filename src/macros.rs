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
	        $($variant:ident = $pattern:pat => $string:literal,)*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($pattern))]
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
	        $($variant:ident = $pattern:pat => $string:literal,)*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($pattern))]
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
	        $($variant:ident = $value:literal => $string:literal,)*
	    }
	) => {
		#[repr($typ)]
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
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
	        $($variant:ident = $value:literal => $string:literal,)*
	    }
	) => {
		#[repr($typ)]
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
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
