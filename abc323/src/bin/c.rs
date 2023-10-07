use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; m],
        s: [String; n],
    }

    // 各プレイヤーの現在のスコアを計算
    let mut scores = Vec::with_capacity(n);
    for i in 0..n {
        let mut score = i as u32 + 1;
        for j in 0..m {
            if s[i].chars().nth(j).unwrap() == 'o' {
                score += a[j];
            }
        }
        scores.push(score);
    }

    // 最大スコアを計算
    let max_score = *scores.iter().max().unwrap();

    // 最大スコアを取ったプレイヤーが複数いるかどうかを判定
    let mut max_score_count = 0;
    for i in 0..n {
        if scores[i] == max_score {
            max_score_count += 1;
        }
    }
    let is_multiple_max_score = max_score_count > 1;

    // 各プレイヤーについて、必要な問題数を計算
    for i in 0..n {
        // すでに最大スコアの場合は0を出力
        if scores[i] == max_score && !is_multiple_max_score {
            println!("0");
            continue;
        }

        // 残りの問題の配点を降順にソート
        let mut heap = BinaryHeap::new();
        for j in 0..m {
            if s[i].chars().nth(j).unwrap() == 'x' {
                heap.push(a[j]);
            }
        }

        // 一つずつ問題を解いていき、他のプレイヤーの最大スコアを超えるまで解き続ける
        let mut count = 0;
        let mut score = scores[i];
        while score <= max_score {
            if let Some(a) = heap.pop() {
                score += a;
                count += 1;
            } else {
                break;
            }
        }
        println!("{}", count);
    }
}
