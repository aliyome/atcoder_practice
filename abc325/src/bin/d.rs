use proconio::input;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }

    // (入る時間,出る時間)のペアを作成し、入る時間でソート
    let mut events = BinaryHeap::new();
    for &(t, d) in &td {
        events.push((Reverse(t), t + d));
    }

    // 印刷待ちを入れるキュー
    let mut print_queue = BinaryHeap::new();
    let mut current_time = 1;
    let mut ans = 0;

    loop {
        // キューが空なら、次に印刷可能になる時間まで時間を進める
        if print_queue.is_empty() {
            if let Some((Reverse(t), _)) = events.peek() {
                current_time = *t;
            } else {
                break;
            }
        }

        // 現在時刻までに印刷可能になった製品をキューに入れる
        while let Some((Reverse(enter_time), exit_time)) = events.pop() {
            if current_time < enter_time {
                events.push((Reverse(enter_time), exit_time));
                break;
            }
            // 出る時間順にキューに入れる
            print_queue.push(Reverse(exit_time));
        }

        // キューから印刷可能な製品を取り出す
        if let Some(Reverse(exit_time)) = print_queue.pop() {
            if current_time <= exit_time {
                ans += 1;
                current_time += 1;
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
