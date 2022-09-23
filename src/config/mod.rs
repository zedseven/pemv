// Modules
pub mod colour_choice;

// Uses
use clap::ArgMatches;
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
	pub profile: Profile,
	pub cli_colour: ColourChoice,
	pub masking_characters: Vec<char>,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			profile: Self::DEFAULT_PROFILE,
			cli_colour: ColourChoice::default(),
			masking_characters: vec!['*'],
		}
	}
}

impl Config {
	// Constants
	pub const DEFAULT_PROFILE: Profile = Profile::const_new("default");
	// Key Names
	pub const PROFILE: &'static str = "profile";
	pub const CLI_COLOUR: &'static str = "cli_colour";
	pub const MASKING_CHARACTERS: &'static str = "masking_characters";

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
					format!("{}{}", ENV_PREFIX, ENV_FILE_NAME_OVERRIDE).as_str(),
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
				format!("{}{}", ENV_PREFIX, ENV_PROFILE).as_str(),
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
	if let Some(colour_choice) = matches.value_of("colour") {
		if colour_choice != "from_config" {
			figment = figment.merge((
				Config::CLI_COLOUR,
				TryInto::<ColourChoice>::try_into(colour_choice)
					.expect("this value's validity is enforced by clap"),
			));
		}
	}

	figment
}
