use proconio::input;
use std::collections::HashMap;

fn dfs(
    current: usize,
    visited: &mut Vec<bool>,
    roads: &HashMap<usize, Vec<(usize, usize)>>,
    current_distance: usize,
) -> usize {
    visited[current] = true;

    let mut max_distance = current_distance;

    if let Some(neighbors) = roads.get(&current) {
        for &(next, distance) in neighbors {
            if !visited[next] {
                max_distance = std::cmp::max(
                    max_distance,
                    dfs(next, visited, roads, current_distance + distance),
                );
            }
        }
    }

    visited[current] = false;

    max_distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(usize, usize, usize); m]
    }

    let mut road_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for (a, b, c) in roads {
        road_map.entry(a).or_insert(vec![]).push((b, c));
        road_map.entry(b).or_insert(vec![]).push((a, c));
    }

    let mut visited = vec![false; n + 1];
    let mut max_distance = 0;

    for i in 1..=n {
        max_distance = std::cmp::max(max_distance, dfs(i, &mut visited, &road_map, 0));
    }

    println!("{}", max_distance);
}
