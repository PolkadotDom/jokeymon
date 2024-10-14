use crate::pallet as OmniPallet;
use crate::types::*;
use frame_support::{derive_impl, parameter_types, weights::constants::RocksDbWeight, BoundedBTreeMap};
use frame_system::mocking::MockBlock;
use sp_runtime::{traits::ConstU64, BuildStorage};

// Configure a mock runtime to test the pallet.
#[frame_support::runtime]
mod test_runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;
    #[runtime::pallet_index(1)]
    pub type OmniModule = crate;
    #[runtime::pallet_index(2)]
    pub type RandomModule = pallet_insecure_randomness_collective_flip;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Nonce = u64;
    type Block = MockBlock<Test>;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = RocksDbWeight;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

parameter_types! {
    pub const MaxJokeymonInRegion : u32 = 50;
    pub const MaxJokeymonHoldable : u32 = 100;
}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type RandomSource = RandomModule;
    type MaxSpeciesInRegion = MaxJokeymonInRegion;
    type MaxJokeymonHoldable = MaxJokeymonHoldable;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = RuntimeGenesisConfig::default()
        .build_storage()
        .unwrap()
        .into();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        setup_test_region();
        setup_test_species_data();
    });
    ext
}

// ----- HELPERS -----

/// Mock a test jokeymon region
pub(super) fn get_test_region<T: crate::Config>() -> Region<T> {
    let mut map: BoundedBTreeMap<JokeymonSpeciesId, JokeymonCount, T::MaxSpeciesInRegion> = BoundedBTreeMap::new();
    map.try_insert(0u32, 150).unwrap();
    map.try_insert(1u32, 150).unwrap();
    map.try_insert(2u32, 150).unwrap();
    Region::<T> {
        id: RegionId::default(),
        total_population: 450,
        population_demographics: map,
        energy_yield: 100_000u32,
        latitude: 0u32,
        longitude: 0u32,
    }
}

/// Set test region in memory
pub(super) fn setup_test_region() {
    let region = get_test_region::<Test>();
    OmniPallet::RegionIdToRegion::set(RegionId::default(), region);
}

/// Set species data for a test
pub(super) fn set_species_data(
    id: JokeymonSpeciesId,
    avg_weight: u16,
    avg_daily_food_consumption: u16,
    diet: Diet,
    evolves_to: Option<JokeymonSpeciesId>,
) {
    OmniPallet::SpeciesIdToSpeciesData::<Test>::set(
        id,
        JokeymonSpeciesData {
            id: id,
            avg_weight: avg_weight,
            avg_daily_food_consumption: avg_daily_food_consumption,
            diet: diet,
            evolves_to: evolves_to,
        },
    );
}

/// Sets up the species data storage
pub(super) fn setup_test_species_data() {
    set_species_data(0, 10, 10, Diet::Herbivore, Some(1));
    set_species_data(1, 20, 25, Diet::Herbivore, Some(2));
    set_species_data(2, 30, 45, Diet::Carnivore, None);
}
