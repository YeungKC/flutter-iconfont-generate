use std::fs;

use anyhow::{bail, Result};
use clap::Parser;

use converter::converter::convert;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
    output_file: String,
    #[clap(short, long)]
    package: Option<String>,
    #[clap(short, long, default_value = "IconFonts")]
    class_name: String,
    #[clap(short, long, default_value = "false")]
    overwrite: bool,
}

fn exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !exists(&args.input_file) {
        bail!("CSS_FILE {} does not exist!", args.input_file);
    }

    let css = fs::read_to_string(args.input_file)?;

    let result = convert(&args.class_name, &css, args.package.as_deref())?;

    if !args.overwrite && exists(&args.output_file) {
        println!("{} is exists, overwrite? [y/N]", args.output_file);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            bail!("User cancel");
        }
    }

    fs::write(args.output_file, result)?;

    Ok(())
}
