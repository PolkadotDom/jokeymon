use sp_runtime::Permill;
use frame_support::BoundedVec;
use crate::{mock::*, pallet as OmniPallet, types::*};

//mock a test jokeymon region
fn get_test_region() -> Region<Test> {
    let rate_one = Permill::from_percent(20);
    let rate_two = Permill::from_percent(30);
    let rate_three = Permill::from_percent(50);
    let chances = vec![(0u32, rate_one), (1u32, rate_two), (2u32, rate_three)];
    Region::<Test> {
        id: 0u32,
        jokeymon_chances: BoundedVec::try_from(chances).expect("Test region set up incorrectly"),
        latitude: 0u32,
        longitude: 0u32,
    }
}

#[test]
fn random_nonces_are_updated() {
    new_test_ext().execute_with(|| {
        // Check nonce value
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 0);
        assert_eq!(OmniPallet::JokeymonIdNonce::<Test>::get(), 0);
        // Dispatch catch extrinsic
        let _ = OmniModule::catch_jokeymon(RuntimeOrigin::signed(1));
        // Check nonce again
        assert_eq!(OmniPallet::RandomNonce::<Test>::get(), 1);
        assert_eq!(OmniPallet::JokeymonIdNonce::<Test>::get(), 1);
    });
}

#[test]
fn catch_works_at_bounds() {
    new_test_ext()
        .execute_with(|| {
            let region = get_test_region();
            // upper bound
            let mut id = OmniPallet::Pallet::<Test>::get_jokeymon_in_region(&region, Permill::one());
            assert_ne!(id, u32::MAX);
            // lower bound
            id = OmniPallet::Pallet::<Test>::get_jokeymon_in_region(&region, Permill::zero());
            assert_ne!(id, u32::MAX);
})
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
// All can be caught
