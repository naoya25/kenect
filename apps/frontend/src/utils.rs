use rand::seq::IndexedRandom;
use shared::game::GameState;
use shared::prefecture::{PrefectureId, all_prefectures};

pub fn random_start() -> PrefectureId {
    let prefs = all_prefectures();
    let candidates: Vec<_> = prefs.iter().filter(|p| !p.neighbors.is_empty()).collect();
    let mut rng = rand::rng();
    candidates.choose(&mut rng).unwrap().id
}

pub fn find_prefecture_by_name(name: &str) -> Vec<PrefectureId> {
    all_prefectures()
        .iter()
        .filter(|p| p.name == name)
        .map(|p| p.id)
        .collect()
}

pub fn used_names(state: &GameState) -> String {
    state
        .used
        .iter()
        .filter_map(|&id| all_prefectures().iter().find(|p| p.id == id))
        .map(|p| p.name)
        .collect::<Vec<_>>()
        .join(" → ")
}
