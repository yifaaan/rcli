use clap::Parser;
use rcli::{process_csv, process_genpass, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);
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
            println!("{:?}", opts);
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?
        }
    }

    Ok(())
}
