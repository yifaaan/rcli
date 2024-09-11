use clap::Parser;
use rcli::{Cli, CmdExcutor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    // println!("{:?}", cli);
    cli.cmd.execute().await
}
