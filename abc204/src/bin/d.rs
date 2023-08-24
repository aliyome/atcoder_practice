use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [usize; n]
    };

    t.sort();
    t.reverse();

    // 貪欲で解ける？
    let mut dp = vec![0; 2];
    for i in 0..n {
        if dp[0] < dp[1] {
            dp[0] += t[i];
        } else {
            dp[1] += t[i];
        }
    }

    println!("{}", dp.iter().max().unwrap());
}
