use crate::Bytes;
use codec::{Decode, Encode};

use num_traits::Zero;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{
    convert::{TryFrom, TryInto},
    vec,
};

pub use interface::*;
pub use t3rn_types::side_effect::{
    ConfirmationOutcome, ConfirmedSideEffect, Error, EventSignature,
    FullSideEffect as HardenedSideEffect, SecurityLvl, SideEffect, SideEffectName, TargetId,
    ADD_LIQUIDITY_SIDE_EFFECT_ID, ASSETS_TRANSFER_SIDE_EFFECT_ID, CALL_SIDE_EFFECT_ID,
    COMPOSABLE_CALL_SIDE_EFFECT_ID, DATA_SIDE_EFFECT_ID, EVM_CALL_SIDE_EFFECT_ID,
    ORML_TRANSFER_SIDE_EFFECT_ID, SWAP_SIDE_EFFECT_ID, TRANSFER_SIDE_EFFECT_ID,
    WASM_CALL_SIDE_EFFECT_ID,
};

use crate::circuit::XExecSignalId;
#[cfg(feature = "no_std")]
use sp_runtime::RuntimeDebug as Debug;

pub mod interface;
pub mod parser;

pub type SideEffectId<T> = <T as frame_system::Config>::Hash;

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct FullSideEffect<AccountId, BlockNumber, BalanceOf> {
    pub input: SideEffect<AccountId, BalanceOf>,
    pub confirmed: Option<ConfirmedSideEffect<AccountId, BlockNumber, BalanceOf>>,
    pub security_lvl: SecurityLvl,
    pub submission_target_height: Bytes,
    pub best_bid: Option<SFXBid<AccountId, BalanceOf, u32>>,
    pub index: u32,
}

/// All Executors from the active set can bid for SFX executions in order to claim the rewards (max_fee) set by users,
///     ultimately competing against one another on the open market rules.
/// In case bid goes on Optimistic SFX, Executor will also have their bonded stake reserve to insure
///     other Optimistic Executors co-executing given Xtx with their bonded collateral (reserved_bond)
/// Their balance
#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct SFXBid<AccountId, BalanceOf, AssetId> {
    /// Bid amount - always below SFX::max_fee requested by a user
    pub bid: BalanceOf,
    /// Insurance in case of optimistic FSX
    pub insurance: BalanceOf,
    /// Optional reserved bond in case of optimistic FSX
    pub reserved_bond: Option<BalanceOf>,
    /// Optional reserved asset id in case execution on foreign assets
    pub reward_asset_id: Option<AssetId>,
    /// Bidding Executor belonging to the active set
    pub executor: AccountId,
    /// Executor - subject of SFX
    pub requester: AccountId,
}

impl<AccountId, BalanceOf, AssetId> SFXBid<AccountId, BalanceOf, AssetId> {
    pub fn new_none_optimistic(
        bid: BalanceOf,
        insurance: BalanceOf,
        executor: AccountId,
        requester: AccountId,
        reward_asset_id: Option<AssetId>,
    ) -> Self {
        SFXBid {
            bid,
            insurance,
            reserved_bond: None,
            executor,
            requester,
            reward_asset_id,
        }
    }

    pub fn expect_reserved_bond(&self) -> &BalanceOf {
        self.reserved_bond
            .as_ref()
            .expect("Accessed reserved_bond and expected it to be a part of SFXBid")
    }

    pub fn get_reserved_bond(&self) -> &Option<BalanceOf> {
        &self.reserved_bond
    }

    pub fn get_insurance(&self) -> &BalanceOf {
        &self.insurance
    }
}

impl<AccountId, BlockNumber, BalanceOf> FullSideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: Encode + Clone,
    BlockNumber: Encode + Clone,
    BalanceOf: Encode + Zero + Clone,
{
    pub fn is_successfully_confirmed(&self) -> bool {
        if let Some(confirmed) = &self.confirmed {
            confirmed.err.is_none()
        } else {
            false
        }
    }

    pub fn is_bid_resolved(&self) -> bool {
        self.best_bid.is_some()
    }

    pub fn expect_sfx_bid(&self) -> &SFXBid<AccountId, BalanceOf, u32> {
        self.best_bid
            .as_ref()
            .expect("Accessed expected Bid and expected it to be a part of FSX")
    }

    pub fn get_bond_value(&self, max_reward: BalanceOf) -> BalanceOf {
        if let Some(sfx_bid) = &self.best_bid {
            sfx_bid.bid.clone()
        } else {
            max_reward
        }
    }

    // an sfx_id is generated by hashing its xtx_id with sfx.nonce. This ensures that each sfx has a unique id.
    pub fn generate_id<Hasher: sp_core::Hasher, Hash: frame_system::Config>(
        &self,
        xtx_id: XExecSignalId<Hash>,
    ) -> <Hasher as sp_core::Hasher>::Out {
        let mut xtx_id_and_index = xtx_id.encode();
        xtx_id_and_index.extend(&self.index.to_be_bytes());
        Hasher::hash(xtx_id_and_index.as_ref())
    }
}

impl<AccountId, BlockNumber, BalanceOf>
    TryInto<HardenedSideEffect<AccountId, BlockNumber, BalanceOf>>
    for FullSideEffect<AccountId, BlockNumber, BalanceOf>
