use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [isize; n],
        xy: [(usize, usize); m]
    };

    // 1-indexed
    a.insert(0, 0);

    // 子から親に辿れるようにする
    let mut parent = vec![0; n + 1];
    let mut leaf = HashSet::new();
    for i in 1..=n {
        leaf.insert(i);
    }
    for &(x, y) in &xy {
        parent[y] = x;
        leaf.remove(&x);
    }

    let mut ans = std::isize::MIN;

    // 葉から順に親を辿っていく
    for i in leaf {
        let mut curr = i;
        while parent[curr] != 0 {
            // 親との差分を計算する
            let diff = a[curr] - a[parent[curr]];
            ans = ans.max(diff);
            // 親に伝播する
            a[parent[curr]] = a[parent[curr]].max(a[curr]);
            curr = parent[curr];
        }
    }

    println!("{}", ans);
}
