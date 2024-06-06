use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

// pub struct StorageProof {
//     pub address_hash: String,
//     pub account_proof: Vec<String>,
//     pub storage_key: String,
//     pub storage_proof: Vec<String>,
//     pub storage_key_ptrs: Vec<usize>,
//     pub account_key_ptrs: Vec<usize>,
//     pub enc_block_header: Vec<u8>,
//     pub block_hash: String,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub safe_address: [u8; 20], // Safe address
    pub msg_hash: [u8; 32],     // Custom msg hash
    pub state_root: [u8; 32],   // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32], // eth_getProof::response.storageHash
    // pub state_trie_key: [u8; 32],    // keccak256(safe)
    // pub storage_trie_key: [u8; 32],  // keccak256(msg_hash + uint256(7))
    //NOTE state_trie_key_nibbles == aerius::account_key_nibbles
    pub state_trie_key_nibbles: [u8; 64],
    //NOTE storage_trie_key_nibbles == aerius::key_nibbles
    pub storage_trie_key_nibbles: [u8; 64],
    //NOTE state_trie_key_ptrs == aerius::account_key_ptrs
    pub state_trie_key_ptrs: Vec<usize>,
    //NOTE storage_trie_key_ptrs == aerius::key_ptrs
    pub storage_trie_key_ptrs: Vec<usize>,
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
    pub header_rlp: Vec<u8>,         // RLP-encoded header
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp1SafeParams {
    pub chain_id: u64,
    pub safe_address: String,
    pub message_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp1SafeResult {
    pub chain_id: u64,
    pub safe_address: String,
    pub message_hash: String,
    pub block_number: u64,
    pub block_hash: String,
    pub challenge: String,
    pub proof: String,
}

pub fn concat_bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // https://stackoverflow.com/a/76573243
    unsafe { core::mem::transmute::<[[u8; 32]; 2], [u8; 64]>([a, b]) }
}

pub fn lpad_bytes32(x: [u8; 20]) -> [u8; 32] {
    core::array::from_fn(|i| if i < 12 { 0u8 } else { x[i - 12] })
}

pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut k = Keccak::v256();
    k.update(input.as_ref());
    k.finalize(&mut out);
    out
}
