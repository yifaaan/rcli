mod b64;
mod csv;
mod genpass;
mod http;

use std::path::{Path, PathBuf};

pub use b64::Base64SubCommand;
use clap::{Parser, Subcommand};
use csv::CsvOpts;
pub use http::HttpSubCommand;

pub use b64::{process_decode, process_encode};
pub use csv::OutputFormat;
pub use genpass::GenPassOpts;

use crate::{process_csv, process_genpass, CmdExcutor};

#[derive(Parser, Debug)]
#[command(name="rcli", version, author, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, name = "base64", about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

impl CmdExcutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Csv(opts) => {
                let output = if let Some(output) = opts.output {
                    output
                } else {
                    format!("output.{}", opts.format)
                };
                process_csv(&opts.input, output, opts.format)
            }
            Self::GenPass(opts) => {
                // println!("{:?}", opts);
                process_genpass(
                    opts.length,
                    opts.uppercase,
                    opts.lowercase,
                    opts.number,
                    opts.symbol,
                )
            }
            Self::Base64(subcmd) => subcmd.execute().await,
            Self::Http(subcmd) => subcmd.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, String> {
    Ok(path.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
    }
}
