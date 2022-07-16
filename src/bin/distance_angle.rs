use anyhow::Result;
use clap::StructOpt;
use clap_derive::Parser;
use katastone::{
    consts::{FERNSEH_LAMBDA_RAD, FERNSEH_PHI_RAD},
    util::{vec_from_reader, windows_encoding_deserializer},
};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufWriter, Write},
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
        default_value = "output/berlin_infos_distance_angle.out",
        parse(from_os_str)
    )]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let f = File::open(args.input)?;
    let mut rdr = windows_encoding_deserializer(f);

    // TODO try with iterators instead of vec.
    let mut vec = vec_from_reader(&mut rdr)?;
    vec.iter_mut()
        .for_each(|r| r.set_angle_direction(FERNSEH_PHI_RAD, FERNSEH_LAMBDA_RAD));
    vec.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });

    let output = File::create(args.output)?;
    let mut writer = BufWriter::new(output);
    for r in vec {
        writer.write_all(format!("{r}\n").as_bytes()).unwrap();
    }
    Ok(())
}
