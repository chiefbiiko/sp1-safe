use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H256},
    utils::keccak256,
};
use sp1_safe_basics::{bytes64, Inputs, SAFE_SIGNED_MESSAGES_SLOT};
use zerocopy::AsBytes;

pub async fn fetch_inputs(rpc: &str, safe: Address, msg_hash: H256) -> Inputs {
    let provider = Provider::try_from(rpc).expect("rpc provider failed");

    let account_key = keccak256(&safe);
    let storage_key = keccak256(bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let latest = provider
        .get_block_number()
        .await
        .expect("fetching latest failed");
    let header = provider
        .get_block(latest)
        .await
        .expect("fetching block failed")
        .expect("no such block");
    let proof = provider
        .get_proof(safe, vec![storage_key.into()], Some(latest.into()))
        .await
        .expect("fetching proof failed");

    Inputs {
        safe: safe.into(),
        msg_hash: msg_hash.into(),
        state_root: header.state_root.into(),
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
    }
}

pub fn bytes20(x: Vec<u8>) -> [u8; 20] {
    x.try_into().expect("invalid address")
}

pub fn bytes32(x: Vec<u8>) -> [u8; 32] {
    x.try_into().expect("invalid hash")
}
