use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [isize; n]
    };

    h.sort();

    let mut ans = std::isize::MAX;
    for l in 0..=n - k {
        let r = l + k - 1;
        ans = ans.min(h[r] - h[l]);
    }

    println!("{}", ans);
}
