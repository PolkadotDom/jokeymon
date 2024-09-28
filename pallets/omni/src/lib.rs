#![cfg_attr(not(feature = "std"), no_std)]

// TODO
// State
//   Jokeymon
// 	 Regions
//   JokeymonPerRegion
//   Region population dynamics
//   Food per region
//   Users jokeymon
//   Breed chart

// Calls
//   Catch - catch random jokeymon from that region
//   Release - release a jokeymon into that region
//   Gather Food - gather food from a region
//   Battle - attack random jokeymon in that region for xp
//   Travel - travel to a region
//   Breed - breed two compatible jokeymon

// Hooks
//   Update jokeymon region population dynamics
//   Update players food resources & jokeymon

// Next

pub use pallet::*;

pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//
// To see a full list of `pallet` macros and their use cases, see:
// <https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/index.html>
#[frame_support::pallet]
pub mod pallet {
    use crate::types::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{BuildGenesisConfig, Randomness},
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;

        /// A source of randomness
        type RandomSource: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// Maximum amount of Jokeymon allowed in a region
        type MaxJokeymonInRegion: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The seed for randomness
    #[pallet::storage]
    pub type RandomNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Region to jokeymon chances
    #[pallet::storage]
    pub type RegionToChances<T: Config> =
        StorageMap<_, Blake2_128Concat, RegionId, Chances<T>, ValueQuery>;

    /// Account to user data
    #[pallet::storage]
    pub type AccountToData<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountData, ValueQuery>;

    /// Genesis Storage
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub region_to_chances: Vec<(RegionId, Chances<T>)>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                region_to_chances : vec![(0u16, Chances::<T> {
                    jokeymon_ids : BoundedVec::try_from(vec![0u16, 1u16, 2u16]).expect("messed up region to chances genesis"),
                    jokeymon_rates : BoundedVec::try_from(vec![100u16, 200u16, 300u16]).expect("messed up region to chances genesis")
                })],
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (a,b) in &self.region_to_chances {
                RegionToChances::<T>::insert(a, b);
            }
        }
    }

    /// Pallets use events to inform users when important changes are made.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A jokeymon was caught
        JokeymonCaptured { id: JokeymonId, who: T::AccountId },
    }

    /// Errors inform users that something went wrong.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables>
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Catch a jokeymon
        ///
        /// The jokeymon given is taken from a distribution
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn catch_jokeymon(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // get user
            let who = ensure_signed(origin)?;

            // get random number
            let seed = Self::get_and_increment_nonce();
            let random = Self::get_random_number(&seed);

            // decide which pokemon
            let region_id = AccountToData::<T>::get(&who).current_region;
            let jokeymon_id = Self::get_jokeymon_in_region(region_id, random);

            // add pokemon to users collection

            // deposit and event
            Self::deposit_event(Event::JokeymonCaptured {
                id: jokeymon_id,
                who: who,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// use and update the nonce
        fn get_and_increment_nonce() -> Vec<u8> {
            let val = RandomNonce::<T>::get();
            RandomNonce::<T>::put(val.wrapping_add(1));
            val.encode()
        }
        /// get a random number given the nonce
        fn get_random_number(seed: &Vec<u8>) -> FindRate {
            let (random, _) = T::RandomSource::random(seed);
            let as_bytes = random.encode();
            u16::from_le_bytes([as_bytes[0], as_bytes[1]])
        }
        /// get a jokeymon in a region, given a random number
        fn get_jokeymon_in_region(_region_id: RegionId, _random_num: FindRate) -> JokeymonId {
            0
        }
    }
}
