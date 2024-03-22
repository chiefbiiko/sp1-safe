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

use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use ethereum_trie::{
    keccak::{keccak_256, KeccakHasher},
    EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256,
};
use light_poseidon::{Poseidon, PoseidonHasher};
use sp1_safe_basics::{lpad_bytes32, Inputs};

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

    let mut poseidon = Poseidon::<Fr>::new_circom(2).expect("poseidon init failed");
    // _mod_order might reduce fr2 i.e. it has 2 msg_hash preimages aka collision;
    // since the 20-byte safe address cannot exceed bn254's scalar field _mod_order 
    // is always a noop for fr1 - fr1 has strictly 1 safe preimage: no collisions;
    // consequently "cross-account" collisions can never occur
    let fr1 = Fr::from_be_bytes_mod_order(&lpad_bytes32(inputs.safe));
    let fr2 = Fr::from_be_bytes_mod_order(&inputs.msg_hash);
    let challenge: [u8; 32] = poseidon
        .hash(&[fr1, fr2])
        .expect("poseidon hash failed")
        .into_bigint()
        .to_bytes_be()
        .try_into()
        .expect("converting field elements to bytes failed");

    sp1_zkvm::io::write_slice(&inputs.state_root);
    sp1_zkvm::io::write_slice(&challenge);
}
