use core::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub msg_hash: [u8; 32],     // Safe::getMessageHash(msg)
    pub state_root: [u8; 32],   // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32], // eth_getProof::response.storageHash
    pub account_key: [u8; 32],  // keccak256(address)
    // slot of the signedMessages mapping within Safe storage equals 5
    pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
}