use std::{fs::File, io::Read, str::FromStr};

use clap::{Parser, Subcommand};

use crate::CmdExcutor;

use super::verify_file;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};

#[derive(Subcommand, Debug)]
pub enum Base64SubCommand {
    #[command(name = "decode", about = "Decode a base 64 string")]
    Decode(Base64DecodeOpts),
    #[command(name = "encode", about = "Encode a base 64 string")]
    Encode(Base64EncodeOpts),
}

impl CmdExcutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Encode(opts) => process_encode(&opts.input, opts.format),
            Self::Decode(opts) => process_decode(&opts.input, opts.format),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = base64_format_parse, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = base64_format_parse, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone)]
pub enum Base64Format {
    Standard,
    URlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Self::Standard),
            "urlsafe" => Ok(Self::URlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim();
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::URlSafe => URL_SAFE.encode(&data),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    // println!("{:?}", data);
    let data = data.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&data),
        Base64Format::URlSafe => URL_SAFE.decode(&data),
    }?;

    println!("{}", String::from_utf8(decoded)?);
    Ok(())
}

pub fn base64_format_parse(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_decode() {
        let input = "tmp.b64";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
