use serde::{Deserialize, Serialize};

use crate::location::{LocationId, RegionDatabase};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub score: u32,
    pub active: bool,
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
            })
            .collect();
        let mut state = Self {
            current: start,
            used: vec![start],
            players,
            current_player_index: 0,
            phase: GamePhase::Playing,
        };
        state.eliminate_stuck_players(db);
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
            self.current = id;
            self.used.push(id);
            self.players[self.current_player_index].score += 1;
            self.advance_to_next_active();
            self.eliminate_stuck_players(db);
            Ok(())
        } else {
            self.eliminate_current_and_advance(db);
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
    fn not_adjacent_eliminates_player() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        let err = game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(err, DeclareError::NotAdjacent);
        assert!(!game.players[0].active);
        assert_eq!(game.active_count(), 1);
    }

    #[test]
    fn used_candidate_eliminates_player() {
        let mut game = GameState::new(
            P::Tokyo.id(),
            vec!["P1".into(), "P2".into()],
            &PREFECTURE_DB,
        );
        game.declare(&[P::Kanagawa.id()], &PREFECTURE_DB).unwrap();
        let err = game.declare(&[P::Tokyo.id()], &PREFECTURE_DB).unwrap_err();
        assert_eq!(err, DeclareError::NotAdjacent);
        assert!(!game.players[1].active);
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
        game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err(); // P1脱落
        game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err(); // P2脱落
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
        game.declare(&[P::Osaka.id()], &PREFECTURE_DB).unwrap_err(); // P2脱落: 0点
        let ranking = game.ranking();
        assert_eq!(ranking[0], 0);
        assert_eq!(ranking[1], 1);
    }
}
