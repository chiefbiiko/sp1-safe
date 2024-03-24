use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use anyhow::{anyhow, Result};
// use serde::{Deserialize, Serialize};
use sp1_core::{SP1Prover, SP1Stdin};
use sp1_safe_basics::{coerce_bytes20, coerce_bytes32, Inputs, Sp1SafeParams, Sp1SafeResult};
use sp1_safe_fetch::fetch_inputs;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

const SEPOLIA_RPC: &str = "https://rpc.sepolia.dev";
const GNOSIS_RPC: &str = "https://rpc.gnosis.gateway.fm";

async fn prove(body: web::Bytes) -> Result<HttpResponse> {
    let params = serde_json::from_slice::<Sp1SafeParams>(&body)?;
    let rpc = match params.chain_id {
        100 => GNOSIS_RPC,
        11155111 => SEPOLIA_RPC,
        _ => "",
    };
    let safe: [u8; 20] = const_hex::decode_to_array::<&str, 20>(&params.safe_address)?;
    let msg_hash: [u8; 32] = const_hex::decode_to_array::<&str, 32>(&params.message_hash)?;

    let (anchor, inputs) = fetch_inputs(&rpc, safe.into(), msg_hash.into()).await?;
    let mut stdin = SP1Stdin::new();
    stdin.write::<Inputs>(&inputs);

    let mut proofwio = SP1Prover::prove(ELF, stdin)?;

    let blockhash = proofwio.stdout.read::<[u8; 32]>();
    let challenge = proofwio.stdout.read::<[u8; 32]>();

    let res = Sp1SafeResult {
        safe_address: params.safe_address,
        message_hash: params.message_hash,
        blocknumber: anchor,
        blockhash: format!("0x{}", const_hex::encode(blockhash)),
        challenge: format!("0x{}", const_hex::encode(challenge)),
        proof: format!("0x{}", const_hex::encode(bincode::serialize(&proofwio.proof)?)),
    };

    Ok(HttpResponse::Ok().json(res)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(prove)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test};

    use super::*;

    #[actix_web::test]
    async fn test_index() {
        let app =
            test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
                .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(MyObj {
                chain_id: 100,
                safe_address: "0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc".to_owned(),
                message_hash: "0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7"
                    .to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert!(body_bytes.len() > 0);
    }
}
