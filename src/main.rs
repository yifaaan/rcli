use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Cli, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // println!("{:?}", cli);
    match cli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            // println!("{:?}", opts);
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?
        }
        SubCommand::Base64(subcmd) => {
            // println!("{:?}", subcmd);
            match subcmd {
                Base64SubCommand::Decode(x) => {
                    process_decode(&x.input, x.format)?;
                }
                Base64SubCommand::Encode(x) => {
                    process_encode(&x.input, x.format)?;
                }
            }
        }
    }

    Ok(())
}
