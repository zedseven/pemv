//! Contains the various colour specs so they're not re-written over and over
//! again elsewhere.

// Uses
use termcolor::{Color, ColorSpec};

/// Bold, without any specified colour.
pub fn bold_colour_spec() -> ColorSpec {
	let mut c = ColorSpec::new();
	c.set_bold(true);
	c
}

/// Bold, with a specified colour. To be used for headers.
pub fn header_colour_spec() -> ColorSpec {
	let mut c = ColorSpec::new();
	c.set_bold(true).set_fg(Some(Color::Cyan));
	c
}
