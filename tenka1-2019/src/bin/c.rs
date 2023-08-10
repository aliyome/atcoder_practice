use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars
    };

    // 1-indexed
    s.insert(0, '-');

    // 累積和
    let mut white = vec![0usize; n + 1];
    let mut black = vec![0usize; n + 1];
    for i in 1..=n {
        white[i] = white[i - 1] + if s[i] == '.' { 1 } else { 0 };
        black[i] = black[i - 1] + if s[i] == '#' { 1 } else { 0 };
    }

    // 黒の始まりの位置
    let mut lefts = vec![];
    for i in 1..n {
        if s[i] == '#' && s[i - 1] != '#' {
            lefts.push(i);
        }
    }

    if lefts.len() == 0 {
        println!("0");
        return;
    }

    let mut ans = std::usize::MAX;
    for black_start in lefts {
        // black_start より左側の黒は白に塗り替える必要がある
        let black_num = if black_start == 0 {
            0
        } else {
            black[black_start - 1]
        };

        // black_start より右側の白は黒に塗り替える必要がある
        let white_num = white[n] - white[black_start];

        ans = ans.min(black_num + white_num);
    }

    println!("{}", ans);
}
