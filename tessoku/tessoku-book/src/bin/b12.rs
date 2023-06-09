use proconio::input;

fn main() {
    input! {
      n: f64,
    }

    let f = |x: f64| x * x * x + x;
    let mut ng = 0.0;
    let mut ok = 100_000.0;
    while ok - ng > 0.001 {
        let mid = (ok + ng) / 2.0;
        if n <= f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
