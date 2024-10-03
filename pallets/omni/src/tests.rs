use frame_support::assert_noop;
use sp_runtime::Permill;
use crate::{mock::*, pallet as OmniPallet, Error};

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
            // lower bound
            let mut id = OmniPallet::Pallet::<Test>::get_jokeymon_in_region(&region, Permill::zero());
            assert_ne!(id, u32::MAX);
            // upper bound
            id = OmniPallet::Pallet::<Test>::get_jokeymon_in_region(&region, Permill::one());
            assert_ne!(id, u32::MAX);
})
}

#[test]
fn jokeymon_catching_works() {
    new_test_ext().execute_with(|| {
        // before
        let account_data = OmniPallet::AccountToData::<Test>::get(0u64);
        assert_eq!(account_data.jokeymon.to_vec().len(), 0);

        // catch
        let _ = OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(0u64));

        // after
        let account_data = OmniPallet::AccountToData::<Test>::get(0u64);
        assert_eq!(account_data.jokeymon.to_vec().len(), 1);
    });
}

#[test]
fn birthdate_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        // catch
        let _ = OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(0u64));
        // check
        let account_data = OmniPallet::AccountToData::<Test>::get(0u64);
        let jokeymon_id = account_data.jokeymon[0];
        let jokeymon_data = OmniPallet::JokeymonIdToData::<Test>::get(jokeymon_id)
        .expect("Jokeymon individual data wasn't set!");
        assert_eq!(jokeymon_data.birth_date, 10);
    });
}

#[test]
fn catching_to_many_fails() {
    new_test_ext().execute_with(|| {
        let bound = <Test as crate::Config>::MaxJokeymonHoldable::get();
        for i in 0..101 {
            let res = OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(0u64));
            if i == bound {
                assert_noop!(
                    res,
                    Error::<Test>::TooManyJokeymon
                );
            }
        }
    });
}