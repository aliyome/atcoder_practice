use proconio::input;

// TODO: 解答ACなので、解説を読んで理解する

fn main() {
    input! {
      n: usize,
    }

    let mut power10 = vec![1; 18];
    for i in 1..16 {
        power10[i] = power10[i - 1] * 10;
    }

    let mut r = vec![vec![0; 10]; 18];
    for i in 0..=14 {
        let digit = (n / power10[i]) % 10;
        for j in 0..10 {
            if j < digit {
                r[i][j] = (n / power10[i + 1]) * power10[i] + power10[i];
            } else if (j == digit) {
                r[i][j] = (n / power10[i + 1]) * power10[i] + (n % power10[i]) + 1;
            } else {
                r[i][j] = (n / power10[i + 1]) * power10[i];
            }
        }
    }

    let mut ans = 0;
    for i in 0..=15 {
        for j in 0..10 {
            ans += 1 * j * r[i][j];
        }
    }

    println!("{}", ans);
}
