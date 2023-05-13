use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m]
    }

    // 全探索 O(M^2) TLE
    // N,M <= 10^3
    // let mut ans = 0;
    // for a in 0..m {
    //     let (la, ra) = lr[a];
    //     for b in 0..m {
    //         let (lb, rb) = lr[b];
    //         if a == b {
    //             continue;
    //         }
    //         if lb < la && rb < ra && la < rb {
    //             ans += 1;
    //         }
    //     }
    // }

    // もと配列をソートしてほんの少しだけ改善
    let mut ans = 0;
    let mut lr = lr;
    lr.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    for i in 0..m {
        let (lb, rb) = lr[i];
        let targets = lr[i..].to_vec();
        for j in 0..targets.len() {
            let (la, ra) = targets[j];
            if lb < la && rb < ra && la < rb {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
