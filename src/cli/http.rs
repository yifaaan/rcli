use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::verify_path;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(about = "Serve a file directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
