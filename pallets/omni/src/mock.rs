use crate::pallet as OmniPallet;
use crate::types::*;
use bounded_collections::BoundedBTreeMap;
use frame_support::{derive_impl, parameter_types, weights::constants::RocksDbWeight};
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

impl bounded_collections::Get<u32> for MaxJokeymonInRegion {
    fn get() -> u32 {
        Self::get()
    }
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
pub(super) fn get_test_region() -> Region<Test> {
    let mut map = BoundedBTreeMap::new();
    map.try_insert(0u32, 150).unwrap();
    map.try_insert(1u32, 150).unwrap();
    map.try_insert(2u32, 150).unwrap();
    Region::<Test> {
        id: RegionId::default(),
        total_population: 450,
        population_demographics: map,
        energy_production: 1_000_000u32,
        latitude: 0u32,
        longitude: 0u32,
    }
}

/// Set test region in memory
pub(super) fn setup_test_region() {
    let region = get_test_region();
    OmniPallet::RegionIdToRegion::set(RegionId::default(), region);
}

/// Sets up the species data storage
pub(super) fn setup_test_species_data() {
    OmniPallet::SpeciesIdToSpeciesData::<Test>::set(
        0,
        JokeymonSpeciesData {
            id: 0,
            avg_weight: 10,
            avg_daily_food_consumption: 10,
            diet: Diet::Herbivore,
            evolves_to: Some(1),
        },
    );
    OmniPallet::SpeciesIdToSpeciesData::<Test>::set(
        1,
        JokeymonSpeciesData {
            id: 1,
            avg_weight: 20,
            avg_daily_food_consumption: 25,
            diet: Diet::Herbivore,
            evolves_to: Some(2),
        },
    );
    OmniPallet::SpeciesIdToSpeciesData::<Test>::set(
        2,
        JokeymonSpeciesData {
            id: 2,
            avg_weight: 30,
            avg_daily_food_consumption: 45,
            diet: Diet::Carnivore,
            evolves_to: None,
        },
    );
}
