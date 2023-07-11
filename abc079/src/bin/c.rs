use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        abcd: Chars,
    };

    let abcd = abcd
        .iter()
        .map(|&x| x.to_digit(10).unwrap() as isize)
        .collect_vec();
    let calc = |x, y, op| {
        if op == '+' {
            x + y
        } else {
            x - y
        }
    };
    for op1 in ['+', '-'] {
        for op2 in ['+', '-'] {
            for op3 in ['+', '-'] {
                if calc(
                    calc(calc(abcd[0], abcd[1], op1), abcd[2], op2),
                    abcd[3],
                    op3,
                ) == 7
                {
                    println!(
                        "{}{}{}{}{}{}{}=7",
                        abcd[0], op1, abcd[1], op2, abcd[2], op3, abcd[3]
                    );
                    return;
                }
            }
        }
    }
}
