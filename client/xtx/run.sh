#!/bin/bash

SIGNER_KEY="${SIGNER_KEY:-$1}"
if [[ -z "$SIGNER_KEY" ]]; then
  echo "missing env var SIGNER_KEY" >&2
  exit 1
fi

set -xEeu

## build the custom justification decoder and standalone circuit
cargo build \
  --manifest-path ./justification-decoder/Cargo.toml \
  --release
cargo build \
  --manifest-path ../../node/standalone/Cargo.toml \
  --release

## killall leftover circuits - if any
set +e
killall circuit-standalone
set -e

## run standalone circuit
cargo run \
  --manifest-path ../../node/standalone/Cargo.toml \
  --release \
  -- \
  --dev \
  --ws-port 9944 \
> /tmp/xtx-circuit.log 2>&1 &
circuit_pid=$!

# await circuit ws rpc available
tail -f /tmp/xtx-circuit.log | sed '/Listening for new connections on 127.0.0.1:9944/ q'

## register rococo gateway on circuit
npm i @polkadot/api @polkadot/types
node ./register_rococo_gateway.js

## run grandpa-ranger
npm start --prefix ../grandpa-ranger &
grandpa_ranger_pid=$!

## run executor
SIGNER_KEY=$SIGNER_KEY npm start --prefix ../executor &
executor_pid=$!

echo -e "circuit pid: $circuit_pid\ngrandpa ranger pid: $grandpa_ranger_pid\nexecutor_pid: $executor_pid"