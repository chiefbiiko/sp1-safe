use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

extern crate alloc;

use core::marker::PhantomData;
// use hash_db::Hasher;
use trie_db::TrieLayout;
use primitive_types::H256;
use hash256_std_hasher::Hash256StdHasher;
// use tiny_keccak::{Hasher as TinyKeccakHasher, Keccak};

mod node_codec;

// pub use ethereum_trie::EIP1186Layout;

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub safe_address: [u8; 20],      // Safe address
    pub msg_hash: [u8; 32],          // Custom msg hash
    pub state_root: [u8; 32],        // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32],      // eth_getProof::response.storageHash
    pub state_trie_key: [u8; 32],    // keccak256(safe)
    pub storage_trie_key: [u8; 32],  // keccak256(msg_hash + uint256(7))
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

// /// Trie layout for EIP-1186 state proof nodes.
// #[derive(Default, Clone)]
// pub struct EIP1186Layout<H>(PhantomData<H>);

// impl<H: Hasher<Out = H256>> TrieLayout for EIP1186Layout<H> {
//     const USE_EXTENSION: bool = true;
//     const ALLOW_EMPTY: bool = false;
//     const MAX_INLINE_VALUE: Option<u32> = None;
//     type Hash = H;
//     type Codec = node_codec::RlpNodeCodec<H>;
// }

//WIP

// #![cfg_attr(not(feature = "std"), no_std)]


// #[cfg(test)]
// mod tests;

// pub use primitive_types::H256;
// pub use storage_proof::{MemoryDB, StorageProof};
// pub use trie_db::{Trie, TrieDBBuilder};

/// Trie layout for EIP-1186 state proof nodes.
#[derive(Default, Clone)]
pub struct EIP1186Layout<H>(PhantomData<H>);

impl<H: hash_db::Hasher<Out = H256>> TrieLayout for EIP1186Layout<H> {
    const USE_EXTENSION: bool = true;
    const ALLOW_EMPTY: bool = false;
    const MAX_INLINE_VALUE: Option<u32> = None;
    type Hash = H;
    type Codec = node_codec::RlpNodeCodec<H>;
}

#[derive(Debug)]
pub struct KeccakHasher;

impl hash_db::Hasher for KeccakHasher {
    type Out = H256;
    type StdHasher = Hash256StdHasher;
    const LENGTH: usize = 32;

    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x).into()
    }
}
