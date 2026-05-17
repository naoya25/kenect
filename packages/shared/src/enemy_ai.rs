//! Undirected Vertex Geography の最適手を選ぶ CPU。
//!
//! 現在地を含む「未使用頂点 + 現在地」の残りグラフ `G` を考えます。
//! `nu(G)` を `G` の最大マッチングサイズとすると、現在地 `current` がすべての
//! 最大マッチングでマッチされる条件は `nu(G) > nu(G - current)` です。
//!
//! この条件が成り立つ局面は現在手番の勝ち局面です。その場合、現在地の最大マッチング上の
//! 相手へ動くことで、相手に負け局面を渡せます。条件が成り立たない負け局面では、
//! この CPU は合法手からランダムに選びます。

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::algorithm::blossom::{matching_size, maximum_matching};
use crate::game::{GamePhase, GameState};
use crate::location::{LocationId, RegionDatabase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionKind {
    /// 現在手番が勝ち局面で、CPU は最適手を選んだ。
    Winning,
    /// 現在手番が負け局面で、CPU は合法手からランダムに選んだ。
    Losing,
}

/// CPU が選んだ手と、その手を選ぶ前の局面の勝敗種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuMove {
    /// 宣言する移動先。
    pub location: LocationId,
    /// 手を選ぶ前の局面が勝ち局面か負け局面か。
    pub position: PositionKind,
}

/// Undirected Vertex Geography の最適 CPU。
///
/// 勝ち局面では最大マッチングに基づく最適手を返し、負け局面では合法手から乱択します。
#[derive(Debug, Clone)]
pub struct EnemyAi {
    rng: XorShift64,
}

impl Default for EnemyAi {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemyAi {
    /// 現在時刻を seed にした CPU を作る。
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(1);
        Self::with_seed(seed)
    }

    /// 指定した seed で CPU を作る。
    ///
    /// テストや再現性が必要な場面ではこちらを使います。
    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: XorShift64::new(seed),
        }
    }

    /// 現在の盤面に対して CPU の手を 1 つ選ぶ。
    ///
    /// `state.phase` が `Playing` でない場合、または合法手がない場合は `None` を返します。
    /// 勝ち局面なら相手に負け局面を渡す最適手を返し、負け局面なら合法手からランダムに返します。
    pub fn choose_move(&mut self, state: &GameState, db: &RegionDatabase) -> Option<CpuMove> {
        if state.phase != GamePhase::Playing {
            return None;
        }

        let valid_moves = db.valid_move_ids(state.current, &state.used);
        if valid_moves.is_empty() {
            return None;
        }

        if let Some(location) = optimal_move(state, db) {
            Some(CpuMove {
                location,
                position: PositionKind::Winning,
            })
        } else {
            let index = self.rng.next_usize(valid_moves.len());
            Some(CpuMove {
                location: valid_moves[index],
                position: PositionKind::Losing,
            })
        }
    }
}

/// 現在手番のプレイヤーにとって勝ち局面かどうかを返す。
///
/// 判定は `nu(G) > nu(G - current)` で行います。ここで `G` は現在地を含む残りグラフ、
/// `nu` は最大マッチングサイズです。
pub fn is_winning_position(state: &GameState, db: &RegionDatabase) -> bool {
    if state.phase != GamePhase::Playing {
        return false;
    }

    let graph = RemainingGraph::from_state(state, db);
    graph.is_forced_matched(state.current)
}

/// 勝ち局面なら、相手に負け局面を渡す移動先を返す。
///
/// 返す手は現在地の最大マッチング上の相手です。負け局面、ゲーム終了後、または合法手がない
/// 局面では `None` を返します。
pub fn optimal_move(state: &GameState, db: &RegionDatabase) -> Option<LocationId> {
    if state.phase != GamePhase::Playing {
        return None;
    }

    let graph = RemainingGraph::from_state(state, db);
    graph.forced_matching_partner(state.current)
}

#[derive(Debug, Clone)]
struct RemainingGraph {
    /// Blossom 用の頂点 index から元の `LocationId` への対応。
    ids: Vec<LocationId>,
    /// 元の `LocationId` から Blossom 用の頂点 index への対応。
    index_by_id: HashMap<LocationId, usize>,
    /// 現在地を含む残りグラフの隣接リスト。
    adjacency: Vec<Vec<usize>>,
}

impl RemainingGraph {
    /// `GameState` と `RegionDatabase` から、最大マッチング用の `usize` グラフを作る。
    ///
    /// `state.current` は `state.used` に含まれますが、現在地としてグラフに残します。
    /// それ以外の使用済み頂点は除外します。
    fn from_state(state: &GameState, db: &RegionDatabase) -> Self {
        let used: HashSet<LocationId> = state.used.iter().copied().collect();
        let ids: Vec<LocationId> = db
            .all_ids()
            .into_iter()
            .filter(|&id| id == state.current || !used.contains(&id))
            .collect();
        let index_by_id: HashMap<LocationId, usize> = ids
            .iter()
            .copied()
            .enumerate()
            .map(|(index, id)| (id, index))
            .collect();
        let mut adjacency = vec![Vec::new(); ids.len()];

        for (from, &id) in ids.iter().enumerate() {
            if let Some(region) = db.get(id) {
                for &neighbor in region.neighbors {
                    if let Some(&to) = index_by_id.get(&neighbor) {
                        adjacency[from].push(to);
                    }
                }
            }
        }

        Self {
            ids,
            index_by_id,
            adjacency,
        }
    }

