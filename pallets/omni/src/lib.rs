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
//   Region to region adjacency matrix

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
// hook that updates population dynamics in the regions
// inherent that uses offchain function

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
    use bounded_collections::Get as BGet;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{BuildGenesisConfig, Randomness},
        Blake2_128Concat,
    };
    use frame_system::{pallet_prelude::*, Pallet as SystemPallet};
    use scale_info::prelude::{vec, collections::BTreeMap};
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

        /// Maximum amount of Jokeymon species allowed in a region (BGet while waiting on sdk merge)
        type MaxSpeciesInRegion: Get<u32> + BGet<u32>;

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

    /// Species id to general species data
    #[pallet::storage]
    pub type SpeciesIdToSpeciesData<T: Config> =
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

    #[cfg_attr(test, derive(PartialEq))]
    #[pallet::error]
    pub enum Error<T> {
        /// No room left for jokeymon in the account's party
        TooManyJokeymon,
        /// No jokeymon left in the region to catch
        NoCatchableJokeymon,
        /// No room left in region for new species
        RegionSpeciesDiversitySaturated,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // For each region, update it's population
            // foreach (_, mut region) in RegionIdToRegion::<T> {
            // Self::update_regional_population(region)
            // }
            Weight::zero()
        }
    }

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

            // check region has available jokeymon
            let current_region_id = account_data.current_region;
            let mut region = RegionIdToRegion::<T>::get(current_region_id);
            if region.total_population == 0 {
                Err(Error::<T>::NoCatchableJokeymon)?
            }

            // decide which jokeymon species, decrement it from region
            let caught_species_id = Self::get_jokeymon_in_region(&region, roll);
            Self::decrement_species_in_population(&mut region, caught_species_id, 1);
            RegionIdToRegion::<T>::set(current_region_id, region);

            // generate jokeymon of that species
            let new_jokeymon_id = Self::get_and_increment_jokeymon_id_nonce();
            // let species_data = JokeymonSpeciesIdToSpeciesData::<T>::get(caught_jokeymon_species_id);
            // let mutated_species_data = Self::mutate_jokeymon_data(species_data);
            let data = JokeymonData::<T> {
                id: caught_species_id,
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
                species_id: caught_species_id,
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
            let eps = Permill::from_parts(1u32);
            let mut eps_added = false;

            // generate chances based on population size
            let total = region.total_population;
            let sub_totals = &region.population_demographics;
            let mut jokeymon_chances = Vec::new();
            for (id, size) in sub_totals.iter() {
                let mut p = Permill::from_rational((*size).into(), total);
                if !eps_added {
                    // Just once to offset rounding issue
                    p = p.saturating_add(eps);
                    eps_added = true;
                }
                jokeymon_chances.push((*id, p));
            }

            // pick species based on chances
            for (id, rate) in jokeymon_chances.iter() {
                catch_roll = catch_roll.saturating_sub(*rate);
                if catch_roll == Permill::zero() {
                    return *id;
                }
            }
            u32::MAX.into()
        }

        /// Decrements the population size of a jokeymon in a region
        pub(super) fn decrement_species_in_population(
            region: &mut Region<T>,
            id: JokeymonSpeciesId,
            amount: u32,
        ) {
            let total_size = &mut region.total_population;
            let sizes = &mut region.population_demographics;

            if let Some(population_size) = sizes.get_mut(&id) {
                //decrement
                let original = *population_size;
                let end = population_size.saturating_sub(amount);
                let actual_diff = original - end;
                *population_size = end;
                *total_size = total_size.saturating_sub(actual_diff.into());
                //remove if 0
                if end == 0 {
                    sizes.remove(&id);
                }
            }
        }

        /// Increments the population size of a jokeymon in a region
        pub(super) fn increment_species_in_population(
            region: &mut Region<T>,
            id: JokeymonSpeciesId,
            amount: u32,
        ) -> Result<(), Error<T>> {
            let total_size = &mut region.total_population;
            let sizes = &mut region.population_demographics;

            if let Some(population_size) = sizes.get_mut(&id) {
                // species already in region, add to size
                let new_size = population_size.saturating_add(amount);
                *population_size = new_size;
            } else {
                // not in region
                sizes
                    .try_insert(id, amount)
                    .map_err(|_| Error::<T>::RegionSpeciesDiversitySaturated)?;
            }
            *total_size = total_size.saturating_add(amount.into());
            Ok(())
        }

        /// Updates a regions population based on the Lotka-Volterra formula
        pub(super) fn update_regional_population(region: &mut Region<T>) {
            // Get number of herbivores and carnivores
            let mut herb_species_count = 0;
            let mut carn_species_count = 0;
            let mut herb_total_count = 0;
            let mut carn_total_count = 0;
            let mut herb_food_intake = 0;
        
            for (id, pop) in &region.population_demographics {
                let data = SpeciesIdToSpeciesData::<T>::get(id);
                match data.diet {
                    Diet::Herbivore => {
                        herb_total_count += pop;
                        herb_food_intake += pop * (data.avg_daily_food_consumption as u32);
                        herb_species_count += 1;
                    }
                    Diet::Carnivore => {
                        carn_total_count += pop;
                        carn_species_count += 1;
                    }
                }
            }
        
            // Calculate carrying capacity
            let carrying_capacity = region.energy_production / herb_food_intake;
        
            // Mock params
            let (alpha, beta, delta, gamma) = (2, 1, 1, 1);
        
            // Calculate growth or decay of each
            let dh: i32 = ((alpha * herb_total_count)
                * (1 - (herb_total_count / carrying_capacity))
                - (beta * herb_total_count * carn_total_count)) as i32;
            let dc: i32 =
                ((delta * herb_total_count * carn_total_count) - (gamma * carn_total_count)) as i32;
            let dh_per_species: i32 = dh / herb_species_count;
            let dc_per_species: i32 = dc / carn_species_count;
        
            // Build new demographics
            let mut new_demographics = BTreeMap::<u32, u32>::new();
        
            for (id, pop) in &region.population_demographics {
                let data = SpeciesIdToSpeciesData::<T>::get(id);
                let new_value = match data.diet {
                    Diet::Herbivore => (*pop as i32 + dh_per_species).max(0) as u32,
                    Diet::Carnivore => (*pop as i32 + dc_per_species).max(0) as u32,
                };
                new_demographics.insert(*id, new_value);
            }
        
            // Create a new BoundedBTreeMap from the updated demographics
            let new_population_demographics =
                bounded_collections::BoundedBTreeMap::<u32, u32, T::MaxSpeciesInRegion>::try_from(new_demographics).unwrap_or(
                    bounded_collections::BoundedBTreeMap::<u32, u32, T::MaxSpeciesInRegion>::new()
                );
        
            // Update the region's population demographics
            region.population_demographics = new_population_demographics;
        }
    }
}
