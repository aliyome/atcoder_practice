use proconio::input;

// 良い整数を生成する関数
fn generate_good_numbers(n: u64) -> u64 {
    let digits = [0, 2, 4, 6, 8];
    let mut result = 0;
    let mut base = 1;

    // `n`を5進数として扱い、各桁を偶数に変換する
    let mut n = n;
    while n > 0 {
        let digit = digits[(n % 5) as usize];
        result += digit * base;
        n /= 5;
        base *= 10;
    }
    result
}

fn main() {
    input! {
        n: u64,
    }

    // `n-1`を使って、0を含むためのオフセットを適用する
    let answer = generate_good_numbers(n - 1);
    println!("{}", answer);
}
