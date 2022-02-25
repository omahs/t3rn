# ⚡*CM* devnet WIP ⚠️

## Run

``` bash
mkdir -p ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2}
docker-compose up
```

Spins up a rococo local devnet consisting of 5 relay chain validators and 2 collators for each parachain.

After startup run:

``` bash
t3rn1_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/t3rn1.key)
t3rn2_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/t3rn2.key)

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
```

to set collator keys that enable t3rn block production.

*tbc... ilustrate how to manually insert the node keys into pchain's keystore as 4some reason pchain doesn't have a key>insert subcommand* 

Then, parachains can be onboarded as illustrated in [this Zenlink README](https://github.com/zenlinkpro/Zenlink-DEX-Module#register-parachain--establish-hrmp-channel) and [this official tutorial](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#parachain-registration).

> **tl;dr** connect UI to `ws://localhost:9944` and use pallet `parasSudoWrapper` and extrinsic `sudoScheduleParaInitialize` with `Alice`; genesis state and wasm are @ `./specs/`, parachain ids in the table below

<table style="margin-bottom:0;">
  <tr>
    <td><b>Network</b></td>
    <td><b>Node</b></td>
    <td colspan="3"><b>Relaychain Ports</b></td>
    <td colspan="3"><b>Parachain Ports</b></td>
    <td><b>Parachain Id</b></td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Alice</td>
    <td>10001</td>
    <td>8844</td>
    <td>9944</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Bob</td>
    <td>10002</td>
    <td>8845</td>
    <td>9945</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Charlie</td>
    <td>10003</td>
    <td>8846</td>
    <td>9946</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Dave</td>
    <td>10004</td>
    <td>8847</td>
    <td>9947</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Eve</td>
    <td>10005</td>
    <td>8848</td>
    <td>9948</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>t3rn1</td>
    <td>33332</td>
    <td>8832</td>
    <td>9932</td>
    <td>33333</td>
    <td>8833</td>
    <td>9933</td>
    <td>3000</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>t3rn2</td>
    <td>33322</td>
    <td>8822</td>
    <td>9922</td>
    <td>33323</td>
    <td>8823</td>
    <td>9923</td>
    <td>3000</td>
  </tr>
  <!-- <tr>
    <td>pchain</td>
    <td>pchain1</td>
    <td>44444</td>
    <td>4488</td>
    <td>4499</td>
    <td>44443</td>
    <td>4487</td>
    <td>4498</td>
    <td>4000</td>
  </tr> -->
</table>

<!-- *The "pchain" is a plain [Substrate parachain instance](https://github.com/substrate-developer-hub/substrate-parachain-template)*. All code uses `polkadot-v0.9.13` Substrate. -->

## Cleanup

<!-- ``` bash
docker-compose down
rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}/*
``` -->
``` bash
docker-compose down
rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2}/*
```

## Specs

To *regenerate* chain specs and artifacts simply run `./build.sh`.