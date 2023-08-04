use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    };

    // 体積
    let s = x / a;

    // パターン分け
    if s >= a * b / 2.0 {
        let h = (2.0 * (a * b - s)) / a;
        let ans = h.atan2(a) * 180.0 / std::f64::consts::PI;
        println!("{}", ans);
    } else {
        let w = 2.0 * s / b;
        let ans = b.atan2(w) * 180.0 / std::f64::consts::PI;
        println!("{}", ans);
    }
}
