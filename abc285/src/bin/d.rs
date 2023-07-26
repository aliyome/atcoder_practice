use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut map = HashMap::new();
    for (s, t) in st {
        map.insert(s, t);
    }

    let mut visited = HashSet::new();
    for (start, _) in map.iter() {
        if visited.contains(start) {
            continue;
        }
        if !dfs(&map, start, &mut visited, &mut HashSet::new()) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn dfs(
    map: &HashMap<String, String>,
    start: &str,
    visited: &mut HashSet<String>,
    visited_in_this_loop: &mut HashSet<String>,
) -> bool {
    if visited_in_this_loop.contains(start) {
        return false;
    }

    visited.insert(start.to_string());
    visited_in_this_loop.insert(start.to_string());

    if let Some(next) = map.get(start) {
        dfs(map, next, visited, visited_in_this_loop)
    } else {
        true
    }
}
