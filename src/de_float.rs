/// Safety: call only with strings containing float literals
pub unsafe fn germanize_float(formatted: &mut String) {
    let mut replaced = formatted.replace('.', ",");

    let bytes = replaced.as_bytes_mut();
    bytes.reverse();

    let mut seen_comma = false;
    let mut counter = 0;
    let mut result = String::with_capacity(bytes.len());
    for i in bytes.iter() {
        result.push(*i as char);

        if !seen_comma {
            if *i == b',' {
                seen_comma = true;
            }
            continue;
        }
        if counter == 2 {
            counter = 0;
            result.push('.');
        } else {
            counter += 1;
        }
    }
    // pop leading dot
    if result.ends_with('.') {
        result.pop();
    }
    let bytes = result.as_bytes_mut();
    bytes.reverse();
    *formatted = result
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
    fn germanizes() {
        let f = 134563435.34534523f64;
        let mut formatted = format!("{f}");
        unsafe { germanize_float(&mut formatted) };
        assert_eq!(formatted, "134.563.435,34534523");
    }
}
