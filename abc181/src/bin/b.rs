use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    let mut acc = vec![0; 1000001];
    for i in 1..acc.len() {
        acc[i] = acc[i - 1] + i;
    }

    let mut ans = 0;
    for &(a, b) in &ab {
        ans += acc[b] - acc[a - 1];
    }

    println!("{}", ans);
}
