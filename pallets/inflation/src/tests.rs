use crate::{
    assert_last_event, assert_last_n_events,
    inflation::{CandidateRole, InflationInfo, Range, RewardsAllocationConfig, RoundInfo},
    mock::{Event as MockEvent, *},
    CandidatesForRewards, CurrentRound, Error, Event, InflationConfig, RewardsPerCandidatePerRound,
};
use frame_support::{assert_err, assert_noop, assert_ok};
use sp_runtime::Perbill;

#[test]
fn mint_for_round_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Inflation::mint_for_round(Origin::signed(419), 1, 1_000_000_000),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

// TODO refactor expects wen connected to acct mngr
#[test]
fn mint_for_round_splits_total_rewards_correctly_amongst_actors() {
    new_test_ext().execute_with(|| {
        let dev = 1;
        let exec = 2;
        let total_round_rewards = 10;

        <CandidatesForRewards<Test>>::insert(dev, CandidateRole::Developer, 0);
        <CandidatesForRewards<Test>>::insert(exec, CandidateRole::Executor, 0);

        assert_ok!(Inflation::mint_for_round(
            Origin::root(),
            1,
            total_round_rewards
        ));

        assert_last_n_events!(
            3,
            vec![
                Event::MintedTokensForCandidate(dev, 5),
                Event::MintedTokensForCandidate(exec, 5),
                Event::MintedTokensForRound(<TreasuryAccount>::get(), total_round_rewards)
            ]
        );
    })
}

#[test]
fn claim_rewards_fails_if_none_available() {
    new_test_ext().execute_with(|| {
        <CandidatesForRewards<Test>>::insert(1, CandidateRole::Developer, 0);

        assert_err!(
            Inflation::claim_rewards(Origin::signed(1)),
            Error::<Test>::NoRewardsAvailable
        );
    })
}

#[test]
fn claim_rewards_accumulates_all_past_rounds_rewards() {
    new_test_ext().execute_with(|| {
        // initialize claimer with some balance
        let claimer = 419;
        Balances::set_balance(Origin::root(), claimer, 100, 0).expect("claimer account");

        // configure pallet storage to reward our claimer
        <CandidatesForRewards<Test>>::insert(claimer, CandidateRole::Executor, 0);
        <RewardsPerCandidatePerRound<Test>>::insert(claimer, 1, 1);
        <RewardsPerCandidatePerRound<Test>>::insert(claimer, 2, 1);

        assert_ok!(Inflation::claim_rewards(Origin::signed(claimer)));

        // assert balance allocated
        assert_eq!(Balances::free_balance(&claimer), 102);

        // assert storage is empty for candidate
        let remaining_storage = <RewardsPerCandidatePerRound<Test>>::iter_key_prefix(&1).count();
        assert_eq!(remaining_storage, 0);
    })
}

#[test]
fn set_inflation_requires_root() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(2),
            max: Perbill::from_percent(1),
        };

        assert_noop!(
            Inflation::set_inflation(Origin::signed(419), new_inflation),
            sp_runtime::DispatchError::BadOrigin
        );
    })
}

#[test]
fn set_inflation_fails_given_an_invalid_inflation_range() {
    new_test_ext().execute_with(|| {
        let new_inflation = Range {
            min: Perbill::from_percent(0),
            ideal: Perbill::from_percent(2),
            max: Perbill::from_percent(1),
        };

        assert_err!(
            Inflation::set_inflation(Origin::root(), new_inflation),
            Error::<Test>::InvalidInflationRange
        );
    })
}

#[test]
fn set_inflation_fails_given_the_existing_inflation_range() {
    new_test_ext().execute_with(|| {
        let existing_inflation = Range {
            min: Perbill::from_parts(3),
            ideal: Perbill::from_parts(4),
            max: Perbill::from_parts(5),
        };

        assert_err!(
            Inflation::set_inflation(Origin::root(), existing_inflation),
            Error::<Test>::ValueNotChanged
        );
    })
}

#[test]
fn setting_annual_inflation_derives_round_inflation() {
    new_test_ext().execute_with(|| {
        // input annual inflation config
        let actual_annual_inflation = Range {
            min: Perbill::from_percent(3),
            ideal: Perbill::from_percent(4),
            max: Perbill::from_percent(5),
        };

        // what we expect to get auto derived as round config
        // derivation also depends upon Inflation::current_round().term = 20
        // which must be at least as big as the active colator set
        let expected_round_inflation = Range {
            min: Perbill::from_parts(225),
            ideal: Perbill::from_parts(299),
            max: Perbill::from_parts(372),
        };

        assert_ok!(Inflation::set_inflation(
            Origin::root(),
            actual_annual_inflation
        ));

        // assert new inflation config got stored
        assert_eq!(
            Inflation::inflation_config(),
            InflationInfo {
                annual: actual_annual_inflation,
                round: expected_round_inflation,
                rewards_alloc: RewardsAllocationConfig {
                    developer: Perbill::from_parts(500_000_000),
                    executor: Perbill::from_parts(500_000_000),
                }
            }
        );

        // assert new inflation config was emitted
        assert_last_event!(MockEvent::Inflation(Event::InflationConfigChanged {
            annual_min: actual_annual_inflation.min,
            annual_ideal: actual_annual_inflation.ideal,
            annual_max: actual_annual_inflation.max,
            round_min: expected_round_inflation.min,
            round_ideal: expected_round_inflation.ideal,
            round_max: expected_round_inflation.max,
        }));
    })
}
