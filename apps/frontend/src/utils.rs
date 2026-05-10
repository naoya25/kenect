use rand::seq::IndexedRandom;
use shared::game::GameState;
use shared::location::{LocationDatabase, LocationId};

pub fn random_start(db: &dyn LocationDatabase) -> LocationId {
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

pub fn used_names(state: &GameState, db: &dyn LocationDatabase) -> String {
    state
        .used
        .iter()
        .filter_map(|&id| db.name_of(id))
        .collect::<Vec<_>>()
        .join(" → ")
}
