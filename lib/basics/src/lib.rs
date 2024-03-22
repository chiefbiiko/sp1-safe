use ethers::{
    utils,
    providers::{Middleware, Provider},
    types::{Address, H256, Block},
};
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};
use zerocopy::AsBytes;
use rlp::RlpStream;

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub safe: [u8; 20],
    pub msg_hash: [u8; 32],     // Safe::getMessageHash(msg)
    pub state_root: [u8; 32],   // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32], // eth_getProof::response.storageHash
    pub account_key: [u8; 32],  // keccak256(address)
    pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
    pub header_rlp: Vec<u8>,         // RLP-encoded header
}

pub fn coerce_bytes20(x: Vec<u8>) -> [u8; 20] {
    x.try_into().expect("invalid address")
}

pub fn coerce_bytes32(x: Vec<u8>) -> [u8; 32] {
    x.try_into().expect("invalid hash")
}

pub fn concat_bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // https://stackoverflow.com/a/76573243
    unsafe { core::mem::transmute::<[[u8; 32]; 2], [u8; 64]>([a, b]) }
}

pub fn lpad_bytes32(x: [u8; 20]) -> [u8; 32] {
    core::array::from_fn(|i| if i < 12 { 0u8 } else { x[i - 12] })
}

/// Performs a Keccak-256 hash on the given input.
pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut k = Keccak::v256();
    k.update(input.as_ref());
    k.finalize(&mut out);
    out
}

pub async fn fetch_inputs(rpc: &str, safe: Address, msg_hash: H256) -> Inputs {
    let provider = Provider::try_from(rpc).expect("rpc provider failed");

    let account_key = keccak256(&safe);
    let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let latest = provider
        .get_block_number()
        .await
        .expect("fetching latest failed");
    let block = provider
        .get_block(latest)
        .await
        .expect("fetching block failed")
        .expect("no such block");
    let proof = provider
        .get_proof(safe, vec![storage_key.into()], Some(latest.into()))
        .await
        .expect("fetching proof failed");

    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("block.hash {:?}", &block.hash.unwrap());
    let header_rlp =  rlp_encode(&block);
    println!("computed blockhash {:?}", H256(keccak256(&header_rlp)));
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");

    Inputs {
        safe: safe.into(),
        msg_hash: msg_hash.into(),
        state_root: block.state_root.into(),
        storage_root: proof.storage_hash.into(),
        account_key,
        storage_key,
        account_proof: proof
            .account_proof
            .iter()
            .map(|b| b.as_bytes().to_vec())
            .collect(),
        storage_proof: proof.storage_proof[0]
            .proof
            .iter()
            .map(|b| b.as_bytes().to_vec())
            .collect(),
        header_rlp,//WIP
    }
}

// https://ethereum.stackexchange.com/a/67332
// https://github.com/ethereum/go-ethereum/blob/14eb8967be7acc54c5dc9a416151ac45c01251b6/core/types/block.go#L65
// pub hash: Option<H256>,
// block hash should be recomputable with the following header components:
// pub parent_hash: H256,
// pub uncles_hash: H256,
// pub author: Option<H160>,
// pub state_root: H256,
// pub transactions_root: H256,
// pub receipts_root: H256,
// pub logs_bloom: Option<Bloom>,
// pub difficulty: U256,
// pub number: Option<U64>,
// pub gas_limit: U256,
// pub gas_used: U256,
// pub timestamp: U256,
// pub extra_data: Bytes,
// pub mix_hash: Option<H256>,
// pub nonce: Option<H64>,
// pub base_fee_per_gas: Option<U256>,
// pub withdrawals_root: Option<H256>,
// pub blob_gas_used: Option<U256>,
// pub excess_blob_gas: Option<U256>,
// pub parent_beacon_block_root: Option<H256>,
pub fn rlp_encode(block: &Block<H256>) -> Vec<u8> {
    let mut rlp = RlpStream::new();
    rlp.begin_list(20);
    rlp.append(&block.parent_hash);
    rlp.append(&block.uncles_hash);
    rlp.append(&block.author);
    rlp.append(&block.state_root);
    rlp.append(&block.transactions_root);
    rlp.append(&block.receipts_root);
    rlp.append(&block.logs_bloom);
    rlp.append(&block.difficulty);
    rlp.append(&block.number);
    rlp.append(&block.gas_limit);
    rlp.append(&block.gas_used);
    rlp.append(&block.timestamp);
    rlp.append(&block.extra_data.as_bytes().to_vec());
    rlp.append(&block.mix_hash);
    rlp.append(&block.nonce);
    rlp.append(&block.base_fee_per_gas);         // london
    rlp.append(&block.withdrawals_root);         // shanghai
    rlp.append(&block.blob_gas_used);            // cancun
    rlp.append(&block.excess_blob_gas);          // cancun
    rlp.append(&block.parent_beacon_block_root); // cancun
    rlp.out().freeze().into()
}