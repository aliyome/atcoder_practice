use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = vec![vec![0; n + 1]; n + 1];
    ans[0][0] = 1;
    for i in 1..n {
        ans[i][0] = 1;
        for j in 1..=i {
            ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j];
        }
    }

    for i in 0..n {
        for j in 0..=i {
            print!("{} ", ans[i][j]);
        }
        println!();
    }
}
