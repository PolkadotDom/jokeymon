//! Some structs used in the omni pallet

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, DefaultNoBound};
use scale_info::TypeInfo;
use sp_runtime::{Permill, RuntimeDebug};

// geographical
pub type RegionId = u16;
pub type Coordinate = u16;

// jokeymon
pub type JokeymonId = u16;

// chances of finding a jokeymon in a region 
pub type RegionJokeymonChances<T: crate::Config> = BoundedVec<(JokeymonId, Permill), T::MaxJokeymonInRegion>; 

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
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct AccountData<T: crate::Config> {
    pub current_region: RegionId,
    pub jokeymon : BoundedVec<JokeymonId, T::MaxJokeymonHoldable>,
}
