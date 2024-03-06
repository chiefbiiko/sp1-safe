use anyhow::Result;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H256}, utils::keccak256,
};
use sp1_safe_basics::{bytes64, Inputs, SAFE_SIGNED_MESSAGES_SLOT};
use zerocopy::AsBytes;

pub async fn fetch_inputs(safe: Address, msg_hash: H256) -> Result<Inputs> {
    let provider = Provider::try_from("https://rpc.gnosis.gateway.fm")?;

    let account_key = keccak256(&safe);
    let storage_key = keccak256(bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let latest = provider.get_block_number().await?;
    let header = provider.get_block(latest).await?.expect("no such block");
    let proof = provider.get_proof(safe, vec![storage_key.into()], Some(latest.into())).await?;

    let inputs = Inputs {
        msg_hash: msg_hash.into(),
        state_root: header.state_root.into(),
        storage_root: proof.storage_hash.into(),
        account_key,
        storage_key,
        account_proof: proof.account_proof.iter().map(|b| b.as_bytes().to_vec()).collect(),
        storage_proof: proof.storage_proof[0].proof.iter().map(|b| b.as_bytes().to_vec()).collect(),
    };

    Ok(inputs)
}