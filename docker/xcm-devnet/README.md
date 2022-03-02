# ⚡*CM* devnet WIP ⚠️

## Run

``` bash
mkdir -p ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}
docker-compose up
```

Spins up a rococo local devnet consisting of 5 relay chain validators and 2 collators for each parachain.

After startup run:

``` bash
pchain1_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/pchain1.key | xargs)
pchain2_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/pchain2.key | xargs)
pchain1_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./keys/pchain1.key)
pchain2_adrs=$(grep -oP '(?<=\(SS58\):\s)[^\n]+' ./keys/pchain2.key)

printf "$pchain1_phrase" > ./data/pchain1/chains/local_testnet/keystore/61757261${pchain1_adrs#0x}
printf "$pchain2_phrase" > ./data/pchain2/chains/local_testnet/keystore/61757261${pchain2_adrs#0x}

t3rn1_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/t3rn1.key | xargs)
t3rn2_phrase=$(grep -oP '(?<=phrase:)[^\n]+' ./keys/t3rn2.key | xargs)

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

to set collator keys.

Then, parachains can be onboarded as illustrated in [this Zenlink README](https://github.com/zenlinkpro/Zenlink-DEX-Module#register-parachain--establish-hrmp-channel) and [this official tutorial](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#parachain-registration).

> **tl;dr** connect Polkadot Apps UI to `ws://localhost:9944`, using `Alice` [reserve a para id](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#reserve-a-para-id), then [via pallet `parasSudoWrapper` submit extrinsic `sudoScheduleParaInitialize`](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#registration-transaction); genesis state and wasm are @ `./specs/`, parachain ids in the table below => 💡 use the browser Polkadot Apps UI as the desktop version kept failing to reserve a para id...

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
  <tr>
    <td>pchain</td>
    <td>pchain1</td>
    <td>44444</td>
    <td>4488</td>
    <td>4499</td>
    <td>44443</td>
    <td>4487</td>
    <td>4498</td>
    <td>4000</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>pchain2</td>
    <td>44404</td>
    <td>4408</td>
    <td>4409</td>
    <td>44403</td>
    <td>4407</td>
    <td>4408</td>
    <td>4000</td>
  </tr>
</table>

*The "pchain" is a plain [Substrate parachain instance](https://github.com/substrate-developer-hub/substrate-parachain-template)*. All code uses `polkadot-v0.9.13` Substrate.

## Cleanup

``` bash
docker-compose down
rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}/*
```

## Specs

To *regenerate* chain specs and artifacts simply run `./build.sh`.

📚

+ https://github.com/paritytech/substrate/pull/10906