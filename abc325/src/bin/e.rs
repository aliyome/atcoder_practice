use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// 状態を示す構造体
#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: i64,       // この状態に到達するためのコスト
    position: usize, // 現在の都市
    by_car: bool,    // 車で移動中かどうか
}

// State構造体をバイナリヒープで扱うための順序付け
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        c: i64,
        distances: [[i64; n]; n]
    }

    // 各都市での最短コストを保存する2次元ベクトル
    // 第二インデックスが 0 の場合は車でのコスト、1 の場合は電車でのコスト
    let mut costs = vec![vec![std::i64::MAX; 2]; n];
    let mut heap = BinaryHeap::new();

    // 都市1からのスタート
    costs[0][0] = 0;
    heap.push(State {
        cost: 0,
        position: 0,
        by_car: true,
    });

    // ダイクストラ法のメインループ
    while let Some(current_state) = heap.pop() {
        if current_state.cost > costs[current_state.position][current_state.by_car as usize] {
            continue;
        }

        for next in 0..n {
            if next == current_state.position {
                continue;
            }

            // 現在の都市から次の都市へのコストを計算
            let next_cost = if current_state.by_car {
                current_state.cost + distances[current_state.position][next] * a
            } else {
                current_state.cost + distances[current_state.position][next] * b + c
            };

            let next_state = State {
                cost: next_cost,
                position: next,
                by_car: current_state.by_car,
            };

            // 新しいコストが既知のコストよりも小さければ更新
            if next_cost < costs[next][current_state.by_car as usize] {
                costs[next][current_state.by_car as usize] = next_cost;
                heap.push(next_state);
            }

            // 車で移動中の場合、電車への乗り換えも考慮
            if current_state.by_car {
                let train_cost =
                    current_state.cost + distances[current_state.position][next] * b + c;
                if train_cost < costs[next][1] {
                    costs[next][1] = train_cost;
                    heap.push(State {
                        cost: train_cost,
                        position: next,
                        by_car: false,
                    });
                }
            }
        }
    }

    println!("{}", std::cmp::min(costs[n - 1][0], costs[n - 1][1]));
}
