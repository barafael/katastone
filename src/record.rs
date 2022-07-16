use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Stadt
    pub city: String,

    /// Straße
    pub street: String,

    /// Hausnummer mit Ergänzung (a, b, …)
    pub street_number: String,

    /// PLZ
    pub zipcode: String,

    /// Stadtbezirk
    pub urban_unit: String,

    /// Stadtbezirk vor Reform
    pub old_unit: String,

    /// Stadtteil
    pub district: String,

    /// Breitengrad der geografischen Position
    pub latitude: f64,

    /// Längengrad der geografischen Position
    pub longitude: f64,

    /// Entfernung vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    pub distance: f64,

    /// Kurswinkel vom Referenzobjekt zu den Koordinaten der Adresse
    #[serde(default, skip_serializing)]
    pub angle: f64,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let district = format!("{},", self.district);
        write!(
            f,
            "{} {} - {district:75}{} {} = {}m / {}°",
            self.zipcode, self.city, self.street, self.street_number, self.distance, self.angle
        )
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.city.partial_cmp(&other.city) {
            Some(Ordering::Equal) => match self.urban_unit.partial_cmp(&other.urban_unit) {
                Some(Ordering::Equal) => match self.district.partial_cmp(&other.district) {
                    Some(Ordering::Equal) => match self.zipcode.partial_cmp(&other.zipcode) {
                        Some(Ordering::Equal) => match self.street.partial_cmp(&other.street) {
                            Some(Ordering::Equal) => {
                                match self.street_number.partial_cmp(&other.street_number) {
                                    Some(Ordering::Equal) => Some(Ordering::Equal),
                                    ord => ord,
                                }
                            }
                            ord => ord,
                        },
                        ord => ord,
                    },
                    ord => ord,
                },
                ord => ord,
            },
            ord => ord,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::{build_reader, vec_from_reader, windows_encoding_deserializer};
    use anyhow::{Context, Result};
    use std::fs::File;

    #[test]
    fn reads_input_with_windows_encoding() -> Result<()> {
        let f = File::open("input/berlin_infos.dat")?;
        let mut rdr = windows_encoding_deserializer(f);

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

    #[test]
    fn displays() -> anyhow::Result<()> {
        let record = Record {
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
        };
        let str = format!("{}", record);
        assert_eq!(
            str,
            r#"10178 Berlin - Mitte,                                                                     Gontardstrasse 9 = 0m / 0°"#
        );

        Ok(())
    }
}
