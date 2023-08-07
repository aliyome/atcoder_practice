use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    };

    let mut min = std::usize::MAX;
    let mut ans = 0usize;
    for i in 0..n {
        min = min.min(p[i]);
        if p[i] <= min {
            ans += 1;
        }
    }

    println!("{}", ans);
}
