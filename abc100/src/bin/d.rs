use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, // 10^4
        m: usize, // 10^4
        xyz: [(isize, isize, isize); n]
    };

    // 8通りの符号の組み合わせを全探索
    let signs = [
        (1, 1, 1),
        (1, 1, -1),
        (1, -1, 1),
        (1, -1, -1),
        (-1, 1, 1),
        (-1, 1, -1),
        (-1, -1, 1),
        (-1, -1, -1),
    ];
    let mut scores = vec![vec![0; n]; 8];
    for (i, &(ix, iy, iz)) in signs.iter().enumerate().collect_vec() {
        for j in 0..n {
            let (x, y, z) = xyz[j];
            scores[i][j] = ix * x + iy * y + iz * z;
        }
        scores[i].sort();
        scores[i].reverse();
    }

    let mut max = 0;
    for i in 0..8 {
        let mut sum = 0;
        for j in 0..m {
            sum += scores[i][j];
        }
        max = max.max(sum);
    }

    println!("{}", max);
}
