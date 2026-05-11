use serde::{Deserialize, Serialize};

/// JIS X 0401（都道府県）: 1–47
/// JIS X 0402（市区町村）: 5桁、先頭2桁が都道府県コード（例: 大阪市 = 27100）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    Prefecture,
    City,
}

pub struct Region {
    pub id: LocationId,
    pub kind: RegionKind,
    /// 市区町村の場合は親の都道府県 ID、都道府県は None
    pub parent: Option<LocationId>,
    pub name: &'static str,
    pub kana: &'static str,
    pub roman: &'static str,
    pub neighbors: &'static [LocationId],
}

pub struct RegionDatabase {
    entries: &'static [Region],
}

/// Parse regions from CSV format: id,name,kana,roman,parent,neighbors
/// parent: empty or numeric ID (or numeric for city parent prefecture)
/// neighbors: pipe-separated numeric IDs
pub fn parse_regions_from_csv(csv_data: &str, kind: RegionKind) -> &'static [Region] {
    let mut regions: Vec<Region> = Vec::new();

    for (i, line) in csv_data.lines().enumerate() {
        if i == 0 {
            continue; // header
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(6, ',');
        let id_str = parts.next().unwrap_or("");
        let name = parts.next().unwrap_or("");
        let kana = parts.next().unwrap_or("");
        let roman = parts.next().unwrap_or("");
        let parent_str = parts.next().unwrap_or("");
        let neighbors_field = parts.next().unwrap_or("");

        let id_num: u32 = id_str.parse().expect("invalid id in CSV");
        let id = LocationId(id_num);

        let name_static: &'static str = Box::leak(name.to_string().into_boxed_str());
        let kana_static: &'static str = Box::leak(kana.to_string().into_boxed_str());
        let roman_static: &'static str = Box::leak(roman.to_string().into_boxed_str());

        let parent: Option<LocationId> = if parent_str.is_empty() {
            None
        } else {
            parent_str.parse::<u32>().ok().map(LocationId)
        };

        let neighbors_vec: Vec<LocationId> = if neighbors_field.is_empty() {
            Vec::new()
        } else {
            neighbors_field
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| LocationId(s.parse::<u32>().expect("invalid neighbor id")))
                .collect()
        };

        let neighbors_box = neighbors_vec.into_boxed_slice();
        let neighbors_static: &'static [LocationId] = Box::leak(neighbors_box);

        regions.push(Region {
            id,
            kind,
            parent,
            name: name_static,
            kana: kana_static,
            roman: roman_static,
            neighbors: neighbors_static,
        });
    }

    let boxed = regions.into_boxed_slice();
    Box::leak(boxed)
}

impl RegionDatabase {
    pub const fn new(entries: &'static [Region]) -> Self {
        Self { entries }
    }

    pub fn get(&self, id: LocationId) -> Option<&Region> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn find_by_name(&self, name: &str) -> Vec<LocationId> {
        let name_lower = name.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.name.to_lowercase() == name_lower
                    || e.kana.to_lowercase() == name_lower
                    || e.roman.to_lowercase() == name_lower
            })
            .map(|e| e.id)
            .collect()
    }

    pub fn is_adjacent(&self, a: LocationId, b: LocationId) -> bool {
        match (self.get(a), self.get(b)) {
            (Some(ea), Some(eb)) => ea.neighbors.contains(&b) && eb.neighbors.contains(&a),
            _ => false,
        }
    }

    pub fn has_valid_move(&self, current: LocationId, used: &[LocationId]) -> bool {
        self.get(current)
            .map(|e| e.neighbors.iter().any(|n| !used.contains(n)))
            .unwrap_or(false)
    }

    pub fn valid_move_count(&self, current: LocationId, used: &[LocationId]) -> usize {
        self.get(current)
            .map(|e| e.neighbors.iter().filter(|n| !used.contains(n)).count())
            .unwrap_or(0)
    }

    pub fn valid_move_ids(&self, current: LocationId, used: &[LocationId]) -> Vec<LocationId> {
        self.get(current)
            .map(|e| {
                e.neighbors
                    .iter()
                    .copied()
                    .filter(|n| !used.contains(n))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn name_of(&self, id: LocationId) -> Option<&str> {
        self.get(id).map(|e| e.name)
    }

    pub fn hint_of(&self, id: LocationId) -> Option<String> {
        self.get(id).map(|e| format!("{} / {}", e.kana, e.roman))
    }

    pub fn all_ids(&self) -> Vec<LocationId> {
        self.entries.iter().map(|e| e.id).collect()
    }

    pub fn all_regions(&self) -> &[Region] {
        self.entries
    }
}
