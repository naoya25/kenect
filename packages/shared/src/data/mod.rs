use crate::location::RegionDatabase;
use crate::location::{RegionKind, parse_regions_from_csv};
use once_cell::sync::Lazy;

pub static PREFECTURES: Lazy<&'static [crate::location::Region]> = Lazy::new(|| {
    let data = include_str!("japan.csv");
    parse_regions_from_csv(data, RegionKind::Prefecture)
});

pub static TOKYO: Lazy<&'static [crate::location::Region]> = Lazy::new(|| {
    let data = include_str!("tokyo.csv");
    parse_regions_from_csv(data, RegionKind::City)
});

pub static PREFECTURE_DB: Lazy<RegionDatabase> = Lazy::new(|| RegionDatabase::new(*PREFECTURES));
pub static TOKYO_DB: Lazy<RegionDatabase> = Lazy::new(|| RegionDatabase::new(*TOKYO));

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bidirectional_adjacency(regions: &[crate::location::Region]) {
        for region in regions {
            for &neighbor_id in region.neighbors {
                let neighbor = regions
                    .iter()
                    .find(|candidate| candidate.id == neighbor_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "{} の隣接先 {} が見つかりません",
                            region.name, neighbor_id.0
                        )
                    });

                assert!(
                    neighbor.neighbors.contains(&region.id),
                    "{} → {} は片方向",
                    region.name,
                    neighbor.name
                );
            }
        }
    }

    #[test]
    fn all_47_prefectures_defined() {
        assert_eq!(PREFECTURES.len(), 47);
    }

    #[test]
    fn tokyo_has_62_regions() {
        assert_eq!(TOKYO.len(), 62);
    }

    #[test]
    fn adjacency_is_bidirectional() {
        assert_bidirectional_adjacency(&PREFECTURES);
        assert_bidirectional_adjacency(&TOKYO);
    }
}
