use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, usize); q],
    };

    let mut adds = vec![0; n + 1];
    for &(p, x) in &px {
        adds[p] += x;
    }

    let mut parent = vec![0; n + 1];
    for &(a, b) in &ab {
        parent[b] = a;
    }

    let mut ancestors = vec![vec![]; n + 1];
    for i in 1..=n {
        let mut p = i;
        while p != 0 {
            ancestors[i].push(p);
            p = parent[p];
        }
    }

    for i in 1..=n {
        let mut ans = 0;
        for &a in &ancestors[i] {
            ans += adds[a];
        }
        print!("{} ", ans);
    }
}
