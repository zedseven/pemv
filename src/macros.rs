//! Provides convenience macros for use in the rest of the project.

/// A non-composite enum value that doesn't assign each `value` to the
/// `variant`'s discriminant, and as such allows for multiple `value`s per
/// `variant`.
///
/// This version supports fallible conversion, so not all patterns must be
/// present.
#[macro_export]
macro_rules! non_composite_value_no_repr_fallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty, $error:path {
	        $($variant:ident = $value:pat => $string:literal,)*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
                $variant,
            )*
        }

        impl TryFrom<$typ> for $name {
            type Error = ParseError;

			fn try_from(value: $typ) -> Result<Self, ParseError> {
				match value {
					$($value => Ok(Self::$variant),)*
					_ => Err($error),
				}
			}
        }

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
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
macro_rules! non_composite_value_no_repr_infallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty {
	        $($variant:ident = $value:pat => $string:literal,)*
	    }
	) => {
		$(#[$outer])*
        $visibility enum $name {
            $(
                #[doc = concat!($string, " - ", stringify!($value))]
                $variant,
            )*
        }

        impl From<$typ> for $name {
			fn from(value: $typ) -> Self {
				match value {
					$($value => Self::$variant,)*
				}
			}
        }

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
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
macro_rules! non_composite_value_repr_fallible {
	(
		$(#[$outer:meta])*
		$visibility:vis enum $name:ident: $typ:ty, $error:path {
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
            type Error = ParseError;

			fn try_from(value: $typ) -> Result<Self, ParseError> {
				match value {
					$($value => Ok(Self::$variant),)*
					_ => Err($error),
				}
			}
        }

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
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
macro_rules! non_composite_value_repr_infallible {
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
                #[doc = $string]
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

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str(match self {
					$(Self::$variant => $string,)*
				})
			}
		}
    };
}
