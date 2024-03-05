//! Prove a storage value in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use sp1_ethereum_trie::{
    keccak::KeccakHasher, EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub root: [u8; 32],
    pub key: [u8; 32],
    pub proof: Vec<Vec<u8>>,
}

pub fn main() {
    // read inputs
    let inputs = sp1_zkvm::io::read::<Inputs>();
    let root = H256(inputs.root);

    // verify storage proof
    let db = StorageProof::new(inputs.proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &root).build();
    let _result = trie.get(&inputs.key).unwrap().unwrap(); //FIXME handle unraps

    // get and output verified state root ??
}
