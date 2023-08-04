use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    };

    // 累積和
    let mut sum = vec![0; n];
    for i in 0..n - 1 {
        sum[i + 1] = sum[i];
        if s[i] == 'A' && s[i + 1] == 'C' {
            sum[i + 1] += 1;
        }
    }
    // println!("{:?}", sum);

    for &(l, r) in &lr {
        let l = l - 1;
        let r = r - 1;
        println!("{}", sum[r] - sum[l]);
    }
}
