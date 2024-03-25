#!/bin/bash

set -eExuo pipefail

params="$(cat << EOF
{
    "chain_id": 100,
    "safe_address": "0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc",
    "message_hash": "0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7"
}
EOF
)"

curl -H "content-type: application/json" -d "$params" http://localhost:4190/