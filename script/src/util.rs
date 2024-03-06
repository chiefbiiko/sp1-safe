use anyhow::Result;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H256},
};
use sp1_safe_primitives::Inputs;

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

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;
    let tx_pool_content = provider.txpool_content().await?;
    
    let inputs = Inputs {
        msg_hash: msg_hash.into(),
        // state_root: 
    }

    Err(())
}