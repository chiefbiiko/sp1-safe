use anyhow::Result;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, Bytes, H256}, utils::keccak256,
};
use sp1_safe_basics::{bytes64, Inputs, SAFE_SIGNED_MESSAGES_SLOT};

// pub struct Inputs {
//     pub msg_hash: [u8; 32],     // Safe::getMessageHash(msg)
//     pub state_root: [u8; 32],   // eth_getBlockBy*::response.stateRoot
//     pub storage_root: [u8; 32], // eth_getProof::response.storageHash
//     pub account_key: [u8; 32],  // keccak256(address)
//     // slot of the signedMessages mapping within Safe storage equals 5
//     pub storage_key: [u8; 32],       // keccak256(msg_hash + uint256(5))
//     pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
//     pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
// }

pub async fn fetch_inputs(safe: Address, msg_hash: H256) -> Result<Inputs> {
    let provider = Provider::try_from("https://rpc.gnosis.gateway.fm")?;

    let account_key = keccak256(&safe);
    let storage_key = keccak256(bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    // let chain_id = provider.get_chainid().await?;
    // let block_number = provider.get_block_number().await?;
    // let tx_pool_content = provider.txpool_content().await?;
    let latest = provider.get_block_number().await?;
    let header = provider.get_block(latest).await?.expect("no such block");
    let proof = provider.get_proof(safe, vec![storage_key.into()], Some(latest.into())).await?;
    // .as_bytes()
    let inputs = Inputs {
        msg_hash: msg_hash.into(),
        state_root: header.state_root.into(),
        storage_root: proof.storage_hash.into(),
        account_key,
        storage_key,
        account_proof: proof.account_proof.iter().map(|&bytes| bytes.as_bytes()).collect() ,
        storage_proof: proof.storage_proof,
    }

    Err(())
}