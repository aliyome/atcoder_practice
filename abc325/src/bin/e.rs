use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: usize,
    by_car: bool,
}

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

    let mut costs = vec![vec![std::i64::MAX; 2]; n];
    let mut heap = BinaryHeap::new();

    costs[0][0] = 0; // 0: by car, 1: by train
    heap.push(State {
        cost: 0,
        position: 0,
        by_car: true,
    });

    while let Some(current_state) = heap.pop() {
        if current_state.cost > costs[current_state.position][current_state.by_car as usize] {
            continue;
        }

        for next in 0..n {
            if next == current_state.position {
                continue;
            }
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

            if next_cost < costs[next][current_state.by_car as usize] {
                costs[next][current_state.by_car as usize] = next_cost;
                heap.push(next_state);
            }

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
