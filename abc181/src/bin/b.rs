use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    let mut ans = 0;
    for &(a, b) in &ab {
        // 1 から N までの総和は (N / 2) x (N + 1) で求められる
        let bb = b * (b + 1) / 2;
        let aa = (a - 1) * ((a - 1) + 1) / 2;
        ans += bb - aa;
    }

    println!("{}", ans);
}
