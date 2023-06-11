use proconio::input;

fn main() {
    input! {
      n: usize,
      hh: [isize; n]
    }

    // 1-indexed
    let mut h = vec![0];
    h.extend(&hh);

    let mut dp = vec![100000000; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = (h[2] - h[1]).abs();
    for i in 3..=n {
        dp[i] = (dp[i - 1] + (h[i] - h[i - 1]).abs()).min(dp[i - 2] + (h[i] - h[i - 2]).abs());
    }

    let mut ans = vec![];
    ans.push(n);
    while let Some(&i) = ans.last() {
        if i <= 1 {
            break;
        }
        if dp[i] == dp[i - 1] + (h[i] - h[i - 1]).abs() {
            ans.push(i - 1);
        } else {
            ans.push(i - 2);
        }
    }
    ans.reverse();

    println!("{}", ans.len());
    for a in ans {
        print!("{} ", a);
    }
}
