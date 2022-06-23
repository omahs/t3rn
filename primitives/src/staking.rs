use crate::common::{OrderedSet, RoundIndex, RoundInfo};
use frame_support::{pallet_prelude::*, traits::ReservableCurrency};
use codec::{Decode, Encode};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Saturating, Zero},
	Perbill, Percent, RuntimeDebug,
};
use sp_std::{cmp::Ordering, collections::btree_map::BTreeMap, prelude::*};

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Bond<AccountId, Balance> {
	pub owner: AccountId,
	pub amount: Balance,
}

impl<A: Decode, B: Default> Default for Bond<A, B> {
	fn default() -> Bond<A, B> {
		Bond {
			owner: A::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
				.expect("infinite length input; no invalid inputs for type; qed"),
			amount: B::default(),
		}
	}
}

impl<A, B: Default> Bond<A, B> {
	pub fn from_owner(owner: A) -> Self {
		Bond {
			owner,
			amount: B::default(),
		}
	}
}

impl<AccountId: Ord, Balance> Eq for Bond<AccountId, Balance> {}

impl<AccountId: Ord, Balance> Ord for Bond<AccountId, Balance> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.owner.cmp(&other.owner)
	}
}

impl<AccountId: Ord, Balance> PartialOrd for Bond<AccountId, Balance> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<AccountId: Ord, Balance> PartialEq for Bond<AccountId, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.owner == other.owner
	}
}

#[derive(PartialEq, Clone, Copy, Encode, Decode, RuntimeDebug, TypeInfo)]
/// Request scheduled to change the executor candidate self-bond
pub struct CandidateBondLessRequest<Balance> {
	pub amount: Balance,
	pub when_executable: RoundIndex,
}

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// The activity status of the executor
pub enum ExecutorStatus {
	/// Committed to be online and producing valid blocks (not equivocating)
	Active,
	/// Temporarily inactive and excused for inactivity
	Idle,
	/// Bonded until the inner round
	Leaving(RoundIndex),
}

impl Default for ExecutorStatus {
	fn default() -> ExecutorStatus {
		ExecutorStatus::Idle
	}
}

#[derive(PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// Capacity status for top or bottom delegations
pub enum CapacityStatus {
	/// Reached capacity
	Full,
	/// Empty aka contains no delegations
	Empty,
	/// Partially full (nonempty and not full)
	Partial,
}
