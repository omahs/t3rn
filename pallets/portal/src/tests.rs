// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities
use codec::{Decode};
use frame_support::{assert_noop, assert_ok};
use hex;
use hex_literal::hex as hexx;
use sp_io::TestExternalities;
use sp_version::{create_runtime_str, RuntimeVersion};
use serde_json::{Value};
use t3rn_primitives::portal::RegistrationData;
use std::fs;
use frame_support::dispatch::PostDispatchInfo;
use frame_system::pallet_prelude::OriginFor;
use sp_runtime::{DispatchError, DispatchErrorWithPostInfo};
    use t3rn_primitives::{
        portal::{RococoBridge},
        abi::{GatewayABIConfig},
        ChainId, EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor, GatewayGenesisConfig,
    };
use t3rn_primitives::xdns::AllowedSideEffect;
use crate::{mock::*, Error};
pub fn new_test_ext() -> TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    TestExternalities::new(t)
}

pub const TEST_RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("test-runtime"),
    impl_name: create_runtime_str!("test-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
    state_version: 1,
};

fn register(
    origin: OriginFor<Test>,
    file: &str,
    valid: bool,
) -> Result<PostDispatchInfo, DispatchErrorWithPostInfo<PostDispatchInfo>> {
    let raw_data = fs::read_to_string("./src/mock-data/".to_owned() + file).unwrap();
    let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
    let url: Vec<u8> =  hex::decode(json["encoded_url"].as_str().unwrap()).unwrap();
    let gateway_id: ChainId = Decode::decode(&mut &*hex::decode(json["encoded_gateway_id"].as_str().unwrap()).unwrap()).unwrap();
    let gateway_abi: GatewayABIConfig = Decode::decode(&mut &*hex::decode(json["encoded_gateway_abi"].as_str().unwrap()).unwrap()).unwrap();
    let gateway_vendor: GatewayVendor = Decode::decode(&mut &*hex::decode(json["encoded_gateway_vendor"].as_str().unwrap()).unwrap()).unwrap();
    let gateway_type: GatewayType = Decode::decode(&mut &*hex::decode(json["encoded_gateway_type"].as_str().unwrap()).unwrap()).unwrap();
    let gateway_genesis: GatewayGenesisConfig = Decode::decode(&mut &*hex::decode(json["encoded_gateway_genesis"].as_str().unwrap()).unwrap()).unwrap();
    let gateway_sys_props: GatewaySysProps = Decode::decode(&mut &*hex::decode(json["encoded_gateway_sys_props"].as_str().unwrap()).unwrap()).unwrap();
    let allowed_side_effects: Vec<AllowedSideEffect> = Decode::decode(&mut &*hex::decode(json["encoded_allowed_side_effects"].as_str().unwrap()).unwrap()).unwrap();
    let encoded_registration_data: Vec<u8> = hex::decode(json["encoded_registration_data"].as_str().unwrap()).unwrap();

    let res = Portal::register_gateway(
        origin,
        url.clone(),
        gateway_id.clone(),
        gateway_abi.clone(),
        gateway_vendor.clone(),
        gateway_type.clone(),
        gateway_genesis.clone(),
        gateway_sys_props.clone(),
        allowed_side_effects.clone(),
        encoded_registration_data.clone()
    );

    if valid {
        let xdns_record = pallet_xdns::XDNSRegistry::<Test>::get(gateway_id).unwrap();
        let stored_side_effects = xdns_record.allowed_side_effects;

        // ensure XDNS writes are correct
        assert_eq!(stored_side_effects, allowed_side_effects);
        assert_eq!(xdns_record.gateway_vendor, gateway_vendor);
        assert_eq!(xdns_record.gateway_abi, gateway_abi);
        assert_eq!(xdns_record.gateway_type, gateway_type);
        assert_eq!(xdns_record.gateway_sys_props, gateway_sys_props);
        assert_eq!(xdns_record.gateway_genesis, gateway_genesis);
    }

    return res
}

#[test]
fn register_rococo_successfully() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::root(); // only sudo access to register new gateways for now
    ext.execute_with(|| {
        assert_ok!(register(origin, "register-roco.json", true));
    });
}

