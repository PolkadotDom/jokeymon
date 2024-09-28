//! Some structs used in the omni pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, DefaultNoBound};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

// geographical
pub type RegionId = u16;
pub type Coordinate = u16;

// jokeymon
pub type JokeymonId = u16;

// jokeymon handling
pub type FindRate = u16;

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
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Region {
    pub id: RegionId,
    pub latitude: Coordinate,
    pub longitude: Coordinate,
}

/// The jokeymon and their associated catch chance
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
pub struct Chances<T: crate::Config> {
    pub jokeymon_ids: BoundedVec<JokeymonId, T::MaxJokeymonInRegion>,
    pub jokeymon_rates: BoundedVec<FindRate, T::MaxJokeymonInRegion>,
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
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct AccountData {
    pub current_region: RegionId,
}
