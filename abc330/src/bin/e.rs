use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize, usize); q],
    }

    let mut counts = vec![0; n + 2];
    let mut set: BTreeSet<usize> = (0..=n + 1).collect();

    for &ai in &a {
        if counts[ai] == 0 {
            set.remove(&ai);
        }
        counts[ai] += 1;
    }

    for (i, x) in queries {
        let index = i - 1; // 0-indexed

        // 元の値を減らす
        counts[a[index]] -= 1;
        if counts[a[index]] == 0 {
            set.insert(a[index]);
        }

        // 新しい値を増やす
        if counts[x] == 0 {
            set.remove(&x);
        }
        counts[x] += 1;

        // mexを見つける
        let mex = set.range(..).next().unwrap();
        println!("{}", mex);

        // 配列を更新
        a[index] = x;
    }
}
