//! Proves a Safe multisig in zero-knowledge in the sp1 zkVM.
//! Allows proving that a message has been "signed", by a Safe via the
//! SignMessage library without doxing the Safe and/or its owners. Still
//! "signed" hereby means approved by at least $threshold owners of the Safe.
//! The circuit takes a message hash, corresponding EIP1186 storage proof, and
//! state root as inputs, and has two outputs: the proven state root and a
//! hash of the storage and account keys. The output state root must be used to
//! calculate the corresponding block hash which in turn must be checked for
//! authenticity on-chain using the blockhash opcode given the block number at
//! which the storage proof was generated. The latter output hash serves as a
//! challenge point that allows associating given proof to a particular
//! Safe and message by recomputing the hash given the account and storage 
//! keys. The message hash must incorporate a nullifier to guard against 
//! rainbow table precomputations.

#![no_main]
sp1_zkvm::entrypoint!(main);

#[macro_use]
extern crate ff_ce;
use ff_ce::*;

use ethereum_trie::{
    keccak::{keccak_256, KeccakHasher},
    EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256, U256
};
use poseidon_rs::{Fr, Poseidon};
use sp1_safe_basics::{bytes64, Inputs};

pub fn main() {
    let inputs = sp1_zkvm::io::read::<Inputs>();
    let state_root = H256(inputs.state_root);
    let storage_root = H256(inputs.storage_root);
    let storage_trie_key = keccak_256(&inputs.storage_key);

    // Verify storage proof
    let db = StorageProof::new(inputs.storage_proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &storage_root).build();
    let val = trie
        .get(&storage_trie_key)
        .expect("storage trie read failed")
        .expect("target storage node is none");
    // Safe's SignMessageLib marks messages as "signed" with a literal 1
    assert_eq!(val[0], 1u8, "msg not signed");

    // Verify account proof
    let db = StorageProof::new(inputs.account_proof).into_memory_db::<KeccakHasher>();
    let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &state_root).build();
    let ok = trie
        .contains(&inputs.account_key)
        .expect("account check failed");
    assert!(ok, "account proof failed");

    sp1_zkvm::io::write_slice(&inputs.state_root);
    // sp1_zkvm::io::write_slice(&keccak_256(&bytes64(
    //     inputs.storage_key,
    //     inputs.account_key,
    // )));
    // U256::from_big_endian()
    let safe_fr = Fr::from_str("1").expect("Fr from safe address failed");
    // let b2: Fr = Fr::from_str(
    //     "12242166908188651009877250812424843524687801523336557272219921456462821518061",
    // )
    // .unwrap();
    // let mut big_arr: Vec<Fr> = Vec::new();
    // big_arr.push(b1.clone());
    // big_arr.push(b2.clone());
    let poseidon = Poseidon::new();

    // c.bench_function("hash", |b| {
    //     b.iter(|| poseidon.hash(big_arr.clone()).unwrap())
    // });

}
