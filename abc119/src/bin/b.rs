use proconio::input;

fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n]
    };

    let mut ans = 0.0f64;
    for (x, u) in &xu {
        if u == "JPY" {
            ans += x;
        } else {
            ans += x * 380000.0;
        }
    }
    println!("{}", ans);
}
