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
// finish the extrinsic, create a unit test for it

pub use pallet::*;

pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

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
    use sp_runtime::{traits::{Saturating, Zero}, Permill, Vec};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The runtime event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;

        /// A source of randomness
        type RandomSource: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// Maximum amount of Jokeymon allowed in a region
        type MaxJokeymonInRegion: Get<u16>;

        /// Maximum jokeymon an account can hold at a time
        type MaxJokeymonHoldable: Get<u16>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The seed for randomness
    #[pallet::storage]
    pub type RandomNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// nonce for region creation
    #[pallet::storage]
    pub type RegionNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Region id to its corresponding region
    #[pallet::storage]
    pub type RegionIdToRegion<T: Config> =
        StorageMap<_, Blake2_128Concat, RegionId, Region<T>, ValueQuery>;

    /// Account to user data
    #[pallet::storage]
    pub type AccountToData<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A jokeymon was caught
        JokeymonCaptured { id: JokeymonId, who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// No room left for jokeymon in the account's party
        TooManyJokeymon,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

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
            let roll = Self::get_random_number(&seed);

            // decide which jokeymon
            let region_id = AccountToData::<T>::get(&who).current_region;
            let region = RegionIdToRegion::<T>::get(region_id);
            let caught_jokeymon_id = Self::get_jokeymon_in_region(region, roll);

            // add jokeymon to a users collection
            let account_data = AccountToData::<T>::get(who);
            account_data.jokeymon.try_push(caught_jokeymon_id).ok_or(Error::<T>::TooManyJokeymon)?;
            AccountToData::<T>::put(account_data);

            // deposit and event
            Self::deposit_event(Event::JokeymonCaptured {
                id: jokeymon_id,
                who: who,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            let rate_one = Permill::from_percent(20);
            let rate_two = Permill::from_percent(30);
            let rate_three = Permill::from_percent(50);
            Self {
                region_to_chances : vec![(0u16, Chances::<T> {
                    jokeymon_ids : BoundedVec::try_from(vec![0u16, 1u16, 2u16]).expect("messed up region to chances genesis"),
                    jokeymon_rates : BoundedVec::try_from(vec![rate_one, rate_two, rate_three]).expect("messed up region to chances genesis")
                })],
            }
        }
    }

    /// Genesis Storage
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub region_to_chances: Vec<(RegionId, Chances<T>)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (a,b) in &self.region_to_chances {
                RegionToChances::<T>::insert(a, b);
            }
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
        fn get_random_number(seed: &Vec<u8>) -> Permill {
            let (random, _) = T::RandomSource::random(seed);
            let as_bytes = random.encode();
            let part = u16::from_le_bytes([as_bytes[0], as_bytes[1]]);
            Permill::from_rational(part, u16::MAX)
        }
        /// get a jokeymon in a region, given a random number
        fn get_jokeymon_in_region(region: Region<T>, catch_roll: Permill) -> JokeymonId {
            for (&id, &rate) in region.jokeymon_chances.iter() {
                if (catch_roll == Permill::zero) {
                    id
                }
                catch_roll = catch_roll.saturating_sub(rate);
            }
        }
    }
}
