use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        xy: [(f64, f64); n]
    };

    let mut ans = 0;
    for (x, y) in xy {
        if (x * x + y * y).sqrt() <= d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