#[test]
fn fails_registration_with_invalid_signer() {
    let mut ext = TestExternalities::new_empty();
    let origin = Origin::signed([0u8; 32].into()); // only sudo access to register new gateways for now
    ext.execute_with(|| {
        assert_noop!(
            register(origin, "register-roco.json", false),
            DispatchError::BadOrigin
        );
    });
}

// ToDo: Update return type of XDNS to enable correct error handling
// #[test]
// fn gateway_can_only_be_registered_once() {
//     // imports encoded RegistrationData from mock-data created by CLI
//     let raw_data = fs::read_to_string("./src/mock-data/register-roco.json").unwrap();
//     let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
//     let bytes = hex::decode(json["encoded"].as_str().unwrap()).unwrap();
//     let registration_data: RegistrationData = Decode::decode(&mut &*bytes).unwrap();
//
//     let mut ext = TestExternalities::new_empty();
//     let origin = Origin::root(); // only sudo access to register new gateways for now
//     ext.execute_with(|| {
//         assert_ok!(Portal::register_gateway(
//             origin.clone(),
//             registration_data.clone()
//         ));
//         assert_ok!(Portal::register_gateway(origin, registration_data).is_err());
//     });
// }

#[test]
fn cant_submit_without_registering() {
    // imports encoded RegistrationData from mock-data created by CLI
    let raw_data = fs::read_to_string("./src/mock-data/submit-header-roco-1237705-1238699-epoch.json").unwrap();
    let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
    let encoded_header_data: Vec<u8> =  hex::decode(json[0]["encoded_data"].as_str().unwrap()).unwrap();

    let mut ext = TestExternalities::new_empty();
    let origin = Origin::root();
    ext.execute_with(|| {
        assert_noop!(
            Portal::submit_headers(
                origin,
                *b"roco",
                encoded_header_data
            ),
            Error::<Test>::GatewayVendorNotFound
        );
    });
}

#[test]
fn can_submit_valid_header_data_with_auth_update() {
    // imports encoded RegistrationData from mock-data created by CLI
    let raw_data = fs::read_to_string("./src/mock-data/submit-header-roco-1237705-1238699-epoch.json").unwrap();
    let json: Value = serde_json::from_str(raw_data.as_str()).unwrap();
    let encoded_header_data_0: Vec<u8> =  hex::decode(json[0]["encoded_data"].as_str().unwrap()).unwrap();
    let encoded_header_data_1: Vec<u8> =  hex::decode(json[1]["encoded_data"].as_str().unwrap()).unwrap();

    let mut ext = TestExternalities::new_empty();
    let root = Origin::root();
    let origin = Origin::signed([0u8; 32].into());
    ext.execute_with(|| {
        assert_ok!(register(root, "register-roco.json", true));
        assert_noop!( // can't pass early headers
            Portal::submit_headers(
                origin.clone(),
                *b"roco",
                encoded_header_data_1.clone()
            ),
            Error::<Test>::SubmitHeaderError
        );
        assert_ok!( // can update auth set
            Portal::submit_headers(
                origin.clone(),
                *b"roco",
                encoded_header_data_0.clone()
            )
        );
        assert_noop!( // can't submit twice
            Portal::submit_headers(
                origin.clone(),
                *b"roco",
                encoded_header_data_0.clone()
            ),
            Error::<Test>::SubmitHeaderError
        );
        assert_ok!( //can validate headers in new auth set
            Portal::submit_headers(
                origin,
                *b"roco",
                encoded_header_data_1
            )
        );
    });
}

