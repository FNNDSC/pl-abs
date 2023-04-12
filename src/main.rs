mod abs_file;
mod path_mapper;

use crate::abs_file::abs_file;
use crate::path_mapper::file_mapper;
use clap::Parser;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A ChRIS plugin to calculate absolute value of numbers in data files",
    propagate_version = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// File extensions of files to include.
    #[clap(
    short, long,
    use_value_delimiter = true, value_delimiter = ',',
    default_values_t = [".txt".to_string()]
    )]
    input_files: Vec<String>,

    /// Change file extensions of affected output files
    #[clap(short, long)]
    output_suffix: Option<String>,

    /// Copy ignored files to output dir
    #[clap(short, long)]
    copy: bool,

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

    for r in file_mapper(
        &args.input_dir,
        &args.output_dir,
        &args.input_files.as_ref(),
    )? {
        let (input_file, output_file) = r?;

        if let Some(output_suffix) = &args.output_suffix {
            let output_file = output_file.with_extension(output_suffix);
            abs_file(&input_file, &output_file)?;
            print_pair(&input_file, &output_file, "abs");
        } else {
            abs_file(&input_file, &output_file)?;
            print_pair(&input_file, &output_file, "abs");
        }
    }

    if args.copy {
        for r in file_mapper(&args.input_dir, &args.output_dir, &[""])? {
            let (input_file, output_file) = r?;
            if !output_file.is_file() {
                fs::copy(&input_file, &output_file)?;
                print_pair(&input_file, &output_file, "copy")
            }
        }
    }

    anyhow::Ok(())
}

const GREEN: &str = "\x1b[0;32m";
const CYAN: &str = "\x1b[0;36m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn print_pair(a: impl Debug, b: impl Debug, note: &str) {
    if std::env::var("NO_COLOR").is_ok() {
        if note.is_empty() {
            eprintln!("{a:?} -> {b:?}")
        } else {
            eprintln!("({note}) {a:?} -> {b:?}")
        }
    } else {
        if note.is_empty() {
            eprintln!("{CYAN}{a:?}{RESET} -> {GREEN}{b:?}{RESET}")
        } else {
            eprintln!("{DIM}({note}){RESET} {CYAN}{a:?}{RESET} -> {GREEN}{b:?}{RESET}")
        }
    }
}
