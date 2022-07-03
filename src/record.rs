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
    pub(crate) latitude: f64,

    /// Längengrad der geografischen Position
    pub(crate) longitude: f64,

    /// Entfernung vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    pub(crate) distance: f64,

    /// Kurswinkel vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    pub(crate) angle: f64,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::{build_reader, deser_with_windows_encoding, vec_from_reader};
    use anyhow::{Context, Result};

    #[test]
    fn reads_input_with_windows_encoding() -> Result<()> {
        let f = std::fs::File::open("berlin_infos.dat")?;
        let mut rdr = deser_with_windows_encoding(f);

        let vec = rdr
            .deserialize::<Record>()
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to deserialize")?;
        assert_eq!(vec.len(), 384860);
        Ok(())
    }

    #[test]
    fn parses_gontardstraße_9() -> anyhow::Result<()> {
        let entry = "Berlin;Gontardstrasse;9;10178;Mitte;Mitte;Mitte;52.5207635;13.4098665";
        let mut reader = build_reader(entry.as_bytes());
        let vec = vec_from_reader(&mut reader)?;
        assert_eq!(
            vec,
            vec![Record {
                city: "Berlin".into(),
                street: "Gontardstrasse".into(),
                street_number: "9".into(),
                zipcode: "10178".into(),
                urban_unit: "Mitte".into(),
                old_unit: "Mitte".into(),
                district: "Mitte".into(),
                latitude: 52.5207635,
                longitude: 13.4098665,
                distance: 0.0,
                angle: 0.0,
            }]
        );
        Ok(())
    }
}
