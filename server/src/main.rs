#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use sp1_core::{SP1Prover, SP1Stdin};
use sp1_safe_basics::{Inputs, Sp1SafeParams, Sp1SafeResult};
use sp1_safe_fetch::fetch_inputs;
use std::env;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[post("/", data = "<params>")]
async fn index(params: Json<Sp1SafeParams>) -> Value {
    let rpc = match params.chain_id {
        100 => env::var("GNOSIS_RPC").unwrap_or("https://rpc.gnosis.gateway.fm".to_string()),
        11155111 => env::var("SEPOLIA_RPC").unwrap_or("https://rpc.sepolia.dev".to_string()),
        _ => panic!("invalid chain_id"),
    };
    let safe: [u8; 20] =
        const_hex::decode_to_array::<&str, 20>(&params.safe_address).expect("safe");
    let msg_hash: [u8; 32] =
        const_hex::decode_to_array::<&str, 32>(&params.message_hash).expect("msg_hash");

    let (anchor, inputs) = fetch_inputs(&rpc, safe.into(), msg_hash.into())
        .await
        .expect("fetch");
    let mut stdin = SP1Stdin::new();
    stdin.write::<Inputs>(&inputs);

    let mut proofwio = SP1Prover::prove(ELF, stdin).expect("prove");

    let blockhash = proofwio.stdout.read::<[u8; 32]>();
    let challenge = proofwio.stdout.read::<[u8; 32]>();

    json!(Sp1SafeResult {
        chain_id: params.chain_id,
        safe_address: params.safe_address.to_owned(),
        message_hash: params.message_hash.to_owned(),
        blocknumber: anchor,
        blockhash: format!("0x{}", const_hex::encode(blockhash)),
        challenge: format!("0x{}", const_hex::encode(challenge)),
        proof: format!(
            "0x{}",
            "" // const_hex::encode(bincode::serialize(&proofwio.proof).expect("bincode"))
        ),
    })
}

#[catch(default)]
fn catch_all() -> &'static str {
    "t(ツ)_/¯"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catch_all])
        .mount("/", routes![index])
}
