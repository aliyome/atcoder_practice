use proconio::input;
use std::collections::HashSet;

// 最大公約数を求める
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    input! {
        n: i64,
        xy: [(i64, i64); n],
    }

    let mut magic = HashSet::new();
    for i in 0..n as usize {
        for j in 0..n as usize {
            if i == j {
                continue;
            }
            // 傾きを求める
            let dx = xy[j].0 - xy[i].0;
            let dy = xy[j].1 - xy[i].1;
            // 約分する
            let gcd = gcd(dx, dy).abs();
            let dx = dx / gcd;
            let dy = dy / gcd;
            magic.insert(format!("{}.{}", dx, dy));
        }
    }

    println!("{}", magic.len());
}
