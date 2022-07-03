use anyhow::{Context, Result};
use clap::StructOpt;
use clap_derive::Parser;
use katastone::{record::Record, util::deser_with_windows_encoding};
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = "berlin_infos.dat", parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.input).context("Failed to open file")?;
    // TODO omit bufreader?
    let reader = BufReader::new(f);
    let mut rdr = deser_with_windows_encoding(reader);

    let vec = rdr
        .deserialize::<Record>()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to deserialize")?;
    assert_eq!(vec.len(), 384860);
    Ok(())
}
