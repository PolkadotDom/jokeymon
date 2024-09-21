use crate::{mock::*, pallet as OmniPallet};
use frame_support::traits::OnInitialize;
use sp_runtime::traits::Header;

//fast forward so that randomness can be used
fn setup_blocks(blocks: u64) {
    let mut parent_hash = System::parent_hash();

    for i in 1..(blocks + 1) {
        System::reset_events();
        System::initialize(&i, &parent_hash, &Default::default());
        RandomModule::on_initialize(i);

        let header = System::finalize();
        parent_hash = header.hash();
        System::set_block_number(*header.number());
    }
}

#[test]
fn random_nonce_is_updated() {
    new_test_ext().execute_with(|| {
        //fast forward for randomness
        setup_blocks(81);
        // Check nonce value
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 0);
        // Dispatch catch extrinsic
        let _ = OmniModule::catch_jokeymon(RuntimeOrigin::signed(1));
        // Check nonce again
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 1);
    });
}
