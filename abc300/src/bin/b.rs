use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h]
    };

    let tate_shift = |a: Vec<Vec<char>>, n: usize| {
        let mut a = a.clone();
        for _ in 0..n {
            let first_row = a[0].clone();
            for i in 0..h - 1 {
                a[i] = a[i + 1].clone();
            }
            a[h - 1] = first_row;
        }
        a
    };

    let yoko_shift = |a: Vec<Vec<char>>, n: usize| {
        let mut a = a.clone();
        for _ in 0..n {
            for i in 0..h {
                let first = a[i][0].clone();
                for j in 0..w - 1 {
                    a[i][j] = a[i][j + 1];
                }
                a[i][w - 1] = first;
            }
        }
        a
    };

    // 未操作
    if a == b {
        println!("Yes");
        return;
    }

    // 縦方向にs回シフト
    let mut a = a.clone();
    for _s in 0..=h {
        a = tate_shift(a, 1);
        if a == b {
            println!("Yes");
            return;
        }
        // 横方向にt回シフト
        for _t in 0..=w {
            a = yoko_shift(a, 1);
            if a == b {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
