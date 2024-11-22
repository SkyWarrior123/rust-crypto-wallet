use hex;

pub fn decode_string(input: &str) -> Vec<u8> {
    return hex::decode(input).expect("hecx-decode")
}

pub fn encode_bytes(input: &[u8]) -> String {
    return hex::encode(&input)
}