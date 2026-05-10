use rand::seq::IndexedRandom;
use shared::game::GameState;
use shared::location::{LocationId, RegionDatabase};

pub fn random_start(db: &RegionDatabase) -> LocationId {
    let candidates: Vec<LocationId> = db
        .all_ids()
        .into_iter()
        .filter(|&id| db.has_valid_move(id, &[]))
        .collect();
    let mut rng = rand::rng();
    candidates
        .choose(&mut rng)
        .copied()
        .expect("no valid start location")
}

pub fn used_names(state: &GameState, db: &RegionDatabase) -> String {
    state
        .used
        .iter()
        .filter_map(|&id| db.name_of(id))
        .collect::<Vec<_>>()
        .join(" → ")
}