where
    AccountId: Encode + Clone,
    BlockNumber: Encode + Clone,
    BalanceOf: Encode + Zero + Clone,
{
    type Error = Error;

    fn try_into(
        self,
    ) -> Result<HardenedSideEffect<AccountId, BlockNumber, BalanceOf>, Self::Error> {
        let confirmation_outcome = self.clone().confirmed.and_then(|c| c.err);
        let confirmed_executioner = self.clone().confirmed.map(|c| c.executioner);
        let confirmed_received_at = self.clone().confirmed.map(|c| c.received_at);
        let confirmed_cost = self.clone().confirmed.and_then(|c| c.cost);
        Ok(HardenedSideEffect::<AccountId, BlockNumber, BalanceOf> {
            target: self.input.target,
            prize: self.input.max_reward,
            encoded_action: TargetId::try_from(self.input.encoded_action.clone())
                .unwrap_or_default(),
            encoded_args: self.input.encoded_args,
            encoded_args_abi: vec![],
            security_lvl: self.security_lvl,
            confirmation_outcome,
            confirmed_executioner,
            confirmed_received_at,
            confirmed_cost,
            index: self.index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bridges::chain_circuit::{
        Balance as CircuitBalance, BlockNumber as CircuitBlockNumber,
    };
    use hex_literal::hex;
    use sp_core::crypto::AccountId32;
    use sp_runtime::testing::H256;
    use std::convert::TryInto;

    type BlockNumber = CircuitBlockNumber;
    type BalanceOf = CircuitBalance;
    type AccountId = AccountId32;
    type Hashing = sp_runtime::traits::BlakeTwo256;

    #[test]
    fn successfully_creates_empty_side_effect() {
        let empty_side_effect = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        assert_eq!(
            empty_side_effect,
            SideEffect {
                target: [0, 0, 0, 0],
                max_reward: 1,
                encoded_action: vec![],
                encoded_args: vec![],
                signature: vec![],
                insurance: 1,
                enforce_executor: None,
                reward_asset_id: None,
            }
        );
    }

    #[test]
    fn successfully_encodes_transfer_full_side_effect_with_confirmation() {
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let optional_insurance = 2u128;
        let optional_reward = 3u128;

        let tsfx_input = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 3,
            insurance: 2,
            encoded_action: vec![],
            encoded_args: vec![
                from.encode(),
                to.encode(),
                value.encode(),
                [optional_insurance.encode(), optional_reward.encode()].concat(),
            ],
            signature: vec![],
            enforce_executor: None,
            reward_asset_id: None,
        };

        let tfsfx = FullSideEffect::<AccountId, BlockNumber, BalanceOf> {
            input: tsfx_input.clone(),
            security_lvl: SecurityLvl::Optimistic,
            submission_target_height: vec![1, 0, 0, 0, 0, 0, 0, 0],
            confirmed: Some(ConfirmedSideEffect::<AccountId, BlockNumber, BalanceOf> {
                err: Some(ConfirmationOutcome::Success),
                output: Some(vec![]),
                inclusion_data: vec![],
                executioner: from,
                received_at: 1 as BlockNumber,
                cost: Some(2 as BalanceOf),
            }),
            best_bid: None,
            index: 0,
        };

        let hsfx: HardenedSideEffect<AccountId, BlockNumber, BalanceOf> = tfsfx.try_into().unwrap();

        assert_eq!(
            hsfx,
            HardenedSideEffect {
                target: [0, 0, 0, 0],
                prize: 3,
                encoded_action: [0, 0, 0, 0],
                encoded_args: vec![
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1
                    ],
                    vec![
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                        2, 2, 2, 2, 2, 2, 2
                    ],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]
                ],
                encoded_args_abi: vec![],
                security_lvl: SecurityLvl::Optimistic,
                confirmation_outcome: Some(ConfirmationOutcome::Success),
                confirmed_executioner: Some(AccountId32::new(hex!(
                    "0101010101010101010101010101010101010101010101010101010101010101"
                ))),
                confirmed_received_at: Some(1),
                confirmed_cost: Some(2),
                index: 0,
            },
        );

        assert_eq!(
            tsfx_input,
            SideEffect {
                target: [0, 0, 0, 0],
                max_reward: 3,
                encoded_action: vec![],
                encoded_args: vec![
                    vec![
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1
                    ],
                    vec![
                        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                        2, 2, 2, 2, 2, 2, 2
                    ],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![
                        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]
                ],
                signature: vec![],
                insurance: 2,
                enforce_executor: None,
                reward_asset_id: None,
            }
        );
    }

    #[test]
    fn successfully_generates_id_for_side_empty_effect() {
        let xtx_id = [0u8; 32];
        let empty_side_effect = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        assert_eq!(
            empty_side_effect.generate_id::<Hashing>(&xtx_id, 0),
            H256::from_slice(&hex!(
                "9f0e444c69f77a49bd0be89db92c38fe713e0963165cca12faf5712d7657120f"
            ))
        );
    }

    #[test]
    fn sfx_ids_do_not_create_collisions() {
        let xtx_id_1 = [0u8; 32];
        let xtx_id_2 = [1u8; 32];

        let empty_side_effect = SideEffect::<AccountId, BalanceOf> {
            target: [0, 0, 0, 0],
            max_reward: 1,
            encoded_action: vec![],
            encoded_args: vec![],
            signature: vec![],
            insurance: 1,
            enforce_executor: None,
            reward_asset_id: None,
        };

        assert_ne!(
            empty_side_effect.generate_id::<Hashing>(&xtx_id_1, 0),
            empty_side_effect.generate_id::<Hashing>(&xtx_id_2, 0),
        );
    }
}
