use serde::{Deserialize, Serialize};

use crate::location::{LocationId, RegionDatabase};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub score: u32,
    pub active: bool,
    /// Number of wrong declares accumulated for this player. 0 = none, 1 = hint1 shown, 2 = hint2 shown.
    pub wrong_count: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Playing,
    GameOver,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub current: LocationId,
    pub used: Vec<LocationId>,
    pub players: Vec<PlayerState>,
    pub current_player_index: usize,
    pub phase: GamePhase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeclareError {
    NotFound,
    NotAdjacent,
    GameAlreadyOver,
}

impl GameState {
    pub fn new(start: LocationId, names: Vec<String>, db: &RegionDatabase) -> Self {
        assert!(!names.is_empty(), "プレイヤーは1人以上必要です");
        let players = names
            .into_iter()
            .map(|name| PlayerState {
                name,
                score: 0,
                active: true,
                wrong_count: 0,
            })
            .collect();
        let mut state = Self {
            current: start,
            used: vec![start],
            players,
            current_player_index: 0,
            phase: GamePhase::Playing,
        };
        if db
            .all_regions()
            .iter()
            .any(|region| !region.neighbors.is_empty())
        {
            state.eliminate_stuck_players(db);
        }
        state
    }

    pub fn active_count(&self) -> usize {
        self.players.iter().filter(|p| p.active).count()
    }

    pub fn current_player(&self) -> &PlayerState {
        &self.players[self.current_player_index]
    }

    pub fn ranking(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.players.len()).collect();
        indices.sort_by(|&a, &b| self.players[b].score.cmp(&self.players[a].score));
        indices
    }

    pub fn declare(
        &mut self,
        candidates: &[LocationId],
        db: &RegionDatabase,
    ) -> Result<(), DeclareError> {
        if self.phase != GamePhase::Playing {
            return Err(DeclareError::GameAlreadyOver);
        }

        if candidates.is_empty() {
            return Err(DeclareError::NotFound);
        }

        let valid = candidates
            .iter()
            .copied()
            .find(|&id| db.is_adjacent(self.current, id) && !self.used.contains(&id));

        if let Some(id) = valid {
            // correct declare: move, score, reset wrong count for this player
            self.current = id;
            self.used.push(id);
            self.players[self.current_player_index].score += 1;
            self.players[self.current_player_index].wrong_count = 0;
            self.advance_to_next_active();
            self.eliminate_stuck_players(db);
            Ok(())
        } else {
            // incorrect declare: increment this player's wrong_count. If it reaches 3, eliminate
            // and advance; otherwise, the turn remains with the same player.
            let wc = &mut self.players[self.current_player_index].wrong_count;
            *wc = wc.saturating_add(1);
            if *wc >= 3 {
                // eliminate this player and advance
                self.players[self.current_player_index].active = false;
                if self.active_count() == 0 {
                    self.phase = GamePhase::GameOver;
                    return Err(DeclareError::NotAdjacent);
                }
                self.advance_to_next_active();
                self.eliminate_stuck_players(db);
            } else {
                // keep the turn with the current player; do not advance
            }
            Err(DeclareError::NotAdjacent)
        }
    }

    fn eliminate_current_and_advance(&mut self, db: &RegionDatabase) {
        self.players[self.current_player_index].active = false;
        if self.active_count() == 0 {
            self.phase = GamePhase::GameOver;
            return;
        }
        self.advance_to_next_active();
        self.eliminate_stuck_players(db);
    }

    fn advance_to_next_active(&mut self) {
        let n = self.players.len();
        for _ in 0..n {
            self.current_player_index = (self.current_player_index + 1) % n;
            if self.players[self.current_player_index].active {
                return;
            }
        }
    }

    fn eliminate_stuck_players(&mut self, db: &RegionDatabase) {
        let n = self.players.len();
        let mut checked = 0;
        while checked < n {
            if !self.players[self.current_player_index].active {
                break;
            }
            if db.has_valid_move(self.current, &self.used) {
                break;
            }
            self.players[self.current_player_index].active = false;
            if self.active_count() == 0 {
                self.phase = GamePhase::GameOver;
                return;
            }
            self.advance_to_next_active();
            checked += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{PREFECTURE_DB, prefecture::P};

    #[test]
    fn new_game_starts_with_first_player() {
        let game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        assert_eq!(game.current_player_index, 0);
        assert_eq!(game.phase, GamePhase::Playing);
        assert_eq!(game.active_count(), 2);
    }

    #[test]
    fn valid_declare_advances_player_and_score() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Kanagawa.id()], &PREFECTURE_DB).unwrap();
        assert_eq!(game.current_player_index, 1);
        assert_eq!(game.players[0].score, 1);
        assert_eq!(game.current, P::Kanagawa.id());
    }

    #[test]
    fn not_adjacent_increments_and_eliminates_on_third() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        // first wrong: increments wrong_count, does not eliminate yet and turn stays
        let err = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(err, DeclareError::NotAdjacent);
        assert_eq!(game.players[0].wrong_count, 1);
        assert!(game.players[0].active);
        assert_eq!(game.active_count(), 2);

        // second wrong by same player (turn didn't advance)
        let _ = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(game.players[0].wrong_count, 2);

        // third wrong -> eliminated
        let _ = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(game.players[0].wrong_count, 3);
        assert!(!game.players[0].active);
        assert_eq!(game.active_count(), 1);
    }

    #[test]
    fn used_candidate_increments_wrong_count() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Kanagawa.id()], &PREFECTURE_DB).unwrap();
        let err = game.declare(&[P::Tokyo.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(err, DeclareError::NotAdjacent);
        // player 1 (index 1) should have wrong_count 1, but still active
        assert_eq!(game.players[1].wrong_count, 1);
        assert!(game.players[1].active);
    }

    #[test]
    fn not_found_does_not_eliminate_player() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        let err = game.declare(&[], &PREFECTURE_DB).unwrap_err();
        assert_eq!(err, DeclareError::NotFound);
        assert!(game.players[0].active);
    }

    #[test]
    fn first_valid_candidate_is_used() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Tokyo.id(), P::Kanagawa.id()], &PREFECTURE_DB)
            .unwrap();
        assert_eq!(game.current, P::Kanagawa.id());
    }

    #[test]
    fn current_stays_on_invalid_declare() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(game.current, P::Tokyo.id());
    }

    #[test]
    fn all_eliminated_causes_game_over() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        // eliminate P1 with 3 wrongs
        for _ in 0..3 {
            let _ = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        }
        // eliminate P2 with 3 wrongs
        for _ in 0..3 {
            let _ = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        }
        assert_eq!(game.phase, GamePhase::GameOver);
    }

    #[test]
    fn single_player_game() {
        let mut game = GameState::new(P::Tokyo.id(), vec!["P1".into()], &PREFECTURE_DB);
        game.declare(&[P::Kanagawa.id()], &PREFECTURE_DB).unwrap();
        assert_eq!(game.players[0].score, 1);
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    fn ranking_by_score() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Kanagawa.id()], &PREFECTURE_DB).unwrap(); // P1: 1点
        // P2 makes three wrong declares and is eliminated
        for _ in 0..3 {
            let _ = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        }
        let ranking = game.ranking();
        assert_eq!(ranking[0], 0);
        assert_eq!(ranking[1], 1);
    }
}
