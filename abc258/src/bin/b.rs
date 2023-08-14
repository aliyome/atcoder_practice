use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    };

    // 方向
    let direction = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut ans = String::new();

    // i,j はスタート地点
    for i in 0..n {
        for j in 0..n {
            // 方向
            for &(vi, vj) in &direction {
                let mut sum = String::new();
                let mut ni = i as i32;
                let mut nj = j as i32;
                // 1マスずつNマス進む
                for _ in 0..n {
                    ni += vi;
                    nj += vj;
                    // 上下左右が繋がっている
                    if ni < 0 {
                        ni = n as i32 - 1;
                    }
                    if nj < 0 {
                        nj = n as i32 - 1;
                    }
                    if ni >= n as i32 {
                        ni = 0;
                    }
                    if nj >= n as i32 {
                        nj = 0;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    sum.extend(a[ni][nj].to_string().chars());
                }
                ans = ans.max(sum);
            }
        }
    }

    println!("{}", ans);
}
