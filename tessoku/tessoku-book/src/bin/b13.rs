use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      a: [usize; n]
    }

    // 累積和
    let mut sum = a.clone();
    for i in 1..n {
        sum[i] += sum[i - 1];
    }

    // しゃくとり法
    let total = |l, r| {
        if l == 0 {
            sum[r]
        } else {
            sum[r] - sum[l - 1]
        }
    };
    let mut r = vec![0; n];
    for l in 0..n {
        if l == 0 {
            r[l] = 0;
        } else {
            r[l] = r[l - 1];
        }
        while r[l] < n - 1 && total(l, r[l] + 1) <= k {
            r[l] += 1;
        }
    }

    // 出力
    let mut ans = 0;
    for l in 0..n {
        ans += r[l] - l + 1
    }

    println!("{}", ans);
}
