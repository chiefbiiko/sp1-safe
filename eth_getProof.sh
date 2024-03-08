curl https://rpc.gnosis.gateway.fm \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0","method": "eth_getProof","id": 1,"params": ["0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc", ["0x8fc241b7eaf929f4c5b3f5bd01abbdc2cc61368ac3c2cca9a28d5d410d4049d5"], "latest"]}'

