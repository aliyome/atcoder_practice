use proconio::input;

fn main() {
    input! {
        n: f64,
        k: f64,
    };

    let ans = ((k - 1f64) * (n - k) * 6f64 + ((k - 1f64) + (n - k)) * 3f64 + 1f64) / (n * n * n);
    println!("{}", ans);
}
