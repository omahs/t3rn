use crate::{
    AccountManager as AccountManagerExt, BalanceOf, Config, ContractsRegistryExecutionNonce, Error,
    Outcome, Pallet, PendingChargesPerRound, SettlementsPerRound,
};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResult,
    traits::{Currency, Get, ReservableCurrency},
};
use sp_runtime::{
    traits::{CheckedAdd, CheckedDiv, CheckedMul, Zero},
    ArithmeticError, DispatchError,
};
use sp_std::{prelude::*, vec};

use t3rn_primitives::{
    account_manager::{RequestCharge, Settlement},
    claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
    clock::Clock,
    common::RoundInfo,
    executors::Executors,
};

use pallet_xbi_portal::sabi::Sabi;

pub struct ActiveSetClaimablePerRound<Account, Balance> {
    pub executor: Account,
    pub claimable: Balance,
}

pub fn percent_ratio<BalanceOf: Zero + CheckedDiv + CheckedMul + From<u8>>(
    amt: BalanceOf,
    percent: u8,
) -> Result<BalanceOf, DispatchError> {
    amt.checked_mul(&BalanceOf::from(percent))
        .ok_or::<DispatchError>("PercentRatio::ChargeOrSettlementCalculationOverflow".into())?
        .checked_div(&BalanceOf::from(100u8))
        .ok_or::<DispatchError>("PercentRatio::ChargeOrSettlementCalculationOverflow".into())
}

