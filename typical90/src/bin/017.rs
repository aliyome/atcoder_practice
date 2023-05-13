use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m]
    }

    // 全探索 O(M^2) TLE
    // N,M <= 10^3
    let mut ans = 0;
    for a in 0..m {
        let (la, ra) = lr[a];
        for b in 0..m {
            let (lb, rb) = lr[b];
            if a == b {
                continue;
            }
            if lb < la && rb < ra && la < rb {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
