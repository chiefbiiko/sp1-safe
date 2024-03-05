//! Prove contract storage in zk (non-doxing) in the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use sp1_ethereum_trie::{
    keccak::{keccak_256, KeccakHasher}, EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub msg_hash: [u8; 32],          // Safe::getMessageHash(msg)
    pub state_root: [u8; 32],        // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32],      // eth_getProof::response.storageHash
    pub account_key: [u8; 32],       // keccak256(address)
    // slot of the signedMessages mapping within Safe storage equals 5
    pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
}

fn bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // https://stackoverflow.com/a/76573243
    unsafe { core::mem::transmute::<[[u8; 32]; 2], [u8; 64]>([a, b]) }
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
    assert_eq!(val, vec![1u8], "msg not signed");

    // verify account proof
    let db = StorageProof::new(inputs.account_proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &_state_root).build();
    let ok = trie.contains(&inputs.account_key).expect("account check failed");
    assert!(ok, "account proof failed");

    // This proven state root must be used to calculate the corresponding 
    // block hash given all other block header components. That resulting block 
    // hash must be checked for authenticity on-chain using the blockhash opcode
    // given the block number at which the storage proof was generated
    sp1_zkvm::io::write_slice(&inputs.state_root);
    // To link this proof to a particular Safe recompute below hash
    // (probably in your dapp's primary circuit) given the account and storage 
    // keys. The account key is a postimage of the Safe address serving as 
    // linking anchor whereas the storage key being a postimage of the message
    // hash serves as the linking blinding.
    // keccak256(storage_key + account_key)
    sp1_zkvm::io::write_slice(&keccak_256(&bytes64(inputs.storage_key, inputs.account_key)))
}
