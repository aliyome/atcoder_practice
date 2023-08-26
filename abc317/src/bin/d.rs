use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(usize, usize, usize); n],
    }

    let mut total = 0;
    for &(_, _, z) in &xyz {
        total += z;
    }

    // 必要数
    let mut dp = vec![std::usize::MAX; total + 1];
    dp[0] = 0;
    for &(x, y, z) in &xyz {
        // 移動人数
        let mov = (y as isize - x as isize + 1) / 2;
        for i in (0..=total).rev() {
            if dp[i] == std::usize::MAX {
                continue;
            }
            if i + z > total {
                continue;
            }
            if mov < 0 {
                dp[i + z] = dp[i + z].min(dp[i]);
            } else {
                dp[i + z] = dp[i + z].min(dp[i] + mov as usize);
            }
        }
    }

    println!("{}", dp[(total + 1) / 2..].iter().min().unwrap());
}
