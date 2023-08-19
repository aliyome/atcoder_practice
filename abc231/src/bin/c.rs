use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        x: [usize; q]
    };

    let mut h = a.clone();
    h.sort();

    for &x in &x {
        let i = h.lower_bound(&x);
        println!("{}", n - i);
    }
}
