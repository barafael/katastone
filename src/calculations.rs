use crate::{consts::R, record::Record};

impl Record {
    /// Requires `PHI` and `LAMBDA` to be in radians.
    pub fn set_angle_direction(&mut self, phi_a: f64, lambda_a: f64) {
        let phi = self.latitude.to_radians();
        let lambda = self.longitude.to_radians();
        let zeta = zeta(phi_a, phi, lambda_a, lambda);

        self.distance = R * zeta;

        let mut alpha = alpha(phi_a, phi, zeta).to_degrees();
        let mut beta = beta(phi_a, phi, zeta).to_degrees();

        compensate_direction(&mut alpha, &mut beta, phi_a, phi, lambda_a, lambda);
        self.angle = if is_eastward(lambda_a, lambda) {
            alpha
        } else {
            beta
        };
    }
}

#[inline]
fn zeta(phi_a: f64, phi_b: f64, lambda_a: f64, lambda_b: f64) -> f64 {
    f64::acos(phi_a.sin() * phi_b.sin() + phi_a.cos() * phi_b.cos() * f64::cos(lambda_b - lambda_a))
}

#[inline]
fn alpha(phi_a: f64, phi_b: f64, zeta: f64) -> f64 {
    f64::acos((phi_b.sin() - phi_a.sin() * zeta.cos()) / (phi_a.cos() * zeta.sin()))
}

#[inline]
fn beta(phi_a: f64, phi_b: f64, zeta: f64) -> f64 {
    f64::acos((phi_a.sin() - phi_b.sin() * zeta.cos()) / (phi_b.cos() * zeta.sin()))
}

#[inline]
fn is_eastward(lambda_a: f64, lambda_b: f64) -> bool {
    lambda_a <= lambda_b
}

#[inline]
fn compensate_direction(
    alpha: &mut f64,
    beta: &mut f64,
    phi_a: f64,
    phi_b: f64,
    lambda_a: f64,
    lambda_b: f64,
) {
    if is_eastward(lambda_a, lambda_b) {
        if phi_a < 0.0 {
            *alpha = 180.0 - *alpha;
        }
        if phi_b >= 0.0 {
            *beta = 180.0 - *beta;
        }
    } else {
        if phi_a >= 0.0 {
            *alpha = 360.0 - *alpha;
        } else {
            *alpha += 180.0;
        }
        if phi_b >= 0.0 {
            *beta += 180.0;
        } else {
            *beta = 360.0 - *beta;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        consts::{FERNSEH_LAMBDA, FERNSEH_LAMBDA_RAD, FERNSEH_PHI, FERNSEH_PHI_RAD},
        util::{build_reader, vec_from_reader},
    };

    #[test]
    fn calculates_gontardstraße_9() {
        let latitude = 52.5207635f64;
        let longitude = 13.4098665f64;

        let zeta = zeta(
            FERNSEH_PHI.to_radians(),
            latitude.to_radians(),
            FERNSEH_LAMBDA.to_radians(),
            longitude.to_radians(),
        );

        let distance = zeta * R;
        assert_eq!(distance as u64, 28);

        let mut alpha = alpha(FERNSEH_PHI.to_radians(), latitude.to_radians(), zeta).to_degrees();
        let mut beta = beta(FERNSEH_PHI.to_radians(), latitude.to_radians(), zeta).to_degrees();

        compensate_direction(
            &mut alpha,
            &mut beta,
            FERNSEH_PHI,
            latitude,
            FERNSEH_LAMBDA,
            longitude,
        );
        assert!(is_eastward(FERNSEH_LAMBDA, longitude));
        assert_eq!(alpha as u64, 98);
    }

    #[test]
    fn sets_gontardstraße_9() {
        let latitude = 52.5207635f64;
        let longitude = 13.4098665f64;
        let mut record = Record {
            city: "Berlin".into(),
            street: "Gontard".into(),
            street_number: "9".into(),
            zipcode: "12456".into(),
            urban_unit: "unit".into(),
            old_unit: "unit".into(),
            district: "district".into(),
            latitude,
            longitude,
            distance: 0.0,
            angle: 0.0,
        };
        record.set_angle_direction(FERNSEH_PHI_RAD, FERNSEH_LAMBDA_RAD);
        assert_eq!(record.distance as u64, 28);
        assert_eq!(record.angle as u64, 98);
    }

    #[test]
    fn sets_karl_liebknecht() -> anyhow::Result<()> {
        let entry =
            "Berlin;Karl-Liebknecht-Stra�e;15;10178;Mitte;Mitte;Mitte;52.5229655;13.4099882";
        let mut reader = build_reader(entry.as_bytes());
        let record = &mut vec_from_reader(&mut reader)?[0];
        record.set_angle_direction(FERNSEH_PHI_RAD, FERNSEH_LAMBDA_RAD);
        assert_eq!(record.distance as u64, 243);
        assert_eq!(record.angle as u64, 8);
        Ok(())
    }
}
