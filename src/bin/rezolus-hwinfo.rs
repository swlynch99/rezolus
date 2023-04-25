use std::{io::Write, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use rezolus::samplers::hwinfo::Hwinfo;

/// Export the output from the rezolus hwinfo sampler as json.
#[derive(Parser)]
#[command(author, version)]
struct Args {
    /// File to write the output to. If not provided it will be stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let hwinfo = Hwinfo::new().context("failed to run oneshot sampler")?;
    let json = serde_json::to_vec(&hwinfo).context("failed to serialize hwinfo")?;

    if let Some(output) = args.output {
        let mut file = std::fs::File::create(&output)
            .with_context(|| format!("could not create file {}", output.display()))?;

        file.write_all(&json)
            .with_context(|| format!("failed to write to {}", output.display()))?;
    } else {
        std::io::stdout()
            .lock()
            .write_all(&json)
            .context("failed to write to stdout")?;
    }

    Ok(())
}
