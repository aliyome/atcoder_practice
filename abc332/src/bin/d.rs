use proconio::input;
use std::collections::{HashMap, VecDeque};

// グリッドの状態を表す構造体
#[derive(Hash, Eq, PartialEq, Clone)]
struct Grid {
    cells: Vec<Vec<i32>>,
}

// BFSを実行して最短経路を見つける関数
fn find_min_operations(start: &Grid, end: &Grid) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((start.clone(), 0));
    visited.insert(start.clone(), true);

    while let Some((current_grid, ops)) = queue.pop_front() {
        if &current_grid == end {
            return ops;
        }

        // ここで、可能なすべての行と列の交換を実行し、キューに追加します
        for new_grid in generate_next_states(&current_grid) {
            if !visited.contains_key(&new_grid) {
                visited.insert(new_grid.clone(), true);
                queue.push_back((new_grid, ops + 1));
            }
        }
    }

    -1 // 到達不可能の場合
}

fn generate_next_states(grid: &Grid) -> Vec<Grid> {
    let mut next_states = Vec::new();
    let h = grid.cells.len();
    let w = grid.cells[0].len();

    // 行の交換
    for i in 0..h - 1 {
        let mut new_grid = grid.clone();
        new_grid.cells.swap(i, i + 1);
        next_states.push(new_grid);
    }

    // 列の交換
    for j in 0..w - 1 {
        let mut new_grid = grid.clone();
        for row in new_grid.cells.iter_mut() {
            row.swap(j, j + 1);
        }
        next_states.push(new_grid);
    }

    next_states
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
        b: [[i32; w]; h],
    }

    let start_grid = Grid { cells: a };
    let end_grid = Grid { cells: b };

    let result = find_min_operations(&start_grid, &end_grid);
    println!("{}", result);
}
