use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let f = |x: f64| {
        let mut res = 0.0;
        for &a in &a {
            res += x + (a as f64) - (a as f64).min(2.0 * x);
        }
        res / n as f64
    };

    let mut left = 0.0;
    let mut right = 1e9;
    for _ in 0..100 {
        // 計算の回数を固定して、確実に収束させる
        let mid_l = left + (right - left) / 3.0;
        let mid_r = right - (right - left) / 3.0;
        if f(mid_l) < f(mid_r) {
            right = mid_r;
        } else {
            left = mid_l;
        }
    }
    println!("{:.10}", f(left));
}
