use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        lrv: [(usize, usize, isize); q],
    };

    // 1-indexed
    a.insert(0, 0);

    let mut b = vec![0; n + 1];
    let mut ans = 0;
    for i in 1..n {
        b[i] = a[i + 1] - a[i];
        ans += b[i].abs();
    }

    for &(l, r, v) in &lrv {
        let t = b[l - 1].abs() + b[r].abs();
        if l >= 2 {
            b[l - 1] += v;
        }
        if r <= n - 1 {
            b[r] -= v;
        }
        let t2 = b[l - 1].abs() + b[r].abs();

        ans += t2 - t;

        println!("{}", ans);
    }
}
