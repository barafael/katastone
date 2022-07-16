use thousands::Separable;

/// Safety: call only with strings containing float literals
pub unsafe fn germanize_float(formatted: &mut String) {
    formatted.as_bytes_mut().iter_mut().for_each(|s| {
        if *s == b'.' {
            *s = b','
        }
    });
    *formatted = formatted.separate_with_dots();
}

#[repr(transparent)]
pub struct DeFloat(f64);

impl std::fmt::Display for DeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = ryu::Buffer::new();
        let printed = buffer.format(self.0);
        f.write_str(printed)
    }
}

#[cfg(test)]
mod test {
    use crate::de_float::DeFloat;

    use super::germanize_float;

    #[test]
    fn formats_float() {
        let f = DeFloat(1.234);
        assert_eq!(format!("{f}"), "1.234");
    }

    #[test]
    fn formats() -> anyhow::Result<()> {
        use thousands::Separable;
        assert_eq!(15575737.683345.separate_with_dots(), "15.575.737.683345");
        Ok(())
    }

    #[test]
    fn germanizes() {
        let f = 134563435.34534523f64;
        let mut formatted = format!("{f}");
        unsafe { germanize_float(&mut formatted) };
        assert_eq!(formatted, "134.563.435,34534523");
    }
}
