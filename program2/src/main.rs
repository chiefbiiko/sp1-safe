#![no_main]
sp1_zkvm::entrypoint!(main);

// use alloy_primitives::{hex, Keccak256};
use rlp::Rlp;

use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
// use ethereum_trie::{keccak::KeccakHasher, EIP1186Layout, StorageProof, Trie, TrieDBBuilder, H256};
use light_poseidon::{Poseidon, PoseidonHasher};
use sp1_safe_basics::{keccak256, lpad_bytes32, Inputs};

// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct StorageProof {
//     pub address_hash: String,
//     pub account_proof: Vec<String>,
//     pub storage_key: String,
//     pub storage_proof: Vec<String>,
//     pub key_ptrs: Vec<usize>,
//     pub account_key_ptrs: Vec<usize>,
//     pub enc_block_header: Vec<u8>,
//     pub block_hash: String,
// }

pub fn main() {
    // // Read storage key
    // let sp = sp1_zkvm::io::read::<StorageProof>();

    // // Verify storage proof
    // let storage_root = sp1_zkvm::io::read::<String>();
    let inputs = sp1_zkvm::io::read::<Inputs>();

    // let mut current_hash = storage_root.clone();
    let mut current_hash = inputs.storage_root.to_vec();

    let key_ptrs = inputs.storage_trie_key_ptrs;
    let account_key_ptrs = inputs.state_trie_key_ptrs;

    let depth_sp = inputs.storage_proof.len();
    let depth_ap = inputs.account_proof.len();

    let key_nibbles = inputs.storage_trie_key_nibbles;
    //   sp
    //     .storage_key
    //     .chars()
    //     .map(|x| x.to_digit(16).unwrap() as usize)
    //     .collect::<Vec<_>>();
    let account_key_nibbles = inputs.state_trie_key_nibbles;
    //   sp
    //     .address_hash
    //     .chars()
    //     .map(|x| x.to_digit(16).unwrap() as usize)
    //     .collect::<Vec<_>>();

    // for (i, p) in sp.storage_proof.iter().enumerate() {
    for (i, bytes) in inputs.storage_proof.iter().enumerate() {
        // let bytes = hex::decode(&p).expect("Decoding proof failed");

        // let mut hasher = Keccak256::new();
        // hasher.update(&bytes);
        // let res = hasher.finalize();
        let res = keccak256(&bytes);

        // assert_eq!(&hex::encode(res), &current_hash);
        assert_eq!(&res, &current_hash);

        let decoded_list = Rlp::new(&bytes);
        // assert!(decoded_list.is_list());

        if i < depth_sp - 1 {
            let nibble = key_nibbles[key_ptrs[i]];
            // current_hash = hex::encode(
            //     decoded_list.iter().collect::<Vec<_>>()[nibble]
            //         .data()
            //         .unwrap(),
            // );
            //PERF use `pub fn val_at<T>(&self, index: usize) -> Result<T, DecoderError>`
            current_hash = decoded_list.iter().collect::<Vec<_>>()[nibble]
                .data()
                .expect("strg node rlp.data() failed")
                .to_vec();
        } else {
            // verify value
            //PERF use `pub fn val_at<T>(&self, index: usize) -> Result<T, DecoderError>`
            let leaf_node = decoded_list.iter().collect::<Vec<_>>();
            // assert_eq!(leaf_node.len(), 2);
            let value_decoded = Rlp::new(leaf_node[1].data().expect("strg leaf node rlp.data() failed"));
            // assert!(value_decoded.is_data());
            // let value = hex::encode(value_decoded.data().unwrap());
            let value = value_decoded.data().expect("strg leaf rlp.data() failed")[0];

            // sp1_zkvm::io::write(&value);
            // Safe's SignMessageLib marks messages as "signed" with a literal 1
            assert_eq!(value, 1u8, "msg not signed");
        }
    }
    //TODOTODOTODOTODOTODOTODOTODOTODOTODO
    // let mut state_root: String = "".to_string();
    // let mut current_hash: String = "".to_string();
    let mut state_root: [u8; 32] = [0; 32];
    let mut current_hash: [u8; 32] = [0; 32];
    // for (i, p) in sp.account_proof.iter().enumerate() {
    for (i, bytes) in sp.account_proof.iter().enumerate() {
        // let bytes = hex::decode(&p).expect("Decoding proof failed");

        // let mut hasher = Keccak256::new();
        // hasher.update(&bytes);
        // let res = hasher.finalize();
        let res = keccak256(&bytes);

        if i == 0 {
            // state_root = hex::encode(res);
            state_root = res;
        } else {
            // assert_eq!(&hex::encode(res), &current_hash);
            assert_eq!(&res, &current_hash);
        }

        let decoded_list = Rlp::new(&bytes);
        // assert!(decoded_list.is_list());

        if i < depth_ap - 1 {
            let nibble = account_key_nibbles[account_key_ptrs[i]];
            // current_hash = hex::encode(
            //     decoded_list.iter().collect::<Vec<_>>()[nibble]
            //         .data()
            //         .unwrap(),
            // );
            //PERF use `pub fn val_at<T>(&self, index: usize) -> Result<T, DecoderError>`
            current_hash = decoded_list.iter().collect::<Vec<_>>()[nibble]
                .data()
                .expect("acct node rlp.data() failed")
                .into(),
        } else {
            // verify value
            let leaf_node = decoded_list.iter().collect::<Vec<_>>();
            // assert_eq!(leaf_node.len(), 2);
            let value_decoded = Rlp::new(leaf_node[1].data().expect("acct leaf node rlp.data() failed"));
            // assert!(value_decoded.is_list());

            // assert_eq!(
            //     storage_root,
            //     hex::encode(value_decoded.iter().collect::<Vec<_>>()[2].data().unwrap())
            // );
            assert_eq!(
                &inputs.storage_root,
                &value_decoded.iter().collect::<Vec<_>>()[2].data().expect("acct leaf rlp.data() failed").into()
            );
        }
    }

    // let rlp_enc_block_header = Rlp::new(sp.enc_block_header.as_slice());
    // let rlp_state_root = rlp_enc_block_header.at(3).unwrap();
    // let rlp_state_root = rlp_state_root
    //     .data()
    //     .unwrap()
    //     .iter()
    //     .map(|byte| format!("{:02x}", byte))
    //     .collect::<String>();
    // assert_eq!(rlp_state_root, state_root);
    // sp1_zkvm::io::write(&state_root);

    // let mut hasher = Keccak256::new();
    // hasher.update(sp.enc_block_header);
    // let calculated_block_hash = hasher.finalize();
    // assert_eq!(hex::encode(calculated_block_hash), sp.block_hash);
    // sp1_zkvm::io::write(&true);

    // recalc blockhash using header_rlp incl proven state_root
    let mut header_rlp = inputs.header_rlp;
    header_rlp[91..123].copy_from_slice(state_root.as_bytes());
    let blockhash = keccak256(&header_rlp);

    let mut poseidon = Poseidon::<Fr>::new_circom(2).expect("poseidon init failed");
    // _mod_order might reduce fr2 i.e. it has 2 msg_hash preimages aka collision;
    // since the 20-byte Safe address cannot exceed bn254's scalar field _mod_order
    // is always a noop for fr1, i.e. it has strictly 1 Safe address preimage: 
    // no collisions; consequently "cross-account" collisions can never occur
    let fr1 = Fr::from_be_bytes_mod_order(&lpad_bytes32(inputs.safe_address));
    let fr2 = Fr::from_be_bytes_mod_order(&inputs.msg_hash);
    let challenge: [u8; 32] = poseidon
        .hash(&[fr1, fr2])
        .expect("poseidon hash failed")
        .into_bigint()
        .to_bytes_be()
        .try_into()
        .expect("converting field elements to bytes failed");

    sp1_zkvm::io::commit_slice(&blockhash);
    sp1_zkvm::io::commit_slice(&challenge);
}