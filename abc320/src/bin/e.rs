use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    }

    // 最初は全員列に並んでいる
    let mut waiting = BinaryHeap::new();
    for i in 1..=n {
        waiting.push(Reverse(i));
    }

    // 最初は誰も食事中じゃない
    let mut eating = BinaryHeap::new();

    let mut ans = vec![0; n + 1];

    for &(t, w, s) in &tws {
        // 食事中の人が t 秒時点で列に並んでいるかどうかを判定
        while let Some((Reverse(next_t), i)) = eating.pop() {
            if next_t <= t {
                // 列に並べる
                waiting.push(Reverse(i));
            } else {
                // 戻す
                eating.push((Reverse(next_t as usize), i));
                break;
            }
        }

        // eprintln!("t:{} waiting:{:?} eating:{:?}", t, waiting, eating);

        // t 秒時点で列に並んでる人の先頭
        if let Some(Reverse(top)) = waiting.pop() {
            ans[top] += w;
            // 食事中にする
            eating.push((Reverse(t + s), top));
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
