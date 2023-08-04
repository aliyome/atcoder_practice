use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r]
    };

    let mut bomb = vec![];
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '#' || b[i][j] == '.' {
                continue;
            }
            let k = b[i][j].to_digit(10).unwrap() as isize;
            bomb.push((i, j, k));
        }
    }
    let mut ans = b.clone();
    for i in 0..r {
        for j in 0..c {
            for &b in &bomb {
                if (i as isize - b.0 as isize).abs() + (j as isize - b.1 as isize).abs() <= b.2 {
                    ans[i][j] = '.';
                }
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