    /// 残りグラフ `G` の最大マッチングを返す。
    fn matching(&self) -> Vec<Option<usize>> {
        maximum_matching(self.adjacency.clone())
    }

    /// 指定した頂点 index を取り除いたグラフの最大マッチングサイズを返す。
    fn matching_size_without(&self, removed: usize) -> usize {
        let mut old_to_new = vec![None; self.ids.len()];
        let mut next = 0;
        for (index, new_index) in old_to_new.iter_mut().enumerate() {
            if index != removed {
                *new_index = Some(next);
                next += 1;
            }
        }

        let mut adjacency = vec![Vec::new(); next];
        for from in 0..self.ids.len() {
            let Some(new_from) = old_to_new[from] else {
                continue;
            };
            for &to in &self.adjacency[from] {
                if let Some(new_to) = old_to_new[to] {
                    adjacency[new_from].push(new_to);
                }
            }
        }

        matching_size(&maximum_matching(adjacency))
    }

    /// `id` がすべての最大マッチングでマッチされるかどうかを返す。
    ///
    /// `nu(G) > nu(G - id)` なら、`id` を未マッチにした最大マッチングは存在しません。
    fn is_forced_matched(&self, id: LocationId) -> bool {
        let Some(&index) = self.index_by_id.get(&id) else {
            return false;
        };
        let matching = self.matching();
        let size = matching_size(&matching);
        size > self.matching_size_without(index)
    }

    /// `id` が強制的にマッチされる場合、その最大マッチング上の相手を返す。
    ///
    /// Undirected Vertex Geography では、この相手へ動くことが勝ち局面での最適手になります。
    fn forced_matching_partner(&self, id: LocationId) -> Option<LocationId> {
        if !self.is_forced_matched(id) {
            return None;
        }

        let index = self.index_by_id[&id];
        self.matching()
            .get(index)
            .and_then(|partner| partner.map(|partner| self.ids[partner]))
    }
}

#[derive(Debug, Clone)]
struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn next_usize(&mut self, n: usize) -> usize {
        debug_assert!(n > 0);
        (self.next_u64() % n as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::PREFECTURE_DB;
    use crate::location::{Region, RegionKind};

    static PATH3: &[Region] = &[
        Region {
            id: LocationId(1),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "A",
            kana: "A",
            roman: "a",
            neighbors: &[LocationId(2)],
        },
        Region {
            id: LocationId(2),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "B",
            kana: "B",
            roman: "b",
            neighbors: &[LocationId(1), LocationId(3)],
        },
        Region {
            id: LocationId(3),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "C",
            kana: "C",
            roman: "c",
            neighbors: &[LocationId(2)],
        },
    ];

    static TRIANGLE: &[Region] = &[
        Region {
            id: LocationId(1),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "A",
            kana: "A",
            roman: "a",
            neighbors: &[LocationId(2), LocationId(3)],
        },
        Region {
            id: LocationId(2),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "B",
            kana: "B",
            roman: "b",
            neighbors: &[LocationId(1), LocationId(3)],
        },
        Region {
            id: LocationId(3),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "C",
            kana: "C",
            roman: "c",
            neighbors: &[LocationId(1), LocationId(2)],
        },
    ];

    #[test]
    fn path_endpoint_is_losing() {
        let db = RegionDatabase::new(PATH3);
        let state = GameState::new(LocationId(1), vec!["cpu".into()], &db);

        assert!(!is_winning_position(&state, &db));
        assert_eq!(optimal_move(&state, &db), None);
    }

    #[test]
    fn path_center_is_winning() {
        let db = RegionDatabase::new(PATH3);
        let state = GameState::new(LocationId(2), vec!["cpu".into()], &db);

        assert!(is_winning_position(&state, &db));
        assert!(matches!(
            optimal_move(&state, &db),
            Some(LocationId(1) | LocationId(3))
        ));
    }

    #[test]
    fn triangle_uses_blossom_and_is_losing_from_any_vertex() {
        let db = RegionDatabase::new(TRIANGLE);
        let state = GameState::new(LocationId(1), vec!["cpu".into()], &db);

        assert!(!is_winning_position(&state, &db));
        assert_eq!(optimal_move(&state, &db), None);
    }

    #[test]
    fn choose_move_reports_losing_when_only_random_move_exists() {
        let db = RegionDatabase::new(PATH3);
        let state = GameState::new(LocationId(1), vec!["cpu".into()], &db);
        let mut ai = EnemyAi::with_seed(1);

        let chosen = ai.choose_move(&state, &db).unwrap();
        assert_eq!(chosen.location, LocationId(2));
        assert_eq!(chosen.position, PositionKind::Losing);
    }

    #[test]
    fn prefecture_database_returns_legal_move_when_possible() {
        let state = GameState::new(LocationId(13), vec!["cpu".into()], &PREFECTURE_DB);
        let mut ai = EnemyAi::with_seed(1);

        let chosen = ai.choose_move(&state, &PREFECTURE_DB).unwrap();
        assert!(PREFECTURE_DB.is_adjacent(state.current, chosen.location));
        assert!(!state.used.contains(&chosen.location));
    }
}
