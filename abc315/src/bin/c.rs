use std::collections::{BinaryHeap, HashMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n]
    };

    // 各種類毎に高い順に格納する
    let mut map = HashMap::new();
    for (a, b) in fs {
        map.entry(a).or_insert(BinaryHeap::new()).push(b);
    }

    // 1種類だけならそのまま出力
    if map.keys().len() == 1 {
        for (_, v) in map.iter_mut() {
            let mut sum = 0;
            sum += v.pop().unwrap();
            sum += v.pop().unwrap() / 2;
            println!("{}", sum);
            return;
        }
    }

    // 各種類ごとの最大値を降順に並び替える
    let mut tops = BinaryHeap::new();
    for (&k, v) in map.iter() {
        tops.push((*v.peek().unwrap(), k));
    }

    // 味の違うトップ2を取得
    let (ts, tf) = tops.pop().unwrap();
    let (ts2, tf2) = tops.pop().unwrap();

    let mut ans = ts + ts2;
    for (k, v) in map.iter_mut() {
        // 味が同じのトップ2
        if v.len() >= 2 {
            let mut sum = 0;
            sum += v.pop().unwrap();
            sum += v.pop().unwrap() / 2;
            ans = ans.max(sum);
        }
    }

    println!("{}", ans);

    // println!("{:?}", map);
}
