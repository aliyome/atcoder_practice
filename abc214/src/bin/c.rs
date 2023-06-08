use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n]
    };

    let mut ans = vec![10usize.pow(9); n + 1];
    for i in 0..n * 2 {
        ans[(i + 1) % n] = ans[(i + 1) % n].min(t[(i + 1) % n]);
        ans[(i + 1) % n] = ans[(i + 1) % n].min(ans[i % n] + s[i % n]);
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
