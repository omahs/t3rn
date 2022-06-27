use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{RuntimeDebug, Perbill, traits::CheckedAdd};

pub const MILLIT3RN: u64 = 1_000_000_000;
pub const T3RN: u64 = 1_000_000_000_000;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum BeneficiaryRole {
    Developer,
    Executor,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, Default, RuntimeDebug, TypeInfo)]
pub struct InflationAllocation {
    pub developer: Perbill,
    pub executor: Perbill,
}

impl InflationAllocation {
    pub fn is_valid(&self) -> bool {
        match self.developer.checked_add(&self.executor) {
            Some(perbill) => perbill == Perbill::one(),
            None => false,
        }
    }
}