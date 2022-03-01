#!/bin/bash

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/pdot/rococo-local.raw.json" \
  -v "$(pwd)/data/alice:/pdot/data" \
  -p "9944:9944" \
  polkadot:release-v0.9.13 \
  --port 10001 \
  --rpc-port 8844 \
  --ws-port 9944 \
  --alice \
  -d /pdot/data \
  --validator \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --chain /pdot/rococo-local.raw.json \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/pdot/rococo-local.raw.json" \
  -v "$(pwd)/data/bob:/pdot/data" \
  -p "9945:9945" \
  polkadot:release-v0.9.13 \
  --port 10002 \
  --rpc-port 8845 \
  --ws-port 9945 \
  --bob \
  -d /pdot/data \
  --validator \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --chain /pdot/rococo-local.raw.json \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/pdot/rococo-local.raw.json" \
  -v "$(pwd)/data/charlie:/pdot/data" \
  -p "9946:9946" \
  polkadot:release-v0.9.13 \
  --port 10003 \
  --rpc-port 8846 \
  --ws-port 9946 \
  --charlie \
  -d /pdot/data \
  --validator \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --chain /pdot/rococo-local.raw.json \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/pdot/rococo-local.raw.json" \
  -v "$(pwd)/data/dave:/pdot/data" \
  -p "9947:9947" \
  polkadot:release-v0.9.13 \
  --port 10004 \
  --rpc-port 8847 \
  --ws-port 9947 \
  --dave \
  -d /pdot/data \
  --validator \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --chain /pdot/rococo-local.raw.json \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/pdot/rococo-local.raw.json" \
  -v "$(pwd)/data/eve:/pdot/data" \
  -p "9948:9948" \
  polkadot:release-v0.9.13 \
  --port 10005 \
  --rpc-port 8848 \
  --ws-port 9948 \
  --eve \
  -d /pdot/data \
  --validator \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --chain /pdot/rococo-local.raw.json \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/para/rococo-local.raw.json" \
  -v "$(pwd)/specs/pchain.raw.json:/para/pchain.raw.json" \
  -v "$(pwd)/data/pchain1:/para/data" \
  -p "4499:4499" \
  parachain-collator:polkadot-v0.9.13 \
  --port 44444 \
  --rpc-port 4488 \
  --ws-port 4499 \
  --base-path /para/data \
  --keystore-path /para/data/keystore \
  --collator \
  --force-authoring \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --execution Native \
  -lruntime=warn,cumulus-collator=debug,cumulus-network=warn,cumulus-consensus=warn,cumulus-pov-recovery=warn,executor=warn \
  --chain /para/pchain.raw.json \
  --name pchain1 \
  -- \
  --chain /para/rococo-local.raw.json \
  --discover-local \
  --port 44443 \
  --rpc-port 4487 \
  --ws-port 4498 \
  --execution Native \
  &

docker run \
  -v "$(pwd)/specs/rococo-local.raw.json:/para/rococo-local.raw.json" \
  -v "$(pwd)/specs/pchain.raw.json:/para/pchain.raw.json" \
  -v "$(pwd)/data/pchain2:/para/data" \
  -p "4409:4409" \
  parachain-collator:polkadot-v0.9.13 \
  --port 44404 \
  --rpc-port 4408 \
  --ws-port 4409 \
  --base-path /para/data \
  --keystore-path /para/data/keystore \
  --collator \
  --force-authoring \
  --rpc-cors all \
  --unsafe-ws-external \
  --unsafe-rpc-external \
  --execution Native \
  -lruntime=warn,cumulus-collator=debug,cumulus-network=warn,cumulus-consensus=warn,cumulus-pov-recovery=warn,executor=warn \
  --chain /para/pchain.raw.json \
  --name pchain2 \
  -- \
  --chain /para/rococo-local.raw.json \
  --discover-local \
  --port 44403 \
  --rpc-port 4407 \
  --ws-port 4418 \
  --execution Native \
  &