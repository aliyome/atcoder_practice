use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        mut long: usize,
    };

    let mut short = 0;
    let mut ans = 0;
    ans += long;

    while (long + short) >= m {
        let remake = (long + short) / m * n;
        short = (long + short) % m;
        long = remake;
        ans += long;
    }

    println!("{}", ans);
}
