use clap::{arg, Parser, Subcommand};
use std::{fmt::Display, path::Path, str::FromStr};

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
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = OutputFormat::Json)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(short = 'y', long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl Into<clap::builder::OsStr> for OutputFormat {
    fn into(self) -> clap::builder::OsStr {
        clap::builder::OsStr::from(<OutputFormat as Into<&'static str>>::into(self))
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
