use anyhow::{Context, Result};
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, Block, H256},
};
use rlp::RlpStream;
use sp1_safe_basics::{concat_bytes64, keccak256, Inputs, SAFE_SIGNED_MESSAGES_SLOT};
use zerocopy::AsBytes;

pub async fn fetch_inputs(rpc: &str, safe_address: Address, msg_hash: H256) -> Result<(u64, Inputs)> {
    let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));

    let provider = Provider::try_from(rpc)?;
    let latest = provider.get_block_number().await?;
    let block = provider.get_block(latest).await?.context("no such block")?;
    let proof = provider
        .get_proof(safe_address, vec![storage_key.into()], Some(latest.into()))
        .await?;

    Ok((
        latest.as_u64(),
        Inputs {
            safe_address: safe_address.into(),
            msg_hash: msg_hash.into(),
            header_rlp: rlp_encode_header(&block),
            state_root: block.state_root.into(),
            storage_root: proof.storage_hash.into(),
            state_trie_key: keccak256(&safe_address),
            storage_trie_key: keccak256(&storage_key),
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
        },
    ))
}

// https://ethereum.stackexchange.com/a/67332
// https://github.com/ethereum/go-ethereum/blob/14eb8967be7acc54c5dc9a416151ac45c01251b6/core/types/block.go#L65
pub fn rlp_encode_header(block: &Block<H256>) -> Vec<u8> {
    let mut rlp = RlpStream::new();
    rlp.begin_list(20);
    rlp.append(&block.parent_hash);
    rlp.append(&block.uncles_hash);
    rlp.append(&block.author.expect("author"));
    rlp.append(&block.state_root);
    rlp.append(&block.transactions_root);
    rlp.append(&block.receipts_root);
    rlp.append(&block.logs_bloom.expect("logs_bloom"));
    rlp.append(&block.difficulty);
    rlp.append(&block.number.expect("number"));
    rlp.append(&block.gas_limit);
    rlp.append(&block.gas_used);
    rlp.append(&block.timestamp);
    rlp.append(&block.extra_data.as_bytes().to_vec());
    rlp.append(&block.mix_hash.expect("mix_hash"));
    rlp.append(&block.nonce.expect("nonce"));
    rlp.append(&block.base_fee_per_gas.expect("base_fee_per_gas")); // london
    rlp.append(&block.withdrawals_root.expect("withdrawals_root")); // shanghai
    rlp.append(&block.blob_gas_used.expect("blob_gas_used")); // cancun
    rlp.append(&block.excess_blob_gas.expect("excess_blob_gas")); // cancun
    rlp.append(
        &block
            .parent_beacon_block_root
            .expect("parent_beacon_block_root"),
    ); // cancun
    rlp.out().freeze().into()
}
