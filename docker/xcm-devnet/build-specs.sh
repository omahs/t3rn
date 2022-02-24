#!/bin/bash

# NOTE:
#  - the docker tags below should stay in sync with those in docker-compose.yml
#  - parachain ids should stay in sync with those in README.md

set -xEeo pipefail

if ! docker inspect polkadot:release-v0.9.13 2>&1 > /dev/null; then
  docker build -t polkadot:release-v0.9.13 -f polkadot.Dockerfile .
fi
if ! docker inspect circuit-collator:latest 2>&1 > /dev/null; then
  docker build -t circuit-collator:latest -f t3rn.Dockerfile ../..
fi
if ! docker inspect parachain-collator:polkadot-v0.9.13 2>&1 > /dev/null; then
  docker build -t parachain-collator:polkadot-v0.9.13 -f pchain.Dockerfile .
fi

mkdir ./{keys,specs}

## gen custom aura keys 4 the 2 parachains

subkey generate --scheme Sr25519 > ./keys/t3rn1.key
subkey generate --scheme Sr25519 > ./keys/t3rn2.key
subkey generate --scheme Sr25519 > ./keys/pchain1.key
subkey generate --scheme Sr25519 > ./keys/pchain2.key

t3rn1_aura=$(grep -oP '(?<=\(SS58\):\s)\S+' ./keys/t3rn1.key)
t3rn2_aura=$(grep -oP '(?<=\(SS58\):\s)\S+' ./keys/t3rn2.key)
pchain1_aura=$(grep -oP '(?<=\(SS58\):\s)\S+' ./keys/pchain1.key)
pchain2_aura=$(grep -oP '(?<=\(SS58\):\s)\S+' ./keys/pchain2.key)

## gen relay chain spec

docker run \
    polkadot:release-v0.9.13 \
    build-spec \
    --chain rococo-local \
    --disable-default-bootnode \
> ./specs/rococo-local.json

sed 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 5000/g' \
    -i ./specs/rococo-local.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    polkadot:release-v0.9.13 \
    build-spec \
    --chain /usr/local/etc/rococo-local.json \
    --disable-default-bootnode \
    --raw \
> ./specs/rococo-local.raw.json

## gen t3rn chain config

docker run circuit-collator:latest build-spec \
    --disable-default-bootnode \
> ./specs/t3rn.json

# rm config fields that would be unprocessable in further steps
sed 's/"forkId": null,//g' -i ./specs/t3rn.json
# set parachain id(s)
sed 's/"paraId": [[:digit:]]\+/"paraId": 3000/g' \
    -i ./specs/t3rn.json
sed 's/"para_id": [[:digit:]]\+/"para_id": 3000/g' \
    -i ./specs/t3rn.json
sed 's/"parachainId": [[:digit:]]\+/"parachainId": 3000/g' \
    -i ./specs/t3rn.json
# set the t3rn1 node aura address
sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$t3rn1_aura/g" \
    -i ./specs/t3rn.json
# set the t3rn2 node aura address
sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$t3rn2_aura/g" \
    -i ./specs/t3rn.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    circuit-collator:latest \
    build-spec \
    --chain /usr/local/etc/t3rn.json \
    --disable-default-bootnode \
    --raw \
> ./specs/t3rn.raw.json

## gen pchain chain config

docker run parachain-collator:latest build-spec \
    --disable-default-bootnode \
> ./specs/pchain.json

# rm config fields that would be unprocessable in further steps
sed 's/"forkId": null,//g' -i ./specs/pchain.json
# set parachain id(s)
sed 's/"paraId": [[:digit:]]\+/"paraId": 4000/g' \
    -i ./specs/pchain.json
sed 's/"para_id": [[:digit:]]\+/"para_id": 4000/g' \
    -i ./specs/pchain.json
sed 's/"parachainId": [[:digit:]]\+/"parachainId": 4000/g' \
    -i ./specs/pchain.json
# set the pchain1 node aura address
sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$pchain1_aura/g" \
    -i ./specs/pchain.json
# set the pchain2 node aura address
sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$pchain2_aura/g" \
    -i ./specs/pchain.json
# rm another unprocessable field
jq 'del(.genesis.runtime.polkadotXcm)' ./specs/pchain.json  > ./specs/_pchain.json
mv ./specs/_pchain.json ./specs/pchain.json

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    build-spec \
    --chain /usr/local/etc/pchain.json \
    --disable-default-bootnode \
    --raw \
> ./specs/pchain.raw.json

## gen parachains' genesis states

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    circuit-collator:latest \
    export-genesis-state \
    --chain /usr/local/etc/t3rn.raw.json \
> ./specs/t3rn.genesis

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    export-genesis-state \
    --chain /usr/local/etc/pchain.raw.json \
> ./specs/pchain.genesis

## gen parachains' genesis wasm

docker run \
    -v "$(pwd)/specs:/usr/local/etc" \
    parachain-collator:polkadot-v0.9.13 \
    export-genesis-wasm \
    --chain /usr/local/etc/pchain.raw.json \
> ./specs/pchain.wasm

docker run circuit-collator:latest export-genesis-wasm \
> ./specs/t3rn.wasm
