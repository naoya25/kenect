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

pub trait LocationDatabase {
    fn find_by_name(&self, name: &str) -> Vec<LocationId>;
    fn is_adjacent(&self, a: LocationId, b: LocationId) -> bool;
    fn has_valid_move(&self, current: LocationId, used: &[LocationId]) -> bool;
    fn name_of(&self, id: LocationId) -> Option<&str>;
    fn hint_of(&self, id: LocationId) -> Option<String>;
    fn all_ids(&self) -> Vec<LocationId>;
}

pub struct RegionDatabase {
    entries: &'static [Region],
}

impl RegionDatabase {
    pub const fn new(entries: &'static [Region]) -> Self {
        Self { entries }
    }

    fn get(&self, id: LocationId) -> Option<&Region> {
        self.entries.iter().find(|e| e.id == id)
    }
}

impl LocationDatabase for RegionDatabase {
    fn find_by_name(&self, name: &str) -> Vec<LocationId> {
        self.entries
            .iter()
            .filter(|e| e.name == name)
            .map(|e| e.id)
            .collect()
    }

    fn is_adjacent(&self, a: LocationId, b: LocationId) -> bool {
        match (self.get(a), self.get(b)) {
            (Some(ea), Some(eb)) => ea.neighbors.contains(&b) && eb.neighbors.contains(&a),
            _ => false,
        }
    }

    fn has_valid_move(&self, current: LocationId, used: &[LocationId]) -> bool {
        self.get(current)
            .map(|e| e.neighbors.iter().any(|n| !used.contains(n)))
            .unwrap_or(false)
    }

    fn name_of(&self, id: LocationId) -> Option<&str> {
        self.get(id).map(|e| e.name)
    }

    fn hint_of(&self, id: LocationId) -> Option<String> {
        self.get(id).map(|e| format!("{} / {}", e.kana, e.roman))
    }

    fn all_ids(&self) -> Vec<LocationId> {
        self.entries.iter().map(|e| e.id).collect()
    }
}
