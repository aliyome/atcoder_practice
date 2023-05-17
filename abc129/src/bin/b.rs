use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n]
    };

    // 累積和
    let mut s = vec![0i64; n + 1];
    let mut r = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + w[i];
        r[i + 1] = r[i] + w[n - i - 1];
    }

    let mut ans = 1000;
    for i in 1..=n {
        ans = ans.min((s[i] - r[n - i]).abs());
    }

    println!("{}", ans);
}
