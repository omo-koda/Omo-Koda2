const BASE64_URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub fn generate_dna_fingerprint(name: &str, birth_timestamp: u64, odu_seed: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(name.as_bytes());
    hasher.update(&birth_timestamp.to_le_bytes());
    hasher.update(odu_seed);

    let mut output = [0u8; 64];
    hasher.finalize_xof().fill(&mut output);

    let fingerprint = encode_base64_url_no_pad(&output);
    debug_assert_eq!(fingerprint.len(), 86);
    fingerprint
}

fn encode_base64_url_no_pad(input: &[u8]) -> String {
    let mut encoded = String::with_capacity((input.len() * 4).div_ceil(3));
    let mut index = 0;

    while index + 3 <= input.len() {
        let chunk = ((input[index] as u32) << 16)
            | ((input[index + 1] as u32) << 8)
            | (input[index + 2] as u32);

        encoded.push(BASE64_URL[((chunk >> 18) & 0x3f) as usize] as char);
        encoded.push(BASE64_URL[((chunk >> 12) & 0x3f) as usize] as char);
        encoded.push(BASE64_URL[((chunk >> 6) & 0x3f) as usize] as char);
        encoded.push(BASE64_URL[(chunk & 0x3f) as usize] as char);
        index += 3;
    }

    match input.len() - index {
        1 => {
            let chunk = (input[index] as u32) << 16;
            encoded.push(BASE64_URL[((chunk >> 18) & 0x3f) as usize] as char);
            encoded.push(BASE64_URL[((chunk >> 12) & 0x3f) as usize] as char);
        }
        2 => {
            let chunk = ((input[index] as u32) << 16) | ((input[index + 1] as u32) << 8);
            encoded.push(BASE64_URL[((chunk >> 18) & 0x3f) as usize] as char);
            encoded.push(BASE64_URL[((chunk >> 12) & 0x3f) as usize] as char);
            encoded.push(BASE64_URL[((chunk >> 6) & 0x3f) as usize] as char);
        }
        _ => {}
    }

    encoded
}
