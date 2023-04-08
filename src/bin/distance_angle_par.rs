use anyhow::Result;
use clap::StructOpt;
use clap_derive::Parser;
use encoding_rs::{EncoderResult, WINDOWS_1252};
use katastone::{
    consts::{FERNSEH_LAMBDA_RAD, FERNSEH_PHI_RAD},
    util::{vec_from_reader, windows_encoding_deserializer},
};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
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
    vec.par_iter_mut()
        .for_each(|r| r.set_angle_direction(FERNSEH_PHI_RAD, FERNSEH_LAMBDA_RAD));
    vec.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });

    let output = File::create(args.output)?;

    let mut writer = BufWriter::new(output);

    let mut encoder = WINDOWS_1252.new_encoder();
    let mut line = Vec::with_capacity(256);
    for r in vec {
        let (a, _b) = encoder.encode_from_utf8_to_vec_without_replacement(
            &format!("{r}\n"),
            &mut line,
            false,
        );
        if !matches!(a, EncoderResult::InputEmpty) {
            eprintln!("Encoder encountered: {a:?}");
        }
        writer.write_all(&line).unwrap();
        line.clear();
    }
    Ok(())
}
