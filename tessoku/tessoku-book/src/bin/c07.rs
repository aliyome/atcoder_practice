use proconio::input;
use superslice::Ext;

fn main() {
    input! {
      n: usize,
      mut c: [usize; n],
      q: usize,
      x: [usize; q]
    }

    c.sort();
    c.insert(0, 0);

    let mut acc = vec![0; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + c[i];
    }

    // O(10^5)
    for &x in &x {
        let count = acc.upper_bound(&x);
        println!("{}", count - 1);
    }
}
