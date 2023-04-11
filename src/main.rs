mod path_mapper;
mod abs_file;

use crate::path_mapper::file_mapper;
use clap::Parser;
use std::path::PathBuf;
use crate::abs_file::abs_file;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A ChRIS plugin to calculate absolute value of numbers in data files",
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// Input path filter. Paths which do not match this regex are excluded.
    #[clap(
    short, long,
    use_value_delimiter = true, value_delimiter = ',',
    default_values_t = [".txt".to_string()]
    )]
    filter: Vec<String>,

    /// deprecated ChRIS flag. Does nothing.
    #[clap(long, hide = true)]
    saveinputmeta: bool,

    /// deprecated ChRIS flag. Does nothing.
    #[clap(long, hide = true)]
    saveoutputmeta: bool,

    /// input directory
    #[clap()]
    input_dir: PathBuf,

    /// output directory
    #[clap()]
    output_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::parse();

    for r in file_mapper(args.input_dir, args.output_dir, args.filter)? {
        let (input_file, output_file) = r?;
        abs_file(&input_file, &output_file)?;
        println!("{input_file:?} -> {output_file:?}");
    }

    anyhow::Ok(())
}

