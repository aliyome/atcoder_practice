use proconio::input;

fn main() {
    input! {
      n: usize,
      aa: [usize; n - 1],
      bb: [usize; n - 2]
    }

    // 0-indexed にするためにダミーの要素を追加
    let mut a = vec![0, 0];
    let mut b = vec![0, 0, 0];
    a.extend(&aa);
    b.extend(&bb);

    // dp
    let mut dp = vec![0; n + 1];
    dp[2] = a[2];
    for i in 3..=n {
        let a = dp[i - 1] + a[i];
        let b = dp[i - 2] + b[i];
        if a < b {
            dp[i] = a;
        } else {
            dp[i] = b;
        }
    }

    // ゴールから逆順に辿って復元
    let mut ans: Vec<usize> = vec![];
    ans.push(n);
    while let Some(&i) = ans.last() {
        if i <= 1 {
            break;
        }
        if dp[i] == dp[i - 1] + a[i] {
            ans.push(i - 1);
        } else {
            ans.push(i - 2);
        }
    }
    ans.reverse();

    println!("{}", ans.len());
    for i in ans {
        print!("{} ", i);
    }
    println!("");
}
