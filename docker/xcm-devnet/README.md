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

> **tl;dr** connect Polkadot Apps UI to `ws://localhost:9944`, using `Alice` [reserve a para id](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#reserve-a-para-id), then [via pallet `parasSudoWrapper` submit extrinsic `sudoScheduleParaInitialize`](https://docs.substrate.io/tutorials/v3/cumulus/connect-parachain/#registration-transaction); genesis state and wasm @ `./specs/`, parachain ids in the table below; 💡 use the browser Polkadot Apps UI as the desktop version kept failing to reserve a para id...

## Cleanup

``` bash
docker-compose down
rm -r ./data/{alice,bob,charlie,dave,eve,t3rn1,t3rn2,pchain1,pchain2}/*
```

## Specs

To *regenerate* chain specs, artifacts, and keys simply run `./build.sh`.

## Overview

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

## 🐛

```
...
t3rn1      | 2022-03-02 09:31:59.750 DEBUG tokio-runtime-worker txpool: [Relaychain] [0xabee3690fd5bdecd68cae3b8d54115f8c3e17b623248e355e06e486873152662] Sent finalization event (block 0x51d1d916810bfc6ffa995a4e15bbc67e0a98517c72ad08e4a01a6d8c086d4927)    
t3rn1      | 2022-03-02 09:31:59.750 DEBUG tokio-runtime-worker afg: [Relaychain] learned-banana-9567: Starting new voter with set ID 3    
t3rn1      | 2022-03-02 09:31:59.750 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x51d1d916810bfc6ffa995a4e15bbc67e0a98517c72ad08e4a01a6d8c086d4927), using onchain code    
t3rn1      | 2022-03-02 09:31:59.750 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 552, max_bumper 1224152    
t3rn1      | 2022-03-02 09:32:00.016 DEBUG tokio-runtime-worker sync: [Relaychain] Pre-validating received block announcement 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 with number 44 from 12D3KooWG5o6KagLVFsM8X3ameZPPke9Y2hrzrqA5744TxCq5ETj    
t3rn1      | 2022-03-02 09:32:00.016 DEBUG tokio-runtime-worker libp2p_swarm::protocols_handler::node_handler: [Relaychain] Substream upgrade protocol override: V1 -> V1Lazy    
t3rn1      | 2022-03-02 09:32:00.016 DEBUG tokio-runtime-worker multistream_select::dialer_select: [Relaychain] Dialer: Proposed protocol: /dot/sync/2    
t3rn1      | 2022-03-02 09:32:00.016 DEBUG tokio-runtime-worker multistream_select::dialer_select: [Relaychain] Dialer: Expecting proposed protocol: /dot/sync/2    
t3rn1      | 2022-03-02 09:32:00.017 DEBUG tokio-runtime-worker multistream_select::negotiated: [Relaychain] Negotiated: Received confirmation for protocol: /dot/sync/2    
t3rn1      | 2022-03-02 09:32:00.017 DEBUG tokio-runtime-worker libp2p_core::upgrade::apply: [Relaychain] Successfully applied negotiated protocol    
t3rn1      | 
t3rn1      | ====================
t3rn1      | 
t3rn1      | Version: 0.1.0
t3rn1      | 
t3rn1      |    0: sp_panic_handler::set::{{closure}}
t3rn1      |    1: std::panicking::rust_panic_with_hook
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/std/src/panicking.rs:610:17
t3rn1      |    2: std::panicking::begin_panic_handler::{{closure}}
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/std/src/panicking.rs:500:13
t3rn1      |    3: std::sys_common::backtrace::__rust_end_short_backtrace
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/std/src/sys_common/backtrace.rs:139:18
t3rn1      |    4: rust_begin_unwind
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/std/src/panicking.rs:498:5
t3rn1      |    5: core::panicking::panic_fmt
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/core/src/panicking.rs:106:14
t3rn1      |    6: core::panicking::panic
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/core/src/panicking.rs:47:5
t3rn1      |    7: <(TupleElement0,TupleElement1) as frame_support::traits::hooks::OnInitialize<BlockNumber>>::on_initialize
t3rn1      |    8: <(TupleElement0,TupleElement1) as frame_support::traits::hooks::OnInitialize<BlockNumber>>::on_initialize
t3rn1      |    9: <(TupleElement0,TupleElement1) as frame_support::traits::hooks::OnInitialize<BlockNumber>>::on_initialize
t3rn1      |   10: frame_executive::Executive<System,Block,Context,UnsignedValidator,AllPallets,COnRuntimeUpgrade>::initialize_block
t3rn1      |   11: std::panicking::try
t3rn1      |   12: std::thread::local::LocalKey<T>::with
t3rn1      |   13: sc_executor::native_executor::WasmExecutor::with_instance::{{closure}}
t3rn1      |   14: sc_executor::wasm_runtime::RuntimeCache::with_instance
t3rn1      |   15: <sc_executor::native_executor::NativeElseWasmExecutor<D> as sp_core::traits::CodeExecutor>::call
t3rn1      |   16: sp_state_machine::execution::StateMachine<B,H,Exec>::execute_aux
t3rn1      |   17: sp_state_machine::execution::StateMachine<B,H,Exec>::execute_using_consensus_failure_handler
t3rn1      |   18: <sc_service::client::call_executor::LocalCallExecutor<Block,B,E> as sc_client_api::call_executor::CallExecutor<Block>>::contextual_call
t3rn1      |   19: sp_api::runtime_decl_for_Core::initialize_block_call_api_at
t3rn1      |   20: <circuit_parachain_runtime::RuntimeApiImpl<__SR_API_BLOCK__,RuntimeApiImplCall> as sp_api::Core<__SR_API_BLOCK__>>::Core_initialize_block_runtime_api_impl
t3rn1      |   21: sp_api::Core::initialize_block_with_context
t3rn1      |   22: sc_block_builder::BlockBuilder<Block,A,B>::new
t3rn1      |   23: sc_basic_authorship::basic_authorship::Proposer<B,Block,C,A,PR>::propose_with::{{closure}}
t3rn1      |   24: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
t3rn1      |   25: <sc_service::task_manager::prometheus_future::PrometheusFuture<T> as core::future::future::Future>::poll
t3rn1      |   26: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
t3rn1      |   27: <tracing_futures::Instrumented<T> as core::future::future::Future>::poll
t3rn1      |   28: tokio::park::thread::CachedParkThread::block_on
t3rn1      |   29: tokio::runtime::handle::Handle::block_on
t3rn1      |   30: tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut
t3rn1      |   31: tokio::runtime::task::harness::Harness<T,S>::poll
t3rn1      |   32: tokio::runtime::blocking::pool::Inner::run
t3rn1      |   33: std::sys_common::backtrace::__rust_begin_short_backtrace
t3rn1      |   34: core::ops::function::FnOnce::call_once{{vtable.shim}}
t3rn1      |   35: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/alloc/src/boxed.rs:1691:9
t3rn1      |       <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/alloc/src/boxed.rs:1691:9
t3rn1      |       std::sys::unix::thread::Thread::new::thread_start
t3rn1      |              at rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/std/src/sys/unix/thread.rs:106:17
t3rn1      |   36: start_thread
t3rn1      |   37: clone
t3rn1      | 
t3rn1      | 
t3rn1      | Thread 'tokio-runtime-worker' panicked at 'attempt to calculate the remainder with a divisor of zero', /rustc/0727994435c75fdedd3e9d226cf434089b0ab585/library/core/src/ops/arith.rs:584
t3rn1      | 
t3rn1      | This is a bug. Please report it at:
t3rn1      | 
t3rn1      | 	https://github.com/t3rn/t3rn/issues/new
t3rn1      | 
t3rn1      | 2022-03-02 09:32:00.017 DEBUG tokio-runtime-worker babe: [Relaychain] We have 3 logs in this header    
t3rn1      | 2022-03-02 09:32:00.017 DEBUG tokio-runtime-worker header: [Relaychain] Retrieving mutable reference to digest    
t3rn1      | 2022-03-02 09:32:00.017 DEBUG tokio-runtime-worker babe: [Relaychain] Verifying secondary VRF block #44 at slot: 274368920    
t3rn1      | 2022-03-02 09:32:00.018 DEBUG tokio-runtime-worker babe: [Relaychain] Skipping `check_inherents` as authoring version is not compatible: `spec_name` does not match `polkadot` vs `rococo`    
t3rn1      | 2022-03-02 09:32:00.018 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x58a51ec51c3deb663b49e83642b1769f3365bf4bcf59d06763842a3219c15021), using onchain code    
t3rn1      | 2022-03-02 09:32:00.019 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 5704, max_bumper 1230912    
t3rn1      | 2022-03-02 09:32:00.019 DEBUG tokio-runtime-worker header: [Relaychain] Retrieving mutable reference to digest    
t3rn1      | 2022-03-02 09:32:00.019 DEBUG tokio-runtime-worker db: [Relaychain] DB Commit 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 (44), best=true, state=true, existing=false    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker txpool: [Relaychain] Starting pruning of block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3) (extrinsics: 2)    
t3rn1      | 2022-03-02 09:32:00.020  INFO tokio-runtime-worker substrate: [Relaychain] ✨ Imported #44 (0x076d…fdf3)    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker sync: [Relaychain] Reannouncing block 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 is_best: true    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker sync: [Relaychain] New best block imported 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3/#44    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x58a51ec51c3deb663b49e83642b1769f3365bf4bcf59d06763842a3219c15021), using onchain code    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 552, max_bumper 1224152    
t3rn1      | 2022-03-02 09:32:00.020 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 616, max_bumper 1223928    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker sync: [Relaychain] Pre-validating received block announcement 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 with number 44 from 12D3KooWBxN4MgimeLn99trjQVZeB6k51fG9gp2hr7yWj36CA79S    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker parachain::collator-protocol: [Relaychain] Removing relay parent because our view changed. relay_parent=0x58a51ec51c3deb663b49e83642b1769f3365bf4bcf59d06763842a3219c15021
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x58a51ec51c3deb663b49e83642b1769f3365bf4bcf59d06763842a3219c15021), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 288, max_bumper 1223552    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 2856, max_bumper 1226192    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker txpool: [Relaychain] Pruning at BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3)    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker txpool: [Relaychain] [0xe39a50112c4a977c5500512245a69eede1a7f6795428719fce2f206876da3a8c] Pruned at 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker txpool: [Relaychain] [0x06fc81c285aad8c3bac5251e41f5067f42fc59de898cd9db86cc28489952ebc6] Pruned at 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 2080, max_bumper 1226624    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 112, max_bumper 1223296    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.021 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 288, max_bumper 1223552    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 648, max_bumper 1223936    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 288, max_bumper 1223552    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker sync: [Relaychain] Pre-validating received block announcement 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 with number 44 from 12D3KooWD5XuBZuNfKq1zXevVMQdNaTSaZcyoM3DHTn7JiEofgVa    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 4192, max_bumper 1227472    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 48, max_bumper 1223240    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker sync: [Relaychain] Pre-validating received block announcement 0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 with number 44 from 12D3KooWBx52uZRXcSkP9BX83LNGxueiVPGaAaiyUAhrtK3AzwNU    
t3rn1      | 2022-03-02 09:32:00.022 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.023 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 552, max_bumper 1224152    
t3rn1      | 2022-03-02 09:32:00.023 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.023 DEBUG tokio-runtime-worker wasm_overrides: [Relaychain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.023 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 552, max_bumper 1223928    
t3rn1      | 2022-03-02 09:32:00.024 DEBUG tokio-runtime-worker parachain::approval-distribution: [Relaychain] Processing NewBlocks
t3rn1      | 2022-03-02 09:32:00.024 DEBUG tokio-runtime-worker parachain::approval-distribution: [Relaychain] Got new blocks [(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3, 44)]
t3rn1      | 2022-03-02 09:32:00.025 DEBUG tokio-runtime-worker wasm-heap: [Relaychain] allocator being destroyed, max_total_size 2097256, max_bumper 3320632    
t3rn1      | 2022-03-02 09:32:00.027  INFO tokio-runtime-worker cumulus-collator: [Parachain] Starting collation. relay_parent=0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3 at=0xc923ef598545bb59d3c26382befba4f9eb4b75480afb2f207127f2808f26b475
t3rn1      | 2022-03-02 09:32:00.027 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.027 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 112, max_bumper 1223296    
t3rn1      | 2022-03-02 09:32:00.027 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0x076db7ce805be13d2d903f175a74a9591eaca28085f2a8c1c24af0e4ac51fdf3), using onchain code    
t3rn1      | 2022-03-02 09:32:00.027 DEBUG tokio-runtime-worker wasm-heap: [Parachain] allocator being destroyed, max_total_size 112, max_bumper 1223296    
t3rn1      | 2022-03-02 09:32:00.032 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0xc923ef598545bb59d3c26382befba4f9eb4b75480afb2f207127f2808f26b475), using onchain code    
t3rn1      | 2022-03-02 09:32:00.032 DEBUG tokio-runtime-worker aura: [Parachain] Starting authorship at slot 137184460; timestamp = 1646213520027    
t3rn1      | 2022-03-02 09:32:00.032  INFO tokio-runtime-worker sc_basic_authorship::basic_authorship: [Parachain] 🙌 Starting consensus session on top of parent 0xc923ef598545bb59d3c26382befba4f9eb4b75480afb2f207127f2808f26b475    
t3rn1      | 2022-03-02 09:32:00.036 DEBUG tokio-runtime-worker wasm_overrides: [Parachain] No WASM override available for block BlockId::Hash(0xc923ef598545bb59d3c26382befba4f9eb4b75480afb2f207127f2808f26b475), using onchain code    
t3rn1      | 2022-03-02 09:32:00.037  WARN tokio-runtime-worker aura: [Parachain] Proposing failed: ClientImport("RuntimeApiError(Application(Execution(RuntimePanicked(\"attempt to calculate the remainder with a divisor of zero\"))))")    
t3rn1      | 2022-03-02 09:32:00.037 DEBUG tokio-runtime-worker parachain::collation-generation: [Relaychain] collator returned no collation on collate para_id=3000
...
```

## 📚

+ https://github.com/paritytech/substrate/pull/10906
+ https://github.com/paritytech/cumulus/pull/1041
+ https://github.com/paritytech/polkadot/pull/4973