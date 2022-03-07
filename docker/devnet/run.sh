#!/bin/bash

set -xEeo pipefail

build_docker_images() {
  # NOTE: docker tags should stay in sync with those in docker-compose.yml
  if ! docker inspect polkadot:release-v0.9.13 > /dev/null; then
    docker build -t polkadot:release-v0.9.13 -f polkadot.Dockerfile .
  fi
  if ! docker inspect circuit-collator:lean > /dev/null; then
    docker build -t circuit-collator:lean -f t3rn.Dockerfile ../..
  fi
  if ! docker inspect parachain-collator:polkadot-v0.9.13 > /dev/null; then
    docker build -t parachain-collator:polkadot-v0.9.13 -f pchain.Dockerfile .
  fi
}

keygen() {
  ## gen custom node keys 4 the 2 parachains
  subkey generate --scheme Sr25519 > ./specs/t3rn1.key
  subkey generate --scheme Sr25519 > ./specs/t3rn2.key
  subkey generate --scheme Sr25519 > ./specs/pchain1.key
  subkey generate --scheme Sr25519 > ./specs/pchain2.key
}

build_relay_chain_spec() {
  docker run \
      polkadot:release-v0.9.13 \
      build-spec \
      --chain rococo-local \
      --disable-default-bootnode \
  > ./specs/rococo-local.json
  sed 's/"nextFreeParaId": [[:digit:]]\+/"nextFreeParaId": 3000/g' \
      -i ./specs/rococo-local.json
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      polkadot:release-v0.9.13 \
      build-spec \
      --chain /usr/local/etc/rococo-local.json \
      --disable-default-bootnode \
      --raw \
  > ./specs/rococo-local.raw.json
}

build_para_chain_specs() {
  # NOTE: included parachain ids should stay in sync with those in README.md
  t3rn1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/t3rn1.key)
  t3rn2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/t3rn2.key)
  pchain1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/pchain1.key)
  pchain2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/pchain2.key)
  ## gen t3rn chain spec
  docker run circuit-collator:lean build-spec \
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
  # set the t3rn1 node address
  sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$t3rn1_adrs/g" \
      -i ./specs/t3rn.json
  # set the t3rn2 node address
  sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$t3rn2_adrs/g" \
      -i ./specs/t3rn.json
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      circuit-collator:lean \
      build-spec \
      --chain /usr/local/etc/t3rn.json \
      --disable-default-bootnode \
      --raw \
  > ./specs/t3rn.raw.json
  ## gen pchain chain spec
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
  # set the pchain1 node address
  sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$pchain1_adrs/g" \
      -i ./specs/pchain.json
  # set the pchain2 node address
  sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$pchain2_adrs/g" \
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
}

build_para_genesis_states() {
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      circuit-collator:lean \
      export-genesis-state \
      --chain /usr/local/etc/t3rn.raw.json \
  > ./specs/t3rn.genesis
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      parachain-collator:polkadot-v0.9.13 \
      export-genesis-state \
      --chain /usr/local/etc/pchain.raw.json \
  > ./specs/pchain.genesis
}

build_para_wasm_runtimes() {
  docker run \
      -v "$(pwd)/specs:/usr/local/etc" \
      parachain-collator:polkadot-v0.9.13 \
      export-genesis-wasm \
      --chain /usr/local/etc/pchain.raw.json \
  > ./specs/pchain.wasm
  docker run circuit-collator:lean export-genesis-wasm \
  > ./specs/t3rn.wasm
}

set_keys() {
  t3rn1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/t3rn1.key | xargs)"
  t3rn2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/t3rn2.key | xargs)"
  pchain1_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/pchain1.key | xargs)"
  pchain2_phrase="$(grep -oP '(?<=phrase:)[^\n]+' ./specs/pchain2.key | xargs)"
  pchain1_adrs="$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/pchain1.key)"
  pchain2_adrs="$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./specs/pchain2.key)"

  docker exec \
    -u t3rn \
    t3rn1 \
    circuit-collator \
    key \
    insert \
    --base-path /t3rn/data \
    --chain /t3rn/t3rn.raw.json \
    --scheme Sr25519 \
    --suri "$t3rn1_phrase" \
    --key-type aura

  docker exec \
    -u t3rn \
    t3rn2 \
    circuit-collator \
    key \
    insert \
    --base-path /t3rn/data \
    --chain /t3rn/t3rn.raw.json \
    --scheme Sr25519 \
    --suri "$t3rn2_phrase" \
    --key-type aura

  echo "$pchain1_phrase" > "./data/pchain1/chains/local_testnet/keystore/61757261${pchain1_adrs#0x}"
  echo "$pchain2_phrase" > "./data/pchain2/chains/local_testnet/keystore/61757261${pchain2_adrs#0x}"
}

case ${1:-devnet} in
devnet|dev|net)
  mkdir -p ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}
  docker-compose up > /dev/null
  ;;
setkeys|keys)
  set_keys
  ;;
clean|cleanup)
  docker-compose down
  rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}/*
  ;;
build|make|mk)
  mkdir -p ./specs
  build_docker_images
  keygen
  build_relay_chain_spec
  build_para_chain_specs
  build_para_genesis_states
  build_para_wasm_runtimes
  ;;
esac