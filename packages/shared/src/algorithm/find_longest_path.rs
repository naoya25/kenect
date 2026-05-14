use super::rng::Rng;
use crate::location::{LocationId, Region};

/// 焼きなまし法で start を始点とする長いパスを返す.
///
/// 初期解にランダム延伸を使い、切り詰め + 再延伸を繰り返す.
/// 最長パスを保証しないが、高速に長いパスを求める.
///
/// ## 引数
/// * `graph`: 隣接リスト (Region のスライス)
/// * `start`: 始点 (Region への参照)
/// ## 返り値
/// * start を始点とする長いパス
pub fn find_longest_path(graph: &[Region], start: &Region) -> Vec<LocationId> {
    let max_id = graph.iter().map(|r| r.id.0 as usize).max().unwrap_or(0);

    let mut id_map: Vec<Option<&Region>> = vec![None; max_id + 1];
    for region in graph {
        id_map[region.id.0 as usize] = Some(region);
    }

    if id_map.get(start.id.0 as usize).and_then(|r| *r).is_none() {
        return Vec::new();
    }

    let total = graph.len();

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42);
    let mut rng = Rng::new(seed ^ 0xcafe_babe_dead_beef);

    // 初期解: ランダム延伸 + Posa rotation
    let mut visited = vec![false; max_id + 1];
    visited[start.id.0 as usize] = true;
    let mut path = vec![start.id];
    extend_randomly(&id_map, &mut path, &mut visited, max_id, &mut rng);

    let mut best = path.clone();
    if best.len() == total {
        return best;
    }

    const ITERATIONS: u32 = 500_000;
    let temp_init = 8.0_f64;
    let temp_min = 0.05_f64;
    let cooling = (temp_min / temp_init).powf(1.0 / ITERATIONS as f64);
    let mut temp = temp_init;

    for _ in 0..ITERATIONS {
        if best.len() == total {
            break;
        }

        if path.len() <= 1 {
            temp *= cooling;
            continue;
        }

        // ランダムな cut point まで保持（start ノードは必ず残す）
        let keep = 1 + rng.next_usize(path.len() - 1);
        let old_path = path.clone();

        // 切り詰め
        for id in path.drain(keep..) {
            visited[id.0 as usize] = false;
        }

        // ランダムに再延伸 + Posa rotation
        extend_randomly(&id_map, &mut path, &mut visited, max_id, &mut rng);

        // 受理判定
        let delta = path.len() as f64 - old_path.len() as f64;
        let accept = delta >= 0.0 || rng.next_f64() < (delta / temp).exp();

        if accept {
            if path.len() > best.len() {
                best.clone_from(&path);
            }
        } else {
            // 棄却: 旧状態を復元
            for id in path.drain(..) {
                visited[id.0 as usize] = false;
            }
            for &id in &old_path {
                visited[id.0 as usize] = true;
            }
            path = old_path;
        }

        temp *= cooling;
    }

    best
}

fn extend_randomly(
    id_map: &[Option<&Region>],
    path: &mut Vec<LocationId>,
    visited: &mut [bool],
    max_id: usize,
    rng: &mut Rng,
) {
    while let Some(&current) = path.last() {
        let nbs = unvisited_neighbors(id_map, current, visited, max_id);

        if nbs.is_empty() {
            if !try_posa_rotation(id_map, path, visited, max_id, rng) {
                break;
            }
            continue;
        }

        let nb = nbs[rng.next_usize(nbs.len())];
        visited[nb.0 as usize] = true;
        path.push(nb);
    }
}

fn unvisited_neighbors(
    id_map: &[Option<&Region>],
    current: LocationId,
    visited: &[bool],
    max_id: usize,
) -> Vec<LocationId> {
    id_map
        .get(current.0 as usize)
        .and_then(|r| *r)
        .map(|r| {
            r.neighbors
                .iter()
                .filter(|&&nb| {
                    let i = nb.0 as usize;
                    i <= max_id && !visited[i] && id_map[i].is_some()
                })
                .copied()
                .collect()
        })
        .unwrap_or_default()
}

fn has_unvisited_neighbor(
    id_map: &[Option<&Region>],
    current: LocationId,
    visited: &[bool],
    max_id: usize,
) -> bool {
    id_map
        .get(current.0 as usize)
        .and_then(|r| *r)
        .map(|r| {
            r.neighbors.iter().any(|&nb| {
                let i = nb.0 as usize;
                i <= max_id && !visited[i] && id_map[i].is_some()
            })
        })
        .unwrap_or(false)
}

fn is_adjacent(id_map: &[Option<&Region>], a: LocationId, b: LocationId) -> bool {
    id_map
        .get(a.0 as usize)
        .and_then(|r| *r)
        .map(|r| r.neighbors.contains(&b))
        .unwrap_or(false)
}

fn try_posa_rotation(
    id_map: &[Option<&Region>],
    path: &mut [LocationId],
    visited: &[bool],
    max_id: usize,
    rng: &mut Rng,
) -> bool {
    if path.len() < 3 {
        return false;
    }

    let end = *path.last().unwrap();
    let mut candidates = Vec::new();

    for i in 0..path.len() - 1 {
        let new_end = path[i + 1];
        if is_adjacent(id_map, end, path[i])
            && has_unvisited_neighbor(id_map, new_end, visited, max_id)
        {
            candidates.push(i);
        }
    }

    if candidates.is_empty() {
        return false;
    }

    let rotation_index = candidates[rng.next_usize(candidates.len())];

    // v0 ... vi, v(i+1) ... vk から
    // v0 ... vi, vk ... v(i+1) へ並べ替える。訪問済み集合は変わらない。
    path[rotation_index + 1..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::PREFECTURE_DB;

    fn hokkaido() -> &'static Region {
        PREFECTURE_DB
            .all_regions()
            .iter()
            .find(|r| r.roman == "Hokkaido")
            .unwrap()
    }

    #[test]
    fn path_is_nonempty() {
        let path = find_longest_path(PREFECTURE_DB.all_regions(), hokkaido());
        println!("Found path: {:?}", path);
        println!("Path length: {}", path.len());
        assert!(!path.is_empty());
    }

    #[test]
    fn path_starts_from_given_node() {
        let start = hokkaido();
        let path = find_longest_path(PREFECTURE_DB.all_regions(), start);
        assert_eq!(path.first(), Some(&start.id));
    }

    #[test]
    fn path_has_no_duplicate_nodes() {
        let path = find_longest_path(PREFECTURE_DB.all_regions(), hokkaido());
        let mut seen = std::collections::HashSet::new();
        for id in &path {
            assert!(seen.insert(id.0), "重複ノード: {}", id.0);
        }
    }

    #[test]
    fn consecutive_nodes_are_adjacent() {
        let path = find_longest_path(PREFECTURE_DB.all_regions(), hokkaido());
        for window in path.windows(2) {
            let (a, b) = (window[0], window[1]);
            let region_a = PREFECTURE_DB
                .all_regions()
                .iter()
                .find(|r| r.id == a)
                .unwrap();
            assert!(
                region_a.neighbors.contains(&b),
                "{} → {} は隣接していない",
                a.0,
                b.0
            );
        }
    }

    #[test]
    fn returns_empty_for_unknown_start() {
        use crate::location::{LocationId, RegionKind};
        let unknown = Region {
            id: LocationId(999),
            kind: RegionKind::Prefecture,
            parent: None,
            name: "存在しない",
            kana: "",
            roman: "",
            neighbors: &[],
        };
        let path = find_longest_path(PREFECTURE_DB.all_regions(), &unknown);
        assert!(path.is_empty());
    }
}
