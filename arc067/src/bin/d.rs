use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: [usize; n]
    };

    let mut dists = vec![0; n - 1];
    for i in 0..n - 1 {
        dists[i] = x[i + 1] - x[i];
    }

    let mut ans = 0;
    for d in dists {
        if d * a < b {
            ans += d * a;
        } else {
            ans += b;
        }
    }

    println!("{}", ans);
}
