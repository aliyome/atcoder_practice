use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    const M: usize = 200001;
    let mut d = vec![0; M + 1];
    for i in 0..n {
        d[a[i]] += 1;
    }

    let mut ans = 0usize;
    for a in 1..=M {
        for b in 1..=M / a {
            let c = a * b;
            ans += d[a] * d[b] * d[c];
        }
    }

    println!("{}", ans);
}
