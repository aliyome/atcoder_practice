use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      aa: [usize; n]
    }
    // 1-index
    let mut a = vec![0usize; n + 1];
    for i in 1..=n {
        a[i] = aa[i - 1];
    }
    // println!("{:?}", a);

    // 累積和
    let mut sum = vec![0usize; n + 1];
    for i in 1..=n {
        sum[i] += sum[i - 1] + a[i];
    }
    // println!("{:?}", sum);

    // しゃくとり法
    let total = |l, r| sum[r] - sum[l - 1];
    let mut r = vec![0; n + 1];
    for l in 1..=n {
        r[l] = r[l - 1];
        while r[l] < n && total(l, r[l] + 1) <= k {
            r[l] += 1;
        }
    }
    // println!("{:?}", r);

    // 出力
    let mut ans = 0;
    for l in 1..=n {
        ans += r[l] - l + 1
    }

    println!("{}", ans);
}
