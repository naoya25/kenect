use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

use shared::game::{GamePhase, GameState};
use shared::location::{LocationId, RegionDatabase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionKind {
    Winning,
    Losing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuMove {
    pub location: LocationId,
    pub position: PositionKind,
}

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
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(1);
        Self::with_seed(seed)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: XorShift64::new(seed),
        }
    }

    /// Returns an optimal move in winning positions and a random legal move in losing positions.
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

pub fn is_winning_position(state: &GameState, db: &RegionDatabase) -> bool {
    if state.phase != GamePhase::Playing {
        return false;
    }

    let graph = RemainingGraph::from_state(state, db);
    graph.is_forced_matched(state.current)
}

pub fn optimal_move(state: &GameState, db: &RegionDatabase) -> Option<LocationId> {
    if state.phase != GamePhase::Playing {
        return None;
    }

    let graph = RemainingGraph::from_state(state, db);
    graph.forced_matching_partner(state.current)
}

#[derive(Debug, Clone)]
struct RemainingGraph {
    ids: Vec<LocationId>,
    index_by_id: HashMap<LocationId, usize>,
    adjacency: Vec<Vec<usize>>,
}

impl RemainingGraph {
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

    fn matching(&self) -> Vec<Option<usize>> {
        Blossom::new(self.adjacency.clone()).maximum_matching()
    }

    fn matching_size_without(&self, removed: usize) -> usize {
        let mut old_to_new = vec![None; self.ids.len()];
        let mut next = 0;
        for (index, slot) in old_to_new.iter_mut().enumerate() {
            if index != removed {
                *slot = Some(next);
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

        matching_size(&Blossom::new(adjacency).maximum_matching())
    }

    fn is_forced_matched(&self, id: LocationId) -> bool {
        let Some(&index) = self.index_by_id.get(&id) else {
            return false;
        };
        let matching = self.matching();
        let size = matching_size(&matching);
        size > self.matching_size_without(index)
    }

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

fn matching_size(matching: &[Option<usize>]) -> usize {
    matching.iter().filter(|partner| partner.is_some()).count() / 2
}

#[derive(Debug, Clone)]
struct Blossom {
    adjacency: Vec<Vec<usize>>,
    matching: Vec<Option<usize>>,
    parent: Vec<Option<usize>>,
    base: Vec<usize>,
    used: Vec<bool>,
    blossom: Vec<bool>,
    queue: VecDeque<usize>,
}

impl Blossom {
    fn new(adjacency: Vec<Vec<usize>>) -> Self {
        let n = adjacency.len();
        Self {
            adjacency,
            matching: vec![None; n],
            parent: vec![None; n],
            base: (0..n).collect(),
            used: vec![false; n],
            blossom: vec![false; n],
            queue: VecDeque::new(),
        }
    }

    fn maximum_matching(mut self) -> Vec<Option<usize>> {
        for root in 0..self.adjacency.len() {
            if self.matching[root].is_none() {
                self.find_path(root);
            }
        }
        self.matching
    }

    fn find_path(&mut self, root: usize) -> Option<usize> {
        self.used.fill(false);
        self.parent.fill(None);
        for (index, base) in self.base.iter_mut().enumerate() {
            *base = index;
        }

        self.queue.clear();
        self.queue.push_back(root);
        self.used[root] = true;

        while let Some(vertex) = self.queue.pop_front() {
            let neighbors = self.adjacency[vertex].clone();
            for to in neighbors {
                if self.base[vertex] == self.base[to] || self.matching[vertex] == Some(to) {
                    continue;
                }

                if to == root
                    || self.matching[to]
                        .and_then(|matched| self.parent[matched])
                        .is_some()
                {
                    let current_base = self.lca(vertex, to);
                    self.blossom.fill(false);
                    self.mark_path(vertex, current_base, to);
                    self.mark_path(to, current_base, vertex);
                    for index in 0..self.adjacency.len() {
                        if self.blossom[self.base[index]] {
                            self.base[index] = current_base;
                            if !self.used[index] {
                                self.used[index] = true;
                                self.queue.push_back(index);
                            }
                        }
                    }
                } else if self.parent[to].is_none() {
                    self.parent[to] = Some(vertex);
                    if let Some(matched) = self.matching[to] {
                        self.used[matched] = true;
                        self.queue.push_back(matched);
                    } else {
                        self.augment(to);
                        return Some(to);
                    }
                }
            }
        }

        None
    }

    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let mut used_path = vec![false; self.adjacency.len()];
        loop {
            a = self.base[a];
            used_path[a] = true;
            if let Some(matched) = self.matching[a]
                && let Some(parent) = self.parent[matched]
            {
                a = parent;
                continue;
            }
            break;
        }

        loop {
            b = self.base[b];
            if used_path[b] {
                return b;
            }
            let matched = self.matching[b].expect("LCA requires an alternating path");
            b = self.parent[matched].expect("LCA requires parent links");
        }
    }

    fn mark_path(&mut self, mut vertex: usize, base: usize, mut child: usize) {
        while self.base[vertex] != base {
            self.blossom[self.base[vertex]] = true;
            let matched = self.matching[vertex].expect("blossom path vertex must be matched");
            self.blossom[self.base[matched]] = true;
            self.parent[vertex] = Some(child);
            child = matched;
            vertex = self.parent[matched].expect("matched vertex must have a parent");
        }
    }

    fn augment(&mut self, mut vertex: usize) {
        while let Some(previous) = self.parent[vertex] {
            let next = self.matching[previous];
            self.matching[vertex] = Some(previous);
            self.matching[previous] = Some(vertex);
            if let Some(next) = next {
                vertex = next;
            } else {
                break;
            }
        }
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
    use shared::data::PREFECTURE_DB;
    use shared::location::{Region, RegionKind};

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
    fn blossom_matches_odd_cycle() {
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let matching = Blossom::new(graph).maximum_matching();
        assert_eq!(matching_size(&matching), 1);
    }

    #[test]
    fn blossom_matches_bruteforce_for_all_graphs_up_to_six_vertices() {
        for n in 0..=6 {
            let edge_count = if n == 0 { 0 } else { n * (n - 1) / 2 };
            for mask in 0..(1usize << edge_count) {
                let mut adjacency = vec![Vec::new(); n];
                let mut bit = 0;
                for a in 0..n {
                    for b in (a + 1)..n {
                        if (mask & (1 << bit)) != 0 {
                            adjacency[a].push(b);
                            adjacency[b].push(a);
                        }
                        bit += 1;
                    }
                }

                let matching = Blossom::new(adjacency.clone()).maximum_matching();
                assert_eq!(
                    matching_size(&matching),
                    brute_force_matching_size(&adjacency),
                    "n={n}, mask={mask}"
                );
            }
        }
    }

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

    fn brute_force_matching_size(adjacency: &[Vec<usize>]) -> usize {
        let Some(first) = (0..adjacency.len()).find(|&index| !adjacency[index].is_empty()) else {
            return 0;
        };

        let mut without_first = adjacency.to_vec();
        for neighbors in &mut without_first {
            neighbors.retain(|&to| to != first);
        }
        without_first[first].clear();
        let mut best = brute_force_matching_size(&without_first);

        for &partner in &adjacency[first] {
            if partner < first {
                continue;
            }
            let mut rest = adjacency.to_vec();
            for neighbors in &mut rest {
                neighbors.retain(|&to| to != first && to != partner);
            }
            rest[first].clear();
            rest[partner].clear();
            best = best.max(1 + brute_force_matching_size(&rest));
        }

        best
    }
}
