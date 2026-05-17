mod cli;
mod encoding;
mod streaming;
mod commands;
#[cfg(test)]
mod test_utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { input } => {
            commands::validate::validate(&input)?;
        }
        Commands::Split { input, output_dir } => {
            commands::split::split(&input, &output_dir)?;
        }
        Commands::Merge { inputs, output } => {
            commands::merge::merge(&inputs, &output)?;
        }
        Commands::Convert { input, output, format } => {
            commands::convert::convert(&input, &output, &format)?;
        }
    }

    Ok(())
}
