use serde::{Deserialize, Serialize};

use crate::prefecture::{PrefectureId, get_neighbors, is_adjacent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub score: u32,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Playing,
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current: PrefectureId,
    pub used: Vec<PrefectureId>,
    pub players: Vec<PlayerState>,
    pub current_player_index: usize,
    pub phase: GamePhase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeclareError {
    NotAdjacent,
    AlreadyUsed,
    GameAlreadyOver,
}

impl GameState {
    pub fn new(start: PrefectureId, player_count: usize) -> Self {
        assert!(player_count >= 1, "プレイヤーは1人以上必要です");
        let mut state = Self {
            current: start,
            used: vec![start],
            players: vec![
                PlayerState {
                    score: 0,
                    active: true
                };
                player_count
            ],
            current_player_index: 0,
            phase: GamePhase::Playing,
        };
        // 最初のプレイヤーが動けるか確認
        state.eliminate_stuck_players();
        state
    }

    pub fn active_count(&self) -> usize {
        self.players.iter().filter(|p| p.active).count()
    }

    pub fn current_player(&self) -> &PlayerState {
        &self.players[self.current_player_index]
    }

    // スコアの高い順にインデックスを返す
    pub fn ranking(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.players.len()).collect();
        indices.sort_by(|&a, &b| self.players[b].score.cmp(&self.players[a].score));
        indices
    }

    pub fn declare(&mut self, id: PrefectureId) -> Result<(), DeclareError> {
        if self.phase != GamePhase::Playing {
            return Err(DeclareError::GameAlreadyOver);
        }

        if self.used.contains(&id) {
            self.eliminate_current_and_advance();
            return Err(DeclareError::AlreadyUsed);
        }

        if !is_adjacent(self.current, id) {
            self.eliminate_current_and_advance();
            return Err(DeclareError::NotAdjacent);
        }

        // 有効な宣言
        self.current = id;
        self.used.push(id);
        self.players[self.current_player_index].score += 1;
        self.advance_to_next_active();
        self.eliminate_stuck_players();

        Ok(())
    }

    fn eliminate_current_and_advance(&mut self) {
        self.players[self.current_player_index].active = false;
        if self.active_count() == 0 {
            self.phase = GamePhase::GameOver;
            return;
        }
        self.advance_to_next_active();
        self.eliminate_stuck_players();
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

    // 有効な手がないプレイヤーを連続して脱落させる
    fn eliminate_stuck_players(&mut self) {
        let n = self.players.len();
        let mut checked = 0;
        while checked < n {
            if !self.players[self.current_player_index].active {
                break;
            }
            let has_valid_move = get_neighbors(self.current)
                .iter()
                .any(|&neighbor| !self.used.contains(&neighbor));
            if !has_valid_move {
                self.players[self.current_player_index].active = false;
                if self.active_count() == 0 {
                    self.phase = GamePhase::GameOver;
                    return;
                }
                self.advance_to_next_active();
                checked += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_starts_with_first_player() {
        let game = GameState::new(PrefectureId::Tokyo, 2);
        assert_eq!(game.current_player_index, 0);
        assert_eq!(game.phase, GamePhase::Playing);
        assert_eq!(game.active_count(), 2);
    }

    #[test]
    fn valid_declare_advances_player_and_score() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        game.declare(PrefectureId::Kanagawa).unwrap();
        assert_eq!(game.current_player_index, 1);
        assert_eq!(game.players[0].score, 1);
        assert_eq!(game.current, PrefectureId::Kanagawa);
    }

    #[test]
    fn not_adjacent_eliminates_player() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        let err = game.declare(PrefectureId::Osaka).unwrap_err();
        assert_eq!(err, DeclareError::NotAdjacent);
        assert!(!game.players[0].active);
        assert_eq!(game.active_count(), 1);
    }

    #[test]
    fn already_used_eliminates_player() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        game.declare(PrefectureId::Kanagawa).unwrap();
        let err = game.declare(PrefectureId::Tokyo).unwrap_err();
        assert_eq!(err, DeclareError::AlreadyUsed);
        assert!(!game.players[1].active);
    }

    #[test]
    fn current_stays_on_invalid_declare() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        game.declare(PrefectureId::Osaka).unwrap_err();
        // 現在地は東京のまま
        assert_eq!(game.current, PrefectureId::Tokyo);
    }

    #[test]
    fn all_eliminated_causes_game_over() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        game.declare(PrefectureId::Osaka).unwrap_err(); // P1脱落
        game.declare(PrefectureId::Osaka).unwrap_err(); // P2脱落
        assert_eq!(game.phase, GamePhase::GameOver);
    }

    #[test]
    fn single_player_game() {
        let mut game = GameState::new(PrefectureId::Tokyo, 1);
        game.declare(PrefectureId::Kanagawa).unwrap();
        assert_eq!(game.players[0].score, 1);
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    fn ranking_by_score() {
        let mut game = GameState::new(PrefectureId::Tokyo, 2);
        game.declare(PrefectureId::Kanagawa).unwrap(); // P1: 1点
        game.declare(PrefectureId::Osaka).unwrap_err(); // P2脱落: 0点
        let ranking = game.ranking();
        assert_eq!(ranking[0], 0); // P1が1位
        assert_eq!(ranking[1], 1); // P2が2位
    }
}
