use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{process_http_serve, CmdExcutor};

use super::verify_path;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(about = "Serve a file directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExcutor for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Serve(opts) => process_http_serve(opts.dir, opts.port).await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
