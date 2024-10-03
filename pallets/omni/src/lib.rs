#![cfg_attr(not(feature = "std"), no_std)]

// TODO
// State
//   Jokeymon
// 	 Regions
//   JokeymonPerRegion
//   Region population dynamics (food, predation, climate)
//   Food per region
//   Users jokeymon
//   Breed chart
//   Infrastructure efficiency and cost?

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
// move any state instantiation for tests outside of genesis default config

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
    use frame_system::{pallet_prelude::*, Pallet as SystemPallet};
    use scale_info::prelude::vec;
    use sp_runtime::{traits::Saturating, Permill, Vec};

    /// Genesis Storage
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub region_id_to_region: Vec<(RegionId, Region<T>)>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                region_id_to_region: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            //regions
            for (a, b) in &self.region_id_to_region {
                RegionIdToRegion::<T>::insert(a, b);
            }
        }
    }

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
        type MaxJokeymonInRegion: Get<u32>;

        /// Maximum jokeymon an account can hold at a time
        type MaxJokeymonHoldable: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The seed for randomness
    #[pallet::storage]
    pub type RandomNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// nonce for region creation
    #[pallet::storage]
    pub type RegionNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// nonce for the jokeymon unique individual id
    #[pallet::storage]
    pub type JokeymonIdNonce<T: Config> = StorageValue<_, JokeymonId, ValueQuery>;

    /// Region id to its corresponding region
    #[pallet::storage]
    pub type RegionIdToRegion<T: Config> =
        StorageMap<_, Blake2_128Concat, RegionId, Region<T>, ValueQuery>;

    /// Account to user data
    #[pallet::storage]
    pub type AccountToData<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T>, ValueQuery>;

    /// Jokeymon unique id to jokeymon data
    #[pallet::storage]
    pub type JokeymonIdToData<T: Config> =
        StorageMap<_, Blake2_128Concat, JokeymonId, JokeymonData<T>, OptionQuery>;

    #[pallet::storage]
    pub type JokeymonSpeciesIdToSpeciesData<T: Config> =
        StorageMap<_, Blake2_128Concat, JokeymonSpeciesId, JokeymonSpeciesData, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A jokeymon was caught
        JokeymonCaptured {
            species_id: JokeymonSpeciesId,
            jokeymon_id: JokeymonId,
            who: T::AccountId,
        },
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
            // get user and account data
            let who = ensure_signed(origin)?;
            let mut account_data = AccountToData::<T>::get(&who);

            // get random number
            let seed = Self::get_and_increment_random_nonce();
            let roll = Self::get_random_number(&seed);

            // decide which jokeymon species
            let current_region_id = account_data.current_region;
            let region = RegionIdToRegion::<T>::get(current_region_id);
            let caught_jokeymon_species_id = Self::get_jokeymon_in_region(&region, roll);

            // generate jokeymon of that species
            let new_jokeymon_id = Self::get_and_increment_jokeymon_id_nonce();
            // let species_data = JokeymonSpeciesIdToSpeciesData::<T>::get(caught_jokeymon_species_id);
            // let mutated_species_data = Self::mutate_jokeymon_data(species_data);
            let data = JokeymonData::<T> {
                id: caught_jokeymon_species_id,
                birth_date: SystemPallet::<T>::block_number(),
            };

            // add that jokeymon to the jokeymon data bank
            JokeymonIdToData::<T>::set(new_jokeymon_id, Some(data));

            // add jokeymon to a users collection
            account_data
                .jokeymon
                .try_push(new_jokeymon_id)
                .map_err(|_| Error::<T>::TooManyJokeymon)?;
            AccountToData::<T>::set(&who, account_data);

            // deposit and event
            Self::deposit_event(Event::JokeymonCaptured {
                species_id: caught_jokeymon_species_id,
                jokeymon_id: new_jokeymon_id,
                who: who,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// use and update the random nonce
        pub(super) fn get_and_increment_random_nonce() -> Vec<u8> {
            let val = RandomNonce::<T>::get();
            RandomNonce::<T>::put(val.wrapping_add(1));
            val.encode()
        }
        /// use and update the jokeymon unique identifier nonce
        pub(super) fn get_and_increment_jokeymon_id_nonce() -> JokeymonId {
            let val = JokeymonIdNonce::<T>::get();
            JokeymonIdNonce::<T>::put(val.wrapping_add(1));
            val
        }
        /// get a random number given the nonce
        pub(super) fn get_random_number(seed: &Vec<u8>) -> Permill {
            let (random, _) = T::RandomSource::random(seed);
            let as_bytes = random.encode();
            let part = u32::from_le_bytes([as_bytes[0], as_bytes[1], as_bytes[2], as_bytes[3]]);
            Permill::from_rational(part, u32::MAX)
        }
        /// get a jokeymon in a region, given a random number
        pub(super) fn get_jokeymon_in_region(
            region: &Region<T>,
            mut catch_roll: Permill,
        ) -> JokeymonSpeciesId {
            for (id, rate) in region.jokeymon_chances.iter() {
                catch_roll = catch_roll.saturating_sub(*rate);
                if catch_roll == Permill::zero() {
                    return *id;
                }
            }
            u32::MAX.into()
        }
    }
}
