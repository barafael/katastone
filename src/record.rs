use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Stadt
    city: String,

    /// Straße
    street: String,

    /// Hausnummer mit Ergänzung (a, b, …)
    street_number: String,

    /// PLZ
    zipcode: String,

    /// Stadtbezirk
    urban_unit: String,

    /// Stadtbezirk vor Reform
    old_unit: String,

    /// Stadtteil
    district: String,

    /// Breitengrad der geografischen Position
    latitude: f64,

    /// Längengrad der geografischen Position
    longitude: f64,

    /// Entfernung vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    distance: f64,

    /// Kurswinkel vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    angle: f64,
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use encoding_rs::WINDOWS_1252;
    use encoding_rs_io::DecodeReaderBytesBuilder;

    #[test]
    fn reads_input_with_windows_encoding() -> Result<()> {
        let f = std::fs::File::open("berlin_infos.dat")?;
        let f = DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(f);

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .from_reader(f);

        let vec = rdr.deserialize::<Record>().collect::<Vec<_>>();
        assert_eq!(vec.len(), 384860);
        Ok(())
    }
}
