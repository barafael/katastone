use anyhow::{Context, Result};
use clap::StructOpt;
use clap_derive::Parser;
use katastone::{record::Record, util::windows_encoding_deserializer};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        short,
        long,
        default_value = "input/berlin_infos.dat",
        parse(from_os_str)
    )]
    input: PathBuf,

    #[clap(
        short,
        long,
        default_value = "output/deserialized.out",
        parse(from_os_str)
    )]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.input).context("Failed to open file")?;
    // TODO omit bufreader?
    let reader = BufReader::new(f);
    let mut rdr = windows_encoding_deserializer(reader);

    let vec = rdr
        .deserialize::<Record>()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to deserialize")?;
    assert_eq!(vec.len(), 384860);

    let output = File::create(args.output)?;
    let mut writer = BufWriter::new(output);
    for r in vec {
        writer.write_all(format!("{r}\n").as_bytes()).unwrap();
    }
    Ok(())
}
