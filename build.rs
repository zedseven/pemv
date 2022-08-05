// Uses
use std::{env::var_os, ffi::OsStr, fs::write as fs_write, io::Error, path::PathBuf};

use clap_complete::{
	generate_to,
	shells::{Bash, Elvish, Fish, PowerShell, Zsh},
	Generator,
};
use clap_mangen::Man;
use dotenv::dotenv;

// Constants
const MAN_PAGE_OUT_DIR_VAR: &str = "MAN_PAGE_OUT_DIR";
const COMPLETION_SCRIPTS_OUT_DIR_VAR: &str = "COMPLETION_SCRIPTS_OUT_DIR";

// Include the CLI source file to get a copy of the CLI definition (since this
// happens before the rest of the program is built)
include!("src/cli.rs");

fn main() -> Result<(), Error> {
	// Load environment variables
	dotenv().ok();

	// Generate install integrations on release builds
	if !cfg!(debug_assertions) {
		// Build the CLI definition
		let mut cli_definition = build_cli();

		// Generate the integrations
		generate_man_page(cli_definition.clone())?;
		generate_shell_completions(&mut cli_definition)?;
	}

	Ok(())
}

fn generate_man_page(cli_definition: Command) -> Result<(), Error> {
	// Get the out directory, or exit if it's not specified
	let out_dir = match var_os(MAN_PAGE_OUT_DIR_VAR) {
		Some(out_dir) => out_dir,
		None => {
			println!(
				"cargo:warning=No man page directory was specified, so no man page will be \
				 generated. Set this value using the environment variable `{}`.",
				MAN_PAGE_OUT_DIR_VAR
			);
			return Ok(());
		}
	};

	// Generate the man page
	let man = Man::new(cli_definition);
	let mut buffer: Vec<u8> = Default::default();
	man.render(&mut buffer)?;

	let output_path = PathBuf::from(out_dir).join(format!("{}.1", APPLICATION_BIN_NAME));

	fs_write(output_path.as_os_str(), buffer)?;

	println!(
		"cargo:warning=Man page generated: {:?}",
		output_path.as_os_str()
	);

	Ok(())
}

fn generate_shell_completions(cli_definition: &mut Command) -> Result<(), Error> {
	// Get the out directory, or exit if it's not specified
	let out_dir = match var_os(COMPLETION_SCRIPTS_OUT_DIR_VAR) {
		Some(out_dir) => out_dir,
		None => {
			println!(
				"cargo:warning=No completion directory was specified, so no shell completion \
				 scripts will be generated. Set this value using the environment variable `{}`.",
				COMPLETION_SCRIPTS_OUT_DIR_VAR
			);
			return Ok(());
		}
	};

	// Generate each shell's completion script
	generate_shell_completion("Bash", Bash, cli_definition, out_dir.as_os_str())?;
	generate_shell_completion("Elvish", Elvish, cli_definition, out_dir.as_os_str())?;
	generate_shell_completion("Fish", Fish, cli_definition, out_dir.as_os_str())?;
	generate_shell_completion(
		"PowerShell",
		PowerShell,
		cli_definition,
		out_dir.as_os_str(),
	)?;
	generate_shell_completion("Zsh", Zsh, cli_definition, out_dir.as_os_str())?;

	Ok(())
}

fn generate_shell_completion<G>(
	shell_name: &str,
	shell: G,
	cli_definition: &mut Command,
	out_dir: &OsStr,
) -> Result<(), Error>
where
	G: Generator,
{
	let output_path = generate_to(shell, cli_definition, APPLICATION_BIN_NAME, out_dir)?;
	println!(
		"cargo:warning=Completion file for {} generated: {:?}",
		shell_name, output_path
	);

	Ok(())
}
