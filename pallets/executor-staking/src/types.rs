
use crate::{
	set::OrderedSet, BalanceOf, BottomDelegations, CandidateInfo, Config, DelegatorState, Error,
	Event, Pallet, Round, RoundIndex, TopDelegations, Total,
};
use frame_support::{pallet_prelude::*, traits::ReservableCurrency};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Saturating, Zero},
	Perbill, Percent, RuntimeDebug,
};
use sp_std::{cmp::Ordering, collections::btree_map::BTreeMap, prelude::*};

#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
/// All candidate info except the top and bottom delegations
pub struct CandidateMetadata<Balance> {
	/// This candidate's self bond amount
	pub bond: Balance,
	/// Total number of delegations to this candidate
	pub delegation_count: u32,
	/// Self bond + sum of top delegations
	pub total_counted: Balance,
	/// The smallest top delegation amount
	pub lowest_top_delegation_amount: Balance,
	/// The highest bottom delegation amount
	pub highest_bottom_delegation_amount: Balance,
	/// The smallest bottom delegation amount
	pub lowest_bottom_delegation_amount: Balance,
	/// Capacity status for top delegations
	pub top_capacity: CapacityStatus,
	/// Capacity status for bottom delegations
	pub bottom_capacity: CapacityStatus,
	/// Maximum 1 pending request to decrease candidate self bond at any given time
	pub request: Option<CandidateBondLessRequest<Balance>>,
	/// Current status of the executor
	pub status: ExecutorStatus,
}

