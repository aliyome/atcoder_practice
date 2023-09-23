use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: isize,
        a: [isize; n],
        b: [isize; m],
    }

    let mut a = a;
    a.sort();

    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + a[i];
    }

    let mut ans = 0;
    for &b in &b {
        let clipped = a.lower_bound(&(p - b)) as usize;
        let x = (n as isize - clipped as isize) * p;
        let y = if clipped > 0 {
            prefix_sum[clipped] + clipped as isize * b
        } else {
            0
        };
        ans += x + y;
    }

    println!("{}", ans);
}
