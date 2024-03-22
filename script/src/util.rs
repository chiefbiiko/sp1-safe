use ethers::{
    providers::{Middleware, Provider},
    types::{Address, H256, Block},
};
use rlp::RlpStream;
use zerocopy::AsBytes;
use sp1_safe_basics::{SAFE_SIGNED_MESSAGES_SLOT, Inputs, keccak256, concat_bytes64};

pub async fn fetch_inputs(rpc: &str, safe: Address, msg_hash: H256) -> Inputs {
    let provider = Provider::try_from(rpc).expect("rpc provider failed");

    let state_trie_key = keccak256(&safe);
    let storage_key = keccak256(&concat_bytes64(msg_hash.into(), SAFE_SIGNED_MESSAGES_SLOT));
    let storage_trie_key = keccak256(&storage_key);

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

    // println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    println!("actual block.hash {:?}", &block.hash.unwrap());
    let header_rlp =  rlp_encode_header(&block);
    // println!("computed blockhash {:?}", H256(keccak256(&header_rlp)));
    // println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");
    // println!("state_root {:?}", &block.state_root);
    // println!("header_rlp len {:?}", &header_rlp.len());
    // println!("header_rlp {:?}", const_hex::encode(&header_rlp));
    // // println!("index of state_root in header_rlp {:?}", &block.state_root);
    // println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");

    Inputs {
        safe: safe.into(),
        msg_hash: msg_hash.into(),
        state_root: block.state_root.into(),
        storage_root: proof.storage_hash.into(),
        state_trie_key,
        storage_trie_key,
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
    rlp.append(&block.blob_gas_used.expect("blob_gas_used"));       // cancun
    rlp.append(&block.excess_blob_gas.expect("excess_blob_gas"));   // cancun
    rlp.append(&block.parent_beacon_block_root.expect("parent_beacon_block_root")); // cancun
    rlp.out().freeze().into()
}