use super::prefecture::P;
use crate::location::{LocationId, Region, RegionKind};

/// JIS X 0402 市区町村コード（チェックディジットなし5桁）
/// 先頭2桁が都道府県コードと一致するため、親県の特定に使える
/// 例: 大阪市 = 27100（27 = 大阪府）
#[repr(u32)]
#[derive(Clone, Copy)]
enum C {
    OsakaCity = 27100,
    Toyonaka = 27204,
}

impl C {
    const fn id(self) -> LocationId {
        LocationId(self as u32)
    }
}

use C::*;

pub const CITIES: &[Region] = &[
    Region {
        id: OsakaCity.id(),
        kind: RegionKind::City,
        parent: Some(P::Osaka.id()),
        name: "大阪市",
        kana: "おおさかし",
        roman: "Osaka",
        neighbors: &[Toyonaka.id()],
    },
    Region {
        id: Toyonaka.id(),
        kind: RegionKind::City,
        parent: Some(P::Osaka.id()),
        name: "豊中市",
        kana: "とよなかし",
        roman: "Toyonaka",
        neighbors: &[OsakaCity.id()],
    },
];
