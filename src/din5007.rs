fn utf8_umlauts_to_ascii(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 2);
    let mut i = 0;
    while i < input.len() - 1 {
        if let Some(next_i) = memchr::memchr(0xc3, &input[..input.len() - 1]) {
            if let Some(replacement) = match input[next_i + 1] {
                0xa4 => Some((b'a', b'e')), // ae
                0xb6 => Some((b'o', b'e')), // oe
                0xbc => Some((b'u', b'e')), // ue
                0x84 => Some((b'A', b'e')), // Ae
                0x96 => Some((b'O', b'e')), // Oe
                0x9c => Some((b'U', b'e')), // Ue
                0x9f => Some((b's', b's')), // ss
                _ => None,
            } {
                output.push(replacement.0);
                output.push(replacement.1);
                i = next_i + 1;
            } else {
                output.push(input[next_i + 1]);
                i = next_i + 2;
            }
        } else {
            break;
        }
    }
    output
}
