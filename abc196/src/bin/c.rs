use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0usize;
    for i in 1..=6 {
        for j in 10usize.pow(i - 1)..10usize.pow(i) {
            if j * 10usize.pow(i) + j <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
