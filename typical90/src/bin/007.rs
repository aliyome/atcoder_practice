use superslice::Ext;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q],
    }

    a.sort();
    for &b in &b {
        let i = a.lower_bound(&b);
        let mut r = 10_000_000_000isize;
        let mut l = 10_000_000_000isize;
        if 0 < i {
            l = (b - a[i - 1]).abs();
        }
        if i < n {
            r = (b - a[i]).abs();
        }
        println!("{}", l.min(r));
    }
}
