curl https://rpc.gnosis.gateway.fm \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0","method": "eth_getProof","id": 1,"params": ["0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc", ["0x26cea30422618ab81af2eb015dca1e31cb79a6d5bdecbd8bbddcf53a6d902fc2"], "latest"]}'

