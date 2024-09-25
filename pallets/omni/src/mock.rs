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
}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type RandomSource = RandomModule;
    type MaxJokeymonInRegion = MaxJokeymonInRegion;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    // crate::GenesisConfig::<Test>::default()
    //     .build_storage()
    //     .unwrap()
    //     .into()

    let t = RuntimeGenesisConfig::default()
        .build_storage()
        .unwrap()
        .into();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext

    // let mut t = frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap();
    // pallet_balances::GenesisConfig::<TestRuntime> { balances: vec![(ENDOWED_ACCOUNT, 1_000_000)] }
    // 	.assimilate_storage(&mut t)
    // 	.unwrap();
    // sp_io::TestExternalities::new(t)
}
