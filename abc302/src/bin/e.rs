use std::collections::HashSet;

use proconio::input;

// O(QN) TLE
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut connected = vec![HashSet::new(); n + 1];
    let mut ans = n;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: usize,
                v: usize,
            }
            // 追加のときは孤立点の数は単調減少 O(1)
            if connected[u].len() == 0 {
                ans -= 1;
            }
            if connected[v].len() == 0 {
                ans -= 1;
            }
            connected[u].insert(v);
            connected[v].insert(u);
        } else {
            input! {
                u: usize,
            }
            // 削除のときは孤立点の数は単調増加 O(N)
            ans = 0;
            connected[u] = HashSet::new();
            for i in 1..=n {
                connected[i].remove(&u);
                if connected[i].len() == 0 {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