impl<
		Balance: Copy
			+ Zero
			+ PartialOrd
			+ sp_std::ops::AddAssign
			+ sp_std::ops::SubAssign
			+ sp_std::ops::Sub<Output = Balance>
			+ sp_std::fmt::Debug
			+ Saturating,
	> CandidateMetadata<Balance>
{
	pub fn new(bond: Balance) -> Self {
		CandidateMetadata {
			bond,
			delegation_count: 0u32,
			total_counted: bond,
			lowest_top_delegation_amount: Zero::zero(),
			highest_bottom_delegation_amount: Zero::zero(),
			lowest_bottom_delegation_amount: Zero::zero(),
			top_capacity: CapacityStatus::Empty,
			bottom_capacity: CapacityStatus::Empty,
			request: None,
			status: ExecutorStatus::Active,
		}
	}
	pub fn is_active(&self) -> bool {
		matches!(self.status, ExecutorStatus::Active)
	}
	pub fn is_leaving(&self) -> bool {
		matches!(self.status, ExecutorStatus::Leaving(_))
	}
	pub fn schedule_leave<T: Config>(&mut self) -> Result<(RoundIndex, RoundIndex), DispatchError> {
		ensure!(!self.is_leaving(), Error::<T>::CandidateAlreadyLeaving);
		let now = <Round<T>>::get().current;
		let when = now + T::LeaveCandidatesDelay::get();
		self.status = ExecutorStatus::Leaving(when);
		Ok((now, when))
	}
	pub fn can_leave<T: Config>(&self) -> DispatchResult {
		if let ExecutorStatus::Leaving(when) = self.status {
			ensure!(
				<Round<T>>::get().current >= when,
				Error::<T>::CandidateCannotLeaveYet
			);
			Ok(())
		} else {
			Err(Error::<T>::CandidateNotLeaving.into())
		}
	}
	pub fn go_offline(&mut self) {
		self.status = ExecutorStatus::Idle;
	}
	pub fn go_online(&mut self) {
		self.status = ExecutorStatus::Active;
	}
	pub fn bond_more<T: Config>(&mut self, who: T::AccountId, more: Balance) -> DispatchResult
	where
		BalanceOf<T>: From<Balance>,
	{
		T::Currency::reserve(&who, more.into())?;
		let new_total = <Total<T>>::get().saturating_add(more.into());
		<Total<T>>::put(new_total);
		self.bond = self.bond.saturating_add(more);
		self.total_counted = self.total_counted.saturating_add(more);
		<Pallet<T>>::deposit_event(Event::CandidateBondedMore {
			candidate: who.clone(),
			amount: more.into(),
			new_total_bond: self.bond.into(),
		});
		Ok(())
	}
	/// Schedule executable decrease of executor candidate self bond
	/// Returns the round at which the executor can execute the pending request
	pub fn schedule_bond_less<T: Config>(
		&mut self,
		less: Balance,
	) -> Result<RoundIndex, DispatchError>
	where
		BalanceOf<T>: Into<Balance>,
	{
		// ensure no pending request
		ensure!(
			self.request.is_none(),
			Error::<T>::PendingCandidateRequestAlreadyExists
		);
		// ensure bond above min after decrease
		ensure!(self.bond > less, Error::<T>::CandidateBondBelowMin);
		ensure!(
			self.bond - less >= T::MinCandidateStk::get().into(),
			Error::<T>::CandidateBondBelowMin
		);
		let when_executable = <Round<T>>::get().current + T::CandidateBondLessDelay::get();
		self.request = Some(CandidateBondLessRequest {
			amount: less,
			when_executable,
		});
		Ok(when_executable)
	}
	/// Execute pending request to decrease the executor self bond
	/// Returns the event to be emitted
	pub fn execute_bond_less<T: Config>(&mut self, who: T::AccountId) -> DispatchResult
	where
		BalanceOf<T>: From<Balance>,
	{
		let request = self
			.request
			.ok_or(Error::<T>::PendingCandidateRequestsDNE)?;
		ensure!(
			request.when_executable <= <Round<T>>::get().current,
			Error::<T>::PendingCandidateRequestNotDueYet
		);
		T::Currency::unreserve(&who, request.amount.into());
		let new_total_staked = <Total<T>>::get().saturating_sub(request.amount.into());
		<Total<T>>::put(new_total_staked);
		// Arithmetic assumptions are self.bond > less && self.bond - less > CollatorMinBond
		// (assumptions enforced by `schedule_bond_less`; if storage corrupts, must re-verify)
		self.bond = self.bond.saturating_sub(request.amount);
		self.total_counted = self.total_counted.saturating_sub(request.amount);
		let event = Event::CandidateBondedLess {
			candidate: who.clone().into(),
			amount: request.amount.into(),
			new_bond: self.bond.into(),
		};
		// reset s.t. no pending request
		self.request = None;
		// update candidate pool value because it must change if self bond changes
		if self.is_active() {
			Pallet::<T>::update_active(who.into(), self.total_counted.into());
		}
		Pallet::<T>::deposit_event(event);
		Ok(())
	}
	/// Cancel candidate bond less request
	pub fn cancel_bond_less<T: Config>(&mut self, who: T::AccountId) -> DispatchResult
	where
		BalanceOf<T>: From<Balance>,
	{
		let request = self
			.request
			.ok_or(Error::<T>::PendingCandidateRequestsDNE)?;
		let event = Event::CancelledCandidateBondLess {
			candidate: who.clone().into(),
			amount: request.amount.into(),
			execute_round: request.when_executable,
		};
		self.request = None;
		Pallet::<T>::deposit_event(event);
		Ok(())
	}
	/// Reset top delegations metadata
	pub fn reset_top_data<T: Config>(
		&mut self,
		candidate: T::AccountId,
		top_delegations: &Delegations<T::AccountId, BalanceOf<T>>,
	) where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		self.lowest_top_delegation_amount = top_delegations.lowest_delegation_amount().into();
		self.top_capacity = top_delegations.top_capacity::<T>();
		let old_total_counted = self.total_counted;
		self.total_counted = self.bond.saturating_add(top_delegations.total.into());
		// CandidatePool value for candidate always changes if top delegations total changes
		// so we moved the update into this function to deduplicate code and patch a bug that
		// forgot to apply the update when increasing top delegation
		if old_total_counted != self.total_counted && self.is_active() {
			Pallet::<T>::update_active(candidate, self.total_counted.into());
		}
	}
	/// Reset bottom delegations metadata
	pub fn reset_bottom_data<T: Config>(
		&mut self,
		bottom_delegations: &Delegations<T::AccountId, BalanceOf<T>>,
	) where
		BalanceOf<T>: Into<Balance>,
	{
		self.lowest_bottom_delegation_amount = bottom_delegations.lowest_delegation_amount().into();
		self.highest_bottom_delegation_amount =
			bottom_delegations.highest_delegation_amount().into();
		self.bottom_capacity = bottom_delegations.bottom_capacity::<T>();
	}
	/// Add delegation
	/// Returns whether delegator was added and an optional negative total counted remainder
	/// for if a bottom delegation was kicked
	/// MUST ensure no delegation exists for this candidate in the `DelegatorState` before call
	pub fn add_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegation: Bond<T::AccountId, BalanceOf<T>>,
	) -> Result<(DelegatorAdded<Balance>, Option<Balance>), DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let mut less_total_staked = None;
		let delegator_added = match self.top_capacity {
			CapacityStatus::Full => {
				// top is full, insert into top iff the lowest_top < amount
				if self.lowest_top_delegation_amount < delegation.amount.into() {
					// bumps lowest top to the bottom inside this function call
					less_total_staked = self.add_top_delegation::<T>(candidate, delegation);
					DelegatorAdded::AddedToTop {
						new_total: self.total_counted,
					}
				} else {
					// if bottom is full, only insert if greater than lowest bottom (which will
					// be bumped out)
					if matches!(self.bottom_capacity, CapacityStatus::Full) {
						ensure!(
							delegation.amount.into() > self.lowest_bottom_delegation_amount,
							Error::<T>::CannotDelegateLessThanOrEqualToLowestBottomWhenFull
						);
						// need to subtract from total staked
						less_total_staked = Some(self.lowest_bottom_delegation_amount);
					}
					// insert into bottom
					self.add_bottom_delegation::<T>(false, candidate, delegation);
					DelegatorAdded::AddedToBottom
				}
			}
			// top is either empty or partially full
			_ => {
				self.add_top_delegation::<T>(candidate, delegation);
				DelegatorAdded::AddedToTop {
					new_total: self.total_counted,
				}
			}
		};
		Ok((delegator_added, less_total_staked))
	}
	/// Add delegation to top delegation
	/// Returns Option<negative_total_staked_remainder>
	/// Only call if lowest top delegation is less than delegation.amount || !top_full
	pub fn add_top_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegation: Bond<T::AccountId, BalanceOf<T>>,
	) -> Option<Balance>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let mut less_total_staked = None;
		let mut top_delegations = <TopDelegations<T>>::get(candidate)
			.expect("CandidateInfo existence => TopDelegations existence");
		let max_top_delegations_per_candidate = T::MaxTopDelegationsPerCandidate::get();
		if top_delegations.delegations.len() as u32 == max_top_delegations_per_candidate {
			// pop lowest top delegation
			let new_bottom_delegation = top_delegations.delegations.pop().expect("");
			top_delegations.total = top_delegations
				.total
				.saturating_sub(new_bottom_delegation.amount);
			if matches!(self.bottom_capacity, CapacityStatus::Full) {
				less_total_staked = Some(self.lowest_bottom_delegation_amount);
			}
			self.add_bottom_delegation::<T>(true, candidate, new_bottom_delegation);
		}
		// insert into top
		top_delegations.insert_sorted_greatest_to_least(delegation);
		// update candidate info
		self.reset_top_data::<T>(candidate.clone(), &top_delegations);
		if less_total_staked.is_none() {
			// only increment delegation count if we are not kicking a bottom delegation
			self.delegation_count = self.delegation_count.saturating_add(1u32);
		}
		<TopDelegations<T>>::insert(&candidate, top_delegations);
		less_total_staked
	}
	/// Add delegation to bottom delegations
	/// Check before call that if capacity is full, inserted delegation is higher than lowest
	/// bottom delegation (and if so, need to adjust the total storage item)
	/// CALLER MUST ensure(lowest_bottom_to_be_kicked.amount < delegation.amount)
	pub fn add_bottom_delegation<T: Config>(
		&mut self,
		bumped_from_top: bool,
		candidate: &T::AccountId,
		delegation: Bond<T::AccountId, BalanceOf<T>>,
	) where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let mut bottom_delegations = <BottomDelegations<T>>::get(candidate)
			.expect("CandidateInfo existence => BottomDelegations existence");
		// if bottom is full, kick the lowest bottom (which is expected to be lower than input
		// as per check)
		let increase_delegation_count = if bottom_delegations.delegations.len() as u32
			== T::MaxBottomDelegationsPerCandidate::get()
		{
			let lowest_bottom_to_be_kicked = bottom_delegations
				.delegations
				.pop()
				.expect("if at full capacity (>0), then >0 bottom delegations exist; qed");
			// EXPECT lowest_bottom_to_be_kicked.amount < delegation.amount enforced by caller
			// if lowest_bottom_to_be_kicked.amount == delegation.amount, we will still kick
			// the lowest bottom to enforce first come first served
			bottom_delegations.total = bottom_delegations
				.total
				.saturating_sub(lowest_bottom_to_be_kicked.amount);
			// update delegator state
			// unreserve kicked bottom
			T::Currency::unreserve(
				&lowest_bottom_to_be_kicked.owner,
				lowest_bottom_to_be_kicked.amount,
			);
			// total staked is updated via propagation of lowest bottom delegation amount prior
			// to call
			let mut delegator_state = <DelegatorState<T>>::get(&lowest_bottom_to_be_kicked.owner)
				.expect("Delegation existence => DelegatorState existence");
			let leaving = delegator_state.delegations.0.len() == 1usize;
			delegator_state.rm_delegation(candidate);
			<Pallet<T>>::delegation_remove_request_with_state(
				&candidate,
				&lowest_bottom_to_be_kicked.owner,
				&mut delegator_state,
			);

			Pallet::<T>::deposit_event(Event::DelegationKicked {
				delegator: lowest_bottom_to_be_kicked.owner.clone(),
				candidate: candidate.clone(),
				unstaked_amount: lowest_bottom_to_be_kicked.amount,
			});
			if leaving {
				<DelegatorState<T>>::remove(&lowest_bottom_to_be_kicked.owner);
				Pallet::<T>::deposit_event(Event::DelegatorLeft {
					delegator: lowest_bottom_to_be_kicked.owner,
					unstaked_amount: lowest_bottom_to_be_kicked.amount,
				});
			} else {
				<DelegatorState<T>>::insert(&lowest_bottom_to_be_kicked.owner, delegator_state);
			}
			false
		} else {
			!bumped_from_top
		};
		// only increase delegation count if new bottom delegation (1) doesn't come from top &&
		// (2) doesn't pop the lowest delegation from the bottom
		if increase_delegation_count {
			self.delegation_count = self.delegation_count.saturating_add(1u32);
		}
		bottom_delegations.insert_sorted_greatest_to_least(delegation);
		self.reset_bottom_data::<T>(&bottom_delegations);
		<BottomDelegations<T>>::insert(candidate, bottom_delegations);
	}
	/// Remove delegation
	/// Removes from top if amount is above lowest top or top is not full
	/// Return Ok(if_total_counted_changed)
	pub fn rm_delegation_if_exists<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		amount: Balance,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let amount_geq_lowest_top = amount >= self.lowest_top_delegation_amount;
		let top_is_not_full = !matches!(self.top_capacity, CapacityStatus::Full);
		let lowest_top_eq_highest_bottom =
			self.lowest_top_delegation_amount == self.highest_bottom_delegation_amount;
		let delegation_dne_err: DispatchError = Error::<T>::DelegationDNE.into();
		if top_is_not_full || (amount_geq_lowest_top && !lowest_top_eq_highest_bottom) {
			self.rm_top_delegation::<T>(candidate, delegator)
		} else if amount_geq_lowest_top && lowest_top_eq_highest_bottom {
			let result = self.rm_top_delegation::<T>(candidate, delegator.clone());
			if result == Err(delegation_dne_err) {
				// worst case removal
				self.rm_bottom_delegation::<T>(candidate, delegator)
			} else {
				result
			}
		} else {
			self.rm_bottom_delegation::<T>(candidate, delegator)
		}
	}
	/// Remove top delegation, bumps top bottom delegation if exists
	pub fn rm_top_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let old_total_counted = self.total_counted;
		// remove top delegation
		let mut top_delegations = <TopDelegations<T>>::get(candidate)
			.expect("CandidateInfo exists => TopDelegations exists");
		let mut actual_amount_option: Option<BalanceOf<T>> = None;
		top_delegations.delegations = top_delegations
			.delegations
			.clone()
			.into_iter()
			.filter(|d| {
				if d.owner != delegator {
					true
				} else {
					actual_amount_option = Some(d.amount);
					false
				}
			})
			.collect();
		let actual_amount = actual_amount_option.ok_or(Error::<T>::DelegationDNE)?;
		top_delegations.total = top_delegations.total.saturating_sub(actual_amount);
		// if bottom nonempty => bump top bottom to top
		if !matches!(self.bottom_capacity, CapacityStatus::Empty) {
			let mut bottom_delegations =
				<BottomDelegations<T>>::get(candidate).expect("bottom is nonempty as just checked");
			// expect already stored greatest to least by bond amount
			let highest_bottom_delegation = bottom_delegations.delegations.remove(0);
			bottom_delegations.total = bottom_delegations
				.total
				.saturating_sub(highest_bottom_delegation.amount);
			self.reset_bottom_data::<T>(&bottom_delegations);
			<BottomDelegations<T>>::insert(candidate, bottom_delegations);
			// insert highest bottom into top delegations
			top_delegations.insert_sorted_greatest_to_least(highest_bottom_delegation);
		}
		// update candidate info
		self.reset_top_data::<T>(candidate.clone(), &top_delegations);
		self.delegation_count = self.delegation_count.saturating_sub(1u32);
		<TopDelegations<T>>::insert(candidate, top_delegations);
		// return whether total counted changed
		Ok(old_total_counted == self.total_counted)
	}
	/// Remove bottom delegation
	/// Returns if_total_counted_changed: bool
	pub fn rm_bottom_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance>,
	{
		// remove bottom delegation
		let mut bottom_delegations = <BottomDelegations<T>>::get(candidate)
			.expect("CandidateInfo exists => BottomDelegations exists");
		let mut actual_amount_option: Option<BalanceOf<T>> = None;
		bottom_delegations.delegations = bottom_delegations
			.delegations
			.clone()
			.into_iter()
			.filter(|d| {
				if d.owner != delegator {
					true
				} else {
					actual_amount_option = Some(d.amount);
					false
				}
			})
			.collect();
		let actual_amount = actual_amount_option.ok_or(Error::<T>::DelegationDNE)?;
		bottom_delegations.total = bottom_delegations.total.saturating_sub(actual_amount);
		// update candidate info
		self.reset_bottom_data::<T>(&bottom_delegations);
		self.delegation_count = self.delegation_count.saturating_sub(1u32);
		<BottomDelegations<T>>::insert(candidate, bottom_delegations);
		Ok(false)
	}
	/// Increase delegation amount
	pub fn increase_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		bond: BalanceOf<T>,
		more: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let lowest_top_eq_highest_bottom =
			self.lowest_top_delegation_amount == self.highest_bottom_delegation_amount;
		let bond_geq_lowest_top = bond.into() >= self.lowest_top_delegation_amount;
		let delegation_dne_err: DispatchError = Error::<T>::DelegationDNE.into();
		if bond_geq_lowest_top && !lowest_top_eq_highest_bottom {
			// definitely in top
			self.increase_top_delegation::<T>(candidate, delegator.clone(), more)
		} else if bond_geq_lowest_top && lowest_top_eq_highest_bottom {
			// update top but if error then update bottom (because could be in bottom because
			// lowest_top_eq_highest_bottom)
			let result = self.increase_top_delegation::<T>(candidate, delegator.clone(), more);
			if result == Err(delegation_dne_err) {
				self.increase_bottom_delegation::<T>(candidate, delegator, bond, more)
			} else {
				result
			}
		} else {
			self.increase_bottom_delegation::<T>(candidate, delegator, bond, more)
		}
	}
	/// Increase top delegation
	pub fn increase_top_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		more: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let mut top_delegations = <TopDelegations<T>>::get(candidate)
			.expect("CandidateInfo exists => TopDelegations exists");
		let mut in_top = false;
		top_delegations.delegations = top_delegations
			.delegations
			.clone()
			.into_iter()
			.map(|d| {
				if d.owner != delegator {
					d
				} else {
					in_top = true;
					let new_amount = d.amount.saturating_add(more);
					Bond {
						owner: d.owner,
						amount: new_amount,
					}
				}
			})
			.collect();
		ensure!(in_top, Error::<T>::DelegationDNE);
		top_delegations.total = top_delegations.total.saturating_add(more);
		top_delegations.sort_greatest_to_least();
		self.reset_top_data::<T>(candidate.clone(), &top_delegations);
		<TopDelegations<T>>::insert(candidate, top_delegations);
		Ok(true)
	}
	/// Increase bottom delegation
	pub fn increase_bottom_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		bond: BalanceOf<T>,
		more: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let mut bottom_delegations =
			<BottomDelegations<T>>::get(candidate).ok_or(Error::<T>::CandidateDNE)?;
		let mut delegation_option: Option<Bond<T::AccountId, BalanceOf<T>>> = None;
		let in_top_after = if (bond.saturating_add(more)).into() > self.lowest_top_delegation_amount
		{
			// bump it from bottom
			bottom_delegations.delegations = bottom_delegations
				.delegations
				.clone()
				.into_iter()
				.filter(|d| {
					if d.owner != delegator {
						true
					} else {
						delegation_option = Some(Bond {
							owner: d.owner.clone(),
							amount: d.amount.saturating_add(more),
						});
						false
					}
				})
				.collect();
			let delegation = delegation_option.ok_or(Error::<T>::DelegationDNE)?;
			bottom_delegations.total = bottom_delegations.total.saturating_sub(bond);
			// add it to top
			let mut top_delegations = <TopDelegations<T>>::get(candidate)
				.expect("CandidateInfo existence => TopDelegations existence");
			// if top is full, pop lowest top
			if matches!(top_delegations.top_capacity::<T>(), CapacityStatus::Full) {
				// pop lowest top delegation
				let new_bottom_delegation = top_delegations
					.delegations
					.pop()
					.expect("Top capacity full => Exists at least 1 top delegation");
				top_delegations.total = top_delegations
					.total
					.saturating_sub(new_bottom_delegation.amount);
				bottom_delegations.insert_sorted_greatest_to_least(new_bottom_delegation);
			}
			// insert into top
			top_delegations.insert_sorted_greatest_to_least(delegation);
			self.reset_top_data::<T>(candidate.clone(), &top_delegations);
			<TopDelegations<T>>::insert(candidate, top_delegations);
			true
		} else {
			let mut in_bottom = false;
			// just increase the delegation
			bottom_delegations.delegations = bottom_delegations
				.delegations
				.clone()
				.into_iter()
				.map(|d| {
					if d.owner != delegator {
						d
					} else {
						in_bottom = true;
						Bond {
							owner: d.owner,
							amount: d.amount.saturating_add(more),
						}
					}
				})
				.collect();
			ensure!(in_bottom, Error::<T>::DelegationDNE);
			bottom_delegations.total = bottom_delegations.total.saturating_add(more);
			bottom_delegations.sort_greatest_to_least();
			false
		};
		self.reset_bottom_data::<T>(&bottom_delegations);
		<BottomDelegations<T>>::insert(candidate, bottom_delegations);
		Ok(in_top_after)
	}
	/// Decrease delegation
	pub fn decrease_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		bond: Balance,
		less: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		let lowest_top_eq_highest_bottom =
			self.lowest_top_delegation_amount == self.highest_bottom_delegation_amount;
		let bond_geq_lowest_top = bond >= self.lowest_top_delegation_amount;
		let delegation_dne_err: DispatchError = Error::<T>::DelegationDNE.into();
		if bond_geq_lowest_top && !lowest_top_eq_highest_bottom {
			// definitely in top
			self.decrease_top_delegation::<T>(candidate, delegator.clone(), bond.into(), less)
		} else if bond_geq_lowest_top && lowest_top_eq_highest_bottom {
			// update top but if error then update bottom (because could be in bottom because
			// lowest_top_eq_highest_bottom)
			let result =
				self.decrease_top_delegation::<T>(candidate, delegator.clone(), bond.into(), less);
			if result == Err(delegation_dne_err) {
				self.decrease_bottom_delegation::<T>(candidate, delegator, less)
			} else {
				result
			}
		} else {
			self.decrease_bottom_delegation::<T>(candidate, delegator, less)
		}
	}
	/// Decrease top delegation
	pub fn decrease_top_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		bond: BalanceOf<T>,
		less: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance> + From<Balance>,
	{
		// The delegation after the `decrease-delegation` will be strictly less than the
		// highest bottom delegation
		let bond_after_less_than_highest_bottom =
			bond.saturating_sub(less).into() < self.highest_bottom_delegation_amount;
		// The top delegations is full and the bottom delegations has at least one delegation
		let full_top_and_nonempty_bottom = matches!(self.top_capacity, CapacityStatus::Full)
			&& !matches!(self.bottom_capacity, CapacityStatus::Empty);
		let mut top_delegations =
			<TopDelegations<T>>::get(candidate).ok_or(Error::<T>::CandidateDNE)?;
		let in_top_after = if bond_after_less_than_highest_bottom && full_top_and_nonempty_bottom {
			let mut delegation_option: Option<Bond<T::AccountId, BalanceOf<T>>> = None;
			// take delegation from top
			top_delegations.delegations = top_delegations
				.delegations
				.clone()
				.into_iter()
				.filter(|d| {
					if d.owner != delegator {
						true
					} else {
						top_delegations.total = top_delegations.total.saturating_sub(d.amount);
						delegation_option = Some(Bond {
							owner: d.owner.clone(),
							amount: d.amount.saturating_sub(less),
						});
						false
					}
				})
				.collect();
			let delegation = delegation_option.ok_or(Error::<T>::DelegationDNE)?;
			// pop highest bottom by reverse and popping
			let mut bottom_delegations = <BottomDelegations<T>>::get(candidate)
				.expect("CandidateInfo existence => BottomDelegations existence");
			let highest_bottom_delegation = bottom_delegations.delegations.remove(0);
			bottom_delegations.total = bottom_delegations
				.total
				.saturating_sub(highest_bottom_delegation.amount);
			// insert highest bottom into top
			top_delegations.insert_sorted_greatest_to_least(highest_bottom_delegation);
			// insert previous top into bottom
			bottom_delegations.insert_sorted_greatest_to_least(delegation);
			self.reset_bottom_data::<T>(&bottom_delegations);
			<BottomDelegations<T>>::insert(candidate, bottom_delegations);
			false
		} else {
			// keep it in the top
			let mut is_in_top = false;
			top_delegations.delegations = top_delegations
				.delegations
				.clone()
				.into_iter()
				.map(|d| {
					if d.owner != delegator {
						d
					} else {
						is_in_top = true;
						Bond {
							owner: d.owner,
							amount: d.amount.saturating_sub(less),
						}
					}
				})
				.collect();
			ensure!(is_in_top, Error::<T>::DelegationDNE);
			top_delegations.total = top_delegations.total.saturating_sub(less);
			top_delegations.sort_greatest_to_least();
			true
		};
		self.reset_top_data::<T>(candidate.clone(), &top_delegations);
		<TopDelegations<T>>::insert(candidate, top_delegations);
		Ok(in_top_after)
	}
	/// Decrease bottom delegation
	pub fn decrease_bottom_delegation<T: Config>(
		&mut self,
		candidate: &T::AccountId,
		delegator: T::AccountId,
		less: BalanceOf<T>,
	) -> Result<bool, DispatchError>
	where
		BalanceOf<T>: Into<Balance>,
	{
		let mut bottom_delegations = <BottomDelegations<T>>::get(candidate)
			.expect("CandidateInfo exists => BottomDelegations exists");
		let mut in_bottom = false;
		bottom_delegations.delegations = bottom_delegations
			.delegations
			.clone()
			.into_iter()
			.map(|d| {
				if d.owner != delegator {
					d
				} else {
					in_bottom = true;
					Bond {
						owner: d.owner,
						amount: d.amount.saturating_sub(less),
					}
				}
			})
			.collect();
		ensure!(in_bottom, Error::<T>::DelegationDNE);
		bottom_delegations.sort_greatest_to_least();
		self.reset_bottom_data::<T>(&bottom_delegations);
		<BottomDelegations<T>>::insert(candidate, bottom_delegations);
		Ok(false)
	}
}