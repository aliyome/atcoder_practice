use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        // c[i][j] := i から j に書き換えるコスト
        c: [[usize; 10]; 10],
        // a[i][j] := i 行目の j 列目の数字
        // a[i][j] が -1 なら書き換えない
        a: [[isize; w]; h],
    };

    // min_cost[i] := 数字 i を 1 に書き換える最小コスト
    // Floyd-Warshall O(n^3) 10^3
    let mut min_costs = vec![vec![10usize.pow(4); 10]; 10];

    // 初期値の設定
    for i in 0..10 {
        for j in 0..10 {
            min_costs[i][j] = c[i][j];
        }
    }

    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                min_costs[i][j] = min_costs[i][j].min(min_costs[i][k] + min_costs[k][j]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let digit = a[i][j];
            if digit == -1 {
                continue;
            }
            ans += min_costs[digit as usize][1];
        }
    }

    println!("{}", ans);
}
