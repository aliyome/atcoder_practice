use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };

    let patterns = vec![
        [['#', '.'], ['.', '.']],
        [['.', '#'], ['.', '.']],
        [['.', '.'], ['.', '#']],
        [['.', '.'], ['#', '.']],
        [['.', '#'], ['#', '#']],
        [['#', '.'], ['#', '#']],
        [['#', '#'], ['#', '.']],
        [['#', '#'], ['.', '#']],
    ];

    let mut ans = 0;
    for i in 1..h {
        for j in 1..w {
            for &p in &patterns {
                if p[0][0] == s[i - 1][j - 1]
                    && p[0][1] == s[i - 1][j]
                    && p[1][0] == s[i][j - 1]
                    && p[1][1] == s[i][j]
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
