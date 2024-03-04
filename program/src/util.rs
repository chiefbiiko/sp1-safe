pub fn to_32_bytes(v: Vec<u8>) -> [u8; 32] {
    v.try_into().expect("vector does not have 32 bytes")
}
