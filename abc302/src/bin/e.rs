use std::collections::HashSet;

use proconio::input;

// O(QN)
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
            let to_remove: Vec<_> = connected[u].iter().copied().collect();
            for &v in &to_remove {
                connected[v].remove(&u);
                if connected[v].len() == 0 {
                    ans += 1;
                }
            }

            if connected[u].len() != 0 {
                ans += 1;
            }
            connected[u] = HashSet::new();
        }
        println!("{}", ans);
    }
}
