use proconio::input;

// n の約数の個数は、n を素因数分解した時の指数の値に 1 を足したもの同士を掛け合わせたものになる
fn main() {
    input! {
        n: usize,
    };

    // エラトステネスの篩を利用した約数の個数を計算
    let divisors = divisor_counts(n);
    let mut ans = 0;
    for k in 1..=n {
        ans += k * divisors[k];
    }

    println!("{}", ans);
}

// n までの各整数の約数の個数を計算
fn divisor_counts(n: usize) -> Vec<usize> {
    let mut count = vec![0; n + 1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            count[j] += 1;
            j += i;
        }
    }
    count
}
