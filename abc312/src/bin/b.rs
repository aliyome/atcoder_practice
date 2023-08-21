use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    };
    let tak = [
        ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
        ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
        ['#', '#', '#', '.', '?', '?', '?', '?', '?'],
        ['.', '.', '.', '.', '?', '?', '?', '?', '?'],
        ['?', '?', '?', '?', '?', '?', '?', '?', '?'],
        ['?', '?', '?', '?', '?', '.', '.', '.', '.'],
        ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
        ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
        ['?', '?', '?', '?', '?', '.', '#', '#', '#'],
    ];

    for i in 0..n {
        for j in 0..m {
            let mut ok = true;
            for ii in 0..9 {
                for jj in 0..9 {
                    if i + ii >= n || j + jj >= m {
                        ok = false;
                        continue;
                    }
                    if s[i + ii][j + jj] != tak[ii][jj] && tak[ii][jj] != '?' {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