// #[test]
// fn test_register_parachain() {
    // let origin = Origin::root(); // only sudo access to register new gateways for now
    // let url = b"ws://localhost:9944".to_vec();
    // let gateway_id = [0; 4];
    // let gateway_abi: GatewayABIConfig = Default::default();
    //
    // let gateway_vendor = GatewayVendor::Substrate;
    // let gateway_type = GatewayType::ProgrammableInternal(0);
    //
    // let _gateway_pointer = GatewayPointer {
    //     id: [0; 4],
    //     vendor: GatewayVendor::Substrate,
    //     gateway_type: GatewayType::ProgrammableInternal(0),
    // };
    //
    // let gateway_genesis = GatewayGenesisConfig {
    //     modules_encoded: None,
    //     genesis_hash: Default::default(),
    //     extrinsics_version: 0u8,
    // };
    //
    // let gateway_sys_props = GatewaySysProps {
    //     ss58_format: 0,
    //     token_symbol: Encode::encode(""),
    //     token_decimals: 0,
    // };
    //
    // let parachain = Some(Parachain {
    //     relay_chain_id: [1, 3, 3, 7],
    //     id: 2015,
    // });
    //
    // let first_header: CurrentHeader<Test, DefaultPolkadotLikeGateway> = test_header(0);
    //
    // let authorities = Some(vec![]);
    // let authority_set_id = None;
    // let allowed_side_effects = vec![];
    //
    // let mut ext = TestExternalities::new_empty();
    // ext.execute_with(|| {
    //     assert_ok!(Portal::register_gateway(
    //         origin,
    //         url,
    //         gateway_id,
    //         parachain,
    //         gateway_abi,
    //         gateway_vendor,
    //         gateway_type,
    //         gateway_genesis,
    //         gateway_sys_props,
    //         first_header.encode(),
    //         authorities,
    //         authority_set_id,
    //         allowed_side_effects,
    //     ));
    // });
// }

