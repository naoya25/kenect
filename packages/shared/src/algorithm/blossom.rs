//! Edmonds' Blossom Algorithm による一般グラフの最大マッチング。
//!
//! この module は `LocationId` や `GameState` を知らない純粋なグラフアルゴリズムです。
//! 頂点は `usize` の index で表し、奇サイクルを blossom として縮約しながら増加路を探します。

use std::collections::VecDeque;

/// 隣接リスト表現の無向グラフに対して最大マッチングを返す。
///
/// `adjacency[v]` は頂点 index `v` に隣接する頂点 index の一覧です。
/// 戻り値 `matching[v]` は `v` のマッチ相手の頂点 index、未マッチなら `None` です。
pub fn maximum_matching(adjacency: Vec<Vec<usize>>) -> Vec<Option<usize>> {
    Blossom::new(adjacency).maximum_matching()
}

/// マッチング配列から、マッチされた辺数を返す。
///
/// `matching` は `maximum_matching` と同じ形式で、1 本のマッチング辺は両端から
/// 2 回数えられるため、この関数はマッチ済み頂点数を 2 で割って返します。
pub fn matching_size(matching: &[Option<usize>]) -> usize {
    matching.iter().filter(|partner| partner.is_some()).count() / 2
}

#[derive(Debug, Clone)]
struct Blossom {
    /// 入力グラフの隣接リスト。頂点は `0..adjacency.len()` の index。
    adjacency: Vec<Vec<usize>>,
    /// 現在のマッチング。`matching[v]` は `v` のマッチ相手、未マッチなら `None`。
    matching: Vec<Option<usize>>,
}

/// `find_path` で使う探索用バッファ。
///
/// `Blossom` が保持する永続状態は `matching` で、こちらは root ごとの増加路探索中だけ
/// 意味を持つ一時状態です。`maximum_matching` 内で 1 回だけ確保し、各 `find_path` で
/// 初期化して再利用します。
#[derive(Debug, Clone)]
struct SearchScratch {
    /// 交互森で各頂点へ到達した直前の頂点。増加路の復元と blossom 縮約に使う。
    parent: Vec<Option<usize>>,
    /// blossom 縮約後の代表頂点。縮約していない頂点では `base[v] == v`。
    base: Vec<usize>,
    /// 現在の BFS で交互森に入っている頂点。
    used: Vec<bool>,
    /// 今回縮約する blossom に含まれる base を示す一時バッファ。
    blossom: Vec<bool>,
    /// 増加路探索用の BFS キュー。
    queue: VecDeque<usize>,
}

impl SearchScratch {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![None; n],
            base: (0..n).collect(),
            used: vec![false; n],
            blossom: vec![false; n],
            queue: VecDeque::new(),
        }
    }

    fn reset(&mut self, root: usize) {
        self.used.fill(false);
        self.parent.fill(None);
        for (index, base) in self.base.iter_mut().enumerate() {
            *base = index;
        }

        self.queue.clear();
        self.queue.push_back(root);
        self.used[root] = true;
    }
}

impl Blossom {
    /// 空のマッチングから探索を開始する内部状態を作る。
    fn new(adjacency: Vec<Vec<usize>>) -> Self {
        let n = adjacency.len();
        Self {
            adjacency,
            matching: vec![None; n],
        }
    }

    /// 未マッチ頂点を root にして増加路探索を繰り返し、`matching` を最大化する。
    fn maximum_matching(mut self) -> Vec<Option<usize>> {
        let mut scratch = SearchScratch::new(self.adjacency.len());
        for root in 0..self.adjacency.len() {
            if self.matching[root].is_none() {
                self.find_path(root, &mut scratch);
            }
        }
        self.matching
    }

    /// `root` から交互森を伸ばし、増加路を見つけたら `matching` を更新する。
    ///
    /// 探索中に奇サイクルを見つけた場合は、`base` と `parent` を更新して blossom を縮約します。
    /// 増加路が見つかった場合は終端頂点を返し、見つからなければ `None` を返します。
    fn find_path(&mut self, root: usize, scratch: &mut SearchScratch) -> Option<usize> {
        scratch.reset(root);

        while let Some(vertex) = scratch.queue.pop_front() {
            let neighbors = self.adjacency[vertex].clone();
            for to in neighbors {
                if scratch.base[vertex] == scratch.base[to] || self.matching[vertex] == Some(to) {
                    continue;
                }

                if to == root
                    || self.matching[to]
                        .and_then(|matched| scratch.parent[matched])
                        .is_some()
                {
                    let current_base = self.lca(vertex, to, scratch);
                    scratch.blossom.fill(false);
                    self.mark_path(vertex, current_base, to, scratch);
                    self.mark_path(to, current_base, vertex, scratch);
                    for index in 0..self.adjacency.len() {
                        if scratch.blossom[scratch.base[index]] {
                            scratch.base[index] = current_base;
                            if !scratch.used[index] {
                                scratch.used[index] = true;
                                scratch.queue.push_back(index);
                            }
                        }
                    }
                } else if scratch.parent[to].is_none() {
                    scratch.parent[to] = Some(vertex);
                    if let Some(matched) = self.matching[to] {
                        scratch.used[matched] = true;
                        scratch.queue.push_back(matched);
                    } else {
                        self.augment(to, scratch);
                        return Some(to);
                    }
                }
            }
        }

        None
    }

    /// blossom 縮約で使う、交互森上の最小共通 base を返す。
    fn lca(&self, mut a: usize, mut b: usize, scratch: &SearchScratch) -> usize {
        let mut used_path = vec![false; self.adjacency.len()];
        loop {
            a = scratch.base[a];
            used_path[a] = true;
            if let Some(matched) = self.matching[a]
                && let Some(parent) = scratch.parent[matched]
            {
                a = parent;
                continue;
            }
            break;
        }

        loop {
            b = scratch.base[b];
            if used_path[b] {
                return b;
            }
            let matched = self.matching[b].expect("LCA requires an alternating path");
            b = scratch.parent[matched].expect("LCA requires parent links");
        }
    }

    /// `vertex` から `base` までの交互路をたどり、縮約対象の blossom を印付けする。
    ///
    /// `blossom` に含まれる base を `true` にし、縮約後も探索を続けられるよう
    /// `parent` の向きを `child` 側へつなぎ直します。
    fn mark_path(
        &self,
        mut vertex: usize,
        base: usize,
        mut child: usize,
        scratch: &mut SearchScratch,
    ) {
        while scratch.base[vertex] != base {
            scratch.blossom[scratch.base[vertex]] = true;
            let matched = self.matching[vertex].expect("blossom path vertex must be matched");
            scratch.blossom[scratch.base[matched]] = true;
            scratch.parent[vertex] = Some(child);
            child = matched;
            vertex = scratch.parent[matched].expect("matched vertex must have a parent");
        }
    }

    /// 見つかった増加路に沿って `matching` を反転する。
    ///
    /// `vertex` は未マッチの終端頂点で、`parent` を root 側へたどりながら
    /// 辺の採用・非採用を入れ替えます。
    fn augment(&mut self, mut vertex: usize, scratch: &SearchScratch) {
        while let Some(previous) = scratch.parent[vertex] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blossom_matches_odd_cycle() {
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let matching = maximum_matching(graph);
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

                let matching = maximum_matching(adjacency.clone());
                assert_eq!(
                    matching_size(&matching),
                    brute_force_matching_size(&adjacency),
                    "n={n}, mask={mask}"
                );
            }
        }
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
