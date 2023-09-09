use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut divisors = vec![];

    // Nの1から9までの約数を見つける
    for i in 1..=9 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    let mut answer = String::new();

    // 各iに対してsiを計算する
    for i in 0..=n {
        let mut min_divisor = 10; // 10は使われない値。siが最小の約数で上書きされる。

        for &d in &divisors {
            if i % (n / d) == 0 {
                min_divisor = min_divisor.min(d);
            }
        }

        // 約数が見つかったかどうかを確認する
        if min_divisor == 10 {
            answer.push('-');
        } else {
            answer.push_str(&min_divisor.to_string());
        }
    }

    println!("{}", answer);
}
