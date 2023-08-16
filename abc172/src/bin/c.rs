use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let mut acc_a = vec![0; n + 1];
    let mut acc_b = vec![0; m + 1];
    for i in 0..n {
        acc_a[i + 1] = acc_a[i] + a[i];
    }
    for i in 0..m {
        acc_b[i + 1] = acc_b[i] + b[i];
    }

    let mut ans = 0;
    for i in 0..=n {
        if acc_a[i] > k {
            break;
        }

        let j = acc_b.upper_bound(&(k - acc_a[i])) - 1;
        ans = ans.max(i + j);
    }

    println!("{}", ans);
}
