//! Autogenerated weights for pallet_xdns
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/circuit
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_xdns
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --raw
// --template=../benchmarking/frame-weight-template.hbs
// --output
// .

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_xdns.
pub trait WeightInfo {
    fn add_new_xdns_record() -> Weight;
    fn update_ttl() -> Weight;
    fn purge_xdns_record() -> Weight;
    fn best_available() -> Weight;
}

/// Weights for pallet_xdns using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_new_xdns_record() -> Weight {
        (72_795_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn update_ttl() -> Weight {
        (73_255_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn purge_xdns_record() -> Weight {
        (58_912_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn best_available() -> Weight {
        (25_265_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn add_new_xdns_record() -> Weight {
        (72_795_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn update_ttl() -> Weight {
        (73_255_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn purge_xdns_record() -> Weight {
        (58_912_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn best_available() -> Weight {
        (25_265_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
    }
}
