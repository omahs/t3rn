//! <!-- markdown-link-check-disable -->
//! # Executor staking pallet
//! </pre></p></details>

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use crate::pallet::*;
use frame_support::{
    pallet_prelude::Weight,
    traits::{Currency, Get},
};
use sp_runtime::Percent;
use t3rn_primitives::{common::{Range}, monetary::{BalanceOf}}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
    use super::{types::*, *};
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        //TODO

        /// Minimum stake required for any candidate to be considered for the active set.
		#[pallet::constant]
		type MinActiveExecutorStake: Get<BalanceOf<Self>>;

        /// Minimum stake required for any candidate to be considered as candidate.
		#[pallet::constant]
		type MinCandidateExecutorStake: Get<BalanceOf<Self>>;

		/// Minimum stake for any registered on-chain account to stake.
		#[pallet::constant]
		type MinStake: Get<BalanceOf<Self>>;

		/// Minimum stake for any registered on-chain account to be a staker.
		#[pallet::constant]
		type MinStakerStake: Get<BalanceOf<Self>>;

        /// Range for the target executor active set size.
        /// The ideal is applied during genesis as default.
        #[pallet::constant]
        type ActiveSetSize: Get<Range<u16>>;

        /// Protocol enforced maximum executor commission.
        #[pallet::constant]
        type MaxCommission: Get<Percent>;

        type WeightInfo: weights::WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    /// Stakes by executor and staker.
    #[pallet::storage]
    #[pallet::getter(fn stakes)]
    pub type Stakes<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::AccountId,
        Twox64Concat,
        T::AccountId,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Executors' commissions.
    #[pallet::storage]
    #[pallet::getter(fn commissions)]
    pub type Commissions<T: Config> =
        StorageMap<_, Identity, T::AccountId, Percent, OptionQuery>;

    /// The pool of executor candidates, each with their total backing stake.
    #[pallet::storage]
    #[pallet::getter(fn candidate_pool)]
    pub(crate) type CandidatePool<T: Config> =
        StorageValue<_, OrderedSet<Bond<T::AccountId, BalanceOf<T>>>, ValueQuery>;

    /// Get executor candidate info associated with an account.
    #[pallet::storage]
    #[pallet::getter(fn candidate_info)]
    pub(crate) type CandidateInfo<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, CandidateMetadata<BalanceOf<T>>, OptionQuery>;
    
    /// Effective size of the executor active set.
    #[pallet::storage]
    #[pallet::getter(fn active_set_size)]
    pub type ActiveSetSize = StorageValue<_, Range<u16>, ValueQuery>;

    /// Active set of executors.
    #[pallet::storage]
    #[pallet::getter(fn active_set)]
    pub type ActiveSet = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Total capital locked by this staking pallet.
    #[pallet::storage]
    #[pallet::getter(fn tvl)]
    pub(crate) type Total<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Sets the executor active set's size.
        #[pallet::weight(10_000)] //TODO
        pub fn set_active_set_size(
            origin: OriginFor<T>,
            size: u16,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <Self as AccountManager<T::AccountId, BalanceOf<T>>>::deposit(
                &payee, &recipient, amount,
            )
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // `on_initialize` is executed at the beginning of the block before any extrinsic are
        // dispatched.
        //
        // This function must return the weight consumed by `on_initialize` and `on_finalize`.
        fn on_initialize(_n: T::BlockNumber) -> Weight {
            0
        }

        // `on_finalize` is executed at the end of block after all extrinsic are dispatched.
        fn on_finalize(_n: T::BlockNumber) {}
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // DepositReceived {
        //     execution_id: ExecutionId,
        //     payee: T::AccountId,
        //     recipient: T::AccountId,
        //     amount: BalanceOf<T>,
        // },
        // ExecutionFinalized {
        //     execution_id: ExecutionId,
        // },
        // Issued {
        //     recipient: T::AccountId,
        //     amount: BalanceOf<T>,
        // },
    }

    #[pallet::error]
    pub enum Error<T> {
        // ExecutionNotRegistered,
        // ExecutionAlreadyRegistered,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        phantom: PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }


    impl<T: Config> Pallet<T> {
    	/// Select the top `active_set_size()` candidates from the pool.
		/// a vec of their AccountIds (in the order of selection)
		pub fn select_active_set() -> Vec<T::AccountId> {
			let mut candidates = <CandidatePool<T>>::get().0;
			// order candidates by stake (least to greatest so requires `rev()`)
			candidates.sort_by(|a, b| a.amount.cmp(&b.amount));
			let top_n = <ActiveSetSize<T>>::get() as usize;
			// choose the top `active_set_size()` qualified candidates, ordered by stake
			let mut collators = candidates
				.into_iter()
				.rev()
				.take(top_n)
				.filter(|x| x.amount >= T::MinActiveExecutorStake::get())
				.map(|x| x.owner)
				.collect::<Vec<T::AccountId>>();
			collators.sort();
			collators
		}
    }
}