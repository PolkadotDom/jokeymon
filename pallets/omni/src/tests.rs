use crate::{mock::*, pallet as OmniPallet};

#[test]
fn random_nonce_is_updated() {
    new_test_ext().execute_with(|| {
        // Check nonce value
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 0);
        // Dispatch catch extrinsic
        let _ = OmniModule::catch_jokeymon(RuntimeOrigin::signed(1));
        // Check nonce again
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 1);
    });
}

#[test]
fn check_nonce_value() {
    new_test_ext().execute_with(|| {
        let x = OmniPallet::RegionToChances::<Test>::get(0u16);
        println!("{:?}", x.jokeymon_ids.to_vec());
        assert_eq!(1, 0);
    });
}
