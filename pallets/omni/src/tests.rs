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
        let x = OmniPallet::RegionIdToRegion::<Test>::get(0u32);
        println!("{:?}", x.jokeymon_chances.to_vec());
        assert_eq!(1, 0);
    });
}

// Too many jokeymon
// Jokeymon caught
// Catch works at bounds
// All can be caught
