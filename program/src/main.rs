//! Prove a storage value in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

// use alloy_rlp::{encode, encode_list, Decodable, Encodable, Error, Header};
use serde::{Deserialize, Serialize};
use sp1_ethereum_trie::{
    keccak::KeccakHasher,
    EIP1186Layout, StorageProof,
};
use trie_db::{Trie, TrieDBBuilder};
// use primitive_types::{H256, U256};
// use rlp::{Decodable, Rlp};
// use rlp_derive::RlpDecodable;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub root: String,
    pub key: String,
    pub proof: Vec<String>,
}

pub fn main() {

    // read inputs
    let inputs = sp1_zkvm::io::read::<Inputs>();



    // verify storage proof
    // source: https://medium.com/@chiqing/eip-1186-explained-the-standard-for-getting-account-proof-444bc1e12e03
    let db = StorageProof::new(inputs.proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &inputs.root).build();
    let result = trie.get(&inputs.key).unwrap().unwrap(); //FIXME handle unraps
    
    // get and output verified state root ??

}
