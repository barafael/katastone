use anyhow::Context;
use csv::Reader;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

use crate::record::Record;

pub fn windows_encoding_deserializer<R>(reader: R) -> Reader<DecodeReaderBytes<R, Vec<u8>>>
where
    R: std::io::Read,
{
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(reader);

    build_reader(decoder)
}

pub fn build_reader<R>(reader: R) -> Reader<R>
where
    R: std::io::Read,
{
    csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(reader)
}

pub fn vec_from_reader<R>(reader: &mut Reader<R>) -> anyhow::Result<Vec<Record>>
where
    R: std::io::Read,
{
    reader
        .deserialize()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to deserialize")
}
