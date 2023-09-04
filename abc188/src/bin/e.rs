use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [isize; n],
        xy: [(usize, usize); m]
    };

    // 1-indexed
    a.insert(0, 0);

    let mut to = vec![vec![]; n + 1];
    for &(x, y) in &xy {
        to[x].push(y);
    }

    let mut ans = std::isize::MIN;
    let mut dp = vec![std::isize::MAX; n + 1];
    for i in 1..=n {
        ans = ans.max(a[i] - dp[i]);
        for &j in &to[i] {
            dp[j] = dp[j].min(dp[i]).min(a[i]);
        }
    }

    println!("{}", ans);
}
