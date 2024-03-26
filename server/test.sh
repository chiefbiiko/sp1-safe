#!/bin/bash

set -eExuo pipefail

source ../lib/bashert.sh

params="$(cat << EOF
{
    "chain_id": 100,
    "safe_address": "0x38Ba7f4278A1482FA0a7bC8B261a9A673336EDDc",
    "message_hash": "0xa225aed0c0283cef82b24485b8b28fb756fc9ce83d25e5cf799d0c8aa20ce6b7"
}
EOF
)"

test_proving_ok() {
  printf "test_proving_ok\n"

  resp_head=$(mktemp)
  resp_body=$(mktemp)

  curl \
    -sS \
    -D $resp_head \
    http:/localhost:4190/ \
    -d "$params" \
  > $resp_body

  assert_status $resp_head 200

  blocknumber=$(jq -r '.blocknumber' $resp_body)
  blockhash=$(jq -r '.blockhash' $resp_body)
  challenge=$(jq -r '.challenge' $resp_body)
  proof=$(jq -r '.proof' $resp_body)

  assert_gt $blocknumber 33119702
  assert_match $blockhash '^0x[a-f0-9]{64}$'
  assert_match $challenge '^0x[a-f0-9]{64}$'
  assert_equal $proof '0x'
}

test_proving_not_ok() {
  printf "test_proving_ok\n"

  resp_head=$(mktemp)
  resp_body=$(mktemp)
  # replace chain_id to point to non-existing contract storage
  not_ok_params="$(echo "$params" | sed 's/100/11155111/')"

  curl \
    -sS \
    -D $resp_head \
    http:/localhost:4190/ \
    -d "$not_ok_params" \
  > $resp_body

  assert_status $resp_head 500
}

test_proving_ok
test_proving_not_ok