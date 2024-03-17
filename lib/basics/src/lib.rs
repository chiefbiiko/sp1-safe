use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H256},
    // utils::keccak256,
};
// use sp1_safe_basics::{concat_bytes64, Inputs, SAFE_SIGNED_MESSAGES_SLOT};
use zerocopy::AsBytes;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

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
    // slot of the signedMessages mapping within Safe storage equals 5
    pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
}

pub fn coerce_bytes20(x: Vec<u8>) -> [u8; 20] {
    x.try_into().expect("invalid address")
}

pub fn coerce_bytes32(x: Vec<u8>) -> [u8; 32] {
    x.try_into().expect("invalid hash")
}

pub fn concat_bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    let mut out: [u8; 64] = [0; 64];
    for i in 0..32 {
        out[i] = a[i];
        out[i + 32] = b[i];
    }
    out
}

pub fn lpad_bytes32(x: [u8;20]) -> [u8; 32] {
    core::array::from_fn(|i| if i < 12 { 0u8 } else { x[i-12] })
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