use rlp::Rlp;

use anyhow::Result;
//WIP
// use serde::{Deserialize, Serialize};
// use sha3::{Digest, Keccak256};

// use super::{eth_rpc::ProofResult, get_block_enc_header};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageProof {
    pub address_hash: String,
    pub account_proof: Vec<String>,
    pub storage_key: String,
    pub storage_proof: Vec<String>,
    pub storage_key_ptrs: Vec<usize>,
    pub account_key_ptrs: Vec<usize>,
    pub enc_block_header: Vec<u8>,
    pub block_hash: String,
}
impl StorageProof {
    pub fn from_parsed(result: ProofResult, eth_address: String, bn: String) -> Result<Self> {
        let (enc_block_header, block_hash) = get_block_enc_header(bn)?;

        let ProofResult {
            storage_proof,
            account_proof,
            ..
        } = result;

        let storage_key = &storage_proof[0].key[2..];
        let storage_key_bytes = hex::decode(odd_to_even_hex(storage_key))?;
        let mut hasher = Keccak256::new();
        hasher.update(&storage_key_bytes);
        let key_hash_bytes = hasher.finalize().to_vec();
        let storage_key_hash = hex::encode(key_hash_bytes);

        let account_path_as_str = account_proof
            .iter()
            .map(|element| element.as_str())
            .collect::<Vec<&str>>();

        let account_key_ptrs = get_key_ptrs(account_path_as_str.clone());

        let account_proof = account_path_as_str
            .iter()
            .map(|x| x[2..].to_string())
            .collect::<Vec<String>>();

        let storage_path_as_str = storage_proof[0]
            .proof
            .iter()
            .map(|element| element.as_str())
            .collect::<Vec<&str>>();

        let storage_key_ptrs = get_key_ptrs(storage_path_as_str.clone());

        let storage_proof = storage_path_as_str
            .iter()
            .map(|x| x[2..].to_string())
            .collect::<Vec<String>>();
        let eth_address = &eth_address[2..]; // Skip '0x'
        let address_bytes = hex::decode(odd_to_even_hex(eth_address))?;

        let mut hasher = Keccak256::new();
        hasher.update(&address_bytes);
        let address_hash = hasher.finalize().to_vec();
        let address_hash = hex::encode(address_hash);

        Ok(Self {
            address_hash,
            account_proof,
            storage_key: storage_key_hash,
            storage_key_ptrs,
            storage_proof,
            account_key_ptrs,
            block_hash,
            enc_block_header,
        })
    }
}

pub fn odd_to_even_hex(hex: &str) -> String {
    if hex.len() % 2 == 0 {
        hex.to_string()
    } else {
        format!("0{}", hex)
    }
}

pub fn get_key_ptrs(proof: Vec<&str>) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    let mut key_index = 0;

    for (i, p) in proof.iter().enumerate() {
        let bytes = hex::decode(&p[2..]).expect("Decoding failed");
        let mut in_res: Vec<String> = Vec::new();
        let decoded_list = Rlp::new(&bytes);
        for value in decoded_list.iter() {
            let hex_representation = format!("0x{}", hex::encode(value.data().unwrap()));
            in_res.push(hex_representation);
        }

        if in_res.len() > 2 {
            //branch node
            result.push(key_index);
            key_index += 1;
        } else if i != proof.len() - 1 && in_res.len() == 2 {
            //extension node
            let extension = &in_res[0][2..];
            let bytes = hex::decode(extension).expect("Decoding failed");
            let decoded: String = rlp::decode(&bytes).expect("Decoding failed");
            result.push(key_index);
            key_index += decoded.len();
        } else if i == proof.len() - 1 && in_res.len() == 2 {
            //leaf node
            result.push(key_index);
        }
    }
    result
}