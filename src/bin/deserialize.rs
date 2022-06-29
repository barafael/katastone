use anyhow::{Context, Result};
use clap::StructOpt;
use clap_derive::Parser;
use csv::Reader;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use katastone::record::Record;
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

    let vec = rdr.deserialize::<Record>().collect::<Vec<_>>();
    assert_eq!(vec.len(), 384860);
    Ok(())
}

fn deser_with_windows_encoding<R>(reader: R) -> Reader<DecodeReaderBytes<R, Vec<u8>>>
where
    R: std::io::Read,
{
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(reader);

    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(decoder)
}
