use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        m: usize,
        xy: [(usize, usize); m]
    }

    // 禁忌の組み合わせ
    let mut ng = vec![vec![false; n]; n];
    for (x, y) in xy {
        ng[x - 1][y - 1] = true;
        ng[y - 1][x - 1] = true;
    }

    // 全探索
    let mut ans = std::i64::MAX;
    for list in (0..n).permutations(n) {
        let mut time = 0i64;
        let mut no_contest = false;
        for i in 0..n {
            // 禁忌チェック
            if i < n - 1 && ng[list[i]][list[i + 1]] {
                no_contest = true;
                break;
            }
            time += a[list[i]][i];
        }
        if !no_contest {
            ans = ans.min(time);
        }
    }

    if ans == std::i64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
