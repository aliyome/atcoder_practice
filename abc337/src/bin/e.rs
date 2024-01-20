use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    };

    // M = log2(N) を切り上げで計算する。これが必要な友達の最小数。
    let m = (n as f64).log2().ceil() as usize;

    println!("{}", m);

    // 各友達に与えるジュースの組み合わせをビットで表現して出力する。
    for i in 0..m {
        let mut bottles = Vec::new();
        for j in 1..=n {
            if (j >> i) & 1 == 1 {
                bottles.push(j);
            }
        }
        println!(
            "{} {}",
            bottles.len(),
            bottles
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    // 標準入力から友達の反応を受け取る。
    input! {
        from &mut source,
        response: String,
    }

    // 悪いボトルの番号を特定する。
    let mut bad_bottle = 0;
    for (i, c) in response.chars().enumerate() {
        if c == '1' {
            bad_bottle |= 1 << i;
        }
    }

    // 悪いボトルの番号を出力する。
    println!("{}", bad_bottle);
}
