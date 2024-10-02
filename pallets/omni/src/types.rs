//! Some structs used in the omni pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, DefaultNoBound};
use scale_info::TypeInfo;
use sp_runtime::{Permill, RuntimeDebug};
use frame_system::pallet_prelude::BlockNumberFor;

// geographical
pub type RegionId = u32;
pub type Coordinate = u32;

// jokeymon
pub type JokeymonSpeciesId = u32; // species identifier
pub type JokeymonId = u64; // individual identifier

// chances of finding a jokeymon in a region
pub type RegionJokeymonChances<T> =
    BoundedVec<(JokeymonSpeciesId, Permill), <T as crate::Config>::MaxJokeymonInRegion>;

/// A jokeymon region
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DefaultNoBound,
    serde::Serialize,
    serde::Deserialize,
)]
#[scale_info(skip_type_params(T))]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct Region<T: crate::Config> {
    pub id: RegionId,
    pub jokeymon_chances: RegionJokeymonChances<T>,
    pub latitude: Coordinate,
    pub longitude: Coordinate,
}

/// The account data associated with an account id
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DefaultNoBound,
    serde::Serialize,
    serde::Deserialize,
)]
#[scale_info(skip_type_params(T))]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct AccountData<T: crate::Config> {
    pub current_region: RegionId,
    pub jokeymon: BoundedVec<JokeymonId, T::MaxJokeymonHoldable>,
}

/// The jokeymon data associated with a unique jokeymon
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    DefaultNoBound,
    serde::Serialize,
    serde::Deserialize,
)]
#[scale_info(skip_type_params(T))]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct JokeymonData<T: crate::Config> {
    pub id: JokeymonSpeciesId,
    pub birth_date: BlockNumberFor<T>,
}

/// The jokeymon data associated with a jokeymon species
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct JokeymonSpeciesData {
    pub id: JokeymonSpeciesId,
    pub avg_weight: u16,
    pub avg_daily_food_consumption: u16,
    pub evolves_to: Option<JokeymonSpeciesId>,
}