//
// #[test]
// fn test_register_gateway_with_u64_substrate_header() {
//     let origin = Origin::root(); // only sudo access to register new gateways for now
//     let url = b"ws://localhost:9944".to_vec();
//     let gateway_id = [0; 4];
//     let gateway_abi: GatewayABIConfig = Default::default();
//
//     let gateway_vendor = GatewayVendor::Substrate;
//     let gateway_type = GatewayType::ProgrammableInternal(0);
//
//     let _gateway_pointer = GatewayPointer {
//         id: [0; 4],
//         vendor: GatewayVendor::Substrate,
//         gateway_type: GatewayType::ProgrammableInternal(0),
//     };
//
//     let gateway_genesis = GatewayGenesisConfig {
//         modules_encoded: None,
//         genesis_hash: Default::default(),
//         extrinsics_version: 0u8,
//     };
//
//     let gateway_sys_props = GatewaySysProps {
//         ss58_format: 0,
//         token_symbol: Encode::encode(""),
//         token_decimals: 0,
//     };
//
//     let first_header: CurrentHeader<Test, PolkadotLikeValU64Gateway> = test_header(0);
//
//     let authorities = Some(vec![]);
//     let allowed_side_effects = vec![];
//     let authority_set_id = None;
//
//     let mut ext = TestExternalities::new_empty();
//     ext.execute_with(|| {
//         assert_ok!(Portal::register_gateway(
//             origin,
//             url,
//             gateway_id,
//             None,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             first_header.encode(),
//             authorities,
//             authority_set_id,
//             allowed_side_effects,
//         ));
//     });
// }
//
// #[test]
// fn test_register_gateway_with_default_eth_like_header() {
//     let origin = Origin::root(); // only sudo access to register new gateways for now
//     let url = b"ws://localhost:9944".to_vec();
//     let gateway_id = [0; 4];
//     let gateway_abi: GatewayABIConfig = Default::default();
//
//     let gateway_vendor = GatewayVendor::Substrate;
//     let gateway_type = GatewayType::ProgrammableInternal(0);
//
//     let _gateway_pointer = GatewayPointer {
//         id: [0; 4],
//         vendor: GatewayVendor::Substrate,
//         gateway_type: GatewayType::ProgrammableInternal(0),
//     };
//
//     let gateway_genesis = GatewayGenesisConfig {
//         modules_encoded: None,
//         genesis_hash: Default::default(),
//         extrinsics_version: 0u8,
//     };
//
//     let gateway_sys_props = GatewaySysProps {
//         ss58_format: 0,
//         token_symbol: Encode::encode(""),
//         token_decimals: 0,
//     };
//
//     let first_header: CurrentHeader<Test, EthLikeKeccak256ValU32Gateway> = test_header(0);
//
//     let authorities = Some(vec![]);
//     let allowed_side_effects = vec![*b"tran"];
//     let authority_set_id = None;
//
//     let mut ext = TestExternalities::new_empty();
//     ext.execute_with(|| {
//         assert_ok!(Portal::register_gateway(
//             origin,
//             url,
//             gateway_id,
//             None,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             first_header.encode(),
//             authorities,
//             authority_set_id,
//             allowed_side_effects,
//         ));
//     });
// }
//
// #[test]
// fn test_register_gateway_with_u64_eth_like_header() {
//     let origin = Origin::root(); // only sudo access to register new gateways for now
//     let url = b"ws://localhost:9944".to_vec();
//     let gateway_id = [0; 4];
//     let gateway_abi: GatewayABIConfig = Default::default();
//
//     let gateway_vendor = GatewayVendor::Substrate;
//     let gateway_type = GatewayType::ProgrammableInternal(0);
//
//     let _gateway_pointer = GatewayPointer {
//         id: [0; 4],
//         vendor: GatewayVendor::Substrate,
//         gateway_type: GatewayType::ProgrammableInternal(0),
//     };
//
//     let gateway_genesis = GatewayGenesisConfig {
//         modules_encoded: None,
//         genesis_hash: Default::default(),
//         extrinsics_version: 0u8,
//     };
//
//     let gateway_sys_props = GatewaySysProps {
//         ss58_format: 0,
//         token_symbol: Encode::encode(""),
//         token_decimals: 0,
//     };
//
//     let first_header: CurrentHeader<Test, EthLikeKeccak256ValU64Gateway> = test_header(0);
//
//     let authorities = Some(vec![]);
//     let allowed_side_effects = vec![];
//     let authority_set_id = None;
//
//     let mut ext = TestExternalities::new_empty();
//     ext.execute_with(|| {
//         assert_ok!(Portal::register_gateway(
//             origin,
//             url,
//             gateway_id,
//             None,
//             gateway_abi,
//             gateway_vendor,
//             gateway_type,
//             gateway_genesis,
//             gateway_sys_props,
//             first_header.encode(),
//             authorities,
//             authority_set_id,
//             allowed_side_effects,
//         ));
//     });
// }
//
// #[test]
// fn test_register_gateway_with_u64_substrate_header_and_allowed_side_effects() {
//     let origin = Origin::root(); // only sudo access to register new gateways for now
//     let url = b"ws://localhost:9944".to_vec();
//     let gateway_id = [0; 4];
//     let gateway_abi: GatewayABIConfig = Default::default();
//
//     let gateway_vendor = GatewayVendor::Substrate;
//     let gateway_type = GatewayType::ProgrammableInternal(0);
//
//     let _gateway_pointer = GatewayPointer {
//         id: [0; 4],
//         vendor: GatewayVendor::Substrate,
//         gateway_type: GatewayType::ProgrammableInternal(0),
//     };
//
//     let gateway_genesis = GatewayGenesisConfig {
//         modules_encoded: None,
//         genesis_hash: Default::default(),
//         extrinsics_version: 0u8,
//     };
//
//     let gateway_sys_props = GatewaySysProps {
//         ss58_format: 0,
//         token_symbol: Encode::encode(""),
//         token_decimals: 0,
//     };
//
//     let first_header: CurrentHeader<Test, PolkadotLikeValU64Gateway> = test_header(0);
//
//     let authorities = Some(vec![]);
//     let authority_set_id = None;
//     let allowed_side_effects: Vec<AllowedSideEffect> = vec![*b"swap"];
//
//     let mut ext = TestExternalities::new_empty();
//     ext.execute_with(|| System::set_block_number(1));
//     ext.execute_with(|| {
//         assert_ok!(Portal::register_gateway(
//             origin,
//             url,
//             gateway_id,
//             None,
//             gateway_abi,
//             gateway_vendor.clone(),
//             gateway_type.clone(),
//             gateway_genesis,
//             gateway_sys_props.clone(),
//             first_header.encode(),
//             authorities,
//             authority_set_id,
//             allowed_side_effects.clone(),
//         ));
//
//         // Assert the stored xdns record
//
//         let xdns_record = pallet_xdns::XDNSRegistry::<Test>::get(gateway_id).unwrap();
//         let stored_side_effects = xdns_record.allowed_side_effects;
//
//         assert_eq!(stored_side_effects.len(), 1);
//         assert_eq!(stored_side_effects, allowed_side_effects);
//
//         // Assert events emitted
//
//         System::assert_last_event(Event::Portal(crate::Event::NewGatewayRegistered(
//             gateway_id,
//             gateway_type,
//             gateway_vendor,
//             gateway_sys_props,
//             allowed_side_effects,
//         )));
//         // XdnsRecordStored and NewGatewayRegistered
//         let events = System::events();
//         assert_eq!(events.len(), 2);
//     });
// }
