use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    // // 愚直 overflow
    // for i in 0.. {
    //     if format!("{}", k * i).chars().all(|c| c == '7') {
    //         println!("{}", format!("{}", k * i).chars().count());
    //         return;
    //     }
    // }

    let mut visited = HashSet::new();
    let mut is_loop = |x| {
        if visited.contains(&x) {
            true
        } else {
            visited.insert(x);
            false
        }
    };

    // k 以上の 777... を探す
    let mut x = 0;
    let mut count = 0;
    while x < k {
        x = x * 10 + 7;
        count += 1;
    }

    // k 回以内に見つかるか
    for i in 0..k {
        let m = x % k;
        if m == 0 {
            println!("{}", i + count);
            return;
        }
        if is_loop(m) {
            println!("-1");
            return;
        }
        x = m * 10 + 7;
    }
}
