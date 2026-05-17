use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validates the structural integrity and encoding of a BinaryCIF file
    Validate {
        /// Path to the BinaryCIF file
        input: String,
    },
    /// Splits a BinaryCIF file into multiple files by DataBlock or Category
    Split {
        /// Path to the input BinaryCIF file
        input: String,
        /// Output directory for split files
        #[arg(short, long, default_value = ".")]
        output_dir: String,
    },
    /// Merges multiple BinaryCIF files into a single output
    Merge {
        /// Paths to the input BinaryCIF files
        inputs: Vec<String>,
        /// Path to the output BinaryCIF file
        #[arg(short, long)]
        output: String,
    },
    /// Converts between BinaryCIF and other formats (e.g., text CIF)
    Convert {
        /// Path to the input file
        input: String,
        /// Path to the output file
        #[arg(short, long)]
        output: String,
        /// Target format (e.g., "cif", "bcif")
        #[arg(short, long)]
        format: String,
    },
}