impl<T: Config> AccountManagerExt<T::AccountId, BalanceOf<T>, T::Hash, T::BlockNumber>
    for Pallet<T>
{
    fn get_charge_or_fail(
        charge_id: T::Hash,
    ) -> Result<RequestCharge<T::AccountId, BalanceOf<T>>, DispatchError> {
        if let Some(pending_charge) =
            PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id)
        {
            Ok(pending_charge)
        } else {
            Err(Error::<T>::NoChargeOfGivenIdRegistered.into())
        }
    }

    fn no_charge_or_fail(charge_id: T::Hash) -> Result<(), DispatchError> {
        if let Some(_pending_charge) =
            PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id)
        {
            Err(Error::<T>::ChargeAlreadyRegistered.into())
        } else {
            Ok(())
        }
    }

    fn bump_contracts_registry_nonce() -> Result<T::Hash, DispatchError> {
        let execution_id = ContractsRegistryExecutionNonce::<T>::get();
        let _ = ContractsRegistryExecutionNonce::<T>::mutate(|nonce| match nonce.checked_add(1) {
            Some(v) => Ok(*nonce = v),
            None => return Err(DispatchError::Arithmetic(ArithmeticError::Overflow)),
        });

        let charge_id = Decode::decode(&mut &Sabi::value_64_2_value_256(execution_id).encode()[..])
            .map_err(|_e| Error::<T>::DecodingExecutionIDFailed)?;

        Self::no_charge_or_fail(charge_id)?;
        Ok(charge_id)
    }

    /// If Called by 3VM as a execution deposit, expect:
    ///     - charge = gas_fees
    ///     - reward = 0
    /// If Called by Circuit as charge deposit, expect:
    ///     - charge = std SFX execution + delivery charge
    ///     - reward = Open Market based offered by requester
    fn deposit(
        charge_id: T::Hash,
        payee: &T::AccountId,
        charge_fee: BalanceOf<T>,
        offered_reward: BalanceOf<T>,
        source: BenefitSource,
        role: CircuitRole,
        maybe_recipient: Option<T::AccountId>,
    ) -> DispatchResult {
        Self::no_charge_or_fail(charge_id).map_err(|_e| Error::<T>::ExecutionAlreadyRegistered)?;

        let total_reserve_deposit = if let Some(v) = charge_fee.checked_add(&offered_reward) {
            v
        } else {
            return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
        };

        if total_reserve_deposit == Zero::zero() {
            return Err(Error::<T>::SkippingEmptyCharges.into())
        }

        T::Currency::reserve(payee, total_reserve_deposit)?;

        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            // todo: Inspect if that's a good idea
            T::EscrowAccount::get()
        };

        PendingChargesPerRound::<T>::insert(
            T::Clock::current_round(),
            charge_id,
            RequestCharge {
                payee: payee.clone(),
                offered_reward,
                charge_fee,
                recipient: recipient.clone(),
                source,
                role,
            },
        );

        Self::deposit_event(crate::Event::DepositReceived {
            charge_id,
            payee: payee.clone(),
            recipient,
            amount: total_reserve_deposit,
        });

        Ok(())
    }

    fn finalize(
        charge_id: T::Hash,
        outcome: Outcome,
        maybe_recipient: Option<T::AccountId>,
        maybe_actual_fees: Option<BalanceOf<T>>,
    ) -> DispatchResult {
        let charge = Self::get_charge_or_fail(charge_id)?;

        // Decide on charges split
        // Simple rules for splitting, for now, we take 1% to keep the account manager alive
        let (payee_split, recipient_split, recipient_bonus): (u8, u8, BalanceOf<T>) = match outcome
        {
            Outcome::Commit => (0, 99, Zero::zero()),
            Outcome::Revert => (99, 0, Zero::zero()),
            Outcome::UnexpectedFailure => (49, 50, Zero::zero()),
        };

        let total_reserved = if let Some(v) = charge.charge_fee.checked_add(&charge.offered_reward)
        {
            v
        } else {
            return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
        };

        let payee_refund: BalanceOf<T> = if let Some(actual_fees) = maybe_actual_fees {
            // ToDo: Better handle case when actual fees outgrow total_reserved
            if actual_fees > total_reserved {
                return Err(Error::<T>::ChargeOrSettlementActualFeesOutgrowReserved.into())
            }
            percent_ratio::<BalanceOf<T>>(total_reserved - actual_fees, payee_split)?
        } else {
            percent_ratio::<BalanceOf<T>>(total_reserved, payee_split)?
        };

        T::Currency::slash_reserved(&charge.payee, total_reserved);
        if payee_refund > Zero::zero() {
            T::Currency::deposit_creating(&charge.payee, payee_refund);
        }

        // Check if recipient has been updated
        let recipient = if let Some(recipient) = maybe_recipient {
            recipient
        } else {
            charge.recipient
        };

        let recipient_rewards = percent_ratio::<BalanceOf<T>>(total_reserved, recipient_split)?;

        // Create Settlement for the future async claim
        if recipient_rewards > Zero::zero() {
            let sttl_amt = if let Some(v) = recipient_rewards.checked_add(&recipient_bonus) {
                v
            } else {
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };
            SettlementsPerRound::<T>::insert(
                T::Clock::current_round(),
                charge_id,
                Settlement::<T::AccountId, BalanceOf<T>> {
                    requester: charge.payee,
                    recipient,
                    settlement_amount: sttl_amt,
                    outcome,
                    source: charge.source,
                    role: charge.role,
                },
            );
        }
        PendingChargesPerRound::<T>::remove(T::Clock::current_round(), charge_id);

        // Take what's left to treasury
        T::Currency::deposit_creating(
            &T::EscrowAccount::get(),
            total_reserved - payee_refund - recipient_rewards,
        );

        Ok(())
    }

    fn try_finalize(
        charge_id: T::Hash,
        outcome: Outcome,
        maybe_recipient: Option<T::AccountId>,
        maybe_actual_fees: Option<BalanceOf<T>>,
    ) {
        if PendingChargesPerRound::<T>::get(T::Clock::current_round(), charge_id).is_some() {
            <Self as AccountManagerExt<T::AccountId, BalanceOf<T>, T::Hash, T::BlockNumber>>::finalize(
                charge_id,
                outcome,
                maybe_recipient,
                maybe_actual_fees
            ).expect("Expect try finalize to be infallible");
        }
    }

    /// Collect claimable (only SFX execution rewards) for Executors and Stakers submitted by Circuit at the duration of the current Round
    fn on_collect_claimable(
        _n: T::BlockNumber,
        r: RoundInfo<T::BlockNumber>,
    ) -> Result<Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>, DispatchError> {
        let mut claimable_artifacts: Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>> = vec![];
        let mut active_set_claimables: Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>> =
            T::Executors::active_set()
                .into_iter()
                .map(|executor: T::AccountId| ActiveSetClaimablePerRound {
                    executor,
                    claimable: Zero::zero(),
                })
                .collect::<Vec<ActiveSetClaimablePerRound<T::AccountId, BalanceOf<T>>>>();

        for settlement in SettlementsPerRound::<T>::iter_prefix_values(r) {
            // fixme: test that actually updates active_set_claimables or are the references wrong
            for active_set_claimable in active_set_claimables.iter_mut() {
                if active_set_claimable.executor == settlement.recipient {
                    if let Some(v) = active_set_claimable
                        .claimable
                        .checked_add(&settlement.settlement_amount)
                    {
                        active_set_claimable.claimable = v
                    } else {
                        return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
                    }
                }
            }
        }

        for active_set_claimable in active_set_claimables {
            let collateral_bond = T::Executors::collateral_bond(&active_set_claimable.executor);
            let nominated_stake =
                T::Executors::total_nominated_stake(&active_set_claimable.executor);
            // calculate % ratio of rewards proportionally to Executor's own Collateral to Nominated Stake
            let total_stake_power = if let Some(v) = collateral_bond.checked_add(&nominated_stake) {
                v
            } else {
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };

            // todo: ensure it's in range (0,1>
            let collateral_bond_power =
                if let Some(v) = collateral_bond.checked_div(&total_stake_power) {
                    v
                } else {
                    return Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero))
                };

            let clltrl_bond_power = if let Some(v) =
                collateral_bond_power.checked_mul(&active_set_claimable.claimable)
            {
                v
            } else {
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };
            claimable_artifacts.push(ClaimableArtifacts {
                beneficiary: active_set_claimable.executor.clone(),
                role: CircuitRole::Executor,
                total_round_claim: clltrl_bond_power,
                benefit_source: BenefitSource::TrafficRewards,
            });

            // todo: ensure it's in range <0,1)
            let nominated_stake_power =
                if let Some(v) = nominated_stake.checked_div(&total_stake_power) {
                    v
                } else {
                    return Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero))
                };

            let claimable_by_all_stakers_of_executor = if let Some(v) =
                nominated_stake_power.checked_mul(&active_set_claimable.claimable)
            {
                v
            } else {
                return Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
            };

            for nominated_stake in T::Executors::stakes_per_executor(&active_set_claimable.executor)
            {
                let staker_power = if let Some(v) = nominated_stake
                    .nominated_stake
                    .checked_div(&nominated_stake_power)
                {
                    v
                } else {
                    return Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero))
                };
                claimable_artifacts.push(ClaimableArtifacts {
                    beneficiary: nominated_stake.staker,
                    role: CircuitRole::Staker,
                    total_round_claim: staker_power * claimable_by_all_stakers_of_executor,
                    benefit_source: BenefitSource::TrafficRewards,
                });
            }
        }

        Ok(claimable_artifacts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_mock_runtime::*;

    use frame_support::{assert_err, assert_ok};

    use sp_core::H256;
    use t3rn_primitives::common::RoundInfo;

    const DEFAULT_BALANCE: Balance = 1_000_000;

    #[test]
    fn test_deposit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                DEFAULT_BALANCE.checked_div(10).unwrap_or(DEFAULT_BALANCE),
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(
                Balances::reserved_balance(&ALICE),
                DEFAULT_BALANCE.checked_div(10).unwrap_or(DEFAULT_BALANCE)
            );

            let charge_item = AccountManager::pending_charges_per_round::<
                RoundInfo<BlockNumber>,
                H256,
            >(Default::default(), execution_id)
            .unwrap();
            assert_eq!(charge_item.payee, ALICE);
            assert_eq!(charge_item.recipient, BOB);
            assert_eq!(
                charge_item.charge_fee,
                DEFAULT_BALANCE.checked_div(10).unwrap_or(DEFAULT_BALANCE)
            );
        });
    }

    #[test]
    fn test_deposit_when_already_exist_fails() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);

            let execution_id: H256 = H256::repeat_byte(0);
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                DEFAULT_BALANCE.checked_div(10).unwrap_or(DEFAULT_BALANCE),
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_err!(
                <AccountManager as AccountManagerExt<
                    AccountId,
                    Balance,
                    Hash,
                    BlockNumber,
                >>::deposit(
                    execution_id,
                    &ALICE,
                    DEFAULT_BALANCE.checked_div(10).unwrap_or(DEFAULT_BALANCE),
                    0,
                    BenefitSource::TrafficRewards,
                    CircuitRole::ContractAuthor,
                    Some(BOB),
                ),
                pallet_account_manager::Error::<Runtime>::ExecutionAlreadyRegistered
            );
        });
    }

    #[test]
    fn test_finalize_revert_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::Revert, None, None,
            ));

            let one_percent_charge_amt = charge_amt.checked_div(100).unwrap_or(charge_amt);
            let _ten_percent_charge_amt = charge_amt.checked_div(10).unwrap_or(charge_amt);

            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt
                    .checked_add(DEFAULT_BALANCE)
                    .unwrap_or(one_percent_charge_amt) // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE
                    .checked_sub(one_percent_charge_amt)
                    .unwrap_or(DEFAULT_BALANCE)
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            // Expect no settlement at revert
            assert_eq!(
                AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );
        });
    }

    #[test]
    fn test_overflow_err_after_actual_fees_exceed_deposit() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            const CHARGE: Balance = 100;
            const INSURANCE: Balance = 10;

            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                CHARGE,
                INSURANCE,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), CHARGE.checked_add(INSURANCE).unwrap_or(CHARGE));

            assert_err!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::Revert, None, Some(CHARGE.checked_add(INSURANCE).unwrap_or(CHARGE).checked_add(1).unwrap_or(CHARGE)),
            ),
                circuit_runtime_pallets::pallet_account_manager::Error::<Runtime>::ChargeOrSettlementActualFeesOutgrowReserved,
            );
        });
    }

    #[test]
    fn percent_ratio_works_for_zero() {
        ExtBuilder::default().build().execute_with(|| {
            assert_eq!(percent_ratio::<Balance>(0, 100).unwrap(), 0);
            assert_eq!(percent_ratio::<Balance>(100, 0).unwrap(), 0);
            assert_eq!(percent_ratio::<Balance>(10, 100).unwrap(), 10);
            assert_eq!(percent_ratio::<Balance>(100, 10).unwrap(), 10);
        });
    }

    #[test]
    fn test_finalize_commit_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::Commit, None, None,
            ));

            let one_percent_charge_amt = charge_amt.checked_div(100).unwrap_or(charge_amt);
            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt
                    .checked_add(DEFAULT_BALANCE)
                    .unwrap_or(one_percent_charge_amt) // 1% left now
            );
            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE
                    .checked_sub(charge_amt)
                    .unwrap_or(DEFAULT_BALANCE)
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            )
            .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(
                settlement.settlement_amount,
                charge_amt - one_percent_charge_amt
            );
        });
    }

    #[test]
    fn test_finalize_unexpected_failure_works() {
        ExtBuilder::default().build().execute_with(|| {
            let _ = Balances::deposit_creating(&ALICE, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(&BOB, DEFAULT_BALANCE);
            let _ = Balances::deposit_creating(
                &<Runtime as pallet_account_manager::Config>::EscrowAccount::get(),
                DEFAULT_BALANCE,
            );
            let charge_amt = 100;
            let execution_id: H256 = H256::repeat_byte(0);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::deposit(
                execution_id,
                &ALICE,
                charge_amt,
                0,
                BenefitSource::TrafficRewards,
                CircuitRole::ContractAuthor,
                Some(BOB),
            ));

            assert_eq!(Balances::reserved_balance(&ALICE), charge_amt);

            assert_ok!(<AccountManager as AccountManagerExt<
                AccountId,
                Balance,
                Hash,
                BlockNumber,
            >>::finalize(
                execution_id, Outcome::UnexpectedFailure, None, None,
            ));

            let test_one_percent_charge_amt = charge_amt.checked_div(100);
            assert_ne!(test_one_percent_charge_amt, None);

            let one_percent_charge_amt = charge_amt.checked_div(100).unwrap_or(charge_amt);
            let fifty_percent_charge_amt = charge_amt
                .checked_div(100)
                .unwrap_or(charge_amt)
                .checked_mul(50)
                .unwrap_or(charge_amt);

            assert_eq!(
                Balances::free_balance(
                    &<Runtime as pallet_account_manager::Config>::EscrowAccount::get()
                ),
                one_percent_charge_amt
                    .checked_add(DEFAULT_BALANCE)
                    .unwrap_or(one_percent_charge_amt) // 1% left now
            );

            assert_eq!(
                Balances::free_balance(&ALICE),
                DEFAULT_BALANCE
                    .checked_sub(fifty_percent_charge_amt)
                    .unwrap_or(DEFAULT_BALANCE)
                    .checked_sub(one_percent_charge_amt)
                    .unwrap_or(DEFAULT_BALANCE)
            );

            assert_eq!(
                AccountManager::pending_charges_per_round::<RoundInfo<BlockNumber>, H256>(
                    Default::default(),
                    execution_id,
                ),
                None
            );

            let settlement = AccountManager::settlements_per_round::<RoundInfo<BlockNumber>, H256>(
                Default::default(),
                execution_id,
            )
            .unwrap();

            assert_eq!(settlement.requester, ALICE);
            assert_eq!(settlement.recipient, BOB);
            assert_eq!(settlement.settlement_amount, fifty_percent_charge_amt);
        });
    }
}
