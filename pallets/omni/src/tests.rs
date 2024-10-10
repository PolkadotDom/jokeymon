use crate::{mock::*, pallet as OmniPallet, Error};
use frame_support::assert_noop;
use sp_runtime::Permill;

// ---- Population Dynamics ----

#[test]
fn population_remains_stable() {
    new_test_ext().execute_with(|| {
        let mut region = get_test_region();
        for _ in 0..10_000 {
            OmniModule::update_regional_population(&mut region);
        }
        println!("{:?}", region.population_demographics);
        assert!(region.total_population > 0);
        assert!(region.total_population < 1000);
    });
}

// ---- Catch Extrinsic ----

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
    new_test_ext().execute_with(|| {
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
fn catching_too_many_fails() {
    new_test_ext().execute_with(|| {
        let bound = <Test as crate::Config>::MaxJokeymonHoldable::get();
        for i in 0..101 {
            let res = OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(0u64));
            if i == bound {
                assert_noop!(res, Error::<Test>::TooManyJokeymon);
            }
        }
    });
}

#[test]
fn depleting_region_works() {
    new_test_ext().execute_with(|| {
        let region = OmniPallet::RegionIdToRegion::<Test>::get(&0);
        let num_jokeymon = region.total_population;

        // catch the entire population
        for i in 0..num_jokeymon {
            let _ = OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(i));
        }

        // next catch should fail
        assert_noop!(
            OmniPallet::Pallet::<Test>::catch_jokeymon(RuntimeOrigin::signed(0u64)),
            Error::<Test>::NoCatchableJokeymon
        );
    });
}

#[test]
fn decrement_species_should_work() {
    new_test_ext().execute_with(|| {
        let mut region = get_test_region();

        // decrement non saturating
        assert_eq!(region.population_demographics[&0], 150);
        OmniPallet::Pallet::<Test>::decrement_species_in_population(&mut region, 0, 1);
        assert_eq!(region.population_demographics[&0], 149);

        // decrement saturating, key should no longer exist
        OmniPallet::Pallet::<Test>::decrement_species_in_population(&mut region, 0, 200);
        assert!(!region.population_demographics.contains_key(&0));
        assert_eq!(region.total_population, 300);
    });
}

#[test]
fn increment_species_should_work() {
    new_test_ext().execute_with(|| {
        let mut region = get_test_region();

        // increment non new
        assert_eq!(region.population_demographics[&0], 150);
        let _ = OmniPallet::Pallet::<Test>::increment_species_in_population(&mut region, 0, 1);
        assert_eq!(region.population_demographics[&0], 151);

        // increment new species
        assert!(!region.population_demographics.contains_key(&3));
        let _ = OmniPallet::Pallet::<Test>::increment_species_in_population(&mut region, 3, 100);
        assert!(region.population_demographics.contains_key(&3));
        assert_eq!(region.population_demographics[&3], 100);
        assert_eq!(region.total_population, 551);

        // add too many species
        let max_species = <Test as crate::Config>::MaxSpeciesInRegion::get();
        for i in 4..max_species {
            let _ = OmniPallet::Pallet::<Test>::increment_species_in_population(&mut region, i, 1);
        }
        let res = OmniPallet::Pallet::<Test>::increment_species_in_population(
            &mut region,
            max_species + 1,
            1,
        );
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            Error::<Test>::RegionSpeciesDiversitySaturated
        );
    });
}
