curl https://rpc.gnosis.gateway.fm \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0","method": "eth_getProof","id": 1,"params": ["0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc", ["0x2e8de2577e7c560a9913fd732cd5ba1f61f809b10c283800da9499091ac562a5"], "latest"]}'

