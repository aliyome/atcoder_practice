use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = 0;
    if n > 0 {
        ans += n * (n - 1) / 2;
    }
    if m > 0 {
        ans += m * (m - 1) / 2;
    }

    println!("{}", ans);
}
