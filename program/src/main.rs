//! Prove contract storage in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use sp1_ethereum_trie::{
    keccak::KeccakHasher, EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub expected_value: Vec<u8>,
    pub block_number: u32,
    pub state_root: [u8; 32],
    pub storage_root: [u8; 32],
    pub account_key: [u8; 32],
    pub storage_key: [u8; 32],
    pub account_proof: Vec<Vec<u8>>,
    pub storage_proof: Vec<Vec<u8>>,
}

pub fn main() {
    // read inputs
    let inputs = sp1_zkvm::io::read::<Inputs>();
    let _state_root = H256(inputs.state_root);
    let _storage_root = H256(inputs.storage_root);

    // verify storage proof
    let db = StorageProof::new(inputs.storage_proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &_storage_root).build();
    let val = trie.get(&inputs.storage_key).expect("storage trie read failed").expect("target storage node is none");
    assert_eq!(val, inputs.expected_value, "");

    // verify account proof
    let db = StorageProof::new(inputs.account_proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &_state_root).build();
    let ok = trie.contains(&inputs.account_key).expect("account check failed");
    assert!(ok, "account proof failed");

    // claimed block number and proven state root
    sp1_zkvm::io::write::<u32>(&inputs.block_number);
    sp1_zkvm::io::write_slice(&inputs.state_root);
}
