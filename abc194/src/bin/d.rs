use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0f64;
    for i in 1..n {
        let n = n as f64;
        ans += n / (n - i as f64);
    }

    println!("{}", ans);
}
