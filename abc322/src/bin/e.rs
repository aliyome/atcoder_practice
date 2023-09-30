use proconio::input;
use std::{cmp::min, collections::HashMap};

fn main() {
    input! {
        n: usize, // 開発計画の数
        k: usize, // パラメータの数
        p: usize, // 目標値
        plans: [(usize, [usize; k]); n], // 各開発計画のコストとパラメータの増加量
    }

    // DPテーブルの初期化
    let mut dp = HashMap::new();
    dp.insert(vec![0; k], 0);

    // 各開発計画に対して
    for (cost, params) in plans {
        let mut next_dp = dp.clone();
        for (kvs, &current_cost) in dp.iter() {
            let mut new_kvs = kvs.clone();
            for (i, &param) in params.iter().enumerate() {
                new_kvs[i] = std::cmp::min(new_kvs[i] + param, p);
            }
            let new_cost = current_cost + cost;
            let entry = next_dp.entry(new_kvs).or_insert(usize::MAX);
            *entry = std::cmp::min(*entry, new_cost);
        }
        dp = next_dp;
    }

    // 目標を達成できるかどうかを判定し、可能であれば最小コストを出力
    let target = vec![p; k];
    if let Some(&min_cost) = dp.get(&target) {
        println!("{}", min_cost);
    } else {
        println!("-1");
    }
}
