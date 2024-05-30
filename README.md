# sp1-safe

[![ci](https://github.com/chiefbiiko/sp1-safe/workflows/ci/badge.svg)](https://github.com/chiefbiiko/sp1-safe/actions/workflows/ci.yml) [![release](https://img.shields.io/github/v/release/chiefbiiko/sp1-safe?include_prereleases)](https://github.com/chiefbiiko/sp1-safe/releases/latest)

Prove a Safe multisig over a message in zk

```
# current program vkey hash
342db4f26628504876548f513b012d5f57559851150e4196197b8d05660fc28f
```

## Commands

Build the RISC-V ELF binary:

```sh
cd ./program
cargo prove build
```

---

Propose a Safe message multisig:

```sh
RPC=https://yorpc.io SAFE=0x249....3423 PRIVATE_KEY=0x... MSG=halloc node ./lib/auxiliary/proposeSignMsg.js
```

Then confirm the Safe transaction with the required number of signers. Once executed you can proceed running the `script` or `server` binarires with the associated message hash...

---

Obtain a Safe message hash:

```sh
RPC=https://yorpc.io SAFE=0x249....3423 MSG=halloc node ./lib/auxiliary/msgHash.js
```

---

Run the example script:

```sh
cd ./script

SAFE=0x38Ba7...336EDDc \
MSG_HASH=0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7 \
RUST_LOG=info \
RUST_BACKTRACE=full \
  time cargo run --release
```

---

Build and run the server:

```sh
cargo build --manifest-path ./server/Cargo.toml --release
./server/target/release/sp1-safe-server
```

---

Fetch a prebuilt `sp1-safe-server` binary and run it as a systemd service:
<!-- https://0pointer.net/blog/dynamic-users-with-systemd.html -->
```sh
VERSION=v0.1.0
case "$(uname -a)" in
  Linux*)  target=x86_64-unknown-linux-gnu  ;;
  Darwin*) target=x86_64-apple-darwin ;;
esac
temp=$(mktemp)
curl -sSfL https://github.com/chiefbiiko/sp1-safe/releases/download/$VERSION/sp1-safe-server-$VERSION-$target.gz | gunzip > $temp
chmod +x $temp
sudo cp $temp /usr/local/bin/sp1-safe-server
curl -sSfL https://raw.githubusercontent.com/chiefbiiko/sp1-safe/main/server/sp1-safe-server.service | sudo tee /etc/systemd/system/sp1-safe-server.service
systemctl daemon-reload
systemctl start sp1-safe-server.service
```

## Endpoints

### `POST /proof`

#### Request

```json
{
  "chain_id": 11155111,
  "safe_address": "0x...",
  "message_hash": "0x..."
}
```

#### Response

`200`

```json
{
  "chain_id": 11155111,
  "safe_address": "0x...",
  "message_hash": "0x...",
  "block_number": 34234234,
  "block_hash": "0x...",
  "challenge": "0x...",
  "proof": "0x..."
}
```

---

### `GET /status`

#### Response

`200`

```json
{ "status": "ok" }
```