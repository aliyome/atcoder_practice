use proconio::input;

fn main() {
    input! {
      n: usize,
      s: usize,
      aa: [usize; n]
    }

    // 1-indexed
    let mut a = vec![0];
    a.extend(&aa);

    // dp[i][j] := i番目までのカードを使って総和jを作れるか
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        dp[i] = dp[i - 1].clone();
        let a = a[i];
        for j in 0..=s {
            if (j as isize) - (a as isize) < 0 {
                continue;
            }
            dp[i][j] = dp[i][j] || dp[i - 1][j - a];
        }
    }
    if !dp[n][s] {
        println!("-1");
        return;
    }

    // 逆順にたどる
    let mut ans = vec![];
    let mut queue = vec![];
    queue.push((n, s));
    while let Some((n, s)) = queue.pop() {
        if n < 1 {
            break;
        }
        if dp[n][s] && dp[n - 1][s] {
            queue.push((n - 1, s));
            continue;
        }
        if dp[n][s] && dp[n - 1][s - a[n]] {
            ans.push(n);
            queue.push((n - 1, s - a[n]));
        }
    }

    ans.reverse();
    println!("{}", ans.len());
    for a in ans {
        print!("{} ", a);
    }
}
