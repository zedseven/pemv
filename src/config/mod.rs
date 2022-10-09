// Modules
pub mod colour_choice;

// Uses
use clap::{ArgMatches, ValueSource};
use figment::{
	providers::{Env, Format, Serialized, Toml},
	value::{Dict, Map},
	Error as FigmentError,
	Figment,
	Metadata,
	Profile,
	Provider,
};
use serde_derive::{Deserialize, Serialize};

use self::colour_choice::ColourChoice;

// Constants
const FILE_NAME: &str = "pemv.toml";
/// Concatenated with the variable names below.
const ENV_PREFIX: &str = "PEMV_";
// TODO: Test this
const ENV_FILE_NAME_OVERRIDE: &str = "CONFIG";
const ENV_PROFILE: &str = "PROFILE";

/// The app configuration.
#[non_exhaustive]
#[derive(Deserialize, Serialize)]
pub struct Config {
	#[serde(skip)]
	pub profile:            Profile,
	pub cli_colour:         ColourChoice,
	pub masking_characters: Vec<char>,
	pub sort_parsed_tags:   bool,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			profile:            Self::DEFAULT_PROFILE,
			cli_colour:         ColourChoice::default(),
			masking_characters: vec!['*'],
			sort_parsed_tags:   true,
		}
	}
}

impl Config {
	// Constants
	pub const CLI_COLOUR: &'static str = "cli_colour";
	pub const DEFAULT_PROFILE: Profile = Profile::const_new("default");
	pub const MASKING_CHARACTERS: &'static str = "masking_characters";
	pub const PROFILE: &'static str = "profile";
	pub const SORT_PARSED_TAGS: &'static str = "sort_parsed_tags";

	/// Allows the configuration to be extracted from any [`Provider`].
	///
	/// The reason this isn't a [`TryFrom`] implementation is because it
	/// conflicts with a default implementation provided by Rust.
	pub fn try_from<P>(provider: P) -> Result<Config, FigmentError>
	where
		P: Provider,
	{
		Figment::from(provider).extract()
	}

	/// Provides a default provider.
	pub fn figment() -> Figment {
		Figment::from(Serialized::defaults(Config::default()))
			.merge(
				Toml::file(Env::var_or(
					format!("{ENV_PREFIX}{ENV_FILE_NAME_OVERRIDE}").as_str(),
					FILE_NAME,
				))
				.nested(),
			)
			.merge(
				Env::prefixed(ENV_PREFIX)
					.ignore(&[ENV_FILE_NAME_OVERRIDE, ENV_PROFILE])
					.global(),
			)
			.select(Profile::from_env_or(
				format!("{ENV_PREFIX}{ENV_PROFILE}").as_str(),
				Self::DEFAULT_PROFILE,
			))
	}
}

impl Provider for Config {
	fn metadata(&self) -> Metadata {
		Metadata::named("App Config")
	}

	fn data(&self) -> Result<Map<Profile, Dict>, FigmentError> {
		Serialized::defaults(self).data()
	}

	fn profile(&self) -> Option<Profile> {
		Some(self.profile.clone())
	}
}

/// Applies values provided via CLI to the `figment`, overriding anything from
/// the configuration or environment variables.
///
/// This effectively sets up the following hierarchy: CLI arguments ->
/// environment variables -> configuration file
pub fn apply_cli_arguments(mut figment: Figment, matches: &ArgMatches) -> Figment {
	// CLI Colour Choice
	if let Some(colour_choice) = matches.get_one::<String>("colour") {
		if matches.value_source("colour").unwrap() != ValueSource::DefaultValue {
			figment = figment.merge((
				Config::CLI_COLOUR,
				ColourChoice::try_from(colour_choice.as_str())
					.expect("this value's validity is enforced by clap"),
			));
		}
	}

	// Masking Characters
	if let Some(masking_characters) = matches.get_many::<char>("masking-character") {
		if matches.value_source("masking-character").unwrap() != ValueSource::DefaultValue {
			figment = figment.merge((
				Config::MASKING_CHARACTERS,
				masking_characters.copied().collect::<Vec<char>>(),
			));
		}
	}

	// Sorting
	let mut cli_provided_sort_preference = None;
	if let Some(&sort_parsed_tags) = matches.get_one::<bool>("sort-parsed-tags") {
		if matches.value_source("sort-parsed-tags").unwrap() != ValueSource::DefaultValue {
			cli_provided_sort_preference = Some(sort_parsed_tags);
		}
	}
	if let Some(&no_sort_parsed_tags) = matches.get_one::<bool>("no-sort-parsed-tags") {
		if matches.value_source("no-sort-parsed-tags").unwrap() != ValueSource::DefaultValue {
			cli_provided_sort_preference = Some(!no_sort_parsed_tags);
		}
	}
	if let Some(sort_parsed_tags) = cli_provided_sort_preference {
		figment = figment.merge((Config::SORT_PARSED_TAGS, sort_parsed_tags));
	}

	figment
}
