//! The terminal type value, typically from EMV tag `0x9F35`.
//!
//! Information for this can be found in EMV Book 4, under section `A1`.

// Uses
use std::cmp::Ordering;

use termcolor::{StandardStream, WriteColor};

use crate::{
	emv::{ber_tlv::parse_tag_metadata, identify_tag, DataObjectType, TagClass},
	error::ParseError,
	header_colour_spec,
	output_colours::bold_colour_spec,
	util::{print_bytes_small, print_indentation},
	DisplayBreakdown,
};

// Constants
const MIN_BYTES: usize = 1;

// Struct Implementation
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TagBasicInfo {
	pub tag:              Vec<u8>,
	pub name:             Option<&'static str>,
	pub class:            TagClass,
	pub data_object_type: DataObjectType,
}

impl TryFrom<&[u8]> for TagBasicInfo {
	type Error = ParseError;

	fn try_from(tag: &[u8]) -> Result<Self, Self::Error> {
		if tag.len() < MIN_BYTES {
			return Err(ParseError::ByteCountIncorrect {
				r#type:   Ordering::Greater,
				expected: MIN_BYTES,
				found:    tag.len(),
			});
		}

		let name = identify_tag(tag);
		let (class, data_object_type) = parse_tag_metadata(tag[0]);

		Ok(Self {
			tag: tag.to_vec(),
			name,
			class,
			data_object_type,
		})
	}
}

#[cfg(not(tarpaulin_include))]
impl DisplayBreakdown for TagBasicInfo {
	fn display_breakdown(&self, stdout: &mut StandardStream, indentation: u8, _: bool) {
		let bold_colour_spec = bold_colour_spec();
		let header_colour_spec = header_colour_spec();

		let name = self.name.unwrap_or("<Unknown>");

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		print!("Name:");
		stdout.reset().ok();
		println!("             {name}");

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		print!("Tag:");
		stdout.reset().ok();
		print!("              0x");
		stdout.set_color(&bold_colour_spec).ok();
		print_bytes_small(self.tag.as_slice());
		stdout.reset().ok();
		println!();

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		print!("Class:");
		stdout.reset().ok();
		println!("            {}", self.class);

		print_indentation(indentation);
		stdout.set_color(&header_colour_spec).ok();
		print!("Data Object Type:");
		stdout.reset().ok();
		println!(" {}", self.data_object_type);
	}
}
