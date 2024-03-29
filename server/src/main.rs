#[macro_use]
extern crate rocket;

use anyhow::{bail, Result};
use rocket::{
    data::{Limits, ToByteUnit},
    http::Status,
    request::Request,
    serde::json::{json, Json, Value},
    Config,
};
use sp1_core::{SP1Prover, SP1Stdin};
use sp1_safe_basics::{Inputs, Sp1SafeParams, Sp1SafeResult};
use sp1_safe_fetch::fetch_inputs;
use std::env;
use std::net::Ipv4Addr;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

async fn prove(params: Json<Sp1SafeParams>) -> Result<Value> {
    log::info!("🏈 incoming request");
    let rpc = match params.chain_id {
        100 => env::var("GNOSIS_RPC").unwrap_or("https://rpc.gnosis.gateway.fm".to_string()),
        11155111 => env::var("SEPOLIA_RPC").unwrap_or("https://1rpc.io/sepolia".to_string()),
        _ => bail!("invalid chain_id {}", params.chain_id),
    };
    let safe: [u8; 20] = const_hex::decode_to_array::<&str, 20>(&params.safe_address)?;
    let msg_hash: [u8; 32] = const_hex::decode_to_array::<&str, 32>(&params.message_hash)?;
    log::info!("🕳️ fetching inputs");
    let (anchor, inputs) = fetch_inputs(&rpc, safe.into(), msg_hash.into()).await?;
    let mut stdin = SP1Stdin::new();
    stdin.write::<Inputs>(&inputs);
    log::info!("🎰 zk proving");
    let mut proofwio = SP1Prover::prove(ELF, stdin)?;
    log::info!("🎱 serving results");
    let blockhash = proofwio.stdout.read::<[u8; 32]>();
    let challenge = proofwio.stdout.read::<[u8; 32]>();
    // let proofbin = bincode::serialize(&proofwio.proof)?;
    Ok(json!(Sp1SafeResult {
        chain_id: params.chain_id,
        safe_address: params.safe_address.to_owned(),
        message_hash: params.message_hash.to_owned(),
        blocknumber: anchor,
        blockhash: format!("0x{}", const_hex::encode(blockhash)),
        challenge: format!("0x{}", const_hex::encode(challenge)),
        proof: format!(
            "0x{}",
            "" // const_hex::encode(proofbin)
        ),
    }))
}

#[post("/", data = "<params>")]
async fn index(params: Json<Sp1SafeParams>) -> (Status, Value) {
    match prove(params).await {
        Ok(res) => (Status::Ok, res),
        Err(err) => {
            log::error!("{}", err);
            (
                Status::BadRequest,
                json!({
                    "error": "t(ツ)_/¯ invalid chain id"
                }),
            )
        }
    }
}

#[catch(500)]
fn internal_server_error(_: &Request) -> Value {
    json!({
        "error": "t(ツ)_/¯ invalid storage proof"
    })
}

#[launch]
fn rocket() -> _ {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let config = Config {
        port: 4190,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ip_header: None,
        limits: Limits::default().limit("json", 256.bytes()),
        ..Config::release_default()
    };

    rocket::custom(&config)
        .register("/", catchers![internal_server_error])
        .mount("/", routes![index])
}
