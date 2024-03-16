use serde::{Deserialize, Serialize};

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub safe: [u8; 20],
    pub msg_hash: [u8; 32],     // Safe::getMessageHash(msg)
    pub state_root: [u8; 32],   // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32], // eth_getProof::response.storageHash
    pub account_key: [u8; 32],  // keccak256(address)
    // slot of the signedMessages mapping within Safe storage equals 5
    pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
}

pub fn bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    let mut out: [u8; 64] = [0; 64];
    for i in 0..32 {
        out[i] = a[i];
        out[i + 32] = b[i];
    }
    out
}
