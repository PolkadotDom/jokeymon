//! Some structs used in the omni pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, DefaultNoBound};
use scale_info::TypeInfo;
use sp_runtime::{Permill, RuntimeDebug};

// geographical
pub type RegionId = u32;
pub type Coordinate = u32;

// jokeymon
pub type JokeymonId = u32;

// chances of finding a jokeymon in a region
pub type RegionJokeymonChances<T> =
    BoundedVec<(JokeymonId, Permill), <T as crate::Config>::MaxJokeymonInRegion>;

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
